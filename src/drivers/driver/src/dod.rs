#![no_std]

#[allow(non_snake_case)]
#[repr(C)]
pub struct DoD {
    Drivers: &'static [Driver]
}

impl DoD {
    pub fn new<'static>(Drivers: &'static [Driver]) -> DoD {
        DoD{Drivers}
    }
}

impl driver::Driver for Drivers {
    fn init(&self) {
        for d in Drivers.iter() {
            d.init();
        }
    }

    fn read(&self, data: &mut [u8]) -> usize {
        // This is a bit weird but, basically, we do a read across all the drivers
.
        // Later.
        0
    }

    fn write(&self, data: &[u8]) -> usize {
        let i;
        for d in Drivers.iter() {
            i += d.write(data);
        }
        i
    }

    fn close(&self) {
        for d in Drivers.iter() {
            d.close();
        }
    }
}

