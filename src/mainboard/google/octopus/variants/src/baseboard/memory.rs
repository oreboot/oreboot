use crate::{
    DRAM_PART_IN_CBI_BOARD_ID_MIN,
    baseboard::gpio::{MEM_CONFIG0, MEM_CONFIG1, MEM_CONFIG2, MEM_CONFIG3},
};
use ec::google::chromeec::ec_boardid::board_id;
use gpio::gpio_base2_value;
use soc::intel::apollolake::meminit::{Lpddr4Cfg, Lpddr4Density, Lpddr4Speed, Lpddr4Sku, Lpddr4SwizzleCfg, Lpddr4ChanSwizzleCfg};

pub static LPDDR4_SWIZZLE: Lpddr4SwizzleCfg = Lpddr4SwizzleCfg {
    phys: [
	    // CH0_DQA[0:31] SoC pins -> U22 LPDDR4 module pins
        Lpddr4ChanSwizzleCfg {
            dqs: [
		        // DQA[0:7] pins of LPDDR4 module.
                [4, 6, 7, 5, 3, 2, 1, 0],
		        // DQA[8:15] pins of LPDDR4 module.
                [12, 15, 13, 8, 9, 10, 11, 14],
		        // DQB[0:7] pins of LPDDR4 module with offset of 16.
                [17, 18, 19, 16, 23, 20, 21, 22],
		        // DQB[7:15] pins of LPDDR4 module with offset of 16.
		        [30, 31, 25, 27, 26, 29, 28, 24],
            ],
        },
        Lpddr4ChanSwizzleCfg {
            dqs: [
		        // DQA[0:7] pins of LPDDR4 module.
		        [1, 3, 2, 0, 5, 4, 6, 7],
		        // DQA[8:15] pins of LPDDR4 module.
                [15, 14, 13, 12, 8, 9, 11, 10],
		        // DQB[0:7] pins of LPDDR4 module with offset of 16.
                [20, 21, 22, 16, 23, 17, 18, 19],
		        // DQB[7:15] pins of LPDDR4 module with offset of 16.
                [30, 26, 24, 25, 28, 29, 31, 27],
            ],
        },
        Lpddr4ChanSwizzleCfg {
            dqs: [
		        // DQA[0:7] pins of LPDDR4 module.
                [15, 14, 13, 12, 8, 9, 10, 11],
		        // DQA[8:15] pins of LPDDR4 module.
                [7, 6, 5, 0, 4, 2, 1, 3],
		        // DQB[0:7] pins of LPDDR4 module with offset of 16.
                [20, 21, 23, 22, 19, 17, 18, 16],
		        // DQB[7:15] pins of LPDDR4 module with offset of 16.
                [24, 27, 26, 30, 25, 31, 28, 29],
            ],
        },
        Lpddr4ChanSwizzleCfg {
            dqs: [
		        // DQA[0:7] pins of LPDDR4 module.
                [0, 4, 7, 1, 6, 5, 3, 2],
		        // DQA[8:15] pins of LPDDR4 module.
                [11, 12, 13, 15, 10, 9, 8, 14],
		        // DQB[0:7] pins of LPDDR4 module with offset of 16.
                [19, 21, 17, 16, 22, 23, 18, 20],
		        // DQB[7:15] pins of LPDDR4 module with offset of 16.
                [30, 26, 25, 24, 31, 29, 28, 27],
            ],
        },
    ],
};

