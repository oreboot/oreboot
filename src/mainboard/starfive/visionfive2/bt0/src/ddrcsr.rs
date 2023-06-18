use crate::ddrlib::*;
use crate::init::{self, read32, write32};
use crate::pac;
use starfive_visionfive2_lib::udelay;

const FREQ_CHANGE: usize = 0x0001;
const FREQ_CHANGE_ACK: usize = 0x0002;

const FANCY_REG1: usize = 0x0504;
const FANCY_REG2: usize = 0x050c;
const FANCY_REG3: usize = 0x0514;
const TRAINING_STATUS_MAYBE: usize = 0x0518;

const VERBOSE: bool = false;

fn train(training_status_reg: usize) {
    let mut rounds: usize = 0;
    let freq_change_req = FREQ_CHANGE;
    let phy = unsafe { &*pac::DMC_PHY::ptr() };
    let ctrl = unsafe { &*pac::DMC_CTRL::ptr() };
    while (ctrl.csr(training_status_reg >> 2).read().bits() & 0x2) != 0x0 {
        let req_type = phy.csr(freq_change_req).read().bits();
        if (req_type & 0x00000020) == 0x00000020 {
            let freq_change_req = req_type & 0x0000001f;
            match freq_change_req {
                0 => {
                    // DDRC_CLOCK = 12.5M
                    init::clk_ddrc_osc_div2();
                }
                1 => {
                    // DDRC_CLOCK = 200M
                    init::clk_ddrc_pll1_div8();
                }
                2 => {
                    // DDRC_CLOCK = 400M
                    init::clk_ddrc_pll1_div2();
                }
                _ => {
                    println!("DRAM freq type unknown {req_type}");
                }
            }

            if VERBOSE {
                println!("DRAM freq change type {freq_change_req}, round {rounds}");
                rounds += 1;
            }

            phy.csr(FREQ_CHANGE_ACK).write(|w| unsafe { w.bits(0x1) });
            while (phy.csr(FREQ_CHANGE_ACK).read().bits() & 0x1) != 0x0 {
                udelay(2);
            }
        }
        udelay(1);
    }
}

// TODO: define build time parameters (!)
const CFG0_X1: u32 = if cfg!(dram_size = "8G") || cfg!(dram_size = "4G") {
    0x00000001
} else {
    // 2G
    0x00800001
};

// see U-Boot drivers/ram/starfive/ddrcsr_boot.c
const DDR_CSR_CFG0: [MemSet; 6] = mem_set_arr![
    // TODO: same value used in original code for 2G/4G and 8G, what is this?
    {0xf00, 0x40001030},
    {0xf04, CFG0_X1},
    {0xf10, 0x00400000},
    {0xf14, 0x043fffff},
    {0xf18, 0x00000000},
    {0xf30, 0x1f000041},
];

const DDR_CSR_CFG1: [MemSet; 6] = mem_set_arr![
    {0x10c, 0x00000505},
    {0x11c, 0x00000000},
    {0x500, 0x00000201},
    {0x514, 0x00000100},
    {0x6a8, 0x00040000},
    {0xea8, 0x00040000},
];

const CFG1_X1: u32 = if cfg!(dram_size = "8G") || cfg!(dram_size = "4G") {
    0x30010006
} else {
    // 2G
    0x10010006
};

const CFG1_X2: u32 = if cfg!(dram_size = "8G") || cfg!(dram_size = "4G") {
    0x30020000
} else {
    // 2G
    0x10020000
};

const CFG1_X3: u32 = if cfg!(dram_size = "8G") || cfg!(dram_size = "4G") {
    0x30030031
} else {
    // 2G
    0x10030031
};

const CFG1_X4: u32 = if cfg!(dram_size = "8G") || cfg!(dram_size = "4G") {
    0x300b0033
} else {
    // 2G
    0x100b0033
};

const CFG1_X5: u32 = if cfg!(dram_size = "8G") || cfg!(dram_size = "4G") {
    0x30160016
} else {
    // 2G
    0x10160016
};

