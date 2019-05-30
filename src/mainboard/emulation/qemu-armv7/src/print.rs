
use core::fmt;

pub struct WriteTo<'a> {
    drv: &'a driver::Driver,
}

impl<'a> WriteTo<'a> {
    pub fn new(drv: &'a driver::Driver) -> Self {
        WriteTo { drv: drv }
    }
}

impl<'a> fmt::Write for WriteTo<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.drv.write(s.as_bytes());
        Ok(())
    }
}
