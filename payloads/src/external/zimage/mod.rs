pub const KERNEL: &[u8] = include_bytes!("zImage");
pub const DTB: &[u8] = include_bytes!("qemu_fdt.dtb");

/*
use crate::payload;
// TODO: Create a struct which tells you where the available memory is.
const MEM: u32 = 0x40200000;
// TODO: Parse from SPI.
pub const PAYLOAD: payload::Payload = payload::Payload {
    typ: payload::ftype::CBFS_TYPE_RAW,
    compression: payload::ctype::CBFS_COMPRESS_NONE,
    offset: 0,
    load_addr: MEM as u64,
    rom_len: KERNEL.len() as u32,
    mem_len: KERNEL.len() as u32,
    segs: &[
        payload::Segment {
            // Kernel segment
            typ: payload::stype::PAYLOAD_SEGMENT_ENTRY,
            base: MEM,
            data: KERNEL,
        },
        payload::Segment {
            // Device tree segment
            typ: payload::stype::PAYLOAD_SEGMENT_DATA,
            // TODO: Assumes the decompression ratio is no more than 5x.
            base: MEM + 5 * KERNEL.len() as u32,
            data: DTB,
        },
    ],
};
*/
