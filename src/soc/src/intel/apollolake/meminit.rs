///! LPDDR4 helper routines for configuring the memory UPD for LPDDR4 operation.
///! There are 4 physical LPDDR4 channels each 32-bits wide. There are 2 logical
///! channels using 2 physical channels together to form a 64-bit interface to
///! memory for each logical channel.
use consts::{GiB, MiB};
use fsp_apl::FSP_M_CONFIG;

use spin::rwlock::RwLock;

pub static MEMORY_SIZE_IN_MIB: RwLock<usize> = RwLock::new(0);

pub fn memory_in_system_in_mib() -> usize {
    *MEMORY_SIZE_IN_MIB.read()
}

pub fn accumulate_channel_memory(density: Lpddr4Density, dual_rank: i32) {
    // For this platform LPDDR4 memory is 4 DRAM parts that are x32. 2 of
    // the parts are composed into a x64 memory channel. Thus there are 2
    // channels composed of 2 DRAMs.
    let mut sz = density as usize;

    // Two DRAMs per channel.
    sz *= 2;

    // Two ranks per channel
    if dual_rank != 0 {
        sz *= 2;
    }

    sz *= GiB / MiB;

    (*MEMORY_SIZE_IN_MIB.write()) += sz;
}

pub fn iohole_in_mib() -> usize {
    2 * (GiB / MiB)
}

pub fn set_lpddr4_defaults(cfg: &mut FSP_M_CONFIG) {
    // Enable memory down BGA since it's the only LPDDR4 packaging.
    cfg.Package = 1;
    cfg.MemoryDown = 1;

    cfg.ScramblerSupport = 1;
    cfg.ChannelHashMask = 0x36;
    cfg.SliceHashMask = 0x9;
    cfg.InterleavedMode = 2;
    cfg.ChannelsSlicesEnable = 0;
    cfg.MinRefRate2xEnable = 0;
    cfg.DualRankSupportEnable = 1;
    // Don't enforce a memory size limit.
    cfg.MemorySizeLimit = 0;
    // Field is in MiB units.
    cfg.LowMemoryMaxValue = iohole_in_mib() as u16;
    // No restrictions on memory above 4GiB
    cfg.HighMemoryMaxValue = 0;

    // Always default to attempt to use saved training data.
    cfg.DisableFastBoot = 0;

    // LPDDR4 is memory down so no SPD addresses.
    cfg.DIMM0SPDAddress = 0;
    cfg.DIMM1SPDAddress = 0;

    // Clear all the rank enables.
    cfg.Ch0_RankEnable = 0x0;
    cfg.Ch1_RankEnable = 0x0;
    cfg.Ch2_RankEnable = 0x0;
    cfg.Ch3_RankEnable = 0x0;

    // Set the device width to x16 which is half a LPDDR4 module as that's
    // what the reference code expects.
    cfg.Ch0_DeviceWidth = 0x1;
    cfg.Ch1_DeviceWidth = 0x1;
    cfg.Ch2_DeviceWidth = 0x1;
    cfg.Ch3_DeviceWidth = 0x1;

    // Enable bank hashing (bit 1) and rank interleaving (bit 0) with
    // a 1KiB address mapping (bits 5:4).
    cfg.Ch0_Option = 0x3;
    cfg.Ch1_Option = 0x3;
    cfg.Ch2_Option = 0x3;
    cfg.Ch3_Option = 0x3;

    // Set CA ODT with default setting of ODT pins of LPDDR4 modules pulled
    // up to 1.1V.
    let odt_config = OdtSettings::OdtAbHighHigh as u8;

    cfg.Ch0_OdtConfig = odt_config;
    cfg.Ch1_OdtConfig = odt_config;
    cfg.Ch2_OdtConfig = odt_config;
    cfg.Ch3_OdtConfig = odt_config;
}

pub struct SpeedMapping {
    pub logical: Lpddr4Speed,
    pub fsp_value: i32,
}

impl SpeedMapping {
    pub const fn create(logical: Lpddr4Speed, fsp_value: i32) -> Self {
        Self { logical, fsp_value }
    }

    pub fn logical(&self) -> Lpddr4Speed {
        self.logical
    }

    pub fn fsp_value(&self) -> i32 {
        self.fsp_value
    }
}

pub struct FspSpeedProfiles<'a> {
    mappings: &'a [SpeedMapping],
}

impl<'a> FspSpeedProfiles<'a> {
    pub const fn create(mappings: &'a [SpeedMapping]) -> Self {
        Self { mappings }
    }

    pub fn mappings(&self) -> &'a [SpeedMapping] {
        self.mappings
    }
}

pub static APL_MAPPINGS: [SpeedMapping; 3] = [
    SpeedMapping::create(Lpddr4Speed::Speed1600, 0x9),
    SpeedMapping::create(Lpddr4Speed::Speed2133, 0xa),
    SpeedMapping::create(Lpddr4Speed::Speed2400, 0xb),
];

