use core::ptr::{read_volatile, write_volatile};
use log::{Error, Serial};

// https://www.lammertbies.nl/comm/info/serial-uart
const UART3_BASE: u32 = 0x1244_0000;

pub const UART3_THR: u32 = UART3_BASE + 0x0000; /* Transmitter holding reg. */
const UART3_RDR: u32 = UART3_BASE + 0x0000; /* Receiver data reg.       */
const UART3_BRDL: u32 = UART3_BASE + 0x0000; /* Baud rate divisor (LSB)  */
const UART3_BRDH: u32 = UART3_BASE + 0x0004; /* Baud rate divisor (MSB)  */
const UART3_IER: u32 = UART3_BASE + 0x0004; /* Interrupt enable reg.    */
const UART3_IIR: u32 = UART3_BASE + 0x0008; /* Interrupt ID reg.        */
const UART3_FCR: u32 = UART3_BASE + 0x0008; /* FIFO control reg.        */
const UART3_LCR: u32 = UART3_BASE + 0x000c; /* Line control reg.        */
const UART3_MDC: u32 = UART3_BASE + 0x0010; /* Modem control reg.       */
const UART3_LSR: u32 = UART3_BASE + 0x0014; /* Line status reg.         */
const UART3_MSR: u32 = UART3_BASE + 0x0018; /* Modem status reg.        */
const UART3_DLF: u32 = UART3_BASE + 0x0300; /* Divisor Latch Fraction   */

/* constants for line control register */

const LCR_CS5: u8 = 0x00; /* 5 bits data size */
const LCR_CS6: u8 = 0x01; /* 6 bits data size */
const LCR_CS7: u8 = 0x02; /* 7 bits data size */
const LCR_CS8: u8 = 0x03; /* 8 bits data size */
const LCR_2_STB: u8 = 0x04; /* 2 stop bits */
const LCR_1_STB: u8 = 0x00; /* 1 stop bit */
const LCR_PEN: u8 = 0x08; /* parity enable */
const LCR_PDIS: u8 = 0x00; /* parity disable */
const LCR_EPS: u8 = 0x10; /* even parity select */
const LCR_SP: u8 = 0x20; /* stick parity select */
const LCR_SBRK: u8 = 0x40; /* break control bit */
const LCR_DLAB: u8 = 0x80; /* divisor latch access enable */

/* constants for line status register */

const LSR_RXRDY: u8 = 0x01; /* receiver data available */
const LSR_OE: u8 = 0x02; /* overrun error */
const LSR_PE: u8 = 0x04; /* parity error */
const LSR_FE: u8 = 0x08; /* framing error */
const LSR_BI: u8 = 0x10; /* break interrupt */
const LSR_EOB_MASK: u8 = 0x1E; /* Error or Break mask */
const LSR_THRE: u8 = 0x20; /* transmit holding register empty */
const LSR_TEMT: u8 = 0x40; /* transmitter empty */

/* equates for FIFO control register */

const FCR_FIFO: u8 = 0x01; /* enable XMIT and RCVR FIFO */
const FCR_RCVRCLR: u8 = 0x02; /* clear RCVR FIFO */
const FCR_XMITCLR: u8 = 0x04; /* clear XMIT FIFO */

const FCR_MODE0: u8 = 0x00; /* set receiver in mode 0 */
const FCR_MODE1: u8 = 0x08; /* set receiver in mode 1 */

/* RCVR FIFO interrupt levels: trigger interrupt with this bytes in FIFO */
const FCR_FIFO_1: u8 = 0x00; /* 1 byte in RCVR FIFO */
const FCR_FIFO_4: u8 = 0x40; /* 4 bytes in RCVR FIFO */
const FCR_FIFO_8: u8 = 0x80; /* 8 bytes in RCVR FIFO */
const FCR_FIFO_14: u8 = 0xC0; /* 14 bytes in RCVR FIFO */

const UART_CLK: u32 = 100_000_000;
const UART_BAUDRATE_32MCLK_115200: u32 = 115200;

fn read_8(reg: u32) -> u8 {
    unsafe { read_volatile(reg as *mut u8) }
}

fn write_8(reg: u32, val: u8) {
    unsafe {
        write_volatile(reg as *mut u8, val);
    }
}

pub fn uart_write(c: char) {
    while read_8(UART3_LSR) & LSR_THRE == 0 {}
    write_8(UART3_THR, c as u8);
}

pub fn uart_init() {
    let divisor = (UART_CLK / UART_BAUDRATE_32MCLK_115200) >> 4;

    let lcr_cache = read_8(UART3_LCR);
    /* clear DLAB */
    write_8(UART3_LCR, LCR_DLAB | lcr_cache);
    write_8(UART3_BRDL, divisor as u8);
    write_8(UART3_BRDH, (divisor >> 8) as u8);
    /* restore the DLAB to access the baud rate divisor registers */
    write_8(UART3_LCR, lcr_cache);

    /* 8 data bits, 1 stop bit, no parity */
    write_8(UART3_LCR, LCR_CS8 | LCR_1_STB | LCR_PDIS);

    /* disable flow control */
    write_8(UART3_MDC, 0);

    /*
     * Program FIFO: enabled, mode 0 (set for compatibility with quark),
     * generate the interrupt at 8th byte
     * Clear TX and RX FIFO
     */
    write_8(
        UART3_FCR,
        FCR_FIFO | FCR_MODE0 | FCR_FIFO_8 | FCR_RCVRCLR | FCR_XMITCLR,
    );

    write_8(UART3_IER, 0); // disable the serial interrupt
}

#[derive(Debug)]
pub struct JH71XXSerial();

impl JH71XXSerial {
    #[inline]
    pub fn new() -> Self {
        uart_init();
        Self()
    }
}

impl Serial for JH71XXSerial {}

impl embedded_hal_nb::serial::ErrorType for JH71XXSerial {
    type Error = Error;
}

impl embedded_hal_nb::serial::Write<u8> for JH71XXSerial {
    #[inline]
    fn write(&mut self, c: u8) -> nb::Result<(), self::Error> {
        if read_8(UART3_LSR) & LSR_THRE == 0 {
            return Err(nb::Error::WouldBlock);
        }
        write_8(UART3_THR, c as u8);
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> nb::Result<(), self::Error> {
        let TFE_EMPTY = true;
        if TFE_EMPTY {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}
