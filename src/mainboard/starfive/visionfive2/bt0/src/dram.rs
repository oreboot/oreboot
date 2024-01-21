use soc::starfive::jh7110::pac;

use crate::ddr_start::start;
use crate::ddrcsr::omc_init;
use crate::ddrphy::{train, util};
use crate::init::{self, read32, udelay, write32};
use crate::pll;

// TODO: support 1G
#[cfg(not(any(dram_size = "2G", dram_size = "4G", dram_size = "8G")))]
core::compile_error!("unsupported DRAM size or none set");

// see StarFive U-Boot drivers/ram/starfive/starfive_ddr.c
pub fn init() {
    // TODO: determine DRAM size from EEPROM at runtime, it's stored on board.
    // That requires I2C first, see `arch/riscv/cpu/jh7110/dram.c` in U-Boot.
    let dram_size = if cfg!(dram_size = "2G") {
        2
    } else if cfg!(dram_size = "4G") {
        4
    } else if cfg!(dram_size = "8G") {
        8
    } else {
        0 // does not actually occur due to build-time check
    };
    println!("DRAM size: {dram_size}G");
    unsafe {
        println!("[DRAM] init start");
        println!("[DRAM] set clk to OSC div2");
        init::clk_ddrc_osc_div2();
        println!("[DRAM] set PLL frequency");
        pll::pll1_set_freq(pll::PLL1_DDR2133_1066000000);
        udelay(500);
        println!("[DRAM] set clk to PLL1 div2");
        init::clk_ddrc_pll1_div2();
        udelay(200);

        println!("[DRAM] asserts");
        let syscrg = pac::syscrg_reg();

        // DDR OSC
        syscrg
            .soft_rst_addr_sel_1()
            .modify(|_, w| w.u0_ddr_osc().set_bit());
        while syscrg
            .syscrg_rst_status_1()
            .read()
            .u0_ddr_osc()
            .bit_is_set()
        {
            udelay(1);
        }
        syscrg
            .soft_rst_addr_sel_1()
            .modify(|_, w| w.u0_ddr_osc().clear_bit());
        while syscrg
            .syscrg_rst_status_1()
            .read()
            .u0_ddr_osc()
            .bit_is_clear()
        {
            udelay(1);
        }
        // DDR APB
        syscrg
            .soft_rst_addr_sel_1()
            .modify(|_, w| w.u0_ddr_apb().set_bit());
        while syscrg
            .syscrg_rst_status_1()
            .read()
            .u0_ddr_apb()
            .bit_is_set()
        {
            udelay(1);
        }
        syscrg
            .soft_rst_addr_sel_1()
            .modify(|_, w| w.u0_ddr_apb().clear_bit());
        while syscrg
            .syscrg_rst_status_1()
            .read()
            .u0_ddr_apb()
            .bit_is_clear()
        {
            udelay(1);
        }
        // DDR AXI
        syscrg
            .soft_rst_addr_sel_1()
            .modify(|_, w| w.u0_ddr_axi().set_bit());
        while syscrg
            .syscrg_rst_status_1()
            .read()
            .u0_ddr_axi()
            .bit_is_set()
        {
            udelay(1);
        }
        syscrg
            .soft_rst_addr_sel_1()
            .modify(|_, w| w.u0_ddr_axi().clear_bit());
        while syscrg
            .syscrg_rst_status_1()
            .read()
            .u0_ddr_axi()
            .bit_is_clear()
        {
            udelay(1);
        }

        // inlined from ddr_setup()
        println!("[DRAM] train"); // dram_pi::init in VF1 code
        train();
        println!("[DRAM] util"); // dram_phy::init in VF1 code
        util();
        println!("[DRAM] start");
        start();
        println!("[DRAM] set clk to OSC div2");
        init::clk_ddrc_osc_div2();
        println!("[DRAM] boot");
        omc_init();
        println!("[DRAM] init done");
    }
}
