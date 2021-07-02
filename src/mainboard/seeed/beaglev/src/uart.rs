#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
use core::ptr;

// This is a temporary hack until the board works, at which point we'll use
// the real drivers.

const uart: u32 = 0x12440000;

// It's really not hex.
const UART_CLK: u32 = 100000000;

const UART_BUADRATE_32MCLK_115200: u32 = 115200;

const UART_REG_ADDR_INTERVAL: u32 = 4;
const REG_THR: u32 = 0x00; /* Transmitter holding reg. */
const REG_RDR: u32 = 0x00; /* Receiver data reg.       */
const REG_BRDL: u32 = 0x00; /* Baud rate divisor (LSB)  */
const REG_BRDH: u32 = 0x01; /* Baud rate divisor (MSB)  */
const REG_IER: u32 = 0x01; /* Interrupt enable reg.    */
const REG_IIR: u32 = 0x02; /* Interrupt ID reg.        */
const REG_FCR: u32 = 0x02; /* FIFO control reg.        */
const REG_LCR: u32 = 0x03; /* Line control reg.        */
const REG_MDC: u32 = 0x04; /* Modem control reg.       */
const REG_LSR: u32 = 0x05; /* Line status reg.         */
const REG_MSR: u32 = 0x06; /* Modem status reg.        */
const REG_DLF: u32 = 0xC0; /* Divisor Latch Fraction   */

const UART_USR: u32 = 31;
const UART_USR_BUSY: u32 = (1 << 0); /* UART is busy (1) */
const UART_USR_Tx_FIFO_NFUL: u32 = (1 << 1); /* Tx FIFO is not full (1) */
const UART_USR_Tx_FIFO_NEMP: u32 = (1 << 2); /* Tx FIFO is empty (1) */
const UART_USR_Rx_FIFO_NHFL: u32 = (1 << 3); /* Rx FIFO is not empty (1) */
const UART_USR_Rx_FIFO_NFUL: u32 = (1 << 4); /* Rx FIFO is full (1) */

/* equates for interrupt enable register */

const IER_RXRDY: u32 = 0x01; /* receiver data ready */
const IER_TBE: u32 = 0x02; /* transmit bit enable */
const IER_LSR: u32 = 0x04; /* line status interrupts */
const IER_MSI: u32 = 0x08; /* modem status interrupts */

/* constants for line control register */

const LCR_CS5: u32 = 0x00; /* 5 bits data size */
const LCR_CS6: u32 = 0x01; /* 6 bits data size */
const LCR_CS7: u32 = 0x02; /* 7 bits data size */
const LCR_CS8: u32 = 0x03; /* 8 bits data size */
const LCR_2_STB: u32 = 0x04; /* 2 stop bits */
const LCR_1_STB: u32 = 0x00; /* 1 stop bit */
const LCR_PEN: u32 = 0x08; /* parity enable */
const LCR_PDIS: u32 = 0x00; /* parity disable */
const LCR_EPS: u32 = 0x10; /* even parity select */
const LCR_SP: u32 = 0x20; /* stick parity select */
const LCR_SBRK: u32 = 0x40; /* break control bit */
const LCR_DLAB: u32 = 0x80; /* divisor latch access enable */

/* constants for line status register */

const LSR_RXRDY: u32 = 0x01; /* receiver data available */
const LSR_OE: u32 = 0x02; /* overrun error */
const LSR_PE: u32 = 0x04; /* parity error */
const LSR_FE: u32 = 0x08; /* framing error */
const LSR_BI: u32 = 0x10; /* break interrupt */
const LSR_EOB_MASK: u32 = 0x1E; /* Error or Break mask */
const LSR_THRE: u32 = 0x20; /* transmit holding register empty */
const LSR_TEMT: u32 = 0x40; /* transmitter empty */

/* equates for FIFO control register */

const FCR_FIFO: u32 = 0x01; /* enable XMIT and RCVR FIFO */
const FCR_RCVRCLR: u32 = 0x02; /* clear RCVR FIFO */
const FCR_XMITCLR: u32 = 0x04; /* clear XMIT FIFO */

