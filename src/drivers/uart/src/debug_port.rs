use model::{Driver, Result};

pub struct DebugPort<'a> {
    address: usize,
    d: &'a mut dyn Driver,
}

impl<'a> DebugPort<'a> {
    pub fn new(address: usize, d: &'a mut dyn Driver) -> DebugPort<'a> {
        DebugPort { address, d }
    }
}

impl<'a> Driver for DebugPort<'a> {
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
        for (_i, &c) in data.iter().enumerate() {
            let mut s = [0u8; 1];
            s[0] = c;
            self.d.pwrite(&s, self.address).unwrap();
        }
        Ok(data.len())
    }

    // Nothing to shut down here
    fn shutdown(&mut self) {}
}
