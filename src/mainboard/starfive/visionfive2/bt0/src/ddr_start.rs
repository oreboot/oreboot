use crate::ddrlib::{MemCfg, MemSet};
use crate::init::{self, read32, write32};
use crate::pac;

// TODO: define build time parameters (!)
const VAL_X2: u32 = if cfg!(dram_size = "8G") {
    0x36000000
} else {
    // 2G and 4G
    0x66000000
};

const VAL_X3: u32 = if cfg!(dram_size = "8G") {
    0xff
} else {
    // 2G and 4G
    0xfb
};

const START_CFG0: [MemCfg; 41] = crate::ddrlib::mem_cfg_arr![
    {89,	0xffffff00,	0x00000051},
    //disable RDLVL VREF
    {78,	0xfffffcff,	0x0 	  },
    {345,	0xffffff00,	0x00000051},
    //disable RDLVL VREF
    {334,	0xfffffcff,	0x0 	  },
    {601,	0xffffff00,	0x00000051},
    //disable RDLVL VREF
    {590,	0xfffffcff,	0x0 	  },
    {857,	0xffffff00,	0x00000051},
    //disable RDLVL VREF
    {846,	0xfffffcff,	0x0 	  },

    //turn off multicast
    {1793,	0xfffffeff,	0x0 	  },
    //set to freq copy 0
    {1793,	0xfffcffff,	0x0 	  },

    //data slice registers
    {125,	0xfff0ffff,	0x00010000},
    {102,	0xfffffffc,	0x00000001},
    {105,	0xffffffe0,	0x00000001},
    {92,	0xfffffffe,	0x00000001},
    {94,	0xffffe0ff,	0x00000200},
    {96,	0xfffff0ff,	0x00000400},
    {89,	0xffffff00,	0x00000051},

    {381,	0xfff0ffff,	0x00010000},
    {358,	0xfffffffc,	0x00000001},
    {361,	0xffffffe0,	0x00000001},
    {348,	0xfffffffe,	0x00000001},
    {350,	0xffffe0ff,	0x00000200},
    {352,	0xfffff0ff,	0x00000400},
    {345,	0xffffff00,	0x00000051},

    {637,	0xfff0ffff,	0x00010000},
    {614,	0xfffffffc,	0x00000001},
    {617,	0xffffffe0,	0x00000001},
    {604,	0xfffffffe,	0x00000001},
    {606,	0xffffe0ff,	0x00000200},
    {608,	0xfffff0ff,	0x00000400},
    {601,	0xffffff00,	0x00000051},

    {893,	0xfff0ffff,	0x00010000},
    {870,	0xfffffffc,	0x00000001},
    {873,	0xffffffe0,	0x00000001},
    {860,	0xfffffffe,	0x00000001},
    {862,	0xffffe0ff,	0x00000200},
    {864,	0xfffff0ff,	0x00000400},
    {857,	0xffffff00,	0x00000051},

    //phy level registers
    {1895,	0xffffe000,	0x00001342},
    {1835,	0xfffff0ff,	0x00000200},
    //turn on multicast
    {1793,	0xfffffeff,	0x00000100},
];

/* PI config */
const START_CFG1: [MemCfg; 28] = crate::ddrlib::mem_cfg_arr![
    {62,	0xfffffeff,	0x0 	  },
    {66,	0xfffffeff,	0x0 	  },
    {166,	0xffffff80,	0x00000001},
    {62,	0xfff0ffff,	0x00010000},
    {62,	0xf0ffffff,	0x01000000},
    {166,	0xffff80ff,	0x00000100},

    {179,	0xff80ffff,	0x00010000},
    {67,	0xffe0ffff,	0x00010000},
    {67,	0xe0ffffff,	0x01000000},
    {179,	0x80ffffff,	0x01000000},

    {166,	0xff80ffff,	0x00010000},
    {62,	0xfff0ffff,	0x00010000},
    {62,	0xf0ffffff,	0x01000000},
    {166,	0x80ffffff,	0x01000000},

    {182,	0xff80ffff,	0x00010000},
    {67,	0xffe0ffff,	0x00010000},
    {67,	0xe0ffffff,	0x01000000},
    {182,	0x80ffffff,	0x01000000},

    {167,	0xffffff80,	0x00000017},
    {62,	0xfff0ffff,	0x00010000},
    {62,	0xf0ffffff,	0x01000000},
    {167,	0xffff80ff,	0x00001700},
    {185,	0xff80ffff,	0x00200000},
    {67,	0xffe0ffff,	0x00010000},
    {67,	0xe0ffffff,	0x01000000},
    {185,	0x80ffffff,	0x20000000},
    {10,	0xffffffe0,	0x00000002},

    {0,	0xfffffffe,	0x00000001},
];

