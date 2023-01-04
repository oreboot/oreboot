#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![allow(clippy::zero_ptr)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const BOOT_WITH_FULL_CONFIGURATION: u32 = 0x00;

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

    // These values are taken from
    // https://github.com/coreboot/coreboot/blob/master/src/soc/intel/cannonlake/romstage/fsp_params.c#L20
    // TODO: Use constant defines for these instead of magic numbers. For example, define TsegSize
    // as CONFIG_SMM_TSEG_SIZE. This can be defined in fsp_cfl_sys.
    fspm_cfg.IgdDvmt50PreAlloc = 0;
    // Enable memory down BGA since it's the only LPDDR4 packaging.
    // taken from
    // https://github.com/coreboot/coreboot/blob/master/src/soc/intel/apollolake/meminit.c#L41
    fspm_cfg.Package = 1;
    fspm_cfg.MemoryDown = 1;

    fspm_cfg.ScramblerSupport = 1;
    fspm_cfg.ChannelHashMask = 0x36;
    fspm_cfg.SliceHashMask = 0x9;
    fspm_cfg.InterleavedMode = 2;
    fspm_cfg.ChannelsSlicesEnable = 0;
    fspm_cfg.MinRefRate2xEnable = 0;
    fspm_cfg.DualRankSupportEnable = 1;
    // Don't enforce a memory size limit.
    fspm_cfg.MemorySizeLimit = 0;
    // Field is in MiB units.
    fspm_cfg.LowMemoryMaxValue = (2 * ((1u32 << 30) / (1u32 << 20))) as u16;
    // No restrictions on memory above 4GiB
    fspm_cfg.HighMemoryMaxValue = 0;

    // Always default to attempt to use saved training data.
    fspm_cfg.DisableFastBoot = 0;

    // LPDDR4 is memory down so no SPD addresses.
    fspm_cfg.DIMM0SPDAddress = 0;
    fspm_cfg.DIMM1SPDAddress = 0;

    // Clear all the rank enables.
    fspm_cfg.Ch0_RankEnable = 0x0;
    fspm_cfg.Ch1_RankEnable = 0x0;
    fspm_cfg.Ch2_RankEnable = 0x0;
    fspm_cfg.Ch3_RankEnable = 0x0;

    // Set the device width to x16 which is half a LPDDR4 module as that's
    // what the reference code expects.
    fspm_cfg.Ch0_DeviceWidth = 0x1;
    fspm_cfg.Ch1_DeviceWidth = 0x1;
    fspm_cfg.Ch2_DeviceWidth = 0x1;
    fspm_cfg.Ch3_DeviceWidth = 0x1;

    // Enable bank hashing (bit 1) and rank interleaving (bit 0) with
    // a 1KiB address mapping (bits 5:4).
    fspm_cfg.Ch0_Option = 0x3;
    fspm_cfg.Ch1_Option = 0x3;
    fspm_cfg.Ch2_Option = 0x3;
    fspm_cfg.Ch3_Option = 0x3;

    // Set CA ODT with default setting of ODT pins of LPDDR4 modules pulled
    // up to 1.1V.
    let odt_config = 1 << 1;

    fspm_cfg.Ch0_OdtConfig = odt_config;
    fspm_cfg.Ch1_OdtConfig = odt_config;
    fspm_cfg.Ch2_OdtConfig = odt_config;
    fspm_cfg.Ch3_OdtConfig = odt_config;

    FSPM_UPD {
        FspUpdHeader: FSP_UPD_HEADER {
            Signature: FSPM_UPD_SIGNATURE,
            Revision: 2, // FSP 2.2
            Reserved: [0u8; 23],
        },
        FspmArchUpd: FSPM_ARCH_UPD {
            BootMode: BOOT_WITH_FULL_CONFIGURATION,
            Revision: 2, // FSP 2.2
            Reserved: [0u8; 3],
            NvsBufferPtr: 0 as *mut core::ffi::c_void, // non-volatile storage not available
            StackBase: 0x20000000 as *mut core::ffi::c_void, // TODO: I picked this at random
            StackSize: 0x4000,  // from https://github.com/coreboot/coreboot/blob/master/src/soc/intel/apollolake/Kconfig#L160
            BootLoaderTolumSize: 0, // Don't reserve "top of low usable memory" for bootloader.
            Reserved1: [0u8; 8],
        },
        FspmConfig: fspm_cfg,
        UnusedUpdSpace2: [0u8; 158],
        UpdTerminator: 0x55AA, // ???
    }
}

pub fn get_fsps_upd() -> FSPS_UPD {
    let mut fsps_config = FSP_S_CONFIG::default();

    fsps_config.LogoSize = 0;
    fsps_config.LogoPtr = 0;
    fsps_config.GraphicsConfigPtr = 0;
    fsps_config.ReservedFspsUpd = [0u8; 12];

    FSPS_UPD {
        FspUpdHeader: FSP_UPD_HEADER {
            Signature: FSPS_UPD_SIGNATURE,
            Revision: 2, // FSP 2.2
            Reserved: [0u8; 23],
        },
        FspsConfig: fsps_config,
        UpdTerminator: 0x55AA, // ???
        UnusedUpdSpace8: [0u8; 46],
    }
}