const DDR_CSR_CFG2: [MemSet; 50] = mem_set_arr![
    {0x310, 0x00020000},
    {0x310, 0x00020001},
    // Write down RCLK-related CRs
    {0x600, 0x002e0176},
    {0x604, 0x002e0176},
    {0x608, 0x001700bb},
    {0x60c, 0x000b005d},
    {0x610, 0x0005002e},
    {0x614, 0x00020017},
    {0x618, 0x00020017},
    {0x61c, 0x00020017},
    {0x678, 0x00000019},
    {0x100, 0x000000f8},
    {0x620, 0x03030404},
    {0x624, 0x04030505},
    {0x628, 0x07030884},
    {0x62c, 0x13150401},
    {0x630, 0x17150604},
    {0x634, 0x00110000},
    {0x638, 0x200a0a08},
    {0x63c, 0x1730f803},
    {0x640, 0x000a0c00},
    {0x644, 0xa005000a},
    {0x648, 0x00000000},
    {0x64c, 0x00081306},
    {0x650, 0x04070304},
    {0x654, 0x00000404},
    {0x658, 0x00000060},
    {0x65c, 0x00030008},
    {0x660, 0x00000000},
    {0x680, 0x00000603},
    {0x684, 0x01000202},
    {0x688, 0x0413040d},
    {0x68c, 0x20002420},
    {0x690, 0x00140000},
    {0x69c, 0x01240074},
    {0x6a0, 0x00000000},
    {0x6a4, 0x20240c00},
    {0x6a8, 0x00040000},

    {0x4, CFG1_X1},
    {0xc, 0x00000002},
    {0x4, CFG1_X2},
    {0xc, 0x00000002},
    {0x4, CFG1_X3},
    {0xc, 0x00000002},
    {0x4, CFG1_X4},
    {0xc, 0x00000002},
    {0x4, CFG1_X5},
    {0xc, 0x00000002},

    {0x10, 0x00000010},
    {0x14, 0x00000001},
];

const DDR_CSR_CFG3: [MemCfg; 29] = mem_cfg_arr![
    // cdns_rdlvl_gate_tr_init( 3,0,0,0,0);
    {0xb8,		0xf0ffffff,		0x3000000},
    {0x84,		0xFEFFFFFF,		0x0 	 },
    {0xb0,		0xFFFEFFFF,		0x0 	 },
    {0xb0,		0xFEFFFFFF,		0x0 	 },
    {0xb4,		0xffffffff,		0x1 	 },
    {0x248,		0xffffffff,		0x3000000},
    {0x24c,		0xffffffff,		0x300 	 },
    {0x24c,		0xffffffff,		0x3000000},
    {0xb0,		0xffffffff,		0x100 	 },
    // cdns_rdlvl_tr_init( 3,0,0,0,0);
    {0xb8,		0xFFF0FFFF,		0x30000  },
    {0x84,		0xFFFEFFFF,		0x0 	 },
    {0xac,		0xFFFEFFFF,		0x0 	 },
    {0xac,		0xFEFFFFFF,		0x0 	 },
    {0xb0,		0xffffffff,		0x1 	 },
    {0x248,		0xffffffff,		0x30000  },
    {0x24c,		0xffffffff,		0x3 	 },
    {0x24c,		0xffffffff,		0x30000  },
    {0x250,		0xffffffff,		0x3000000},
    {0x254,		0xffffffff,		0x3000000},
    {0x258,		0xffffffff,		0x3000000},
    {0xac,		0xffffffff,		0x100 	 },
    // cdns_wdqlvl_tr_init( 3,0,0,0,0);
    {0x10c,		0xFFFFF0FF,		0x300 	 },
    {0x110,		0xFFFFFEFF,		0x0 	 },
    {0x11c,		0xFFFEFFFF,		0x0 	 },
    {0x11c,		0xFEFFFFFF,		0x0 	 },
    {0x120,		0xffffffff,		0x100 	 },
    {0x2d0,		0xffffffff,		0x300 	 },
    {0x2dc,		0xffffffff,		0x300 	 },
    {0x2e8,		0xffffffff,		0x300 	 },
];

const CFG3_X1: u32 = if cfg!(dram_size = "8G") || cfg!(dram_size = "4G") {
    0x30010036
} else {
    // 2G
    0x10010036
};

const CFG3_X2: u32 = if cfg!(dram_size = "8G") || cfg!(dram_size = "4G") {
    0x3002001b
} else {
    // 2G
    0x10010036
};

