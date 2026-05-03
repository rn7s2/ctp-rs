use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    let root = env!("CARGO_MANIFEST_DIR");
    let lib_dir = Path::new(root).join("lib");

    // ctp-rs will only ship a market-data archive for macOS — there is
    // no prebuilt traderapi shipped — so localctp is mandatory there.
    let use_localctp = std::env::var("CARGO_FEATURE_LOCALCTP").is_ok() || cfg!(target_os = "macos");

    println!("cargo:rustc-link-search={}", lib_dir.display());
    if cfg!(target_os = "macos") {
        let darwin_lib_dir = lib_dir.join("darwin");
        println!("cargo:rustc-link-search={}", darwin_lib_dir.display());
        // mdapi: universal static archive shipped under lib/darwin/.
        println!("cargo:rustc-link-lib=static=thostmduserapi_se");
        // traderapi: static archive built from localctp; lives in OUT_DIR.
        println!(
            "cargo:rustc-link-search={}",
            std::env::var("OUT_DIR").unwrap()
        );
        println!("cargo:rustc-link-lib=static=thosttraderapi_se");
        // Converter.cpp uses iconv on Unix; macOS ships libiconv separately.
        println!("cargo:rustc-link-lib=iconv");
    } else {
        println!("cargo:rustc-link-lib=thostmduserapi_se");
        println!("cargo:rustc-link-lib=thosttraderapi_se");
    }

    let mut cpp_files = vec![
        "wrapper/src/MdApi.cpp",
        "wrapper/src/TraderApi.cpp",
        "wrapper/src/CMdSpi.cpp",
        "wrapper/src/CTraderSpi.cpp",
        "wrapper/src/Converter.cpp",
    ];
    if cfg!(target_os = "macos") {
        // Bridges the linux header's 4-arg CreateFtdcMdApi to the darwin
        // archive's 3-arg version; see the file for details.
        cpp_files.push("wrapper/src/MdApiDarwinShim.cpp");
    }

    let rust_files = vec!["src/lib.rs"];
    let wrapper_files = vec![
        "wrapper/include/Converter.h",
        "wrapper/include/CMdSpi.h",
        "wrapper/include/CTraderSpi.h",
        "wrapper/include/MdApi.h",
        "wrapper/include/TraderApi.h",
        "wrapper/src/Converter.cpp",
        "wrapper/src/CMdSpi.cpp",
        "wrapper/src/CTraderSpi.cpp",
        "wrapper/src/MdApi.cpp",
        "wrapper/src/TraderApi.cpp",
        "wrapper/src/MdApiDarwinShim.cpp",
    ];

    cxx_build::bridges(rust_files)
        .define("CXX_RS", None)
        .flag_if_supported("/EHsc")
        .flag_if_supported("/std:c++20")
        .flag_if_supported("/utf-8")
        .flag_if_supported("/w")
        .flag_if_supported("-std=c++20")
        .flag_if_supported("-w")
        .files(cpp_files)
        .compile("ctp_rs");

    println!("cargo:rerun-if-changed=src/lib.rs");
    for file in wrapper_files.iter() {
        println!("cargo:rerun-if-changed={}", file);
    }

    // copy DLL/SO to out dir
    let out_dir = {
        let mut path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
        _ = path.pop() && path.pop() && path.pop();
        path
    };

    // Md library: copy the dynamic lib to out_dir on win/linux for runtime
    // loading. macOS ships mdapi as a static archive, nothing to copy.
    if cfg!(target_os = "windows") {
        let md = "thostmduserapi_se.dll";
        fs::copy(lib_dir.join(md), out_dir.join(md))
            .unwrap_or_else(|e| panic!("Copy {} failed: {}", md, e));
    } else if cfg!(target_os = "linux") {
        let md = "libthostmduserapi_se.so";
        fs::copy(lib_dir.join(md), out_dir.join(md))
            .unwrap_or_else(|e| panic!("Copy {} failed: {}", md, e));
    }

    // Trader library:
    //   * win/linux without localctp: copy the prebuilt dynamic lib from lib/.
    //   * win/linux with localctp: build, then copy the result to out_dir.
    //   * macOS: build a static archive from localctp; the linker picks it up
    //     via cargo:rustc-link-search=OUT_DIR above. No runtime copy needed.
    if use_localctp {
        println!("cargo:rerun-if-changed=localctp");
        let built = build_localctp(&Path::new(root).join("localctp"));
        if !cfg!(target_os = "macos") {
            let trader = if cfg!(target_os = "windows") {
                "thosttraderapi_se.dll"
            } else {
                "libthosttraderapi_se.so"
            };
            fs::copy(&built, out_dir.join(trader))
                .unwrap_or_else(|e| panic!("Copy {} failed: {}", trader, e));
        }
    } else {
        let trader = if cfg!(target_os = "windows") {
            "thosttraderapi_se.dll"
        } else {
            "libthosttraderapi_se.so"
        };
        fs::copy(lib_dir.join(trader), out_dir.join(trader))
            .unwrap_or_else(|e| panic!("Copy {} failed: {}", trader, e));
    }
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
    let c_sources = ["sqlite/sqlite3.c", "sqlite/shell.c"];

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
