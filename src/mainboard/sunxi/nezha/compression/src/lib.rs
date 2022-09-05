#![no_std]

use core::{fmt::Write, ptr::read_volatile, slice};

const EI: usize = 12;
type MyLzss = lzss::Lzss<EI, 4, 0x00, { 1 << EI }, { 2 << EI }>;

pub fn decompress(
    mut w: impl Write,
    compressed_addr: usize,
    target_addr: usize,
    payload_size: usize,
) {
    // first four bytes are the compressed size
    let in_ptr = (compressed_addr + 4) as *const u8;
    let out_ptr = target_addr as *mut u8;
    let compressed_size = unsafe { read_volatile(compressed_addr as *const u32) };
    write!(
        w,
        "Decompress {} bytes from {:?} to {:?}, reserved {:?} bytes\n",
        compressed_size, &in_ptr, &out_ptr, payload_size
    )
    .ok();

    let input = unsafe { slice::from_raw_parts(in_ptr, compressed_size as usize) };
    let output = unsafe { slice::from_raw_parts_mut(out_ptr, payload_size) };

    let result = MyLzss::decompress(
        lzss::SliceReader::new(input),
        lzss::SliceWriter::new(output),
    );
    match result {
        Ok(r) => write!(w, "Success, decompressed {r} bytes :)\n"),
        Err(e) => write!(w, "Decompression error {e}\n"),
    }
    .ok();
}