pub static NON_CBI_SKUS: [Lpddr4Sku; 8] = [
	 // K4F6E304HB-MGCJ - both logical channels While the parts
	 // are listed at 16Gb there are 2 ranks per channel so indicate
	 // the density as 8Gb per rank.
     Lpddr4Sku {
         speed: Lpddr4Speed::Speed2400,
         ch0_rank_density: Lpddr4Density::Density8Gb,
         ch1_rank_density: Lpddr4Density::Density8Gb,
         ch0_dual_rank: 1,
         ch1_dual_rank: 1,
         part_num: "K4F6E304HB-MGCJ",
         disable_periodic_retraining: false,
     },
	 // K4F8E304HB-MGCJ - both logical channels
     Lpddr4Sku {
         speed: Lpddr4Speed::Speed2400,
         ch0_rank_density: Lpddr4Density::Density8Gb,
         ch1_rank_density: Lpddr4Density::Density8Gb,
         ch0_dual_rank: 0,
         ch1_dual_rank: 0,
         part_num: "K4F8E304HB-MGCJ",
         disable_periodic_retraining: false,
     },
	 // MT53B512M32D2NP-062WT:C - both logical channels. While the parts
	 // are listed at 16Gb there are 2 ranks per channel so indicate
	 // the density as 8Gb per rank.
     Lpddr4Sku {
         speed: Lpddr4Speed::Speed2400,
         ch0_rank_density: Lpddr4Density::Density8Gb,
         ch1_rank_density: Lpddr4Density::Density8Gb,
         ch0_dual_rank: 1,
         ch1_dual_rank: 1,
         part_num: "MT53B512M32D2NP",
         disable_periodic_retraining: false,
     },
	 // MT53B256M32D1NP-062 WT:C - both logical channels
     Lpddr4Sku {
         speed: Lpddr4Speed::Speed2400,
         ch0_rank_density: Lpddr4Density::Density8Gb,
         ch1_rank_density: Lpddr4Density::Density8Gb,
         ch0_dual_rank: 0,
         ch1_dual_rank: 0,
         part_num: "MT53B256M32D1NP",
         disable_periodic_retraining: false,
     },
	 // H9HCNNNBPUMLHR-NLE - both logical channels. While the parts
	 // are listed at 16Gb there are 2 ranks per channel so indicate the
	 // density as 8Gb per rank.
     Lpddr4Sku {
         speed: Lpddr4Speed::Speed2400,
         ch0_rank_density: Lpddr4Density::Density8Gb,
         ch1_rank_density: Lpddr4Density::Density8Gb,
         ch0_dual_rank: 1,
         ch1_dual_rank: 1,
         part_num: "H9HCNNNBPUMLHR",
         disable_periodic_retraining: false,
     },
	 // H9HCNNN8KUMLHR-NLE - both logical channels
     Lpddr4Sku {
         speed: Lpddr4Speed::Speed2400,
         ch0_rank_density: Lpddr4Density::Density8Gb,
         ch1_rank_density: Lpddr4Density::Density8Gb,
         ch0_dual_rank: 0,
         ch1_dual_rank: 0,
         part_num: "H9HCNNN8KUMLHR",
         disable_periodic_retraining: false,
     },
	 /* K4F6E3S4HM-MGCJ - both logical channels */
     Lpddr4Sku {
         speed: Lpddr4Speed::Speed2400,
         ch0_rank_density: Lpddr4Density::Density16Gb,
         ch1_rank_density: Lpddr4Density::Density16Gb,
         ch0_dual_rank: 0,
         ch1_dual_rank: 0,
         part_num: "K4F6E3S4HM-MGCJ",
         disable_periodic_retraining: false,
     },
	 // MT53E512M32D2NP-046 - both logical channels
     Lpddr4Sku {
         speed: Lpddr4Speed::Speed2400,
         ch0_rank_density: Lpddr4Density::Density16Gb,
         ch1_rank_density: Lpddr4Density::Density16Gb,
         ch0_dual_rank: 0,
         ch1_dual_rank: 0,
         part_num: "MT53E512M32D2NP",
         disable_periodic_retraining: false,
     },
];

pub static NON_CBI_LP4CFG: Lpddr4Cfg = Lpddr4Cfg {
    skus: &NON_CBI_SKUS,
    swizzle_config: &LPDDR4_SWIZZLE,
};

