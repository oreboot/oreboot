use crate::dram_lib::*;

const CLINT_BASE: usize = 0x0200_0000;
const CLINT_MTIMER: usize = CLINT_BASE + 0xbff8;

fn get_mtime() -> u32 {
    unsafe { core::ptr::read_volatile(CLINT_MTIMER as *mut u32) }
}

fn udelay(t: usize) {
    let curr_time = get_mtime();
    while get_mtime() < (curr_time + t as u32) {}
}

const FREQ_CHANGE_REQ_TYPE: usize = 0x0004;
const FREQ_CHANGE_ACK: usize = 0x0008;
const TRAINING_STATUS_MAYBE: usize = 0x0518;

const VERBOSE: bool = false;

fn train(phy_base_addr: usize, cfg_base_addr: usize, ddr_num: usize) {
    let status = cfg_base_addr + TRAINING_STATUS_MAYBE;
    let freq_change_ack: usize = phy_base_addr + FREQ_CHANGE_ACK;
    let freq_change_req_type: usize = phy_base_addr + FREQ_CHANGE_REQ_TYPE;
    let mut rounds: usize = 0;
    while (read(status) & 0x00000002) != 0x00000000 {
        let req_type = read(freq_change_req_type);
        if (req_type & 0x00000020) == 0x00000020 {
            match req_type & 0x0000001f {
                0 => {
                    // DDRC_CLOCK = 12.5M
                    if ddr_num == 0 {
                        crate::init::clk_ddrc0_osc_div2();
                        udelay(100);
                    } else {
                        crate::init::clk_ddrc1_osc_div2();
                        udelay(100);
                    }
                }
                1 => {
                    // DDRC_CLOCK = 200M
                    if ddr_num == 0 {
                        crate::init::clk_ddrc0_pll_div4();
                        // crate::init::clk_ddrc0_osc_div2();
                        udelay(100);
                    } else {
                        crate::init::clk_ddrc1_pll_div4();
                        // crate::init::clk_ddrc1_osc_div2();
                        udelay(100);
                    }
                }
                2 => {
                    // DDRC_CLOCK = 400M
                    if ddr_num == 0 {
                        crate::init::clk_ddrc0_pll_div2();
                        // crate::init::clk_ddrc0_osc_div2();
                        udelay(100);
                    } else {
                        crate::init::clk_ddrc1_pll_div2();
                        // crate::init::clk_ddrc1_osc_div2();
                        udelay(100);
                    }
                }
                _ => {
                    println!("DRAM freq type unknown {freq_change_req_type}");
                }
            }

            if VERBOSE {
                println!("DRAM freq change type {}, round {rounds}", req_type & 0x1f);
                rounds = rounds + 1;
            }

            write(freq_change_ack, 0x1);
            while (read(freq_change_ack) & 0x1) != 0x0 {}
        }

        udelay(1);
    }
}

