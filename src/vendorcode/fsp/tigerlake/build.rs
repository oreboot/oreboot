extern crate bindgen;

use std::env;
use std::fs;
use std::path::PathBuf;

fn generate_bindings(oreboot_root: PathBuf) -> std::io::Result<()> {
    let root_path = oreboot_root;
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let include_paths: Vec<PathBuf> = [
        "3rdparty/fsp/TigerLakeFspBinPkg/Include",
        // FSP structs have a number of dependencies on edk2 structs.
        "3rdparty/edk2/IntelFsp2Pkg/Include",
        "3rdparty/edk2/MdePkg/Include",
        "3rdparty/edk2/MdePkg/Include/Ia32",
    ]
    .iter()
    .map(|include| root_path.join(include))
    .collect();
    println!("include paths: {:?}", include_paths);

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // The bindgen::Builder is the main entry point to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header(format!("{}/src/wrapper.h", manifest_dir))
        .clang_args(
            include_paths
                .iter()
                .map(|include| format!("{}{}", "-I", include.display())),
        )
        .clang_args(&["-DEFIAPI=__attribute__((ms_abi))"])
        // Tell cargo to invalidate the built crate whenever any of the included header files
        // changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Use core:: instead of std::
        .use_core()
        .ctypes_prefix("cty")
        // Only generate types and constants.
        .with_codegen_config(bindgen::CodegenConfig::TYPES | bindgen::CodegenConfig::VARS)
        .derive_default(true)
        // Allowlist of types and constants to import.
        // Blacklist types implemented in Rust.
        .allowlist_type("FSP[MST]_UPD")
        .allowlist_type("FSP_MULTI_PHASE_SI_INIT")
        .allowlist_type("FSP_NOTIFY_PHASE")
        .allowlist_type("FSP_[MST]_CONFIG")
        .allowlist_type("FSP_EVENT_HANDLER")
        .allowlist_var("FSP[MST]_UPD_SIGNATURE")
        .allowlist_var("BOOT_.*") // BOOT_MODE consts
        .blocklist_type("GUID")
        .blocklist_type("EFI_GUID")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=src/wrapper.h");

    let oreboot_root = {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let mut path = PathBuf::from(manifest_dir);
        for _ in 0..4 {
            path.pop();
        }
        path
    };
    
    let mut fsp_bin = oreboot_root.clone();
    fsp_bin.push("3rdparty/fsp/CoffeeLakeFspBinPkg/Fsp.fd");

    let mut out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    out_dir.push("FSP.fd");

    fs::copy(fsp_bin, out_dir)?;

    generate_bindings(oreboot_root)?;

    Ok(())
}
