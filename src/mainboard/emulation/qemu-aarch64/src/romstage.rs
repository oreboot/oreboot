use crate::halt;
use payloads::payload;
use wrappers::{Memory, SectionReader};
use device_tree::print_fdt;

/* TODO: better way to get kernel / dtb memory address */
const KERNEL_ROM_ADDR: usize = 0x200000;
const KERNEL_ROM_SIZE: usize = 32 * 1024 * 1024;
const KERNEL_LOAD_ADDR: usize = 0x41000000;
const DTB_ROM_ADDR: usize = 0x100000;
const DTB_ROM_SIZE: usize = 1024 * 1024;
const DTB_LOAD_ADDR: usize = 0x45000000;

pub fn romstage(w: &mut impl core::fmt::Write) -> ! {
    let kernel_segs = &[payload::Segment { typ: payload::stype::PAYLOAD_SEGMENT_ENTRY, base: KERNEL_LOAD_ADDR, data: &mut SectionReader::new(&Memory {}, KERNEL_ROM_ADDR, KERNEL_ROM_SIZE) }, payload::Segment {
        typ: payload::stype::PAYLOAD_SEGMENT_DATA,
        base: DTB_LOAD_ADDR,
        data: &mut SectionReader::new(&Memory {}, DTB_ROM_ADDR, DTB_ROM_SIZE),
    }];
    let mut payload = payload::Payload { typ: payload::ftype::CBFS_TYPE_RAW, compression: payload::ctype::CBFS_COMPRESS_NONE, offset: 0, entry: KERNEL_LOAD_ADDR as usize, rom_len: 0 as usize, mem_len: 0 as usize, segs: kernel_segs, dtb: DTB_LOAD_ADDR };

    payload.load();

    let fdt = SectionReader::new(&Memory {}, DTB_LOAD_ADDR, DTB_ROM_SIZE);

    if let Err(err) = print_fdt(&fdt, w) {
        write!(w, "error: {}\n", err).expect(err);
    }

    write!(w, "Jumping to payload...\r\n\r\n").unwrap();
    payload.run_aarch64();

    halt();
}
