extern crate bindgen;

use std::env;
use std::path::PathBuf;

// Convert FSP structs from C to Rust.
// See https://rust-lang.github.io/rust-bindgen/tutorial-2.html for a bindgen tutorial.
fn main() {
    let oreboot_root = PathBuf::from("../../../../");
    let include_paths: Vec<PathBuf> = [
        "3rdparty/fsp/CoffeeLakeFspBinPkg/Include",
        // FSP structs have a number of dependencies on edk2 structs.
        "3rdparty/edk2/IntelFsp2Pkg/Include",
        "3rdparty/edk2/MdePkg/Include",
        "3rdparty/edk2/MdePkg/Include/X64",
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
        // Tell cargo to invalidate the built crate whenever any of the included header files
        // changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Use core:: instead of std::
        .use_core()
        // Only generate types and constants.
        .with_codegen_config(bindgen::CodegenConfig::TYPES | bindgen::CodegenConfig::VARS)
        // Whitelist of types and constants to import.
        .whitelist_type("AUDIO_AZALIA_VERB_TABLE")
        .whitelist_type("AZALIA_HEADER")
        .whitelist_type("CHIPSET_INIT_INFO")
        .whitelist_type("CONTROLLER_INFO")
        .whitelist_type("FIRMWARE_VERSION")
        .whitelist_type("FIRMWARE_VERSION_INFO")
        .whitelist_type("FIRMWARE_VERSION_INFO_HOB")
        .whitelist_type("FSPM_UPD")
        .whitelist_type("FSPS_UPD")
        .whitelist_type("FSPT_CORE_UPD")
        .whitelist_type("FSPT_UPD")
        .whitelist_type("FSP_INFO_HOB")
        .whitelist_type("FSP_M_CONFIG")
        .whitelist_type("FSP_M_TEST_CONFIG")
        .whitelist_type("FSP_S_CONFIG")
        .whitelist_type("FSP_S_TEST_CONFIG")
        .whitelist_type("FSP_T_CONFIG")
        .whitelist_type("GPIO_CONFIG")
        .whitelist_type("GPIO_DIRECTION")
        .whitelist_type("GPIO_ELECTRICAL_CONFIG")
        .whitelist_type("GPIO_GROUP")
        .whitelist_type("GPIO_HARDWARE_DEFAULT")
        .whitelist_type("GPIO_HOSTSW_OWN")
        .whitelist_type("GPIO_INT_CONFIG")
        .whitelist_type("GPIO_LOCK_CONFIG")
        .whitelist_type("GPIO_OTHER_CONFIG")
        .whitelist_type("GPIO_OUTPUT_STATE")
        .whitelist_type("GPIO_PAD")
        .whitelist_type("GPIO_PAD_MODE")
        .whitelist_type("GPIO_RESET_CONFIG")
        .whitelist_type("HOB_USAGE_DATA_HOB")
        .whitelist_type("MEMORY_INFO_DATA_HOB")
        .whitelist_type("MEMORY_PLATFORM_DATA")
        .whitelist_type("MEMORY_PLATFORM_DATA_HOB")
        .whitelist_type("MRC_CH_TIMING")
        .whitelist_type("MRC_TA_TIMING")
        .whitelist_type("SI_PCH_DEVICE_INTERRUPT_CONFIG")
        .whitelist_type("SI_PCH_INT_PIN")
        .whitelist_type("SI_PCH_MAX_DEVICE_INTERRUPT_CONFIG")
        .whitelist_type("SMBIOS_CACHE_INFO")
        .whitelist_type("SMBIOS_PROCESSOR_INFO")
        .whitelist_type("SMBIOS_STRUCTURE")
        .whitelist_type("SiMrcVersion")
        .whitelist_var("B_GPIO_ELECTRICAL_CONFIG_TERMINATION_MASK")
        .whitelist_var("B_GPIO_INT_CONFIG_INT_SOURCE_MASK")
        .whitelist_var("B_GPIO_INT_CONFIG_INT_TYPE_MASK")
        .whitelist_var("B_GPIO_LOCK_CONFIG_OUTPUT_LOCK_MASK")
        .whitelist_var("B_GPIO_LOCK_CONFIG_PAD_CONF_LOCK_MASK")
        .whitelist_var("B_GPIO_OTHER_CONFIG_RXRAW_MASK")
        .whitelist_var("B_RANK0_PRS")
        .whitelist_var("B_RANK1_PRS")
        .whitelist_var("B_RANK2_PRS")
        .whitelist_var("B_RANK3_PRS")
        .whitelist_var("CHANNEL_DISABLED")
        .whitelist_var("CHANNEL_INFO")
        .whitelist_var("CHANNEL_NOT_PRESENT")
        .whitelist_var("CHANNEL_PRESENT")
        .whitelist_var("DIMM_DISABLED")
        .whitelist_var("DIMM_ENABLED")
        .whitelist_var("DIMM_INFO")
        .whitelist_var("DIMM_NOT_PRESENT")
        .whitelist_var("DIMM_PRESENT")
        .whitelist_var("FSPM_UPD_SIGNATURE")
        .whitelist_var("FSPS_UPD_SIGNATURE")
        .whitelist_var("FSPT_UPD_SIGNATURE")
        .whitelist_var("MAX_CH")
        .whitelist_var("MAX_DIMM")
        .whitelist_var("MAX_NODE")
        .whitelist_var("MAX_PROFILE_NUM")
        .whitelist_var("MAX_SPD_SAVE")
        .whitelist_var("MAX_XMP_PROFILE_NUM")
        .whitelist_var("MRC_DDR_TYPE_DDR3")
        .whitelist_var("MRC_DDR_TYPE_DDR4")
        .whitelist_var("MRC_DDR_TYPE_LPDDR3")
        .whitelist_var("MRC_DDR_TYPE_LPDDR4")
        .whitelist_var("MRC_DDR_TYPE_UNKNOWN")
        .whitelist_var("R_MC_CHNL_RANK_PRESENT")
        .whitelist_var("WARM_BOOT")
        .whitelist_var("bmCold")
        .whitelist_var("bmFast")
        .whitelist_var("bmS3")
        .whitelist_var("bmWarm")
        // Blacklist types implemented in Rust.
        .blacklist_type("INT8")
        .blacklist_type("INT16")
        .blacklist_type("INT32")
        .blacklist_type("INT64")
        .blacklist_type("UINT8")
        .blacklist_type("UINT16")
        .blacklist_type("UINT32")
        .blacklist_type("UINT64")
        .blacklist_type("UINTN")
        .blacklist_type("BOOLEAN")
        .blacklist_type("CHAR8")
        .blacklist_type("CHAR16")
        .blacklist_type("GUID")
        .blacklist_type("EFI_GUID")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings!");
}
