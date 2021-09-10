extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn generate_bindings(oreboot_root: &str) -> std::io::Result<()> {
    let root_path = PathBuf::from(oreboot_root);

    let include_paths: Vec<PathBuf> = ["3rdparty/fspsdk/IntelFsp2Pkg/Include"]
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
        .allowlist_type("FSP_INFO_HEADER")
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
        .write_to_file(out_dir.join("fsp_bindings.rs"))
        .expect("Couldn't write bindings!");

    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=src/wrapper.h");
    generate_bindings("../../../..")?;

    Ok(())
}
