use crate::halt;
use payloads::payload;
use wrappers::{Memory, SectionReader};
use device_tree::print_fdt;

/* TODO: get kernel / dtb information from the loader dtb */
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

    let loader_fdt = SectionReader::new(&Memory {}, 0x80000, 0x80000);
    if let Err(err) = print_fdt(&loader_fdt, w) {
        write!(w, "error: {}\n", err).expect(err);
    }

    let kernel_fdt = SectionReader::new(&Memory {}, DTB_LOAD_ADDR, DTB_ROM_SIZE);
    if let Err(err) = print_fdt(&kernel_fdt, w) {
        write!(w, "error: {}\n", err).expect(err);
    }

    write!(w, "Jumping to payload...\r\n\r\n").unwrap();
    payload.run_aarch64();

    halt()
}
