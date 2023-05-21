use bl808_pac::GLB;

const GLB_BASE: usize = 0x2000_0000;
pub const SWRST_CFG2: usize = GLB_BASE + 0x0548;
const CPU_RST: u32 = 1 << 1;

pub const MM_BASE: usize = 0x3000_0000;

const MM_GLB_BASE: usize = 0x3000_7000;
const MM_SYS_CTRL: usize = MM_GLB_BASE;
const MM_SW_SYS_RESET: usize = MM_GLB_BASE + 0x0040;
const SYS_RESET: u32 = 1 << 0;
const PWRON_RST: u32 = 1 << 2;
const MMCPU0_RESET: u32 = 1 << 8;
const MMCPU1_RESET: u32 = 1 << 9;
const WL2MM_RST_MSK: u32 = 1 << 15;
const MM_RESUME: u32 = SYS_RESET | PWRON_RST | MMCPU1_RESET | WL2MM_RST_MSK;
const MM_CLK_EN: u32 = 1 << 12;

pub fn reset_cpu() {
    unsafe {
        let s = core::ptr::read_volatile(SWRST_CFG2 as *mut u32);
        core::ptr::write_volatile(SWRST_CFG2 as *mut u32, s | CPU_RST);
    }
}

// see https://github.com/smaeul/opensbi/commit/487866632bba84871e773c90bf874ab6d81065aa
pub fn resume_mm(entry: u32) {
    unsafe {
        /* Flush the data cache */
        core::arch::asm!(".word 0x0010000b");
        core::arch::asm!(".word 0x01a0000b");
        core::ptr::write_volatile(MM_BASE as *mut u32, entry);
        let e = core::ptr::read_volatile(MM_SYS_CTRL as *mut u32);
        core::ptr::write_volatile(MM_SYS_CTRL as *mut u32, e | MM_CLK_EN);
        let s = core::ptr::read_volatile(MM_SW_SYS_RESET as *mut u32);
        core::ptr::write_volatile(MM_SW_SYS_RESET as *mut u32, s & MM_RESUME);
    }
}

pub fn gpio_uart_init(glb: &GLB) {
    /* GPIO mode config */
    glb.gpio_config[14].write(|w| w.alternate().uart().output_set().set_bit());
    glb.gpio_config[15].write(|w| {
        w.alternate()
            .uart()
            .input_function()
            .set_bit()
            .pull_up()
            .set_bit()
    });
    glb.gpio_config[16].write(|w| w.alternate().uart().output_set().set_bit());
    glb.gpio_config[17].write(|w| {
        w.alternate()
            .uart()
            .input_function()
            .set_bit()
            .pull_up()
            .set_bit()
    });
    /* GPIO UART function config */
    glb.uart_signal_0.write(|w| {
        w.function_02()
            .uart0_txd()
            .function_03()
            .uart0_rxd()
            .function_04()
            .uart1_txd()
            .function_05()
            .uart1_rxd()
    });
    /* Enable UART clock */
    glb.uart_config.write(|w| w.clock_enable().set_bit());
}
