use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    let root = env!("CARGO_MANIFEST_DIR");
    let lib_dir = Path::new(root).join("lib");
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let use_localctp = std::env::var("CARGO_FEATURE_LOCALCTP").is_ok();

    // True on macOS when the trader-side library is the embedded framework
    // dylib (i.e. the default, non-localctp build). The cxx wrapper checks
    // CTP_RS_DARWIN_TRADER_DLOPEN to route CreateFtdcTraderApi/GetApiVersion
    // through the runtime shim instead of calling the static class methods,
    // which aren't link-resolvable when the dylib is loaded dynamically.
    let darwin_trader_dlopen = cfg!(target_os = "macos") && !use_localctp;

    println!("cargo:rustc-link-search={}", lib_dir.display());
    if cfg!(target_os = "macos") {
        // mdapi: always embedded from lib/darwin/thostmduserapi_se.framework
        // and dlopen()'d at runtime. Nothing to link against.
        // traderapi: either localctp's static archive (use_localctp=true) or
        // the embedded framework dylib (use_localctp=false), wired up below.
        if use_localctp {
            println!("cargo:rustc-link-search={}", out_dir.display());
            println!("cargo:rustc-link-lib=static=thosttraderapi_se");
        }
        // Converter.cpp uses iconv on Unix; macOS ships libiconv separately.
        println!("cargo:rustc-link-lib=iconv");
        // Used by the framework dylibs (loaded at runtime).
        println!("cargo:rustc-link-lib=dylib=c++");
    } else {
        println!("cargo:rustc-link-lib=thostmduserapi_se");
        println!("cargo:rustc-link-lib=thosttraderapi_se");
    }

    let mut cpp_files: Vec<String> = vec![
        "wrapper/src/MdApi.cpp".into(),
        "wrapper/src/TraderApi.cpp".into(),
        "wrapper/src/CMdSpi.cpp".into(),
        "wrapper/src/CTraderSpi.cpp".into(),
        "wrapper/src/Converter.cpp".into(),
    ];
    if cfg!(target_os = "macos") {
        // The DarwinDylibLoader helper is shared by the md and trader shims.
        cpp_files.push("wrapper/src/DarwinDylibLoader.cpp".into());
        // mdapi shim: bridges the linux header's 4-arg CreateFtdcMdApi to
        // the darwin dylib's 3-arg version + dlopen()s the embedded blob.
        cpp_files.push("wrapper/src/MdApiDarwinShim.cpp".into());

        let md_blob_cpp = embed_macos_framework_dylib(
            &lib_dir,
            &out_dir,
            "thostmduserapi_se",
            "md",
        );
        cpp_files.push(md_blob_cpp);

        if darwin_trader_dlopen {
            cpp_files.push("wrapper/src/TraderApiDarwinShim.cpp".into());
            let trader_blob_cpp = embed_macos_framework_dylib(
                &lib_dir,
                &out_dir,
                "thosttraderapi_se",
                "trader",
            );
            cpp_files.push(trader_blob_cpp);
        }
    }

    let rust_files = vec!["src/lib.rs"];
    let wrapper_files = vec![
        "wrapper/include/Converter.h",
        "wrapper/include/CMdSpi.h",
        "wrapper/include/CTraderSpi.h",
        "wrapper/include/DarwinDylibLoader.h",
        "wrapper/include/MdApi.h",
        "wrapper/include/TraderApi.h",
        "wrapper/src/Converter.cpp",
        "wrapper/src/CMdSpi.cpp",
        "wrapper/src/CTraderSpi.cpp",
        "wrapper/src/DarwinDylibLoader.cpp",
        "wrapper/src/MdApi.cpp",
        "wrapper/src/MdApiDarwinShim.cpp",
        "wrapper/src/TraderApi.cpp",
        "wrapper/src/TraderApiDarwinShim.cpp",
    ];

    let mut build = cxx_build::bridges(rust_files);
    build
        .define("CXX_RS", None)
        .flag_if_supported("/EHsc")
        .flag_if_supported("/std:c++20")
        .flag_if_supported("/utf-8")
        .flag_if_supported("/w")
        .flag_if_supported("-std=c++20")
        .flag_if_supported("-w");
    if darwin_trader_dlopen {
        build.define("CTP_RS_DARWIN_TRADER_DLOPEN", None);
    }
    if use_localctp {
        // Toggles wrapper/src/TraderApi.cpp's call to localctp_wait_until_ready(),
        // which blocks CreateTraderApi() until LocalCTP's first-pass settlement
        // init has completed (avoids a startup race with the SPI callback path).
        build.define("CTP_RS_LOCALCTP", None);
    }
    build.files(cpp_files).compile("ctp_rs");

    println!("cargo:rerun-if-changed=src/lib.rs");
    for file in wrapper_files.iter() {
        println!("cargo:rerun-if-changed={}", file);
    }

    // Per-profile target directory (e.g. target/debug/) — where executables
    // land and where Win/Linux dyloaders look for sibling .dll/.so. macOS
    // doesn't need a runtime copy: both md and trader are either embedded
    // (default) or statically linked (localctp), so the binary is fully
    // self-contained.
    let target_dir = {
        let mut path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
        _ = path.pop() && path.pop() && path.pop();
        path
    };

    // Md library: copy the dynamic lib next to the binary on win/linux.
    // macOS embeds the framework dylib into the binary itself.
    if cfg!(target_os = "windows") {
        let md = "thostmduserapi_se.dll";
        fs::copy(lib_dir.join(md), target_dir.join(md))
            .unwrap_or_else(|e| panic!("Copy {} failed: {}", md, e));
    } else if cfg!(target_os = "linux") {
        let md = "libthostmduserapi_se.so";
        fs::copy(lib_dir.join(md), target_dir.join(md))
            .unwrap_or_else(|e| panic!("Copy {} failed: {}", md, e));
    }

    // Trader library:
    //   * win/linux without localctp: copy the prebuilt dynamic lib from lib/.
    //   * win/linux with localctp: build, then copy the result next to the bin.
    //   * macOS with localctp: localctp is built as a static archive that the
    //     linker pulls in via cargo:rustc-link-lib=static=. No runtime copy.
    //   * macOS without localctp: trader framework dylib is embedded, no copy.
    if use_localctp {
        println!("cargo:rerun-if-changed=localctp");
        let built = build_localctp(&Path::new(root).join("localctp"));
        if !cfg!(target_os = "macos") {
            let trader = if cfg!(target_os = "windows") {
                "thosttraderapi_se.dll"
            } else {
                "libthosttraderapi_se.so"
            };
            fs::copy(&built, target_dir.join(trader))
                .unwrap_or_else(|e| panic!("Copy {} failed: {}", trader, e));
        }
    } else if !cfg!(target_os = "macos") {
        let trader = if cfg!(target_os = "windows") {
            "thosttraderapi_se.dll"
        } else {
            "libthosttraderapi_se.so"
        };
        fs::copy(lib_dir.join(trader), target_dir.join(trader))
            .unwrap_or_else(|e| panic!("Copy {} failed: {}", trader, e));
    }
}