const START_CFG2: [MemCfg; 36] = crate::ddrlib::mem_cfg_arr![
    //set CS0 MR13.VRCG=1
    {247,	0xffffffff,	0x00000008},
    //set CS1 MR13.VRCG=1
    {249,	0xffffffff,	0x00000800},
    //set CS2 MR13.VRCG=1
    {252,	0xffffffff,	0x00000008},
    //set CS3 MR13.VRCG=1
    {254,	0xffffffff,	0x00000800},

    //PI_MR11_DATA_F1_X
    {281,	0xffffffff,	0x33000000},
    {305,	0xffffffff,	0x33000000},
    {329,	0xffffffff,	0x33000000},
    {353,	0xffffffff,	0x33000000},

    //PI_MR11_DATA_F2_X
    {289,	0xffffffff, VAL_X2},
    {313,	0xffffffff,	VAL_X2},
    {337,	0xffffffff,	VAL_X2},
    {361,	0xffffffff,	VAL_X2},

    //PI_MR22_DATA_F1_X
    {282,	0xffffffff,	0x00160000},
    {306,	0xffffffff,	0x00160000},
    {330,	0xffffffff,	0x00160000},
    {354,	0xffffffff,	0x00160000},
    //PI_MR22_DATA_F2_X
    {290,	0xffffffff,	0x00160000},
    {314,	0xffffffff,	0x00160000},
    {338,	0xffffffff,	0x00160000},
    {362,	0xffffffff,	0x00160000},

    {282,	0xffffff00,	0x17},
    {306,	0xffffff00,	0x17},
    {330,	0xffffff00,	0x17},
    {354,	0xffffff00,	0x17},
    {290,	0xffffff00,	0x17},
    {314,	0xffffff00,	0x17},
    {338,	0xffffff00,	0x17},
    {362,	0xffffff00,	0x17},

    {282,	0xffff00ff,	0x2000},
    {306,	0xffff00ff,	0x2000},
    {330,	0xffff00ff,	0x2000},
    {354,	0xffff00ff,	0x2000},
    {290,	0xffff00ff,	0x2000},
    {314,	0xffff00ff,	0x2000},
    {338,	0xffff00ff,	0x2000},
    {362,	0xffff00ff,	0x2000},
];

const START_CFG3: [MemCfg; 4] = crate::ddrlib::mem_cfg_arr![
    {65,	0xffffffff,	0x00000100},
    {321,	0xffffffff,	0x00000100},
    {577,	0xffffffff,	0x00000100},
    {833,	0xffffffff,	0x00000100},
];

const START_CFG4: [MemCfg; 24] = crate::ddrlib::mem_cfg_arr![
    //PHY_WDQLVL_CLK_JITTER_TOLERANCE_X: 8'h20 -> 8'h40
    {33,	0xffffff00,	0x0040},
    {289,	0xffffff00,	0x0040},
    {545,	0xffffff00,	0x0040},
    {801,	0xffffff00,	0x0040},

    {1038,	0xfcffffff,	0x03000000},
    {1294,	0xfcffffff,	0x03000000},
    {1550,	0xfcffffff,	0x03000000},

    //PHY_PAD_DSLICE_IO_CFG_x:0->7
    {83,	0xffc0ffff,	0x70000},
    {339,	0xffc0ffff,	0x70000},
    {595,	0xffc0ffff,	0x70000},
    {851,	0xffc0ffff,	0x70000},
    //PHY_PAD_ADR_IO_CFG_x:0->7
    {1062,	0xf800ffff,	0x70000},
    {1318,	0xf800ffff,	0x70000},
    {1574,	0xf800ffff,	0x70000},

    //PHY_PAD_CAL_IO_CFG_0:0->0x15547
    // NOTE: was set to 0x7 in JH7100 code
    {1892,	0xfffc0000,	0x15547},
    //PHY_PAD_ACS_IO_CFG:0->7
    {1893,	0xfffc0000,	0x7    },
    //PHY_CAL_MODE_0
    //NOTE: comment in JH7100 code says "TODO" and sets to 0x078
    {1852,	0xffffe000,	0x07a  },
    {1853,	0xffffffff,	0x0100 },
    //PHY_PLL_WAIT
    {1822,	0xffffffff,	0xFF   },
    //PHY_PAD_VREF_CTRL_AC:10'h0100->10'h3d5
    {1896,	0xfffffc00,	0x03d5 },

    //PHY_PAD_VREF_CTRL_DQ_x:10'h11f->10'h3d5
    {91,	0xfc00ffff,	0x03d50000},
    {347,	0xfc00ffff,	0x03d50000},
    {603,	0xfc00ffff,	0x03d50000},
    {859,	0xfc00ffff,	0x03d50000},
];