fn omc_init(phy_base_addr: usize, cfg_base_addr: usize, sec_base_addr: usize, ddr_num: usize) {
    // NOTE This is also commented out in the original implementation.
    //write(cfg_base_addr + 0x000, 0x00000401);
    write(cfg_base_addr + 0x000, 0x00000001);

    write(sec_base_addr + 0xf00, 0x40001030);
    write(sec_base_addr + 0xf04, 0x00000001);
    write(sec_base_addr + 0xf10, 0x00800000);
    write(sec_base_addr + 0xf14, 0x027fffff);
    write(sec_base_addr + 0xf18, 0x00000001);
    write(sec_base_addr + 0xf30, 0x0f000031);
    write(sec_base_addr + 0xf34, 0x0f000031);
    write(sec_base_addr + 0x110, 0xc0000001);
    write(sec_base_addr + 0x114, 0xffffffff);

    write(cfg_base_addr + 0x10c, 0x00000505);
    write(cfg_base_addr + 0x11c, 0x00000000);
    write(cfg_base_addr + 0x500, 0x00000201);
    write(cfg_base_addr + 0x514, 0x00000100);
    write(cfg_base_addr + 0x6a8, 0x00040000);
    write(cfg_base_addr + 0xea8, 0x00040000);
    // Memory frequency should be changed below 50MHz somewhere before here
    write(cfg_base_addr + 0x504, 0x40000000);
    udelay(300);
    //cdns_dll_rst_deassert()

    // some polling for reset?
    while (read(cfg_base_addr + 0x504) & 0x80000000) != 0x80000000 {
        udelay(1);
    }
    write(cfg_base_addr + 0x504, 0x00000000);

    // tINIT0 is controlled by System
    write(cfg_base_addr + 0x50c, 0x00000000);
    // Waits tINIT1 (200 us): Minimum RESET_n LOW time after completion of voltage ramp
    udelay(300);
    write(cfg_base_addr + 0x50c, 0x00000001);
    // Waits tINIT3 (2 ms): Minimum CKE low time after RESET_n high
    udelay(3000);
    // Drive CKE high
    write(cfg_base_addr + 0x010, 0x0000003c);
    write(cfg_base_addr + 0x014, 0x00000001);
    // Waits tINIT5 (2 us): Minimum idle time before first MRW/MRR command
    udelay(4);
    write(cfg_base_addr + 0x310, 0x00020000);
    write(cfg_base_addr + 0x310, 0x00020001);
    // Write down RCLK-related CRs
    write(cfg_base_addr + 0x600, 0x002e0176);
    write(cfg_base_addr + 0x604, 0x002e0176);
    write(cfg_base_addr + 0x608, 0x001700bb);
    write(cfg_base_addr + 0x60c, 0x000b005d);
    write(cfg_base_addr + 0x610, 0x0005002e);
    write(cfg_base_addr + 0x614, 0x00020017);
    write(cfg_base_addr + 0x618, 0x00020017);
    write(cfg_base_addr + 0x61c, 0x00020017);
    write(cfg_base_addr + 0x678, 0x00000019);
    write(cfg_base_addr + 0x100, 0x000000f8);
    write(cfg_base_addr + 0x620, 0x03030404);
    write(cfg_base_addr + 0x624, 0x04030505);
    write(cfg_base_addr + 0x628, 0x07030884);
    write(cfg_base_addr + 0x62c, 0x13150401);
    write(cfg_base_addr + 0x630, 0x17150604);
    write(cfg_base_addr + 0x634, 0x00110000);
    write(cfg_base_addr + 0x638, 0x200a0a08);
    write(cfg_base_addr + 0x63c, 0x1730f803);
    write(cfg_base_addr + 0x640, 0x00080c00);
    write(cfg_base_addr + 0x644, 0xa0040007);
    write(cfg_base_addr + 0x648, 0x00000000);
    write(cfg_base_addr + 0x64c, 0x00081306);
    write(cfg_base_addr + 0x650, 0x04070304);
    write(cfg_base_addr + 0x654, 0x00000404);
    write(cfg_base_addr + 0x658, 0x00000060);
    write(cfg_base_addr + 0x65c, 0x00030008);
    write(cfg_base_addr + 0x660, 0x00000000);
    write(cfg_base_addr + 0x680, 0x00000603);
    write(cfg_base_addr + 0x684, 0x01000202);
    write(cfg_base_addr + 0x688, 0x0413040d);
    write(cfg_base_addr + 0x68c, 0x20002420);
    write(cfg_base_addr + 0x690, 0x00140000);
    write(cfg_base_addr + 0x69c, 0x01240074);
    write(cfg_base_addr + 0x6a0, 0x00000000);
    write(cfg_base_addr + 0x6a4, 0x20240c00);
    write(cfg_base_addr + 0x6a8, 0x00040000);
    write(cfg_base_addr + 0x004, 0x30010006);
    write(cfg_base_addr + 0x00c, 0x00000002);
    write(cfg_base_addr + 0x004, 0x30020000);
    write(cfg_base_addr + 0x00c, 0x00000002);
    write(cfg_base_addr + 0x004, 0x30030031);
    write(cfg_base_addr + 0x00c, 0x00000002);
    write(cfg_base_addr + 0x004, 0x300b0000);
    write(cfg_base_addr + 0x00c, 0x00000002);
    write(cfg_base_addr + 0x004, 0x30160000);
    write(cfg_base_addr + 0x00c, 0x00000002);
    write(cfg_base_addr + 0x010, 0x00000010);
    write(cfg_base_addr + 0x014, 0x00000001);
    // Waits tZQCAL (1 us)
    udelay(4);
    write(cfg_base_addr + 0x010, 0x00000011);
    write(cfg_base_addr + 0x014, 0x00000001);
    write(cfg_base_addr + 0x010, 0x00000020);
    write(cfg_base_addr + 0x014, 0x00000001);
    // Waits tZQCAL (1 us)
    udelay(4);
    write(cfg_base_addr + 0x010, 0x00000021);
    write(cfg_base_addr + 0x014, 0x00000001);
    write(cfg_base_addr + 0x514, 0x00000000);

    // some polling again
    let mut tmp = read(cfg_base_addr + 0x518);
    while (tmp & 0x00000002) != 0x00000002 {
        udelay(1);
        tmp = read(cfg_base_addr + 0x518);
    }

    train(phy_base_addr, cfg_base_addr, ddr_num);

    //cdns_pi_end( 3);
    //read(PHY_APB_BASE_ADDR + (16'd2048 +16'd11 <<2), tmp);//`DENALI_PI_11_DATA
    //write(PHY_APB_BASE_ADDR + (16'd2048 +16'd11 <<2), tmp & ~4'hF | 4'h3);//set PI_CS_MAP=3
    tmp = read_pi(phy_base_addr, 83); //read pi_int_status
    tmp = read_pi(phy_base_addr, 84);
    write_pi(phy_base_addr, 84, tmp & 0xf8000000);

    //cdns_rdlvl_gate_tr_init( 3,0,0,0,0);
    tmp = read_pi(phy_base_addr, 46);
    write_pi(phy_base_addr, 46, tmp & !(0xf << 24) | (0x3 << 24));
    tmp = read_pi(phy_base_addr, 33);
    write_pi(phy_base_addr, 33, tmp & !(0x1 << 24));
    tmp = read_pi(phy_base_addr, 44);
    write_pi(phy_base_addr, 44, tmp & !(0x1 << 16));
    tmp = read_pi(phy_base_addr, 44);
    write_pi(phy_base_addr, 44, tmp & !(0x1 << 24));
    tmp = read_pi(phy_base_addr, 45);
    write_pi(phy_base_addr, 45, tmp | 0x1);
    tmp = read_pi(phy_base_addr, 146);
    write_pi(phy_base_addr, 146, tmp | (0x3 << 24));
    tmp = read_pi(phy_base_addr, 147);
    write_pi(phy_base_addr, 147, tmp | (0x3 << 8));
    tmp = read_pi(phy_base_addr, 147);
    write_pi(phy_base_addr, 147, tmp | (0x3 << 24));
    tmp = read_pi(phy_base_addr, 44);
    write_pi(phy_base_addr, 44, tmp | (0x1 << 8));

    //cdns_rdlvl_tr_init( 3,0,0,0,0);
    tmp = read_pi(phy_base_addr, 46);
    write_pi(phy_base_addr, 46, tmp & !(0xf << 16) | (0x3 << 16));
    tmp = read_pi(phy_base_addr, 33);
    write_pi(phy_base_addr, 33, tmp & !(0x1 << 16));
    tmp = read_pi(phy_base_addr, 43);
    write_pi(phy_base_addr, 43, tmp & !(0x1 << 16));
    tmp = read_pi(phy_base_addr, 43);
    write_pi(phy_base_addr, 43, tmp & !(0x1 << 24));
    tmp = read_pi(phy_base_addr, 44);
    write_pi(phy_base_addr, 44, tmp | 0x1);
    tmp = read_pi(phy_base_addr, 146);
    write_pi(phy_base_addr, 146, tmp | (0x3 << 16));
    tmp = read_pi(phy_base_addr, 147);
    write_pi(phy_base_addr, 147, tmp | 0x3);
    tmp = read_pi(phy_base_addr, 147);
    write_pi(phy_base_addr, 147, tmp | (0x3 << 16));
    tmp = read_pi(phy_base_addr, 148);
    write_pi(phy_base_addr, 148, tmp | (0x3 << 24));
    tmp = read_pi(phy_base_addr, 149);
    write_pi(phy_base_addr, 149, tmp | (0x3 << 24));
    tmp = read_pi(phy_base_addr, 150);
    write_pi(phy_base_addr, 150, tmp | (0x3 << 24));
    tmp = read_pi(phy_base_addr, 43);
    write_pi(phy_base_addr, 43, tmp | (0x1 << 8));

    //cdns_wdqlvl_tr_init( 3,0,0,0,0);
    tmp = read_pi(phy_base_addr, 67);
    write_pi(phy_base_addr, 67, tmp & !(0xf << 8) | (0x3 << 8));
    tmp = read_pi(phy_base_addr, 68);
    write_pi(phy_base_addr, 68, tmp & !(0x1 << 8));
    tmp = read_pi(phy_base_addr, 71);
    write_pi(phy_base_addr, 71, tmp & !(0x1 << 16));
    tmp = read_pi(phy_base_addr, 71);
    write_pi(phy_base_addr, 71, tmp & !(0x1 << 24));
    tmp = read_pi(phy_base_addr, 72);
    write_pi(phy_base_addr, 72, tmp | (0x1 << 8));
    tmp = read_pi(phy_base_addr, 180);
    write_pi(phy_base_addr, 180, tmp | (0x3 << 8));
    tmp = read_pi(phy_base_addr, 183);
    write_pi(phy_base_addr, 183, tmp | (0x3 << 8));
    tmp = read_pi(phy_base_addr, 186);
    write_pi(phy_base_addr, 186, tmp | (0x3 << 8));

    write(cfg_base_addr + 0x100, 0x000000e0);
    write(cfg_base_addr + 0x620, 0x03031417);
    write(cfg_base_addr + 0x624, 0x08100608);
    write(cfg_base_addr + 0x628, 0x402d0894);
    write(cfg_base_addr + 0x62c, 0x271e0b2b);
    write(cfg_base_addr + 0x630, 0x281b140a);
    write(cfg_base_addr + 0x634, 0x001c0000);
    write(cfg_base_addr + 0x638, 0x200f0f08);
    write(cfg_base_addr + 0x63c, 0x28410a06); //twrcsgap[11:8]>=a(2*5), trdcsgap[3:0]>=6(2*3)
    write(cfg_base_addr + 0x640, 0x01331f8b);
    write(cfg_base_addr + 0x644, 0x1096012b);
    write(cfg_base_addr + 0x648, 0x00000000);
    write(cfg_base_addr + 0x64c, 0x00082714);
    write(cfg_base_addr + 0x650, 0x16442f0d);
    write(cfg_base_addr + 0x654, 0x00001916);
    write(cfg_base_addr + 0x658, 0x00000060);
    write(cfg_base_addr + 0x65c, 0x00600020);
    write(cfg_base_addr + 0x660, 0x00000000);
    write(cfg_base_addr + 0x680, 0x0c00040f); //trdcslat=0xe->0xc;trddata_en=0xe->0xf
    write(cfg_base_addr + 0x684, 0x03000604); //twrcslat=0x4->0x3;twrlat=0x6;twrdata=0x4
    write(cfg_base_addr + 0x688, 0x0415040d);
    write(cfg_base_addr + 0x68c, 0x20002c20);
    write(cfg_base_addr + 0x690, 0x00140000);
    write(cfg_base_addr + 0x69c, 0x01240074);
    write(cfg_base_addr + 0x6a0, 0x00000000);
    write(cfg_base_addr + 0x6a4, 0x202c0c00);
    write(cfg_base_addr + 0x6a8, 0x00040000);
    write(cfg_base_addr + 0x004, 0x30010036);
    write(cfg_base_addr + 0x00c, 0x00000002);
    write(cfg_base_addr + 0x004, 0x3002001b);
    write(cfg_base_addr + 0x00c, 0x00000002);
    write(cfg_base_addr + 0x004, 0x30030031);
    write(cfg_base_addr + 0x00c, 0x00000002);
    write(cfg_base_addr + 0x004, 0x300b0036); //0x06->0x66
    write(cfg_base_addr + 0x00c, 0x00000002);
    write(cfg_base_addr + 0x004, 0x3016001e); //0x06->0x1e
    write(cfg_base_addr + 0x00c, 0x00000002);
    write(cfg_base_addr + 0x004, 0x300c0030); //vref(ca)
    write(cfg_base_addr + 0x00c, 0x00000002);
    write(cfg_base_addr + 0x004, 0x300e0020); //vref(dq)
    write(cfg_base_addr + 0x00c, 0x00000002);
    write(cfg_base_addr + 0x410, 0x00101010);
    write(cfg_base_addr + 0x420, 0x0c181006);
    write(cfg_base_addr + 0x424, 0x20200820);
    write(cfg_base_addr + 0x428, 0x80000020);
    write(cfg_base_addr + 0x000, 0x00000001);
    write(cfg_base_addr + 0x108, 0x00003000);
    write(sec_base_addr + 0x704, 0x00000007);
    write(cfg_base_addr + 0x330, 0x09313fff);
    write(cfg_base_addr + 0x508, 0x00000013);
    write(cfg_base_addr + 0x324, 0x00002000);
    write(cfg_base_addr + 0x104, 0x90000000);
    write(cfg_base_addr + 0x510, 0x00000100);
    write(cfg_base_addr + 0x514, 0x00000000);
    write(sec_base_addr + 0x700, 0x00000003);
    write(cfg_base_addr + 0x514, 0x00000600);
    write(cfg_base_addr + 0x020, 0x00000001);
}

