use crate::halt;
// use payloads::external::zimage::{DTB, KERNEL};
// use payloads::payload;
// use wrappers::SliceReader;

// const MEM: usize = 0x40200000;
// const DTB_BASE: usize = MEM + 5 * KERNEL.len();

pub fn romstage() -> ! {
    // let kernel_segs = &[
    //    payload::Segment {
    //        // Kernel segment
    //        typ: payload::stype::PAYLOAD_SEGMENT_ENTRY,
    //        base: MEM,
    //        data: &mut SliceReader::new(KERNEL),
    //    },
    //    payload::Segment {
    //        // Device tree segment
    //        typ: payload::stype::PAYLOAD_SEGMENT_DATA,
    //        // TODO: Assumes the decompression ratio is no more than 5x.
    //        base: DTB_BASE,
    //        data: &mut SliceReader::new(DTB),
    //    },
    // ];
    // let mut p = payload::Payload { typ: payload::ftype::CBFS_TYPE_RAW, compression: payload::ctype::CBFS_COMPRESS_NONE, offset: 0, entry: 0, dtb: DTB_BASE, rom_len: KERNEL.len() as usize, mem_len: KERNEL.len() as usize, segs: kernel_segs };
    // p.load();
    // p.run();

    halt()
}
