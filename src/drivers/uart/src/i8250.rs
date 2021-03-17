use model::*;

pub struct I8250<'a> {
    base: usize,
    _baud: u32,
    d: &'a mut dyn Driver,
}

impl<'a> I8250<'a> {
    pub fn new(base: usize, _baud: u32, d: &'a mut dyn Driver) -> I8250<'a> {
        I8250 { base, _baud, d }
    }

    /// Poll the status register until the specified field is set to the given value.
    /// Returns false iff it timed out.
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
#[allow(dead_code)]
impl<'a> Driver for I8250<'a> {
    // TODO: properly use the register crate.
    fn init(&mut self) -> Result<()> {
        const DLL: usize = 0x00; // Divisor Latch Low Byte               RW
        const DLH: usize = 0x01; // Divisor Latch High Byte              RW
        const IER: usize = 0x01; // Interrupt Enable Register            RW
        const IIR: usize = 0x02; // Interrupt Identification Register    R
        const FCR: usize = 0x02; // Fifo Control Register                W
        const LCR: usize = 0x03; // Line Control Register                RW
        const MCR: usize = 0x04; // Modem Control Register               RW
        const LSR: usize = 0x05; // Line Status Register                 R
        const MSR: usize = 0x06; // Modem Status Register                R
        const SCR: usize = 0x07; // Scratch Register                     RW

        const FIFOENABLE: u8 = 1;
        const DLAB: u8 = 0b1000_0000; // Divisor Latch Access bit
        const EIGHTN1: u8 = 0b0011;

        let mut s: [u8; 1] = [0u8; 1];
        self.d.pwrite(&s, self.base + IER).unwrap();

        /* Enable FIFOs */
        s[0] = FIFOENABLE;
        self.d.pwrite(&s, self.base + FCR).unwrap();

        /* DLAB on */
        // so we can set baud rate.
        s[0] = DLAB | EIGHTN1;
        self.d.pwrite(&s, self.base + LCR).unwrap();

        /* Set Baud Rate Divisor. 12 ==> 9600 Baud */
        // 1 for 115200
        s[0] = 1;
        self.d.pwrite(&s, self.base + DLL).unwrap();
        s[0] = 0;
        self.d.pwrite(&s, self.base + DLH).unwrap();

        /* Set to 3 for 8N1 (8 data bits, no parity, 1 stop bit)*/
        s[0] = EIGHTN1;
        self.d.pwrite(&s, self.base + LCR).unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;

    /*
     * UART pushing into a vec
     */
    extern crate heapless;
    use heapless::Vec;
    pub struct MockPort<'a> {
        ldata: &'a mut Vec<u8, heapless::consts::U8>,
    }

    impl<'a> MockPort<'a> {
        pub fn new(v: &'a mut Vec<u8, heapless::consts::U8>) -> MockPort {
            MockPort { ldata: v }
        }
    }

    impl<'a> Driver for MockPort<'a> {
        fn init(&mut self) -> Result<()> {
            Ok(())
        }

        fn pread(&self, data: &mut [u8], offset: usize) -> Result<usize> {
            if self.ldata.len() <= offset {
                return EOF;
            }
            data[0] = self.ldata[offset];
            return Ok(0);
        }

        fn pwrite(&mut self, data: &[u8], offset: usize) -> Result<usize> {
            while self.ldata.len() < offset + data.len() {
                self.ldata.push(0).unwrap();
            }

            for (i, &c) in data.iter().enumerate() {
                self.ldata[offset + i] = c;
            }

            Ok(data.len())
        }

        fn shutdown(&mut self) {}
    }

    const FCR: usize = 0x02; // Fifo Control Register
    const LCR: usize = 0x03; // Line Control Register

    #[test]
    fn uart_driver_enables_fifos() {
        let mut vec = Vec::<u8, heapless::consts::U8>::new();
        let port = &mut MockPort::new(&mut vec);
        let test_uart = &mut I8250::new(0, 0, port);
        test_uart.init().unwrap();

        assert_eq!(1 & vec[FCR], 1); // FIFOs enabled
    }

    // Line control register should have the bottom bits be 0b011 for 8 data bits and one stop bit
    #[test]
    fn uart_driver_sets_wordlength_and_stopbit() {
        let mut vec = Vec::<u8, heapless::consts::U8>::new();
        let port = &mut MockPort::new(&mut vec);
        let test_uart = &mut I8250::new(0, 0, port);
        test_uart.init().unwrap();

        assert_eq!(vec[LCR], 0b011);
    }

    // TODO: test baud rate usage
}