const DRAM_BASE_UNCACHED: usize = 0x10_0000_0000;
const DRAM_BASE_CACHED: usize = 0x00_8000_0000;

fn dram_test_pattern(base: usize, reg: usize, pattern: u32) {
    let addr = base + reg * 4;
    let next = addr + 4;
    write(addr, pattern);
    let value = read(addr);
    if value != pattern {
        println!("{addr:08x} = {value:08x} expected {pattern:08x}");
    }
    // also test next address
    write(next, pattern);
    let value = read(next);
    if value != pattern {
        println!("{next:08x} = {value:08x} expected {pattern:08x}");
    }
    // recheck on previous address
    let value = read(addr);
    if value != pattern {
        println!("{addr:08x} = {value:08x} expected {pattern:08x}");
    }
    // let's see if this messes things up...
    write(addr + 4096, pattern);
    read(addr + 4096);
    // ... does it?
    let value = read(addr);
    if value != pattern {
        println!("{addr:08x} = {value:08x} expected {pattern:08x}");
    }
}

const FULL_TEST: bool = false;

fn dram_test_range(start: usize, end: usize) {
    for reg in start..end {
        dram_test_pattern(DRAM_BASE_UNCACHED, reg, 0xa5a5a5a5);
        dram_test_pattern(DRAM_BASE_UNCACHED, reg, 0x5a5a5a5a);
        dram_test_pattern(DRAM_BASE_UNCACHED, reg, 0x00000000);
        dram_test_pattern(DRAM_BASE_UNCACHED, reg, 0xffffffff);

        dram_test_pattern(DRAM_BASE_CACHED, reg, 0xa5a5a5a5);
        dram_test_pattern(DRAM_BASE_CACHED, reg, 0x5a5a5a5a);
        dram_test_pattern(DRAM_BASE_CACHED, reg, 0x00000000);
        dram_test_pattern(DRAM_BASE_CACHED, reg, 0xffffffff);

        // progress every megabyte
        if ((reg + 1) % 0x40000) == 0 {
            println!("DDR @{:08x}, {}M test done", (reg + 1) * 4, (reg + 1) >> 18);
        }
    }
}