const CFG3_X3: u32 = if cfg!(dram_size = "8G") || cfg!(dram_size = "4G") {
    0x30030031
} else {
    // 2G
    0x10030031
};

const CFG3_X4: u32 = if cfg!(dram_size = "8G") {
    0x300b0036
} else if cfg!(dram_size = "4G") {
    0x300b0066
} else {
    // 2G
    0x100b0066
};

const CFG3_X5: u32 = if cfg!(dram_size = "8G") || cfg!(dram_size = "4G") {
    0x30160016
} else {
    // 2G
    0x10160016
};

const CFG3_X6: u32 = if cfg!(dram_size = "8G") || cfg!(dram_size = "4G") {
    0x09313fff
} else {
    // 2G
    0x09311fff
};

const CFG3_X7: u32 = if cfg!(dram_size = "8G") || cfg!(dram_size = "4G") {
    0x00000033
} else {
    0x00000013
    // 2G
};

const DDR_CSR_CFG4: [MemSet; 43] = mem_set_arr![
    {0x100,		0x000000e0},
    {0x620,		0x04041417},
    {0x624,		0x09110609},
    {0x628,		0x442d0994},
    {0x62c,		0x271e102b},
    {0x630,		0x291b140a},
    {0x634,		0x001c0000},
    {0x638,		0x200f0f08},
    {0x63c,		0x29420a06},
    {0x640,		0x019e1fc1},
    {0x644,		0x10cb0196},
    {0x648,		0x00000000},
    {0x64c,		0x00082714},
    {0x650,		0x16442f0d},
    {0x654,		0x00001916},
    {0x658,		0x00000060},
    {0x65c,		0x00600020},
    {0x660,		0x00000000},
    {0x680,		0x0c00040f},
    {0x684,		0x03000604},
    {0x688,		0x0515040d},
    {0x68c,		0x20002c20},
    {0x690,		0x00140000},
    {0x69c,		0x01240074},
    {0x6a0,		0x00000000},
    {0x6a4,		0x202c0c00},
    {0x6a8,		0x00040000},

    {0x4,		CFG3_X1},
    {0xc,		0x00000002},
    {0x4,		CFG3_X2},
    {0xc,		0x00000002},
    {0x4,		CFG3_X3},
    {0xc,		0x00000002},
    {0x4,		CFG3_X4},
    {0xc,		0x00000002},
    {0x4,		CFG3_X5},
    {0xc,		0x00000002},

    {0x410,		0x00101010},
    {0x420,		0x0c181006},
    {0x424,		0x20200820},
    {0x428,		0x80000020},
    {0x0,		0x00000001},
    {0x108,		0x00003000},
];

const DDR_CSR_CFG5: [MemSet; 6] = mem_set_arr![
    {0x330,		CFG3_X6},
    {0x508,		CFG3_X7},
    {0x324,		0x00002000},
    {0x104,		0x90000000},
    {0x510,		0x00000100},
    {0x514,		0x00000000},
];

