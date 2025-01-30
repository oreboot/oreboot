use crate::util::{read32, write32};

fn check32(addr: usize, val: u32) {
    let v = read32(addr);
    if v != val {
        println!("Error @ {addr:08x}: expected {val:08x}, got {v:08x}");
    }
}

const DRAM_TEST_PATTERN_0: u32 = 0x2233_ccee;
const DRAM_TEST_PATTERN_1: u32 = 0x5577_aadd;
const DRAM_TEST_PATTERN_2: u32 = 0x1144_bbff;
const DRAM_TEST_PATTERN_3: u32 = 0x6688_9900;

pub fn mem_test(base: usize, size: usize) {
    let limit = base + size;
    let step_size = 0x10;
    // print 64 steps, which gets slower with a higher size to test
    let print_step = size / step_size / 64;

    println!("DRAM test: write patterns...");
    for (i, a) in (base..limit).step_by(step_size).enumerate() {
        if i % print_step == 0 {
            print!(".");
        }
        #[allow(clippy::identity_op)]
        write32(a + 0x0, DRAM_TEST_PATTERN_0 | i as u32);
        write32(a + 0x4, DRAM_TEST_PATTERN_1 | i as u32);
        write32(a + 0x8, DRAM_TEST_PATTERN_2 | i as u32);
        write32(a + 0xc, DRAM_TEST_PATTERN_3 | i as u32);
    }
    println!();

    println!("DRAM test: reading back...");
    for (i, a) in (base..limit).step_by(step_size).enumerate() {
        if i % print_step == 0 {
            print!(".");
        }
        #[allow(clippy::identity_op)]
        check32(a + 0x0, DRAM_TEST_PATTERN_0 | i as u32);
        check32(a + 0x4, DRAM_TEST_PATTERN_1 | i as u32);
        check32(a + 0x8, DRAM_TEST_PATTERN_2 | i as u32);
        check32(a + 0xc, DRAM_TEST_PATTERN_3 | i as u32);
    }
    println!();

    println!("DRAM test: done :)");
}