pub static APL_PROFILE: FspSpeedProfiles = FspSpeedProfiles::create(&APL_MAPPINGS);

pub static GLK_MAPPINGS: [SpeedMapping; 3] = [
    SpeedMapping::create(Lpddr4Speed::Speed1600, 0x4),
    SpeedMapping::create(Lpddr4Speed::Speed2133, 0x6),
    SpeedMapping::create(Lpddr4Speed::Speed2400, 0x7),
];

pub static GLK_PROFILE: FspSpeedProfiles = FspSpeedProfiles::create(&GLK_MAPPINGS);

pub fn get_fsp_profile() -> &'static FspSpeedProfiles<'static> {
    if cfg!(feature = "geminilake") {
        &GLK_PROFILE
    } else {
        &APL_PROFILE
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub enum Lpddr4Phys {
    Ch0A,
    Ch0B,
    Ch1A,
    Ch1B,
    NumPhysChannels,
}

/// Logical channel identification.
#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub enum LogicalId {
    Ch0,
    Ch1,
}

/// The DQs within a physical channel can be bit-swizzled within each byte.
/// Within a channel the bytes can be swapped, but the DQs need to be routed
/// with the corresponding DQS (strobe).
#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub enum Lpddr4Dqs {
    Dqs0,
    Dqs1,
    Dqs2,
    Dqs3,
    NumByteLanes,
    DqBitsPerDqs = 8,
}

/// RL-tRCD-tRP
#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub enum Lpddr4Speed {
    // 0 speed indicates disabled channel
    Speed0 = 0,
    /// 14-15-15
    Speed1600 = 1600,
    /// 20-20-20
    Speed2133 = 2133,
    /// 24-22-22
    Speed2400 = 2400,
}

impl Lpddr4Speed {
    pub fn validate(&self) {
        let fsp_profile = get_fsp_profile();

        for mapping in fsp_profile.mappings().iter() {
            if mapping.logical() == *self {
                return;
            }
        }

        //debug!("Invalid LPDDR4 speed: {}\r\n", *self as i32);
    }

    pub fn meminit_lpddr4(&self, cfg: &mut FSP_M_CONFIG) {
        self.validate();

        //info!("LPDDR4 speed is {}MHz\r\n", *self as i32);
        cfg.Profile = self.fsp_memory_profile() as u8;

        set_lpddr4_defaults(cfg);
    }

    pub fn fsp_memory_profile(&self) -> i32 {
        let fsp_profile = get_fsp_profile();

        for mapping in fsp_profile.mappings().iter() {
            if mapping.logical() == *self {
                return mapping.fsp_value();
            }
        }

        // should never happen
        return -1;
    }
}

/// LPDDR4 module density in bits.
#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub enum Lpddr4Density {
    // 0Gb indicates disabled channel
    Density0Gb = 0,
    Density4Gb = 4,
    Density6Gb = 6,
    Density8Gb = 8,
    Density12Gb = 12,
    Density16Gb = 16,
}

/// ODT settings :
/// If ODT PIN to LP4 DRAM is pulled HIGH for ODT_A, and HIGH for ODT_B,
/// choose ODT_AB_HIGH_HIGH. If ODT PIN to LP4 DRAM is pulled HIGH for ODT_A,
/// and LOW for ODT_B, choose ODT_AB_HIGH_LOW.
///
/// Note that the enum values correspond to the interpreted UPD fields
/// within Ch[3:0]_OdtConfig parameters.
#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub enum OdtSettings {
    OdtAbHighLow = 0 << 1,
    OdtAbHighHigh = 1 << 1,
    Nwr24 = 1 << 5,
}

/// Provide bit swizzling per DQS and byte swapping within a channel.
#[repr(C)]
pub struct Lpddr4ChanSwizzleCfg {
    pub dqs: [[u8; Lpddr4Dqs::DqBitsPerDqs as usize]; Lpddr4Dqs::NumByteLanes as usize],
}

#[repr(C)]
pub struct Lpddr4SwizzleCfg {
    pub phys: [Lpddr4ChanSwizzleCfg; Lpddr4Phys::NumPhysChannels as usize],
}

