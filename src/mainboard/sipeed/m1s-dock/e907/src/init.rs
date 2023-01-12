use core::ptr::{read_volatile, write_volatile};

/*
    GPIO_FUN_SDH = 0,
    GPIO_FUN_SPI0 = 1,
    GPIO_FUN_FLASH = 2,
    GPIO_FUN_I2S = 3,
    GPIO_FUN_PDM = 4,
    GPIO_FUN_I2C0 = 5,
    GPIO_FUN_I2C1 = 6,
    GPIO_FUN_UART = 7,
    GPIO_FUN_ETHER_MAC = 8,
    GPIO_FUN_CAM = 9,
    GPIO_FUN_ANALOG = 10,
    GPIO_FUN_GPIO = 11,
    GPIO_FUN_PWM0 = 16,
    GPIO_FUN_PWM1 = 17,
    GPIO_FUN_SPI1 = 18,
    GPIO_FUN_I2C2 = 19,
    GPIO_FUN_I2C3 = 20,
    GPIO_FUN_MM_UART = 21,
    GPIO_FUN_DBI_B = 22,
    GPIO_FUN_DBI_C = 23,
    GPIO_FUN_DPI = 24,
    GPIO_FUN_JTAG_LP = 25,
    GPIO_FUN_JTAG_M0 = 26,
    GPIO_FUN_JTAG_D0 = 27,
    GPIO_FUN_CLOCK_OUT = 31,
 */

/**
|    31    30    29    28                 27    26    25    24    |

|    23    22    21    20                 19    18    17    16    |

|    15    14    13    12                 11    10     9     8    |
                       ------------ GPIO function/mode -----------
|     7     6     5     4                  3     2     1     0    |
          output      pull up                              input
*/

const GPIO_FUN_UART: u32 = 7 << 8;
const GPIO_FUN_MM_UART: u32 = 21 << 8;

const GPIO14_UART0TX: u32 = 2 << 8;
const GPIO15_UART0RX: u32 = 3 << 12;
const GPIO16_UART1TX: u32 = 6 << 16;
const GPIO17_UART1RX: u32 = 7 << 20;
const UART_GPIO_CFG: u32 = GPIO14_UART0TX | GPIO15_UART0RX | GPIO16_UART1TX | GPIO17_UART1RX;
const UART_GPIO_MASK: u32 = 0xff0000ff;

const UART_CLK_EN: u32 = 1 << 4;

const GPIO_MODE_IN: u32 = 1 << 0;
const GPIO_PULL_UP: u32 = 1 << 4;
const GPIO_MODE_OUT: u32 = 1 << 6;

const GLB_BASE: usize   = 0x2000_0000;
const UART_CFG0: usize  = GLB_BASE + 0x0150;
const UART_CFG1: usize  = GLB_BASE + 0x0154;
pub const SWRST_CFG2: usize = GLB_BASE + 0x0548;
const GPIO_CFG0: usize  = GLB_BASE + 0x08c4;
const GPIO_CFG11: usize = GLB_BASE + 0x08f0;
const GPIO_CFG12: usize = GLB_BASE + 0x08f4;
const GPIO_CFG13: usize = GLB_BASE + 0x08f8;
const GPIO_CFG14: usize = GLB_BASE + 0x08fc;
const GPIO_CFG15: usize = GLB_BASE + 0x0900;
const GPIO_CFG16: usize = GLB_BASE + 0x0904;
const GPIO_CFG17: usize = GLB_BASE + 0x0908;

const UART0_BASE: usize = 0x2000_a000;
const UART0_TX_CFG: usize = UART0_BASE;
const UART0_BIT_PRD: usize = UART0_BASE + 0x0008;
pub const UART0_FIFO_WDATA: usize = UART0_BASE + 0x0088;

const UART1_BASE: usize = 0x2000_a100;
const UART1_TX_CFG: usize = UART1_BASE;
const UART1_BIT_PRD: usize = UART1_BASE + 0x0008;
pub const UART1_FIFO_WDATA: usize = UART1_BASE + 0x0088;

const UART_TX_STOP: u32 = 2 << 11; // stop bits
const UART_TX_LEN: u32 = 7 << 8; // word size
const UART_TX_FRM_EN: u32 = 1 << 2; // freerun mode
const UART_TX_EN: u32 = 1 << 0;
const UART_TX_CFG: u32 = UART_TX_STOP | UART_TX_LEN | UART_TX_FRM_EN | UART_TX_EN;

pub unsafe fn gpio_uart_init() {
    /* GPIO mode config */
    let cfg_uart_tx = GPIO_FUN_UART | GPIO_MODE_OUT;
    let cfg_uart_rx = GPIO_FUN_UART | GPIO_PULL_UP | GPIO_MODE_IN;
    write_volatile(GPIO_CFG14 as *mut u32, cfg_uart_tx);
    write_volatile(GPIO_CFG15 as *mut u32, cfg_uart_rx);
    write_volatile(GPIO_CFG16 as *mut u32, cfg_uart_tx);
    write_volatile(GPIO_CFG17 as *mut u32, cfg_uart_rx);
    
    /* GPIO UART function config */
    // GPIO14: UART0 TXD
    // GPIO15: UART0 RXD
    // GPIO16: UART1 TXD
    // GPIO17: UART1 RXD
    let cfg1 = read_volatile(UART_CFG1 as *mut u32);
    let uart_cfg = cfg1 & UART_GPIO_MASK | UART_GPIO_CFG;
    write_volatile(UART_CFG1 as *mut u32, uart_cfg);

    /* Enable UART clock */
    let cfg0 = read_volatile(UART_CFG0 as *mut u32);
    write_volatile(UART_CFG0 as *mut u32, cfg0 | UART_CLK_EN);
}

pub unsafe fn uart_init() {
    // TX config
    write_volatile(UART0_TX_CFG as *mut u32, UART_TX_CFG);
    write_volatile(UART1_TX_CFG as *mut u32, UART_TX_CFG);

    /* baud rate configuration */
    // lower 16 bits are for TX; default (mask ROM) is 0x02b4 or 0x02b5
    let b0 = read_volatile(UART0_BIT_PRD as *mut u32);
    // let b1 = read_volatile(UART1_BIT_PRD as *mut u32);
    // set to the same as b0
    write_volatile(UART1_BIT_PRD as *mut u32, b0);
}

#[derive(Debug)]
pub struct Serial();

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

impl Serial {
    #[inline]
    pub fn new() -> Self {
        unsafe { uart_init(); }
        Self()
    }
}

impl embedded_hal::serial::ErrorType for Serial {
    type Error = Error;
}

// impl embedded_hal::serial::nb::Write<u8> for Serial {
impl embedded_hal::serial::blocking::Write<u8> for Serial {
    #[inline]
    // fn write(&mut self, c: u8) -> nb::Result<(), self::Error> {
    fn write(&mut self, c: &[u8]) -> Result<(), self::Error> {
        // TODO
        if false {
            // return Err(nb::Error::WouldBlock);
        }
        unsafe { write_volatile(UART1_FIFO_WDATA as *mut u32, c[0] as u32); }
        Ok(())
    }

    #[inline]
    // fn flush(&mut self) -> nb::Result<(), self::Error> {
    fn flush(&mut self) -> Result<(), self::Error> {
        // TODO
        /*
        if true {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
        */
        Ok(())
    }
}