const START_CFG5: [MemSet; 21] = crate::ddrlib::mem_set_arr![
    {1912,	0xcc3bfc7},
    {1913,	0xff8f},
    {1914,	0x33f07ff},
    {1915,	0xc3c37ff},
    {1916,	0x1fffff10},
    {1917,	0x230070},

    // TODO: same value used in original code for 2G/4G and 8G, what is this?
    {1918, 0x3ff7ffff},

    {1919, 0xe10},
    {1920, 0x1fffffff},
    {1921, 0x188411},
    {1922, 0x1fffffff},
    {1923, 0x180400},
    {1924, 0x1fffffff},
    {1925, 0x180400},
    {1926, 0x1fffffcf},
    {1927, 0x188400},
    {1928, 0x1fffffff},
    {1929, 0x4188411},
    {1837, 0x24410},
    {1840, 0x24410},
    {1842, 0x2ffff},
];

const START_CFG6: [MemCfg; 14] = crate::ddrlib::mem_cfg_arr![
    {76,	0xff0000f8,	0x00ff8f07},
    {332,	0xff0000f8,	0x00ff8f07},
    {588,	0xff0000f8,	0x00ff8f07},
    {844,	0xff0000f8,	0x00ff8f07},

    {77,	0xffff0000,	0xff8f},
    {333,	0xffff0000,	0xff8f},
    {589,	0xffff0000,	0xff8f},
    {845,	0xffff0000,	0xff8f},

    //PHY_ADR_TSEL_SELECT_X:bit[7:0]:{ENSLICEP_ODT/DRV,PENSLICEN_ODT/DRV}
    {1062,	0xffffff00,	VAL_X3}, // addr5-0
    {1318,	0xffffff00,	VAL_X3}, // addr11-6
    {1574,	0xffffff00,	VAL_X3}, // addr15-12

    //PHY_TST_CLK_PAD_CTRL_x
    {1028,	0xffffffff,	0x1000000},
    {1284,	0xffffffff,	0x1000000},
    {1540,	0xffffffff,	0x1000000},
];

// PHY_TST_CLK_PAD_CTRL_x
const START_CFG7: [MemSet; 4] = crate::ddrlib::mem_set_arr![
    {1848, 0x3cf07f8},
    {1849, 0x3f},
    {1850, 0x1fffff},
    {1851, 0x060000},
];

const START_CFG8: [MemCfg; 25] = crate::ddrlib::mem_cfg_arr![
    // PHY_DSLICE_PAD_BOOSTPN_SETTING_x
    {130,	0x0000ffff,	0xffff0000},
    {386,	0x0000ffff,	0xffff0000},
    {642,	0x0000ffff,	0xffff0000},
    {898,	0x0000ffff,	0xffff0000},
    // ???
    {131,	0xfffffff0,	0xf},
    {387,	0xfffffff0,	0xf},
    {643,	0xfffffff0,	0xf},
    {899,	0xfffffff0,	0xf},
    //PHY_WRLVL_CAPTURE_CNT_X
    {29,	0xc0ffffff,	0x10000000},
    {285,	0xc0ffffff,	0x10000000},
    {541,	0xc0ffffff,	0x10000000},
    {797,	0xc0ffffff,	0x10000000},
    // PHY_GTLVL_CAPTURE_CNT_X
    {30,	0xffffffff,	0x00080000},
    {286,	0xffffffff,	0x00080000},
    {542,	0xffffffff,	0x00080000},
    {798,	0xffffffff,	0x00080000},
    // PHY_RDLVL_CAPTURE_CNT_X
    {31,	0xffffffc0,	0x00000010},
    {287,	0xffffffc0,	0x00000010},
    {543,	0xffffffc0,	0x00000010},
    {799,	0xffffffc0,	0x00000010},
    // PHY_ADRLVL_CAPTURE_CNT_X
    {1071,	0xfffffff0,	0x00000008},
    {1327,	0xfffffff0,	0x00000008},
    {1583,	0xfffffff0,	0x00000008},
    // PHY_CSLVL_COARSECAPTURE_CNT
    {1808,	0xfffffff0,	0x00000008},
    // PHY_CSLVL_CAPTURE_CNT_X
    {1896,	0xfff0ffff,	0x00080000},
];

