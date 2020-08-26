//use core::ops;
use model::*;
//use register::mmio::{ReadOnly, ReadWrite};
//use register::{register_bitfields, Field};

/*
#[repr(C)]
pub struct RegisterBlock {
    d: ReadWrite<u8, D::Register>,
    ie: ReadWrite<u8, IE::Register>,
    fc: ReadWrite<u8, FC::Register>,
    lc: ReadWrite<u8, LC::Register>,
    mc: ReadWrite<u8, MC::Register>,
    ls: ReadOnly<u8, LS::Register>,
}
*/
pub struct I8250<'a> {
    base: usize,
    baud: u32,
    d: &'a mut dyn Driver,
}

// it is possible that trying to make this work is a fool's errand but
// ... would be nice if the deref used the Driver in the 8250 ... dream on.
// impl ops::Deref for I8250 {
//     type Target = RegisterBlock;

//     fn deref(&self) -> &Self::Target {
//         unsafe { &*self.ptr() }
//     }
// }

impl<'a> I8250<'a> {
    // why is base a usize? for mmio 8250.
    pub fn new(base: usize, baud: u32, d: &'a mut dyn Driver) -> I8250<'a> {
        I8250 { base: base, baud: baud, d: d }
    }

    /// Returns a pointer to the register block
    // fn ptr(&self) -> *const RegisterBlock {
    //     self.base as *const _
    // }
    /// Poll the status register until the specified field is set to the given value.
    /// Returns false iff it timed out.
    //    fn poll_status(&self, bit: Field<u8, LS::Register>, val: bool) -> bool {
    fn poll_status(&self, mask: u8, val: u8) -> bool {
        // Timeout after a few thousand cycles to prevent hanging forever.
        for _ in 0..100_000 {
            let mut s = [0; 1];
            self.d.pread(&mut s, self.base + 5).unwrap();
            if s[0] & mask == val {
                return true;
            }
        }
        return false;
    }
}

impl<'a> Driver for I8250<'a> {
    // TODO: properly use the register crate.
    fn init(&mut self) -> Result<()> {
        const ier: usize = 0x01;
        const iir: usize = 0x02;
        const fcr: usize = 0x02;
        const lcr: usize = 0x03;
        const mcr: usize = 0x04;
        const mcr_dma_en: usize = 0x04;
        const mcr_tx_dfr: usize = 0x08;
        const dll: usize = 0x00;
        const dlm: usize = 0x01;
        const lsr: usize = 0x05;
        const msr: usize = 0x06;
        const scr: usize = 0x07;

        const fifoenable: u8 = 1;
        const dlab: u8 = 0x80;
        const eightn1: u8 = 3;

        let mut s: [u8; 1] = [0u8; 1];
        self.d.pwrite(&s, self.base + ier).unwrap();
        //outb(0x0, base_port + UART8250_IER);

        /* Enable FIFOs */
        //outb(&s, base_port + fcr);
        s[0] = fifoenable;
        self.d.pwrite(&s, self.base + fcr).unwrap();

        /* assert DTR and RTS so the other end is happy */
        // 3 wires don't care.
        //outb(UART8250_MCR_DTR | UART8250_MCR_RTS, base_port + UART8250_MCR);

        /* DLAB on */
        // so we can set baud rate.
        s[0] = dlab | eightn1;
        self.d.pwrite(&s, self.base + lcr).unwrap();

        /* Set Baud Rate Divisor. 12 ==> 9600 Baud */
        // 1 for 115200
        s[0] = 1;
        self.d.pwrite(&s, self.base + dll).unwrap();
        s[0] = 0;
        self.d.pwrite(&s, self.base + dlm).unwrap();
        //outb(divisor & 0xFF,   base_port + UART8250_DLL);
        //outb((divisor >> 8) & 0xFF,    base_port + UART8250_DLM);

        /* Set to 3 for 8N1 */
        s[0] = eightn1;
        self.d.pwrite(&s, self.base + lcr).unwrap();
        //        outb(CONFIG_TTYS0_LCS, base_port + UART8250_LCR);
        // /* disable all interrupts */
        // self.ie.set(0u8);
        // /* Enable dLAB */
        // self.lc.write(LC::DivisorLatchAccessBit::BaudRate);
        // // Until we know the clock rate the divisor values are kind of
        // // impossible to know. Throw in a phony value.
        // self.lc.write(LC::WLEN::WLEN_8);
        // // TOdO: what are these bits. how do we write them.
        // self.fc.set(0xc7);
        // self.mc.set(0x0b);
        // self.lc.write(LC::DivisorLatchAccessBit::Normal);
        Ok(())
    }

    fn pread(&self, data: &mut [u8], _offset: usize) -> Result<usize> {
        for c in data.iter_mut() {
            let mut s = [0u8; 1];
            while !self.poll_status(1, 1) {}
            self.d.pread(&mut s, self.base).unwrap();
            *c = s[0];
        }
        Ok(data.len())
    }

    fn pwrite(&mut self, data: &[u8], _offset: usize) -> Result<usize> {
        for (_i, &c) in data.iter().enumerate() {
            // Poll the status for long enough to let a char out; then push it out anyway.
            while !self.poll_status(0x20, 0x20) && !self.poll_status(0x40, 0x40) {}
            let mut s = [0u8; 1];
            s[0] = c;
            self.d.pwrite(&s, self.base).unwrap();
        }
        Ok(data.len())
    }

    fn shutdown(&mut self) {}
}

// // TODO: bitfields
// register_bitfields! {
//     u8,
//     // Data register
//     D [
//         DATA OFFSET(0) NUMBITS(8) []
//     ],
//     IE [
//         RX OFFSET(0) NUMBITS(1) [],
//         TX OFFSET(1) NUMBITS(1) [],
//         Error OFFSET(2) NUMBITS(1) [],
//         StatusChange OFFSET(3) NUMBITS(1) []
//     ],
//     FC [
//         DATA OFFSET(0) NUMBITS(8) []
//     ],
//     LC [
//         WLEN OFFSET(0) NUMBITS(2) [
//             WLEN_5 = 0,
//             WLEN_6 = 1,
//             WLEN_7 = 2,
//             WLEN_8 = 3
//         ],
//         StopBits OFFSET(3) NUMBITS(1) [],
//         ParityEnable OFFSET(4) NUMBITS(1) [],
//         EvenParity OFFSET(5) NUMBITS (1) [],
//         StickParity OFFSET(6) NUMBITS (1) [],
//         DivisorLatchAccessBit OFFSET(7) NUMBITS (1) [
//             Normal = 0,
//             BaudRate = 1
//         ]
//     ],
//     MC [
//         DATA OFFSET(0) NUMBITS(8) []
//     ],
//     LS [
//         IF OFFSET(0) NUMBITS(1) [],
//         OE OFFSET(1) NUMBITS(1) []
//     ]
// }