pub static CBI_SKUS: [Lpddr4Sku; 8] = [
	 // [0] Dual Channel Config 4GiB System Capacity
     Lpddr4Sku {
         speed: Lpddr4Speed::Speed2400,
         ch0_rank_density: Lpddr4Density::Density8Gb,
         ch1_rank_density: Lpddr4Density::Density8Gb,
         ch0_dual_rank: 0,
         ch1_dual_rank: 0,
         part_num: "",
         disable_periodic_retraining: false,
     },
	 // [1] Dual Channel Config 8GiB System Capacity
     Lpddr4Sku {
         speed: Lpddr4Speed::Speed2400,
         ch0_rank_density: Lpddr4Density::Density16Gb,
         ch1_rank_density: Lpddr4Density::Density16Gb,
         ch0_dual_rank: 0,
         ch1_dual_rank: 0,
         part_num: "",
         disable_periodic_retraining: false,
     },
	 // [2] Dual Channel Config 8GiB System Capacity
     Lpddr4Sku {
         speed: Lpddr4Speed::Speed2400,
         ch0_rank_density: Lpddr4Density::Density8Gb,
         ch1_rank_density: Lpddr4Density::Density8Gb,
         ch0_dual_rank: 1,
         ch1_dual_rank: 1,
         part_num: "",
         disable_periodic_retraining: false,
     },
	 // [3] Single Channel Configs 4GiB System Capacity Ch0 populated.
     Lpddr4Sku {
         speed: Lpddr4Speed::Speed2400,
         ch0_rank_density: Lpddr4Density::Density16Gb,
         ch1_rank_density: Lpddr4Density::Density0Gb,
         ch0_dual_rank: 0,
         ch1_dual_rank: 0,
         part_num: "",
         disable_periodic_retraining: false,
     },
	 // [4] Single Channel Configs 4GiB System Capacity Ch0 populated.
     Lpddr4Sku {
         speed: Lpddr4Speed::Speed2400,
         ch0_rank_density: Lpddr4Density::Density8Gb,
         ch1_rank_density: Lpddr4Density::Density0Gb,
         ch0_dual_rank: 1,
         ch1_dual_rank: 0,
         part_num: "",
         disable_periodic_retraining: false,
     },
	 // [5] Dual Channel / Dual Rank Config 4GiB System Capacity
     Lpddr4Sku {
         speed: Lpddr4Speed::Speed2400,
         ch0_rank_density: Lpddr4Density::Density4Gb,
         ch1_rank_density: Lpddr4Density::Density4Gb,
         ch0_dual_rank: 1,
         ch1_dual_rank: 1,
         part_num: "",
         disable_periodic_retraining: false,
     },
     // [6] Default SKU skipped in coreboot
     Lpddr4Sku {
         speed: Lpddr4Speed::Speed0,
         ch0_rank_density: Lpddr4Density::Density0Gb,
         ch1_rank_density: Lpddr4Density::Density0Gb,
         ch0_dual_rank: 0,
         ch1_dual_rank: 0,
         part_num: "",
         disable_periodic_retraining: false,
     },
	 // [7] Dual Channel Config 6GiB System Capacity
     Lpddr4Sku {
         speed: Lpddr4Speed::Speed2400,
         ch0_rank_density: Lpddr4Density::Density12Gb,
         ch1_rank_density: Lpddr4Density::Density12Gb,
         ch0_dual_rank: 0,
         ch1_dual_rank: 0,
         part_num: "",
         disable_periodic_retraining: false,
     },
];

pub static CBI_LP4CFG: Lpddr4Cfg = Lpddr4Cfg {
    skus: &CBI_SKUS,
    swizzle_config: &LPDDR4_SWIZZLE,
};

pub fn lpddr4_config() -> &'static Lpddr4Cfg<'static, 'static> {
    if cfg!(feature = "dram_part_num_not_always_in_cbi") {
        // Fall back non cbi memory config
        if (board_id().unwrap_or(0) as i32) < DRAM_PART_IN_CBI_BOARD_ID_MIN {
            return &NON_CBI_LP4CFG;
        }
    }

    &CBI_LP4CFG
}

pub fn memory_sku() -> usize {
    let pads = [MEM_CONFIG0 as u32, MEM_CONFIG1 as u32, MEM_CONFIG2 as u32, MEM_CONFIG3 as u32];

    gpio_base2_value(pads.as_ref())
}