const DEBUG: bool = false;

pub unsafe fn start() {
    let phy = &*pac::DMC_PHY::ptr();

    START_CFG0.iter().for_each(|cfg| {
        phy.ac_base(cfg.reg_nr as usize)
            .modify(|r, w| w.bits((r.bits() & cfg.mask) | cfg.value));
    });
    START_CFG1.iter().for_each(|cfg| {
        phy.base(cfg.reg_nr as usize)
            .modify(|r, w| w.bits((r.bits() & cfg.mask) | cfg.value));
    });
    // NOTE: Commented out in VF1 code
    if cfg!(dram_size = "2G") {
        phy.base(11)
            .modify(|r, w| w.bits((r.bits() & 0xffff_fff0) | 0x0000_0005));
    }
    START_CFG2.iter().for_each(|cfg| {
        phy.base(cfg.reg_nr as usize)
            .modify(|r, w| w.bits((r.bits() & cfg.mask) | cfg.value));
    });
    START_CFG3.iter().for_each(|cfg| {
        phy.ac_base(cfg.reg_nr as usize)
            .modify(|r, w| w.bits((r.bits() & cfg.mask) | cfg.value));
    });
    // PHY_RPTR_UPDATE_x: bit[11:8]+=3
    // NOTE: Special handling: write back current val + val to register
    // That was the behavior in the previous implementation, too. No clue why...
    // The registers might be reset to 0 on cold boot. If they retain their
    // current value on hot reset, it could turn into weird behavior.
    // With `START_CFG0`, we set this to `0x00000400`, so it should now become
    // `0x00000700`.
    [96, 352, 608, 864].iter().for_each(|&reg| {
        phy.ac_base(reg).modify(|r, w| w.bits(r.bits() + 0x300));
    });
    // PHY_WRLVL_DLY_STEP_X: 8'hC -> 8'h12
    // NOTE: This is h18 in the JH7100 code
    // This is for G_SPEED_2133.
    //    G_SPEED_2666: 0x00140000
    //    G_SPEED_3200: 0x00180000
    // TODO: try lower speed?
    [96, 352, 608, 864].iter().for_each(|&reg| {
        phy.ac_base(reg)
            .modify(|r, w| w.bits((r.bits() & 0xff00_ffff) | 0x0012_0000));
    });

    START_CFG4.iter().for_each(|cfg| {
        phy.ac_base(cfg.reg_nr as usize)
            .modify(|r, w| w.bits((r.bits() & cfg.mask) | cfg.value));
    });
    START_CFG5.iter().for_each(|cfg| {
        phy.ac_base(cfg.reg_nr as usize)
            .write(|w| w.bits(cfg.value));
    });
    START_CFG6.iter().for_each(|cfg| {
        phy.ac_base(cfg.reg_nr as usize)
            .modify(|r, w| w.bits((r.bits() & cfg.mask) | cfg.value));
    });
    START_CFG7.iter().for_each(|cfg| {
        phy.ac_base(cfg.reg_nr as usize)
            .write(|w| w.bits(cfg.value));
    });
    START_CFG8.iter().for_each(|cfg| {
        phy.ac_base(cfg.reg_nr as usize)
            .modify(|r, w| w.bits((r.bits() & cfg.mask) | cfg.value));
    });
    phy.csr(0).write(|w| w.bits(0x1));
}
