#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![allow(clippy::zero_ptr)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Don't mangle as this is referenced from a linker script to place in a specific location in
// flash.
macro_rules! blob_macro {
    () => {
        include_bytes!(concat!(env!("OUT_DIR"), "/", "FSP.fd"))
    };
}
#[no_mangle]
#[used]
#[link_section = ".fspblob"]
static FSP_BLOB: [u8; blob_macro!().len()] = *blob_macro!();

pub fn get_fspm_upd() -> FSPM_UPD {
    let mut fspm_cfg = FSP_M_CONFIG::default();
    let mut fspm_test_cfg = FSP_M_TEST_CONFIG::default();

    // These values are taken from
    // https://github.com/coreboot/coreboot/blob/master/src/soc/intel/cannonlake/romstage/fsp_params.c#L20
    // TODO: Fill out the remaining parameters
    // TODO: Use constant defines for these instead of magic numbers. For example, define TsegSize
    // as CONFIG_SMM_TSEG_SIZE. This can be defined in fsp_cfl_sys.
    fspm_cfg.InternalGfx = 0;
    fspm_cfg.IgdDvmt50PreAlloc = 0;
    fspm_cfg.TsegSize = 0x800000;
    fspm_cfg.IedSize = 0x400000;
    fspm_cfg.SaGv = 0; // Allows memory training to happen at different frequencies? Disabled for now

    fspm_test_cfg.PanelPowerEnable = 0;

    FSPM_UPD {
        FspUpdHeader: FSP_UPD_HEADER {
            Signature: FSPM_UPD_SIGNATURE,
            Revision: 2, // FSP 2.2
            Reserved: [0u8; 23],
        },
        FspmArchUpd: FSPM_ARCH_UPD {
            Revision: 2, // FSP 2.2
            Reserved: [0u8; 3],
            NvsBufferPtr: 0 as *mut core::ffi::c_void, // non-volatile storage not available
            StackBase: 0x20000000 as *mut core::ffi::c_void, // TODO: I picked this at random
            StackSize: 0x10000,                        // TODO: I picked this at random
            BootLoaderTolumSize: 0, // Don't reserve "top of low usable memory" for bootloader.
            BootMode: BOOT_WITH_FULL_CONFIGURATION,
            Reserved1: [0u8; 8],
        },
        FspmConfig: fspm_cfg,
        FspmTestConfig: fspm_test_cfg,
        UnusedUpdSpace6: 0u8,
        UpdTerminator: 0x55AA, // ???
    }
}

pub fn get_fsps_upd() -> FSPS_UPD {
    let mut fsps_config = FSP_S_CONFIG::default();
    let fsps_test_config = FSP_S_TEST_CONFIG::default();

    fsps_config.LogoSize = 0;
    fsps_config.LogoPtr = 0;
    fsps_config.GraphicsConfigPtr = 0;
    fsps_config.ReservedFspsUpd = [0u8];

    FSPS_UPD {
        FspUpdHeader: FSP_UPD_HEADER {
            Signature: FSPS_UPD_SIGNATURE,
            Revision: 2, // FSP 2.2
            Reserved: [0u8; 23],
        },
        FspsConfig: fsps_config,
        FspsTestConfig: fsps_test_config,
        UpdTerminator: 0x55AA, // ???
    }
}
