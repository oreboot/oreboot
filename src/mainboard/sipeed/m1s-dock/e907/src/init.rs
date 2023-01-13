use bl808_pac::GLB;

const GLB_BASE: usize   = 0x2000_0000;
pub const SWRST_CFG2: usize = GLB_BASE + 0x0548;
const CPU_RST: u32 = 1 << 1;

const MM_GLB_BASE: usize = 0x3000_7000;
const MM_SW_SYS_RESET: usize = MM_GLB_BASE + 0x0040;
const SYS_RESET: u32 = 1 << 0;
const PWRON_RST: u32 = 1 << 2;
const MMCPU0_RESET: u32 = 1 << 8;
const MMCPU1_RESET: u32 = 1 << 9;
const WL2MM_RST_MSK: u32 = 1 << 15;
const MM_RESUME: u32 = SYS_RESET | PWRON_RST | MMCPU1_RESET | WL2MM_RST_MSK;

pub fn reset_cpu() {
    unsafe {
        let s = core::ptr::read_volatile(SWRST_CFG2 as *mut u32);
        core::ptr::write_volatile(SWRST_CFG2 as *mut u32, s | CPU_RST);
    }
}

pub fn resume_mm() {
    unsafe {
        let s = core::ptr::read_volatile(MM_SW_SYS_RESET as *mut u32);
        core::ptr::write_volatile(MM_SW_SYS_RESET as *mut u32, s & MM_RESUME);
    }
}

pub unsafe fn gpio_uart_init(glb: &GLB) {
    /* GPIO mode config */
    glb.gpio_config[14].write(|w| {
        w.alternate().uart().output_set().set_bit()
    });
    glb.gpio_config[15].write(|w| {
        w.alternate().uart().input_function().set_bit().pull_up().set_bit()
    });
    glb.gpio_config[16].write(|w| {
        w.alternate().uart().output_set().set_bit()
    });
    glb.gpio_config[17].write(|w| {
        w.alternate().uart().input_function().set_bit().pull_up().set_bit()
    });
    /* GPIO UART function config */
    glb.uart_signal_0.write(|w| {
        w.function_02().uart0_txd()
         .function_03().uart0_rxd()
         .function_04().uart1_txd()
         .function_05().uart1_rxd()
    });
    /* Enable UART clock */
    glb.uart_config.write(|w| w.clock_enable().set_bit());
}

use core::ops::Deref;

pub trait Instance0: Deref<Target = bl808_pac::uart::RegisterBlock> {}
pub trait Instance1: Deref<Target = bl808_pac::uart::RegisterBlock> {}

impl Instance0 for bl808_pac::UART0 {}
impl Instance1 for bl808_pac::UART1 {}

#[derive(Debug)]
pub struct Serial<UART0: Instance0, UART1: Instance1> {
    u0: UART0,
    u1: UART1,
}

/// Error types that may happen when serial transfer
#[derive(Debug)]
pub struct Error {
    kind: embedded_hal::serial::ErrorKind,
}

impl embedded_hal::serial::Error for Error {
    #[inline]
    fn kind(&self) -> embedded_hal::serial::ErrorKind {
        self.kind
    }
}

impl<UART0: Instance0, UART1: Instance1> Serial<UART0, UART1> {
    #[inline]
    pub fn new(u0: UART0, u1: UART1) -> Self {
        // TX config
        u0.transmit_config.write(|w| {
            w.word_length().eight()
                .stop_bits().one()
                .freerun().enable()
                .function().enable()
        });
        u1.transmit_config.write(|w| {
            w.word_length().eight()
                .stop_bits().one()
                .freerun().enable()
                .function().enable()
        });
        /* baud rate configuration */
        let period = u0.bit_period.read();
        let rxp = period.receive().bits();
        let txp = period.transmit().bits();
        u1.bit_period.write(|w| {
            w.transmit().variant(txp)
                .receive().variant(rxp)
        });
        Self{u0, u1}
    }

    pub fn debug(&mut self, num: u8) {
        self.u0.data_write.write(|w| w.value().variant(num));
    }
}

impl<UART0: Instance0, UART1: Instance1> embedded_hal::serial::ErrorType for Serial<UART0, UART1> {
    type Error = Error;
}

impl<UART0: Instance0, UART1: Instance1> embedded_hal::serial::nb::Write<u8> for Serial<UART0, UART1> {
    #[inline]
    fn write(&mut self, c: u8) -> nb::Result<(), self::Error> {
        if self.u1.bus_state.read().transmit_busy().is_busy() {
            return Err(nb::Error::WouldBlock);
        }
        self.u1.data_write.write(|w| w.value().variant(c));
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> nb::Result<(), self::Error> {
        // TODO
        if true {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}
