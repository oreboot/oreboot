#![no_std]

use core::slice;
use log::println;

use miniz_oxide::inflate as moi;
use miniz_oxide::inflate::core as moc;

// This flag means inflate should try parsing a zlib header.
// We provide zlib compressed payloads including the header by convetion.
const FLAGS: u32 = moc::inflate_flags::TINFL_FLAG_PARSE_ZLIB_HEADER;

/// # Safety
///
/// The caller needs to ensure correctness of source and target addresses.
/// The payload size is the maximum expected size after decompression.
/// Ensure that it fits, i.e., `payload_size` bytes after target address
/// are unused. We allocate a buffer of that size; exceeding it will error.
pub unsafe fn decompress(
    source_addr: usize,
    target_addr: usize,
    compressed_size: usize,
    payload_size: usize,
) {
    let in_ptr = source_addr as *const u8;
    let out_ptr = target_addr as *mut u8;
    let input = unsafe { slice::from_raw_parts(in_ptr, compressed_size) };
    let output = unsafe { slice::from_raw_parts_mut(out_ptr, payload_size) };

    println!(
        "Decompress {compressed_size} bytes from {input:p} to {output:p}, reserved {payload_size} bytes",
    );

    let mut decompressor = moc::DecompressorOxide::new();
    let (status, _, out_bytes) = moc::decompress(&mut decompressor, &input, output, 0, FLAGS);

    match status {
        moi::TINFLStatus::Done => {
            println!("Success, decompressed {out_bytes} bytes :)");
        }
        _ => {
            panic!("Decompression error {status:?}");
        }
    }
}
