use crate::halt;
pub const KERNEL: &[u8] = include_bytes!("notmain.bin");
use device_tree::area::{get_kernel_area};
use payloads::payload;
use wrappers::{Memory, SectionReader};

const MEM: usize = 0x40200000;
const DTFS_BASE: usize = 0x800000;
const DTFS_SIZE: usize = 0x80000;


pub fn romstage(w: &mut impl core::fmt::Write) -> ! {
    let kernel_area = get_kernel_area(DTFS_BASE, DTFS_SIZE);

    // => found payload <RomPayload DTFS A> @ 0x980000, and would copy it to 0x40200000
    write!(
        w,
        "found payload <{}> @ 0x{:x}, and would copy it to 0x{:x}\n",
        kernel_area.description,
        kernel_area.offset.unwrap(),
        MEM
    )
    .expect("Failed printing rompayload location");

    let mut kernel = SectionReader::new(
        &Memory {},
        kernel_area.offset.unwrap() as usize,
        kernel_area.size as usize,
    );
    // TODO: Need capacity+size in dtb to size this appropriately 
    let mut dtb = SectionReader::new(&Memory {}, DTFS_BASE, 0x800);

    let kernel_segs = &[
        payload::Segment {
            // Kernel segment
            typ: payload::stype::PAYLOAD_SEGMENT_ENTRY,
            base: MEM,
            data: &mut kernel,
        },
        payload::Segment {
            // Device tree segment
            typ: payload::stype::PAYLOAD_SEGMENT_DATA,
            // TODO: Assumes the decompression ratio is no more than 5x.
            base: DTFS_BASE,
            data: &mut dtb,
        },
    ];
    let mut p = payload::Payload {
        typ: payload::ftype::CBFS_TYPE_RAW,
        compression: payload::ctype::CBFS_COMPRESS_NONE,
        offset: 0,
        entry: 0,
        dtb: DTFS_BASE,
        rom_len: KERNEL.len() as usize,
        mem_len: KERNEL.len() as usize,
        segs: kernel_segs,
    };
    p.load();
    p.run();

    halt()
}