/// Re-signs and embeds a CTP framework dylib so it can be `dlopen`'d at
/// runtime out of /tmp. Returns the path to a generated .cpp under OUT_DIR
/// that uses `.incbin` to splice the re-signed bytes into __DATA,__const,
/// with `_ctp_rs_<sym_prefix>_dylib_{start,end}` markers.
///
/// The shipped frameworks are signed with an "Apple Development" certificate
/// which Gatekeeper rejects for `dlopen` from arbitrary paths. We make a
/// private OUT_DIR copy, strip the existing signature, and ad-hoc re-sign;
/// the re-signed bytes are what end up in the binary, so end users never
/// run `codesign` themselves.
fn embed_macos_framework_dylib(
    lib_dir: &Path,
    out_dir: &Path,
    framework_name: &str, // e.g. "thostmduserapi_se" — both fw dirname and binary name
    sym_prefix: &str,     // e.g. "md" → _ctp_rs_md_dylib_{start,end}
) -> String {
    let src_dylib = lib_dir
        .join("darwin")
        .join(format!("{}.framework", framework_name))
        .join("Versions")
        .join("A")
        .join(framework_name);
    assert!(
        src_dylib.exists(),
        "missing macOS framework dylib at {} — drop {}.framework into \
         lib/darwin/, or build with --features localctp to use the embedded \
         simulator instead",
        src_dylib.display(),
        framework_name,
    );

    let signed_dylib = out_dir.join(format!("{}_adhoc.dylib", framework_name));
    fs::copy(&src_dylib, &signed_dylib)
        .unwrap_or_else(|e| panic!("copy {}: {}", framework_name, e));
    // Strip the quarantine xattr that arrived with the framework download +
    // any other extended attributes — codesign refuses otherwise.
    let _ = Command::new("xattr").arg("-c").arg(&signed_dylib).status();
    // Best-effort signature removal; no-op-with-error if there isn't one.
    let _ = Command::new("codesign")
        .arg("--remove-signature")
        .arg(&signed_dylib)
        .status();
    let status = Command::new("codesign")
        .arg("--force")
        .arg("--sign")
        .arg("-")
        .arg(&signed_dylib)
        .status()
        .expect("codesign not found — required to re-sign CTP dylibs on macOS");
    assert!(
        status.success(),
        "codesign --sign - failed for {}",
        signed_dylib.display()
    );

    // .incbin path must be absolute — clang's integrated assembler resolves
    // it relative to its CWD, which we don't control here.
    let dylib_abs = signed_dylib.canonicalize().unwrap();
    let blob_cpp = out_dir.join(format!("{}_dylib_blob.cpp", sym_prefix));
    let blob_src = format!(
        "// Auto-generated by build.rs — do not edit.\n\
         __asm__(\n\
         \"    .section __DATA,__const\\n\"\n\
         \"    .p2align 4\\n\"\n\
         \"    .globl _ctp_rs_{prefix}_dylib_start\\n\"\n\
         \"_ctp_rs_{prefix}_dylib_start:\\n\"\n\
         \"    .incbin \\\"{path}\\\"\\n\"\n\
         \"    .globl _ctp_rs_{prefix}_dylib_end\\n\"\n\
         \"_ctp_rs_{prefix}_dylib_end:\\n\"\n\
         );\n",
        prefix = sym_prefix,
        path = dylib_abs.display()
    );
    fs::write(&blob_cpp, blob_src).unwrap();
    println!("cargo:rerun-if-changed={}", src_dylib.display());
    blob_cpp.to_string_lossy().into_owned()
}