/*
 * Per PC16550D (Literature Number: SNLS378B):
 *
 * RXRDY, Mode 0: When in the 16450 Mode (FCR0 = 0) or in
 * the FIFO Mode (FCR0 = 1, FCR3 = 0) and there is at least 1
 * character in the RCVR FIFO or RCVR holding register, the
 * RXRDY pin (29) will be low active. Once it is activated the
 * RXRDY pin will go inactive when there are no more charac-
 * ters in the FIFO or holding register.
 *
 * RXRDY, Mode 1: In the FIFO Mode (FCR0 = 1) when the
 * FCR3 = 1 and the trigger level or the timeout has been
 * reached, the RXRDY pin will go low active. Once it is acti-
 * vated it will go inactive when there are no more characters
 * in the FIFO or holding register.
 *
 * TXRDY, Mode 0: In the 16450 Mode (FCR0 = 0) or in the
 * FIFO Mode (FCR0 = 1, FCR3 = 0) and there are no charac-
 * ters in the XMIT FIFO or XMIT holding register, the TXRDY
 * pin (24) will be low active. Once it is activated the TXRDY
 * pin will go inactive after the first character is loaded into the
 * XMIT FIFO or holding register.
 *
 * TXRDY, Mode 1: In the FIFO Mode (FCR0 = 1) when
 * FCR3 = 1 and there are no characters in the XMIT FIFO, the
 * TXRDY pin will go low active. This pin will become inactive
 * when the XMIT FIFO is completely full.
 */
const FCR_MODE0: u32 = 0x00; /* set receiver in mode 0 */
const FCR_MODE1: u32 = 0x08; /* set receiver in mode 1 */

/* RCVR FIFO interrupt levels: trigger interrupt with this bytes in FIFO */
const FCR_FIFO_1: u32 = 0x00; /* 1 byte in RCVR FIFO */
const FCR_FIFO_4: u32 = 0x40; /* 4 bytes in RCVR FIFO */
const FCR_FIFO_8: u32 = 0x80; /* 8 bytes in RCVR FIFO */
const FCR_FIFO_14: u32 = 0xC0; /* 14 bytes in RCVR FIFO */

/*
 * UART NS16750 supports 64 bytes FIFO, which can be enabled
 * via the FCR register
 */
const FCR_FIFO_64: u32 = 0x20; /* Enable 64 bytes FIFO */

fn poke32(a: u32, v: u32) {
    let y = a as *mut u32;
    unsafe {
        ptr::write_volatile(y, v);
    }
}

fn peek32(a: u32) -> u32 {
    let y = a as *const u32;
    unsafe { ptr::read_volatile(y) }
}

fn serial_in(reg: u32) -> u32 {
    peek32(uart + (reg << 2))
}

fn serial_out(reg: u32, v: u32) -> () {
    poke32(uart + (reg << 2), v);
}

pub fn uart_init() -> () {
    let divisor = (UART_CLK / UART_BUADRATE_32MCLK_115200) >> 4;

    let lcr_cache = serial_in(REG_LCR);
    serial_out(REG_LCR, (LCR_DLAB | lcr_cache));
    serial_out(REG_BRDL, (divisor & 0xff));
    serial_out(REG_BRDH, ((divisor >> 8) & 0xff));

    /* restore the DLAB to access the baud rate divisor registers */
    serial_out(REG_LCR, lcr_cache);

    /* 8 data bits, 1 stop bit, no parity, clear DLAB */
    serial_out(REG_LCR, (LCR_CS8 | LCR_1_STB | LCR_PDIS));

    serial_out(REG_MDC, 0); /*disable flow control*/

    /*
     * Program FIFO: enabled, mode 0 (set for compatibility with quark),
     * generate the interrupt at 8th byte
     * Clear TX and RX FIFO
     */
    serial_out(
        REG_FCR,
        (FCR_FIFO | FCR_MODE1 | /*FCR_FIFO_1*/FCR_FIFO_8 | FCR_RCVRCLR | FCR_XMITCLR),
    );

    serial_out(REG_IER, 0); //dis the ser interrupt
}

pub fn putc(c: char) -> () {
    loop {
        let lsr = serial_in(REG_LSR) & LSR_THRE;
        if lsr != 0 {
            break;
        }
    }
    serial_out(REG_THR, c as u32);
}
