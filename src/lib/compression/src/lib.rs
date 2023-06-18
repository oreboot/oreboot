#![no_std]

use core::{ptr::read_volatile, slice};
use log::println;

// offset_bits aka EI, usually 10..13
// length_bits aka EJ, usually 4..5
// buf_fill_byte (often 0x20, space) - we use 0x00, the zero byte
// decompress_buf_size, always 1 << EI
// compress_buf_size, always 2 << EI
const EI: usize = 12;
const DECOMP_BUF_SIZE: usize = 1 << EI;
const COMP_BUF_SIZE: usize = 2 << EI;
pub type OreLzss = lzss::Lzss<EI, 4, 0x00, DECOMP_BUF_SIZE, COMP_BUF_SIZE>;

/// Safety:
///
/// The caller needs to ensure correctness of source and target addresses.
/// The first four bytes at source address must hold the compressed size.
/// The payload size is the maximum expected size after decompression.
/// Ensure that it fits, i.e., `payload_size` bytes after target address
/// are unused. We allocate a buffer of that size; exceeding it will error.
pub unsafe fn decompress(source_addr: usize, target_addr: usize, payload_size: usize) {
    // NOTE: This must be u32 because it is 4 bytes, so cast later.
    let compressed_size = unsafe { read_volatile(source_addr as *const u32) };
    let in_ptr = (source_addr + 4) as *const u8;
    let out_ptr = target_addr as *mut u8;
    let input = unsafe { slice::from_raw_parts(in_ptr, compressed_size as usize) };
    let output = unsafe { slice::from_raw_parts_mut(out_ptr, payload_size) };

    println!(
        "Decompress {compressed_size} bytes from {input:p} to {output:p}, reserved {payload_size} bytes",
    );

    let reader = lzss::SliceReader::new(input);
    let writer = lzss::SliceWriter::new(output);
    match OreLzss::decompress_stack(reader, writer) {
        Ok(r) => {
            println!("Success, decompressed {r} bytes :)");
        }
        Err(e) => {
            panic!("Decompression error {e}");
        }
    }
}