fn build_localctp(localctp_dir: &Path) -> PathBuf {
    let build_out = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    // All C++ sources — dllmain.cpp is guarded by #ifdef _WIN32 inside, safe on Linux/macOS
    let cpp_sources = [
        "LocalCTP.cpp",
        "LocalTraderApi.cpp",
        "CSqliteHandler.cpp",
        "LeeDateTime.cpp",
        "dllmain.cpp",
        "Properties.cpp",
        "stdafx.cpp",
        "auto_generated_code/CTPSQLWrapper.cpp",
    ];
    // SQLite must be compiled as C, not C++
    let c_sources = ["sqlite/sqlite3.c"];

    if cfg!(target_os = "windows") {
        let output = build_out.join("thosttraderapi_se.dll");
        let obj_dir = build_out.join("localctp_obj");
        fs::create_dir_all(&obj_dir).unwrap();

        // Use cc to locate cl.exe and set up the MSVC environment (PATH, INCLUDE, LIB)
        let compiler = cc::Build::new().cpp(true).get_compiler();
        let mut cmd = Command::new(compiler.path());
        for (k, v) in compiler.env() {
            cmd.env(k, v);
        }

        // Release|x64 settings from LocalCTP.vcxproj
        cmd.arg("/nologo")
            .arg("/LD") // build DLL
            .arg(format!("/Fe:{}", output.display()))
            .arg(format!("/Fo{}\\", obj_dir.display()))
            .arg("/EHsc")
            .arg("/O2")
            .arg("/Gy") // function-level linking
            .arg("/Oi") // intrinsic functions
            .arg("/MT") // MultiThreaded static runtime (vcxproj Release|x64)
            .arg("/source-charset:.936") // localctp sources are GBK/CP936
            .arg("/std:c++20")
            .arg("/w");

        for def in &[
            "NDEBUG",
            "LOCALCTP_EXPORTS",
            "_WINDOWS",
            "_USRDLL",
            "_CRT_SECURE_NO_WARNINGS",
            "_SILENCE_CXX17_CODECVT_HEADER_DEPRECATION_WARNING",
        ] {
            cmd.arg(format!("/D{}", def));
        }

        cmd.arg(format!(
            "/I{}",
            localctp_dir.join("auto_generated_code").display()
        ))
        .arg(format!(
            "/I{}",
            localctp_dir.parent().unwrap().join("lib").display()
        ));

        for src in &cpp_sources {
            cmd.arg(localctp_dir.join(src));
        }
        for src in &c_sources {
            cmd.arg(localctp_dir.join(src));
        }

        // Run from build_out so the generated .lib/.exp land there, not in the source tree
        cmd.current_dir(&build_out);

        let status = cmd
            .status()
            .expect("cl.exe not found — install Visual Studio Build Tools");
        assert!(status.success(), "localctp MSVC build failed");

        output
    } else {
        let is_macos = cfg!(target_os = "macos");
        let cxx = if is_macos { "c++" } else { "g++" };
        let cc = if is_macos { "cc" } else { "gcc" };
        let obj_dir = build_out.join("localctp_obj");
        fs::create_dir_all(&obj_dir).unwrap();

        let includes = [
            format!("-I{}", localctp_dir.join("auto_generated_code").display()),
            format!("-I{}", localctp_dir.parent().unwrap().join("lib").display()),
        ];

        // Step 1: compile C++ sources (makefile CFLAGS: -std=c++11 -Wall -Wno-format-security -fPIC)
        let mut cpp_objs: Vec<PathBuf> = Vec::new();
        for src in &cpp_sources {
            let obj = obj_dir
                .join(Path::new(src).file_stem().unwrap())
                .with_extension("o");
            let mut cmd = Command::new(cxx);
            cmd.arg("-c")
                .arg("-fPIC")
                .arg("-std=c++11")
                .arg("-Wall")
                .arg("-Wno-format-security")
                .arg("-DLOCALCTP_EXPORTS")
                .args(&includes)
                .arg("-o")
                .arg(&obj)
                .arg(localctp_dir.join(src));
            let status = cmd.status().unwrap_or_else(|_| panic!("{} not found", cxx));
            assert!(status.success(), "localctp: failed to compile {}", src);
            cpp_objs.push(obj);
        }

        // Step 2: compile C sources (makefile CFLAGS2: -Wall -fPIC, no -std=c++11)
        let mut c_objs: Vec<PathBuf> = Vec::new();
        for src in &c_sources {
            let obj = obj_dir
                .join(Path::new(src).file_stem().unwrap())
                .with_extension("o");
            let mut cmd = Command::new(cc);
            cmd.arg("-c")
                .arg("-fPIC")
                .arg("-Wall")
                .arg("-o")
                .arg(&obj)
                .arg(localctp_dir.join(src));
            let status = cmd.status().unwrap_or_else(|_| panic!("{} not found", cc));
            assert!(status.success(), "localctp: failed to compile {}", src);
            c_objs.push(obj);
        }

        if is_macos {
            // Pack into a static archive. cargo will link it into the final
            // binary via cargo:rustc-link-lib=static=thosttraderapi_se. dlopen
            // and pthread are part of libSystem on macOS, no extra link flags
            // are needed at the binary level.
            let output = build_out.join("libthosttraderapi_se.a");
            let _ = fs::remove_file(&output);
            let mut cmd = Command::new("ar");
            cmd.arg("rcs").arg(&output);
            cmd.args(&cpp_objs).args(&c_objs);
            let status = cmd.status().expect("ar not found");
            assert!(status.success(), "localctp: archiving failed");
            output
        } else {
            // Linux: link a shared object (makefile: g++ $^ -o $@ -pthread -ldl -shared)
            let output = build_out.join("libthosttraderapi_se.so");
            let mut cmd = Command::new(cxx);
            cmd.arg("-shared").arg("-o").arg(&output);
            cmd.args(&cpp_objs).args(&c_objs);
            cmd.arg("-pthread").arg("-ldl");
            let status = cmd.status().unwrap_or_else(|_| panic!("{} not found", cxx));
            assert!(status.success(), "localctp: linking failed");
            output
        }
    }
}