impl Lpddr4SwizzleCfg {
    pub fn enable_logical_chan0(
        &self,
        cfg: &mut FSP_M_CONFIG,
        rank_density: Lpddr4Density,
        dual_rank: i32,
    ) {
        // Number of bytes to copy per DQS
        let sz = Lpddr4Dqs::DqBitsPerDqs as usize;

        // Logical channel 0 is comprised of physical channel 0 and 1.
        // Physical channel 0 is comprised of the CH0_DQB signals.
        // Physical channel 1 is comprised of the CH0_DQA signals.
        cfg.Ch0_DramDensity = rank_density as u8;
        cfg.Ch1_DramDensity = rank_density as u8;

        // Enable ranks on both channels depending on dual rank option.
        let rank_mask = if dual_rank != 0 { 0x3 } else { 0x1 };
        cfg.Ch0_RankEnable = rank_mask;
        cfg.Ch1_RankEnable = rank_mask;

        // CH0_DQB byte lanes in the bit swizzle configuration field are
        // not 1:1. The mapping within the swizzling field is:
        //   indices [0:7]   - byte lane 1 (DQS1) DQ[8:15]
        //   indices [8:15]  - byte lane 0 (DQS0) DQ[0:7]
        //   indices [16:23] - byte lane 3 (DQS3) DQ[24:31]
        //   indices [24:31] - byte lane 2 (DQS2) DQ[16:23]
        let mut chan = &self.phys[Lpddr4Phys::Ch0B as usize];

        let mut offset = Lpddr4Dqs::Dqs1 as usize;
        cfg.Ch0_Bit_swizzling[..sz].copy_from_slice(&chan.dqs[offset][..sz]);

        offset = Lpddr4Dqs::Dqs0 as usize;
        cfg.Ch0_Bit_swizzling[8..8 + sz].copy_from_slice(&chan.dqs[offset][..sz]);

        offset = Lpddr4Dqs::Dqs3 as usize;
        cfg.Ch0_Bit_swizzling[16..16 + sz].copy_from_slice(&chan.dqs[offset][..sz]);

        offset = Lpddr4Dqs::Dqs2 as usize;
        cfg.Ch0_Bit_swizzling[24..24 + sz].copy_from_slice(&chan.dqs[offset][..sz]);

        // CH0_DQA byte lanes in the bit swizzle configuration field are 1:1.
        chan = &self.phys[Lpddr4Phys::Ch0A as usize];

        offset = Lpddr4Dqs::Dqs0 as usize;
        cfg.Ch1_Bit_swizzling[..sz].copy_from_slice(&chan.dqs[offset][..sz]);

        offset = Lpddr4Dqs::Dqs1 as usize;
        cfg.Ch1_Bit_swizzling[8..8 + sz].copy_from_slice(&chan.dqs[offset][..sz]);

        offset = Lpddr4Dqs::Dqs2 as usize;
        cfg.Ch1_Bit_swizzling[16..16 + sz].copy_from_slice(&chan.dqs[offset][..sz]);

        offset = Lpddr4Dqs::Dqs3 as usize;
        cfg.Ch1_Bit_swizzling[24..24 + sz].copy_from_slice(&chan.dqs[offset][..sz]);
    }

    pub fn enable_logical_chan1(
        &self,
        cfg: &mut FSP_M_CONFIG,
        rank_density: Lpddr4Density,
        dual_rank: i32,
    ) {
        // Number of bytes to copy per DQS.
        let sz = Lpddr4Dqs::DqBitsPerDqs as usize;

        // Logical channel 1 is comprised of physical channel 2 and 3.
        // Physical channel 2 is comprised of the CH1_DQB signals.
        // Physical channel 3 is comprised of the CH1_DQA signals.
        cfg.Ch2_DramDensity = rank_density as u8;
        cfg.Ch3_DramDensity = rank_density as u8;
        // Enable ranks on both channels depending on dual rank option.
        let rank_mask = if dual_rank != 0 { 0x3 } else { 0x1 };
        cfg.Ch2_RankEnable = rank_mask;
        cfg.Ch3_RankEnable = rank_mask;

        // CH1_DQB byte lanes in the bit swizzle configuration field are
        // not 1:1. The mapping within the swizzling field is:
        //   indices [0:7]   - byte lane 1 (DQS1) DQ[8:15]
        //   indices [8:15]  - byte lane 0 (DQS0) DQ[0:7]
        //   indices [16:23] - byte lane 3 (DQS3) DQ[24:31]
        //   indices [24:31] - byte lane 2 (DQS2) DQ[16:23]
        let mut chan = &self.phys[Lpddr4Phys::Ch1B as usize];

        let mut offset = Lpddr4Dqs::Dqs1 as usize;
        cfg.Ch2_Bit_swizzling[..sz].copy_from_slice(&chan.dqs[offset][..sz]);

        offset = Lpddr4Dqs::Dqs0 as usize;
        cfg.Ch2_Bit_swizzling[8..8 + sz].copy_from_slice(&chan.dqs[offset][..sz]);

        offset = Lpddr4Dqs::Dqs3 as usize;
        cfg.Ch2_Bit_swizzling[16..16 + sz].copy_from_slice(&chan.dqs[offset][..sz]);

        offset = Lpddr4Dqs::Dqs2 as usize;
        cfg.Ch2_Bit_swizzling[24..24 + sz].copy_from_slice(&chan.dqs[offset][..sz]);

        // CH1_DQA byte lanes in the bit swizzle configuration field are 1:1.
        chan = &self.phys[Lpddr4Phys::Ch1A as usize];

        offset = Lpddr4Dqs::Dqs0 as usize;
        cfg.Ch3_Bit_swizzling[..sz].copy_from_slice(&chan.dqs[offset][..sz]);

        offset = Lpddr4Dqs::Dqs1 as usize;
        cfg.Ch3_Bit_swizzling[8..8 + sz].copy_from_slice(&chan.dqs[offset][..sz]);

        offset = Lpddr4Dqs::Dqs2 as usize;
        cfg.Ch3_Bit_swizzling[16..16 + sz].copy_from_slice(&chan.dqs[offset][..sz]);

        offset = Lpddr4Dqs::Dqs3 as usize;
        cfg.Ch3_Bit_swizzling[24..24 + sz].copy_from_slice(&chan.dqs[offset][..sz]);
    }

