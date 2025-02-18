use core::slice;

use log::{print, println};

use crate::mmio::{read32, write32};

// NOTE: Print an empty line _before_ instead of doing println!(), such that
// after progress dots, there is always a line break before the error message.
fn check32(addr: usize, val: u32) -> Result<(), ()> {
    let v = read32(addr);
    if v != val {
        print!("\nError @ {addr:08x}: expected {val:08x}, got {v:08x}");
        Err(())
    } else {
        Ok(())
    }
}

const TEST_PATTERN0: u32 = 0x2233_ccee;
const TEST_PATTERN1: u32 = 0x5577_aadd;
const TEST_PATTERN2: u32 = 0x1144_bbff;
const TEST_PATTERN3: u32 = 0x6688_9900;

pub fn test(base: usize, size: usize) {
    let mut error_count = 0;
    let limit = base + size;
    let step_size = 0x10;
    // print 64 steps, which gets slower with a higher size to test
    let print_step = size / step_size / 64;

    println!("Memory test: {size}bytes @ 0x{base:016x}");

    println!("Memory test: write patterns...");
    for (i, a) in (base..limit).step_by(step_size).enumerate() {
        if i % print_step == 0 {
            print!(".");
        }
        #[allow(clippy::identity_op)]
        write32(a + 0x0, TEST_PATTERN0 | i as u32);
        write32(a + 0x4, TEST_PATTERN1 | i as u32);
        write32(a + 0x8, TEST_PATTERN2 | i as u32);
        write32(a + 0xc, TEST_PATTERN3 | i as u32);
    }
    println!();

    println!("Memory test: reading back...");
    for (i, a) in (base..limit).step_by(step_size).enumerate() {
        if i % print_step == 0 {
            print!(".");
        }
        #[allow(clippy::identity_op)]
        if check32(a + 0x0, TEST_PATTERN0 | i as u32).is_err() {
            error_count += 1;
        }
        if check32(a + 0x4, TEST_PATTERN1 | i as u32).is_err() {
            error_count += 1;
        }
        if check32(a + 0x8, TEST_PATTERN2 | i as u32).is_err() {
            error_count += 1;
        }
        if check32(a + 0xc, TEST_PATTERN3 | i as u32).is_err() {
            error_count += 1;
        }
    }
    println!();
    if error_count > 0 {
        let checks = size / 4;
        println!(
            "Memory test: {error_count} errors, {checks} addresses tested ({}% failed)",
            error_count * 100 / checks
        );
        panic!("Errors encountered.");
    }
    println!("Memory test: pass");
}

pub fn copy(source: usize, target: usize, size: usize) {
    for o in (0..size).step_by(4) {
        write32(target + o, read32(source + o));
        if o % 0x4_0000 == 0 {
            print!(".");
        }
    }
    println!(" done.");
}

// Hex dump a memory region, useful for small size.
pub fn dump(base: usize, size: usize) {
    let s = unsafe { slice::from_raw_parts(base as *const u8, size) };
    for w in s.iter() {
        print!("{w:02x}");
    }
    println!();
}

// Hex dump a larger memory region in blocks, step_size per row.
pub fn dump_block(base: usize, size: usize, step_size: usize) {
    println!("dump {size} bytes @{base:08x}");
    for b in (base..base + size).step_by(step_size) {
        dump(b, step_size);
    }
}
