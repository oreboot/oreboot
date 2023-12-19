use crate::ddr_start::start;
use crate::ddrcsr::omc_init;
use crate::ddrphy::{train, util};
use crate::init::{self, read32, udelay, write32};
use crate::{pac, pll};

// see StarFive U-Boot drivers/ram/starfive/starfive_ddr.c
pub fn init() {
    // TODO: determine DRAM size from EEPROM at runtime, it's stored on board.
    // That requires I2C first, see `arch/riscv/cpu/jh7110/dram.c` in U-Boot.
    // FIXME: This does not work as of now. It did in a std test project though.
    println!(
        "DRAM: 4G: {} 2G: {}",
        cfg!(dram_size = "4G"),
        cfg!(dram_size = "2G")
    );
    if cfg!(dram_size = "2G") {
        println!("2G shitty DRAM");
    }
    if cfg!(dram_size = "4G") {
        println!("4G sparkling DRAM");
    }
    if cfg!(dram_size = "8G") {
        println!("8G shiny DRAM");
    }
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
        train(init::DDR_PHY_CTRL_BASE);
        println!("[DRAM] util"); // dram_phy::init in VF1 code
        util(init::DDR_PHY_AC_BASE);
        println!("[DRAM] start");
        start(
            init::DDR_PHY_BASE,
            init::DDR_PHY_CTRL_BASE,
            init::DDR_PHY_AC_BASE,
        );
        println!("[DRAM] set clk to OSC div2");
        init::clk_ddrc_osc_div2();
        println!("[DRAM] boot");
        omc_init(
            init::DDR_PHY_BASE,
            init::DDR_CTRL_BASE,
            init::DDR_SEC_CTRL_BASE,
            init::DDR_PHY_CTRL_BASE,
        );
        println!("[DRAM] init done");
    }
}