    pub fn meminit_lpddr4_enable_channel(
        &self,
        cfg: &mut FSP_M_CONFIG,
        logical_chan: LogicalId,
        rank_density_gb: Lpddr4Density,
        dual_rank: i32,
    ) {
        if rank_density_gb == Lpddr4Density::Density0Gb {
            //error!("Invalid LPDDR4 density: {} Gb", rank_density_gb as i32);
            return;
        }

        match logical_chan {
            LogicalId::Ch0 => self.enable_logical_chan0(cfg, rank_density_gb, dual_rank),
            LogicalId::Ch1 => self.enable_logical_chan1(cfg, rank_density_gb, dual_rank),
        }

        accumulate_channel_memory(rank_density_gb, dual_rank);
    }
}

#[repr(C)]
pub struct Lpddr4Sku<'a> {
    pub speed: Lpddr4Speed,
    pub ch0_rank_density: Lpddr4Density,
    pub ch1_rank_density: Lpddr4Density,
    pub ch0_dual_rank: i32,
    pub ch1_dual_rank: i32,
    pub part_num: &'a str,
    pub disable_periodic_retraining: bool,
}

impl<'a> Lpddr4Sku<'a> {
    pub fn speed(&self) -> Lpddr4Speed {
        self.speed
    }

    pub fn speed_mut(&mut self) -> &mut Lpddr4Speed {
        &mut self.speed
    }

    pub fn ch0_rank_density(&self) -> Lpddr4Density {
        self.ch0_rank_density
    }

    pub fn ch1_rank_density(&self) -> Lpddr4Density {
        self.ch1_rank_density
    }

    pub fn ch0_dual_rank(&self) -> i32 {
        self.ch0_dual_rank
    }

    pub fn ch1_dual_rank(&self) -> i32 {
        self.ch1_dual_rank
    }

    pub fn part_num(&self) -> &'a str {
        self.part_num
    }

    pub fn disable_periodic_retraining(&self) -> bool {
        self.disable_periodic_retraining
    }
}

#[repr(C)]
pub struct Lpddr4Cfg<'a, 'b> {
    pub skus: &'a [Lpddr4Sku<'a>],
    pub swizzle_config: &'b Lpddr4SwizzleCfg,
}

impl<'a, 'b> Lpddr4Cfg<'a, 'b> {
    pub fn meminit_lpddr4_by_sku(&mut self, cfg: &mut FSP_M_CONFIG, sku_id: usize) {
        if sku_id >= self.skus.len() {
            //error!(
            //    "Too few LPDDR4 SKUs: 0x{:x}/0x{:x}\r\n",
            //    sku_id,
            //    self.skus.len()
            //);
            return;
        }

        //info!("LPDDR4 SKU id = 0x{:x}\r\n", sku_id);

        let sku = &self.skus[sku_id];

        sku.speed().meminit_lpddr4(cfg);

        if sku.ch0_rank_density != Lpddr4Density::Density0Gb {
            //info!(
            //    "LPDDR4 Ch0 density = {} Gb\r\n",
            //    sku.ch0_rank_density as u32
            //);
            self.swizzle_config.meminit_lpddr4_enable_channel(
                cfg,
                LogicalId::Ch0,
                sku.ch0_rank_density,
                sku.ch0_dual_rank,
            );
        }

        if sku.ch1_rank_density != Lpddr4Density::Density0Gb {
            //info!(
            //    "LPDDR4 Ch1 density = {} Gb\r\n",
            //    sku.ch1_rank_density as u32
            //);
            self.swizzle_config.meminit_lpddr4_enable_channel(
                cfg,
                LogicalId::Ch1,
                sku.ch1_rank_density,
                sku.ch1_dual_rank,
            );
        }

        cfg.PeriodicRetrainingDisable = sku.disable_periodic_retraining as u8;
    }
}
