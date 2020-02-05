// null is a simple Null driver.
// TODO: move it out of uart.
use model::*;

pub struct Null;

impl Driver for Null {
    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    fn pread(&self, _data: &mut [u8], _offset: usize) -> Result<usize> {
        Ok(0)
    }

    fn pwrite(&mut self, data: &[u8], _offset: usize) -> Result<usize> {
        Ok(data.len())
    }

    fn shutdown(&mut self) {}
}
