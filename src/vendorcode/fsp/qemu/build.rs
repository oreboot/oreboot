extern crate bindgen;

use build_utils::{build_qemu_fsp, FspArchitecture};
use std::env;
use std::path::PathBuf;

fn generate_bindings(oreboot_root: &str) -> std::io::Result<()> {
    let root_path = PathBuf::from(oreboot_root);
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Convert FSP include files from C to Rust.
    let include_paths: Vec<PathBuf> = [
        "3rdparty/fspsdk/Build/QemuFspPkg/DEBUG_GCC5/FV",
        // FSP structs have a number of dependencies on edk2 structs.
        "3rdparty/fspsdk/IntelFsp2Pkg/Include",
        "3rdparty/fspsdk/MdePkg/Include",
        "3rdparty/fspsdk/MdePkg/Include/X64",
    ]
    .iter()
    .map(|include| root_path.join(include))
    .collect();

    // The bindgen::Builder is the main entry point to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header("src/wrapper.h")
        .clang_args(
            include_paths
                .iter()
                .map(|include| format!("{}{}", "-I", include.display())),
        )
        .clang_args(&["-DEFIAPI=__attribute__((ms_abi))"])
        // Tell cargo to invalidate the built crate whenever any of the included header files
        // change.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Use core:: instead of std::
        .use_core()
        .ctypes_prefix("cty")
        // Only generate types and constants.
        .with_codegen_config(bindgen::CodegenConfig::TYPES | bindgen::CodegenConfig::VARS)
        // Allowlist of types and constants to import.
        .allowlist_type("FSP[MST]_UPD")
        .allowlist_type("FSP_MULTI_PHASE_SI_INIT")
        .allowlist_type("FSP_NOTIFY_PHASE")
        .allowlist_type("FSP_[ST]_CONFIG")
        .allowlist_var("FSP[MST]_UPD_SIGNATURE")
        .allowlist_var("BOOT_.*") // BOOT_MODE consts
        .allowlist_type("EFI_HOB_HANDOFF_INFO_TABLE")
        // Blacklist types implemented in Rust.
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

// Build FSP binary and convert FSP include files from C to Rust.
// See https://rust-lang.github.io/rust-bindgen/tutorial-2.html for a bindgen tutorial.
fn main() -> std::io::Result<()> {
    // Tell cargo to invalidate the built crate whenever wrapper.h changes.
    println!("cargo:rerun-if-changed=src/wrapper.h");

    let oreboot_root = "../../../../";
    build_qemu_fsp(&oreboot_root, FspArchitecture::X64)?;
    generate_bindings(&oreboot_root)?;

    Ok(())
}
