use std::{
    fs,
    path::{Path, PathBuf},
};

fn main() {
    let root = env!("CARGO_MANIFEST_DIR");
    let lib_dir = Path::new(&root).join("lib");

    println!("cargo:rustc-link-search={}", lib_dir.display());
    println!("cargo:rustc-link-lib=thostmduserapi_se");
    println!("cargo:rustc-link-lib=thosttraderapi_se");

    // C++ interop
    let cpp_files = vec![
        "wrapper/src/MdApi.cpp",
        "wrapper/src/TraderApi.cpp",
        "wrapper/src/CMdSpi.cpp",
        "wrapper/src/CTraderSpi.cpp",
        "wrapper/src/Converter.cpp",
    ];
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
    ];

    cxx_build::bridges(rust_files)
        .define("CXX_RS", None)
        .flag_if_supported("/EHsc")
        .flag_if_supported("/std:c++20")
        .flag_if_supported("/w")
        .flag_if_supported("-std=c++20")
        .flag_if_supported("-w")
        .files(cpp_files)
        .compile("ctp_rs");

    println!("cargo:rerun-if-changed=src/lib.rs");
    for file in wrapper_files.iter() {
        println!("cargo:rerun-if-changed={}", file);
    }

    // copy DLL to out dir
    let out_dir = {
        let mut path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
        _ = path.pop() && path.pop() && path.pop();
        path
    };

    let files = {
        if cfg!(target_os = "windows") {
            vec!["thostmduserapi_se.dll", "thosttraderapi_se.dll"]
        } else {
            vec!["libthostmduserapi_se.so", "libthosttraderapi_se.so"]
        }
    };
    for file in files {
        fs::copy(lib_dir.join(file), out_dir.join(file)).expect(&format!("Copy {} failed", file));
    }
}