pub fn dram_test() {
    // 2MB @ lower 4GB
    dram_test_range(0x0000_0000, 0x0008_0000);
    if !FULL_TEST {
        return;
    }
    // 2MB @ 512M
    dram_test_range(0x0800_0000, 0x0808_0000);
    // 2MB @ 4G
    dram_test_range(0x4000_0000, 0x4008_0000);
    // 2MB @ 6G
    dram_test_range(0x6000_0000, 0x6008_0000);
}

fn dram_clocks() {
    // reset ddrphy,unvalid
    crate::init::assert_rstgen_rstn_ddrphy_ahb();
    // Set PLL to 15750M
    crate::init::clk_dla_root_osc_sys();
    crate::init::syscon_pll1_reset(true);
    udelay(10); //wait(500*(1/25M))
    crate::init::syscon_pll1_reset(false);
    udelay(10); //wait(500*(1/25M))

    crate::init::clk_dla_root_pll1_out();
    // set clock dividers
    crate::init::clk_ddrc0_osc_div2();
    crate::init::clk_ddrc1_osc_div2();
    // enable clocks
    crate::init::enable_clk_ddrc0();
    crate::init::enable_clk_ddrc1();
}

pub fn init() {
    println!("DRAM init");
    dram_clocks();
    println!("DRAM clocks done");

    //---- config DDRPHY0/OMC0 ----
    println!("DRAM PHY0 init");
    crate::dram_phy::init(PHY0_BASE_ADDR);
    println!("DRAM PHY0 PI");
    crate::dram_pi::init(PHY0_BASE_ADDR);
    println!("DRAM PHY0 start");
    crate::dram_pi_start::init(PHY0_BASE_ADDR, 0);
    println!("DRAM PHY0 clock");
    crate::init::clk_ddrc0_osc_div2();
    udelay(300);
    // release DLL_RST_N
    write(PHY0_BASE_ADDR, 0x01);
    udelay(300);
    omc_init(PHY0_BASE_ADDR, CFG0_BASE_ADDR, SEC0_BASE_ADDR, 0);
    println!("DRAM PHY0 done");

    //---- config DDRPHY1/OMC1 ----
    crate::dram_phy::init(PHY1_BASE_ADDR);
    crate::dram_pi::init(PHY1_BASE_ADDR);
    crate::dram_pi_start::init(PHY1_BASE_ADDR, 1);
    crate::init::clk_ddrc1_osc_div2();
    udelay(300);
    // release DLL_RST_N
    write(PHY1_BASE_ADDR, 0x01);
    udelay(300);
    omc_init(PHY1_BASE_ADDR, CFG1_BASE_ADDR, SEC1_BASE_ADDR, 1);
    println!("DRAM PHY1 done");
    dram_test();
    println!("DRAM test done");
    crate::init::disable_u74_memaxi_remap(1);
    let addr = DRAM_BASE_CACHED;
    let value = read(addr);
    match value {
        0xffffffff => {
            println!("0x{addr:08x} is 0x{value:08x} - good!");
        }
        _ => {
            println!("0x{addr:08x} is 0x{value:08x} - expected 0xffffffff");
        }
    }
}
