use crate::halt;
use payloads::payload;
use wrappers::{Memory, SectionReader};

/* TODO: better way to get kernel / dtb memory address */
pub fn romstage() -> ! {
    let kernel_segs = &[payload::Segment { typ: payload::stype::PAYLOAD_SEGMENT_ENTRY, base: 0x41000000, data: &mut SectionReader::new(&Memory {}, 0x200000, 6 * 1024 * 1024) }, payload::Segment {
        typ: payload::stype::PAYLOAD_SEGMENT_DATA,
        base: 0x42000000,
        data: &mut SectionReader::new(&Memory {}, 0x100000, 1024 * 1024),
    }];
    let mut payload = payload::Payload { typ: payload::ftype::CBFS_TYPE_RAW, compression: payload::ctype::CBFS_COMPRESS_NONE, offset: 0, entry: 0x41000000 as usize, rom_len: 0 as usize, mem_len: 0 as usize, segs: kernel_segs, dtb: 0x42000000 };

    payload.load();
    payload.run_aarch64();

    halt();
}
