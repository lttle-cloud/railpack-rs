use std::env;
use std::path::PathBuf;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target = format!("{}-{}", target_os, target_arch);

    let lib_dir = match target.as_str() {
        "linux-x86_64" => "prebuilt/linux-amd64/",
        "macos-aarch64" => "prebuilt/darwin-arm64/",
        _ => {
            println!(
                "cargo:warning=No prebuilt library for target {}, falling back to bins/",
                target
            );
            "bins"
        }
    };

    println!("cargo:rustc-link-search=native={}", lib_dir);
    println!("cargo:rustc-link-lib=static=railpack");

    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=m");

    println!("cargo:rerun-if-changed=src/railpack.h");

    let bindings = bindgen::Builder::default()
        .header("src/railpack.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_type("RpConfig")
        .allowlist_type("RpKeyValue")
        .allowlist_type("RpLogEntry")
        .allowlist_type("RpMetadata")
        .allowlist_type("RpBuildResult")
        .allowlist_function("rp_generate_build_plan")
        .allowlist_function("rp_mem_free")
        .derive_debug(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
