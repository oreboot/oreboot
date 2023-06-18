use core::ptr::{read_volatile, write_volatile};
use log::{Error, Serial};

// https://www.lammertbies.nl/comm/info/serial-uart
const UART0_BASE: usize = 0x1000_0000;

pub const UART0_THR: usize = UART0_BASE + 0x0000; /* Transmitter holding reg. */
const UART0_BRDL: usize = UART0_BASE + 0x0000; /* Baud rate divisor (LSB)  */
const UART0_BRDH: usize = UART0_BASE + 0x0004; /* Baud rate divisor (MSB)  */
const UART0_IER: usize = UART0_BASE + 0x0004; /* Interrupt enable reg.    */
const UART0_IIR: usize = UART0_BASE + 0x0008; /* Interrupt ID reg.        */
const UART0_FCR: usize = UART0_BASE + 0x0008; /* FIFO control reg.        */
const UART0_LCR: usize = UART0_BASE + 0x000c; /* Line control reg.        */
const UART0_MDC: usize = UART0_BASE + 0x0010; /* Modem control reg.       */
const UART0_LSR: usize = UART0_BASE + 0x0014; /* Line status reg.         */

/* constants for line control register */

const LCR_CS8: u8 = 0x03; /* 8 bits data size */
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

/* RCVR FIFO interrupt levels: trigger interrupt with this bytes in FIFO */
const FCR_FIFO_8: u8 = 0x80; /* 8 bytes in RCVR FIFO */

/*
   FIXME: gotta figure out clocks and stuff...
*/
const UART_CLK: u32 = 100_000_000;
const UART_BAUDRATE_32MCLK_115200: u32 = 115200;
const DIVISOR: u32 = (UART_CLK / UART_BAUDRATE_32MCLK_115200) >> 4;

fn read_8(reg: usize) -> u8 {
    unsafe { read_volatile(reg as *mut u8) }
}

fn write_8(reg: usize, val: u8) {
    unsafe {
        write_volatile(reg as *mut u8, val);
    }
}

#[derive(Debug)]
pub struct JH71XXSerial();

impl JH71XXSerial {
    #[inline]
    pub fn new() -> Self {
        let lcr_cache = read_8(UART0_LCR);
        /* clear DLAB */
        write_8(UART0_LCR, LCR_DLAB | lcr_cache);
        /* NOTE: Setting the divisor requires knowing the clock. */
        /*
        write_8(UART0_BRDL, DIVISOR as u8);
        write_8(UART0_BRDH, (DIVISOR >> 8) as u8);
        */
        /* restore the DLAB to access the baud rate divisor registers */
        write_8(UART0_LCR, lcr_cache);

        /* 8 data bits, 1 stop bit, no parity */
        write_8(UART0_LCR, LCR_CS8 | LCR_1_STB | LCR_PDIS);

        /* disable flow control */
        write_8(UART0_MDC, 0);

        /*
         * Program FIFO: enabled, mode 0 (set for compatibility with quark),
         * generate the interrupt at 8th byte
         * Clear TX and RX FIFO
         */
        write_8(
            UART0_FCR,
            FCR_FIFO | FCR_MODE0 | FCR_FIFO_8 | FCR_RCVRCLR | FCR_XMITCLR,
        );

        write_8(UART0_IER, 0); // disable the serial interrupt

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
        if read_8(UART0_LSR) & LSR_THRE == 0 {
            return Err(nb::Error::WouldBlock);
        }
        write_8(UART0_THR, c);
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> nb::Result<(), self::Error> {
        let tfe_empty = true;
        if tfe_empty {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}
