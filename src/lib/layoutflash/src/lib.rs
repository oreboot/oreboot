#![no_std]
use core::result::Result::Ok;
use core::result::Result::Err;
use core::option::Option;
use core::result::Result;


extern crate alloc;
use alloc::string::String;

// NOTE: we don't use u32. At the rate that SPI flash is expanding, we're going to see
// 5B addressing soon I bet. The size limitation should be a function of the destination,
// not this program. This problem should just stupidly arrange things.
#[derive(Debug, PartialEq)]
struct Area {
    pub name: String,
    pub offset: Option<usize>,
    pub size: usize,
    pub file: Option<String>,
}

pub fn find_fdt<'a>(data: &'a [u8]) -> Result<fdt::Fdt<'a>, fdt::FdtError> {
    // The informal standard is that the fdt must be on a 0x1000
    // boundary. It is a fine line between too coarse a boundary
    // and falling into an false positive.
    // yuck. Make a better iterator.
    for pos in 0..data.len() - 0x1000 {
        match fdt::Fdt::new(&data[pos..]) {
            Err(_) => {}
            Ok(fdt) => {
                return Ok(fdt);
            }
        };
    }

    Err(fdt::FdtError::BadMagic)
}