// NOTE: `reg_nr` here is actually the _offset_! So do not shift << 2.
pub unsafe fn omc_init() {
    println!("[DRAM] OMC init");
    let ctrl = &*pac::DMC_CTRL::ptr();
    let phy = &*pac::DMC_PHY::ptr();
    ctrl.csr(0).write(|w| w.bits(0x1));

    DDR_CSR_CFG0.iter().for_each(|cfg| {
        ctrl.sec((cfg.reg_nr >> 2) as usize)
            .write(|w| w.bits(cfg.value));
    });
    if cfg!(dram_size = "8G") || cfg!(dram_size = "4G") {
        ctrl.sec(0xf34 >> 2).write(|w| w.bits(0x1f00_0041));
    } else {
        // skipped in original code
    }
    ctrl.sec(0x0110 >> 2).write(|w| w.bits(0xc000_0001));
    ctrl.sec(0x0114 >> 2).write(|w| w.bits(0xffff_ffff));

    DDR_CSR_CFG1.iter().for_each(|cfg| {
        ctrl.csr((cfg.reg_nr >> 2) as usize)
            .write(|w| w.bits(cfg.value));
    });

    // This seems to trigger some sort of readiness.
    // Memory frequency should be changed below 50MHz somewhere before here
    ctrl.csr(FANCY_REG1 >> 2).write(|w| w.bits(0x4000_0000));
    while ctrl.csr(FANCY_REG1 >> 2).read().bits() & 0x80000000 != 0x80000000 {
        udelay(1);
    }
    ctrl.csr(FANCY_REG1 >> 2).write(|w| w.bits(0x0000_0000));

    // tINIT0 is controlled by System
    ctrl.csr(FANCY_REG2 >> 2).write(|w| w.bits(0x0));
    // Waits tINIT1 (300 us): Minimum RESET_n LOW time after completion of
    // voltage ramp
    // NOTE: 200 us in VF1 code
    udelay(300);
    ctrl.csr(FANCY_REG2 >> 2).write(|w| w.bits(0x1));
    udelay(3000);

    // Drive CKE high (clock enable)
    // TODO: skip for 16G (?)
    let val = if cfg!(dram_size = "8G") || cfg!(dram_size = "4G") {
        0x0000003c
    } else {
        0x0000001c
    };
    ctrl.csr(0x0010 >> 2).write(|w| w.bits(val));
    ctrl.csr(0x0014 >> 2).write(|w| w.bits(0x0000_0001));
    // Waits tINIT5 (2 us): Minimum idle time before first MRW/MRR command
    udelay(4);
    DDR_CSR_CFG2.iter().for_each(|cfg| {
        ctrl.csr((cfg.reg_nr >> 2) as usize)
            .write(|w| w.bits(cfg.value));
    });
    // Waits tZQCAL (1 us)
    udelay(4);
    ctrl.csr(0x0010 >> 2).write(|w| w.bits(0x0000_0011));
    ctrl.csr(0x0014 >> 2).write(|w| w.bits(0x0000_0001));

    if cfg!(dram_size = "8G") || cfg!(dram_size = "4G") {
        ctrl.csr(0x0010 >> 2).write(|w| w.bits(0x0000_0020));
        ctrl.csr(0x0014 >> 2).write(|w| w.bits(0x0000_0001));
        // Waits tZQCAL (1 us)
        udelay(4);
        ctrl.csr(0x0010 >> 2).write(|w| w.bits(0x0000_0021));
        ctrl.csr(0x0014 >> 2).write(|w| w.bits(0x0000_0001));
    }
    ctrl.csr(FANCY_REG3 >> 2).write(|w| w.bits(0x0000_0000));

    // This register seems to first indicate that we are ready for training,
    // and then, that training is done. See the train() function using the same
    // mask again.
    while ctrl.csr(TRAINING_STATUS_MAYBE >> 2).read().bits() & 0x2 != 0x2 {
        println!(
            "[DRAM] Training status maybe value: {}",
            ctrl.csr(TRAINING_STATUS_MAYBE >> 2).read().bits()
        );
        udelay(1);
    }

    println!("[DRAM] OMC init train");
    train(TRAINING_STATUS_MAYBE);

    println!("[DRAM] OMC init PHY");
    // NOTE: This here even worked when I was accidentally off to 0x150 / 0x154.
    phy.base(0x14c >> 2).read().bits();
    let val = phy.base(85).read().bits(); // 84 << 2
    phy.base(81).write(|w| w.bits(val & 0xF800_0000));

    DDR_CSR_CFG3.iter().for_each(|cfg| {
        phy.base((cfg.reg_nr >> 2) as usize)
            .modify(|r, w| w.bits((r.bits() & cfg.mask) | cfg.value));
    });

    DDR_CSR_CFG4.iter().for_each(|cfg| {
        ctrl.csr((cfg.reg_nr >> 2) as usize)
            .write(|w| w.bits(cfg.value));
    });
    ctrl.sec(0x0704 >> 2).write(|w| w.bits(0x0000_0007));
    DDR_CSR_CFG5.iter().for_each(|cfg| {
        ctrl.csr((cfg.reg_nr >> 2) as usize)
            .write(|w| w.bits(cfg.value));
    });
    ctrl.sec(0x0700 >> 2).write(|w| w.bits(0x0000_0003));
    ctrl.csr(0x0514 >> 2).write(|w| w.bits(0x0000_0600));
    ctrl.csr(0x0020 >> 2).write(|w| w.bits(0x0000_0001));
    println!("[DRAM] OMC init done");
}
