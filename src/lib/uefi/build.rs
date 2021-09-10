extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn generate_bindings(oreboot_root: &str) -> std::io::Result<()> {
    let root_path = PathBuf::from(oreboot_root);

    let include_paths: Vec<PathBuf> = ["3rdparty/fspsdk/MdePkg/Include/"]
        .iter()
        .map(|include| root_path.join(include))
        .collect();

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
        .allowlist_type("EFI_COMMON_SECTION_HEADER2?")
        .allowlist_type("EFI_FFS_FILE_HEADER2?")
        .allowlist_type("EFI_FIRMWARE_VOLUME_(EXT_)?HEADER")
        .allowlist_var("EFI_FVB2_ERASE_POLARITY")
        .allowlist_var("EFI_FV_FILETYPE_.*")
        .allowlist_var("EFI_SECTION_RAW")
        .allowlist_var("FFS_ATTRIB_CHECKSUM")
        .allowlist_var("FFS_ATTRIB_LARGE_FILE")
        // Blocklist types implemented in Rust.
        .blocklist_type("GUID")
        .blocklist_type("EFI_GUID")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_dir.join("efi_bindings.rs"))
        .expect("Couldn't write bindings!");

    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=src/wrapper.h");
    generate_bindings("../../..")?;

    Ok(())
}
