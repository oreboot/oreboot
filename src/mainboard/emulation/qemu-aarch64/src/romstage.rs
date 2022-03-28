use crate::halt;
use core::intrinsics::transmute;
use device_tree::print_fdt;
use oreboot_drivers::wrappers::{Memory, SectionReader};
use payloads::payload;

/* TODO: get kernel / dtb information from the loader dtb */
const LOADER_DTB_ADDR: usize = 0x80000;
const LOADER_DTB_SIZE: usize = 0x80000;
const KERNEL_ROM_ADDR: usize = 0x200000;
const KERNEL_ROM_SIZE: usize = 32 * 1024 * 1024;
const KERNEL_LOAD_ADDR: usize = 0x41000000;
const DTB_ROM_ADDR: usize = 0x100000;
const DTB_ROM_SIZE: usize = 1024 * 1024;
const DTB_LOAD_ADDR: usize = 0x45000000;

/* TODO: move to payload implementation? */
type EntryPoint = unsafe extern "C" fn(dtb: usize, rsv0: usize, rsv1: usize, rsv2: usize);

fn boot_to_kernel(kernel_entry: usize, dtb_addr: usize) -> ! {
    unsafe {
        let f = transmute::<usize, EntryPoint>(kernel_entry);
        f(dtb_addr, 0, 0, 0);
    }
    halt()
}

pub fn romstage(w: &mut impl core::fmt::Write) -> ! {
    let kernel_segs = &[
        payload::Segment {
            typ: payload::stype::PAYLOAD_SEGMENT_ENTRY,
            base: KERNEL_LOAD_ADDR,
            data: &mut SectionReader::new(&Memory {}, KERNEL_ROM_ADDR, KERNEL_ROM_SIZE),
        },
        payload::Segment {
            typ: payload::stype::PAYLOAD_SEGMENT_DATA,
            base: DTB_LOAD_ADDR,
            data: &mut SectionReader::new(&Memory {}, DTB_ROM_ADDR, DTB_ROM_SIZE),
        },
    ];
    let mut payload = payload::Payload {
        typ: payload::ftype::CBFS_TYPE_RAW,
        compression: payload::ctype::CBFS_COMPRESS_NONE,
        offset: 0,
        entry: KERNEL_LOAD_ADDR as usize,
        rom_len: 0,
        mem_len: 0,
        segs: kernel_segs,
        dtb: DTB_LOAD_ADDR,
    };

    payload.load();

    let loader_fdt = SectionReader::new(&Memory {}, LOADER_DTB_ADDR, LOADER_DTB_SIZE);
    if let Err(err) = print_fdt(&loader_fdt, w) {
        writeln!(w, "error: {}", err).expect(err);
    }

    let kernel_fdt = SectionReader::new(&Memory {}, DTB_LOAD_ADDR, DTB_ROM_SIZE);
    if let Err(err) = print_fdt(&kernel_fdt, w) {
        writeln!(w, "error: {}", err).expect(err);
    }

    writeln!(w, "Jumping to payload...\r\n\r").unwrap();
    boot_to_kernel(payload.entry, payload.dtb)
}
