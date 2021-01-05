extern crate bindgen;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{exit, Command};

// Build FSP binary and convert FSP include files from C to Rust.
// See https://rust-lang.github.io/rust-bindgen/tutorial-2.html for a bindgen tutorial.
fn main() -> std::io::Result<()> {
    let oreboot_root = PathBuf::from("../../../../");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Build FSP binaries and generate header files.
    let status = Command::new("python3").args(&["BuildFsp.py", "build", "-p", "qemu", "-a", "x64"]).current_dir(oreboot_root.join("3rdparty/fspsdk")).status().expect("failed to build FSP");
    if !status.success() {
        println!("Failed to build FSP");
        exit(1);
    }

    // Copy FSP binaries to output path.
    let edk2_fv_dir = oreboot_root.join("3rdparty/fspsdk/Build/QemuFspPkg/DEBUG_GCC5/FV");
    fs::copy(edk2_fv_dir.join("QEMUFSP.fd"), out_dir.join("QEMUFSP.fd"))?;

    // Convert FSP include files from C to Rust.
    let include_paths: Vec<PathBuf> = [
        "3rdparty/fspsdk/Build/QemuFspPkg/DEBUG_GCC5/FV",
        // FSP structs have a number of dependencies on edk2 structs.
        "3rdparty/fspsdk/IntelFsp2Pkg/Include",
        "3rdparty/fspsdk/MdePkg/Include",
        "3rdparty/fspsdk/MdePkg/Include/X64",
    ]
    .iter()
    .map(|include| oreboot_root.join(include))
    .collect();

    // Tell cargo to invalidate the built crate whenever wrapper.h changes.
    println!("cargo:rerun-if-changed=src/wrapper.h");

    // The bindgen::Builder is the main entry point to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header("src/wrapper.h")
        .clang_args(include_paths.iter().map(|include| format!("{}{}", "-I", include.display())))
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
        .allowlist_type("EFI_COMMON_SECTION_HEADER2?")
        .allowlist_type("EFI_FFS_FILE_HEADER2?")
        .allowlist_type("EFI_FIRMWARE_VOLUME_(EXT_)?HEADER")
        .allowlist_type("FSP[MST]_UPD")
        .allowlist_type("FSP_INFO_HEADER")
        .allowlist_type("FSP_MEMORY_INIT")
        .allowlist_type("FSP_MULTI_PHASE_SI_INIT")
        .allowlist_type("FSP_NOTIFY_PHASE")
        .allowlist_type("FSP_SILICON_INIT")
        .allowlist_type("FSP_TEMP_RAM_EXIT")
        .allowlist_type("FSP_TEMP_RAM_INIT")
        .allowlist_type("FSP_[ST]_CONFIG")
        .allowlist_var("EFI_FVB2_ERASE_POLARITY")
        .allowlist_var("EFI_FV_FILETYPE_.*")
        .allowlist_var("EFI_SECTION_RAW")
        .allowlist_var("FFS_ATTRIB_CHECKSUM")
        .allowlist_var("FFS_ATTRIB_LARGE_FILE")
        .allowlist_var("FSP[MST]_UPD_SIGNATURE")
        .allowlist_var("FSP_STATUS_RESET_REQUIRED_.*")
        .allowlist_var("BOOT_.*") // BOOT_MODE consts
        // Blacklist types implemented in Rust.
        .blocklist_type("GUID")
        .blocklist_type("EFI_GUID")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings.write_to_file(out_dir.join("bindings.rs")).expect("Couldn't write bindings!");

    Ok(())
}
