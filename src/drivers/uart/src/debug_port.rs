use model::{Driver, Result};
use timer::hpet::HPET;

pub struct DebugPort<D: Driver> {
    address: usize,
    d: D,
    timer: HPET,
}

impl<D: Driver> DebugPort<D> {
    pub fn new(address: usize, d: D) -> DebugPort<D> {
        DebugPort {
            address,
            d,
            timer: HPET::hpet(),
        }
    }
}

impl<D: Driver> Driver for DebugPort<D> {
    // Nothing to set up here
    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    // DebugPort can only be used to write, nothing here
    fn pread(&self, _data: &mut [u8], _offset: usize) -> Result<usize> {
        Ok(0)
    }

    // Just write out byte for byte :)
    fn pwrite(&mut self, data: &[u8], _offset: usize) -> Result<usize> {
        for &c in data {
            let mut s = [0u8; 1];
            s[0] = c;
            // 0.5 microseconds
            for _j in 0..125 {
                // shorter sleep time here so that it also works in 32 bit
                self.timer.sleep(4_000_000); // that's in fs
            }
            self.d.pwrite(&s, self.address).unwrap();
        }
        Ok(data.len())
    }

    // Nothing to shut down here
    fn shutdown(&mut self) {}
}
