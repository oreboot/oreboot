extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn generate_bindings(oreboot_root: &str) {
    let root_path = PathBuf::from(oreboot_root);
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let include_paths: Vec<PathBuf> = [
        "3rdparty/fsp/CoffeeLakeFspBinPkg/Include",
        // FSP structs have a number of dependencies on edk2 structs.
        "3rdparty/edk2/IntelFsp2Pkg/Include",
        "3rdparty/edk2/MdePkg/Include",
        "3rdparty/edk2/MdePkg/Include/X64",
    ]
    .iter()
    .map(|include| root_path.join(include))
    .collect();

    // Tell cargo to invalidate the built crate whenever wrapper.h changes.
    println!("cargo:rerun-if-changed=src/wrapper.h");

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
        // Tell cargo to invalidate the built crate whenever any of the included header files
        // changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Use core:: instead of std::
        .use_core()
        // Only generate types and constants.
        .with_codegen_config(bindgen::CodegenConfig::TYPES | bindgen::CodegenConfig::VARS)
        // Allowlist of types and constants to import.
        .allowlist_type("AUDIO_AZALIA_VERB_TABLE")
        .allowlist_type("AZALIA_HEADER")
        .allowlist_type("CHIPSET_INIT_INFO")
        .allowlist_type("CONTROLLER_INFO")
        .allowlist_type("FIRMWARE_VERSION")
        .allowlist_type("FIRMWARE_VERSION_INFO")
        .allowlist_type("FIRMWARE_VERSION_INFO_HOB")
        .allowlist_type("FSPM_UPD")
        .allowlist_type("FSPS_UPD")
        .allowlist_type("FSPT_CORE_UPD")
        .allowlist_type("FSPT_UPD")
        .allowlist_type("FSP_INFO_HOB")
        .allowlist_type("FSP_M_CONFIG")
        .allowlist_type("FSP_M_TEST_CONFIG")
        .allowlist_type("FSP_S_CONFIG")
        .allowlist_type("FSP_S_TEST_CONFIG")
        .allowlist_type("FSP_T_CONFIG")
        .allowlist_type("GPIO_CONFIG")
        .allowlist_type("GPIO_DIRECTION")
        .allowlist_type("GPIO_ELECTRICAL_CONFIG")
        .allowlist_type("GPIO_GROUP")
        .allowlist_type("GPIO_HARDWARE_DEFAULT")
        .allowlist_type("GPIO_HOSTSW_OWN")
        .allowlist_type("GPIO_INT_CONFIG")
        .allowlist_type("GPIO_LOCK_CONFIG")
        .allowlist_type("GPIO_OTHER_CONFIG")
        .allowlist_type("GPIO_OUTPUT_STATE")
        .allowlist_type("GPIO_PAD")
        .allowlist_type("GPIO_PAD_MODE")
        .allowlist_type("GPIO_RESET_CONFIG")
        .allowlist_type("HOB_USAGE_DATA_HOB")
        .allowlist_type("MEMORY_INFO_DATA_HOB")
        .allowlist_type("MEMORY_PLATFORM_DATA")
        .allowlist_type("MEMORY_PLATFORM_DATA_HOB")
        .allowlist_type("MRC_CH_TIMING")
        .allowlist_type("MRC_TA_TIMING")
        .allowlist_type("SI_PCH_DEVICE_INTERRUPT_CONFIG")
        .allowlist_type("SI_PCH_INT_PIN")
        .allowlist_type("SI_PCH_MAX_DEVICE_INTERRUPT_CONFIG")
        .allowlist_type("SMBIOS_CACHE_INFO")
        .allowlist_type("SMBIOS_PROCESSOR_INFO")
        .allowlist_type("SMBIOS_STRUCTURE")
        .allowlist_type("SiMrcVersion")
        .allowlist_var("B_GPIO_ELECTRICAL_CONFIG_TERMINATION_MASK")
        .allowlist_var("B_GPIO_INT_CONFIG_INT_SOURCE_MASK")
        .allowlist_var("B_GPIO_INT_CONFIG_INT_TYPE_MASK")
        .allowlist_var("B_GPIO_LOCK_CONFIG_OUTPUT_LOCK_MASK")
        .allowlist_var("B_GPIO_LOCK_CONFIG_PAD_CONF_LOCK_MASK")
        .allowlist_var("B_GPIO_OTHER_CONFIG_RXRAW_MASK")
        .allowlist_var("B_RANK0_PRS")
        .allowlist_var("B_RANK1_PRS")
        .allowlist_var("B_RANK2_PRS")
        .allowlist_var("B_RANK3_PRS")
        .allowlist_var("CHANNEL_DISABLED")
        .allowlist_var("CHANNEL_INFO")
        .allowlist_var("CHANNEL_NOT_PRESENT")
        .allowlist_var("CHANNEL_PRESENT")
        .allowlist_var("DIMM_DISABLED")
        .allowlist_var("DIMM_ENABLED")
        .allowlist_var("DIMM_INFO")
        .allowlist_var("DIMM_NOT_PRESENT")
        .allowlist_var("DIMM_PRESENT")
        .allowlist_var("FSPM_UPD_SIGNATURE")
        .allowlist_var("FSPS_UPD_SIGNATURE")
        .allowlist_var("FSPT_UPD_SIGNATURE")
        .allowlist_var("MAX_CH")
        .allowlist_var("MAX_DIMM")
        .allowlist_var("MAX_NODE")
        .allowlist_var("MAX_PROFILE_NUM")
        .allowlist_var("MAX_SPD_SAVE")
        .allowlist_var("MAX_XMP_PROFILE_NUM")
        .allowlist_var("MRC_DDR_TYPE_DDR3")
        .allowlist_var("MRC_DDR_TYPE_DDR4")
        .allowlist_var("MRC_DDR_TYPE_LPDDR3")
        .allowlist_var("MRC_DDR_TYPE_LPDDR4")
        .allowlist_var("MRC_DDR_TYPE_UNKNOWN")
        .allowlist_var("R_MC_CHNL_RANK_PRESENT")
        .allowlist_var("WARM_BOOT")
        .allowlist_var("bmCold")
        .allowlist_var("bmFast")
        .allowlist_var("bmS3")
        .allowlist_var("bmWarm")
        // Blacklist types implemented in Rust.
        .blocklist_type("INT8")
        .blocklist_type("INT16")
        .blocklist_type("INT32")
        .blocklist_type("INT64")
        .blocklist_type("UINT8")
        .blocklist_type("UINT16")
        .blocklist_type("UINT32")
        .blocklist_type("UINT64")
        .blocklist_type("UINTN")
        .blocklist_type("BOOLEAN")
        .blocklist_type("CHAR8")
        .blocklist_type("CHAR16")
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
}

fn main() {
    generate_bindings("../../../../");
}
