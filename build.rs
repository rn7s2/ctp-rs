use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

// CTP native libraries (headers + framework dylibs + Win .dll/.lib + Linux .so)
// are too large for crates.io's compressed package limit, so they're hosted on
// R2 and fetched on first build instead of being shipped inside the crate.
// The zip is extracted into `$OUT_DIR/lib/` (build.rs must not write to the
// source tree — `cargo publish` rejects that). The zip's top-level layout is
// what the rest of build.rs expects: darwin/*.framework, lib*.so, *.dll/.lib,
// headers, error.xml.
const LIB_ASSET_URL: &str = "https://ctp-api.ruiqilei.com/ctp.6.7.11.darwin.6.7.7.zip";
const LIB_ASSET_VERSION: &str = "ctp.6.7.11.darwin.6.7.7";

fn main() {
    // docs.rs builds in a sandbox without network access, so the native-lib
    // download in `ensure_lib_dir` would fail. Rustdoc only type-checks the
    // crate (cxx::bridge expands on the Rust side; no link step), so we can
    // skip the download + C++ build + lib copies entirely.
    if std::env::var_os("DOCS_RS").is_some() {
        return;
    }

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    // Native libs land under OUT_DIR (not the source tree) so `cargo publish`
    // verification doesn't reject this build script for modifying source.
    let lib_dir = out_dir.join("lib");

    ensure_lib_dir(&lib_dir);

    println!("cargo:rustc-link-search={}", lib_dir.display());
    if cfg!(target_os = "macos") {
        // mdapi and traderapi are both embedded from lib/darwin/*.framework
        // and dlopen()'d at runtime. Nothing to link against.
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

        let md_blob_cpp =
            embed_macos_framework_dylib(&lib_dir, &out_dir, "thostmduserapi_se", "md");
        cpp_files.push(md_blob_cpp);

        // trader shim: the trader-side library is the embedded framework dylib,
        // dlopen()'d at runtime, mirroring the mdapi case.
        cpp_files.push("wrapper/src/TraderApiDarwinShim.cpp".into());
        let trader_blob_cpp =
            embed_macos_framework_dylib(&lib_dir, &out_dir, "thosttraderapi_se", "trader");
        cpp_files.push(trader_blob_cpp);
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
    // CTP headers live alongside the prebuilt libraries in `lib_dir` (under
    // OUT_DIR). Wrapper headers include them as plain `<ThostFtdc*.h>`.
    build.include(&lib_dir);
    build
        .define("CXX_RS", None)
        .flag_if_supported("/EHsc")
        .flag_if_supported("/std:c++20")
        .flag_if_supported("/utf-8")
        .flag_if_supported("/w")
        .flag_if_supported("-std=c++20")
        .flag_if_supported("-w");
    if cfg!(target_os = "macos") {
        // Routes CreateFtdcTraderApi/GetApiVersion through the runtime shim
        // (TraderApiDarwinShim.cpp) instead of the static class methods, which
        // aren't link-resolvable when the dylib is loaded dynamically.
        build.define("CTP_RS_DARWIN_TRADER_DLOPEN", None);
    }
    build.files(cpp_files).compile("ctp_rs");

    println!("cargo:rerun-if-changed=src/lib.rs");
    for file in wrapper_files.iter() {
        println!("cargo:rerun-if-changed={}", file);
    }

    // Per-profile target directory (e.g. target/debug/) — where executables
    // land and where Win/Linux dyloaders look for sibling .dll/.so. macOS
    // doesn't need a runtime copy: both md and trader are embedded into the
    // binary, so it's fully self-contained.
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
    //   * win/linux: copy the prebuilt dynamic lib from lib/ next to the bin.
    //   * macOS: trader framework dylib is embedded, no copy.
    if !cfg!(target_os = "macos") {
        let trader = if cfg!(target_os = "windows") {
            "thosttraderapi_se.dll"
        } else {
            "libthosttraderapi_se.so"
        };
        fs::copy(lib_dir.join(trader), target_dir.join(trader))
            .unwrap_or_else(|e| panic!("Copy {} failed: {}", trader, e));
    }
}

/// Downloads and extracts the CTP native-library bundle into `lib_dir`
/// (which lives under `$OUT_DIR`) if it isn't already there. The bundle is
/// hosted on R2 (see `LIB_ASSET_URL`) and unzips into the shape the rest of
/// build.rs expects: `darwin/*.framework`, `lib*.so`, `*.dll`/`*.lib`,
/// top-level CTP headers, and `error.xml`.
///
/// `OUT_DIR` is per-build (target/profile/build-script hash), and any change
/// to `LIB_ASSET_URL` re-hashes build.rs and yields a fresh `OUT_DIR`. So if
/// `lib_dir` already exists here it must be the bundle this build wants —
/// no version check needed. Extraction happens into a sibling `.partial`
/// dir and is renamed in atomically, so a partial extraction can never be
/// mistaken for a complete one.
///
/// Uses `curl` + `unzip` (Unix) / `tar` (Windows) rather than Rust HTTP/zip
/// crates: those add multi-megabyte build-time deps and don't preserve the
/// framework's symlink layout that codesign/dlopen rely on. The system tools
/// are universally available on supported platforms — see README "Build
/// requirements".
fn ensure_lib_dir(lib_dir: &Path) {
    if lib_dir.exists() {
        return;
    }

    let staging = lib_dir.with_file_name("lib.partial");
    if staging.exists() {
        fs::remove_dir_all(&staging)
            .unwrap_or_else(|e| panic!("failed to clear {}: {}", staging.display(), e));
    }
    fs::create_dir_all(&staging)
        .unwrap_or_else(|e| panic!("failed to create {}: {}", staging.display(), e));

    let zip_path = staging.join(".ctp-rs-asset.zip");
    // Not `cargo:warning=` — cargo caches those in the build script's `output`
    // file and replays them on every subsequent build of this package, even
    // when build.rs doesn't re-run, so a one-time download would otherwise
    // produce a permanent warning. `eprintln!` goes to captured stderr (shown
    // with `cargo build -vv` or on failure) and isn't replayed.
    eprintln!(
        "ctp-rs: downloading native libraries ({}) — this happens once per OUT_DIR",
        LIB_ASSET_VERSION
    );
    let status = Command::new("curl")
        .arg("-fsSL")
        .arg("--retry")
        .arg("3")
        .arg("-o")
        .arg(&zip_path)
        .arg(LIB_ASSET_URL)
        .status()
        .expect("`curl` not found — required to fetch CTP native libraries (see README)");
    assert!(
        status.success(),
        "failed to download CTP native libraries from {}",
        LIB_ASSET_URL
    );

    // Extraction: prefer `unzip` on Unix (ships with macOS, available in
    // every mainstream Linux distro's base repos). On Windows fall back to
    // `tar -xf`, which ships with Windows 10 1803+ and handles zip via
    // libarchive. Both preserve the framework's internal symlinks, which
    // codesign relies on at runtime.
    let extract_status = if cfg!(target_os = "windows") {
        Command::new("tar")
            .arg("-xf")
            .arg(&zip_path)
            .arg("-C")
            .arg(&staging)
            .status()
            .expect("`tar` not found — required to extract on Windows (see README)")
    } else {
        Command::new("unzip")
            .arg("-q")
            .arg("-o")
            .arg(&zip_path)
            .arg("-d")
            .arg(&staging)
            .status()
            .expect("`unzip` not found — required to extract CTP libraries (see README)")
    };
    assert!(
        extract_status.success(),
        "failed to extract CTP native libraries into {}",
        staging.display()
    );

    // __MACOSX/ holds AppleDouble resource forks that the macOS Finder
    // injects when zipping; noise on every platform, just clutters the dir.
    let macosx_junk = staging.join("__MACOSX");
    if macosx_junk.exists() {
        let _ = fs::remove_dir_all(&macosx_junk);
    }
    let _ = fs::remove_file(&zip_path);

    // Atomic publish: lib_dir only ever exists when extraction succeeded.
    fs::rename(&staging, lib_dir).unwrap_or_else(|e| {
        panic!(
            "failed to rename {} → {}: {}",
            staging.display(),
            lib_dir.display(),
            e
        )
    });
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
        "missing macOS framework dylib at {} — the auto-downloaded CTP bundle \
         appears incomplete; run `cargo clean` and rebuild to re-fetch",
        src_dylib.display(),
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
