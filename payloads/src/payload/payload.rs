use core::fmt::Write;
use core::intrinsics::{copy, transmute};
use model::{Driver, EOF};
use postcard::from_bytes;
use print;
use serde::Deserialize;
use wrappers::{Memory, SectionReader};
pub type EntryPoint = unsafe extern "C" fn(r0: usize, dtb: usize);

/// compression types
#[derive(PartialEq, Debug)]
#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum ctype {
    CBFS_COMPRESS_NONE = 0,
    CBFS_COMPRESS_LZMA = 1,
    CBFS_COMPRESS_LZ4 = 2,
}

/// cbfs file attrs
#[derive(PartialEq, Debug)]
#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum fattr {
    CBFS_FILE_ATTR_TAG_UNUSED = 0,
    CBFS_FILE_ATTR_TAG_UNUSED2 = 0xffffffff,
    CBFS_FILE_ATTR_TAG_COMPRESSION = 0x42435a4c,
    CBFS_FILE_ATTR_TAG_HASH = 0x68736148,
    CBFS_FILE_ATTR_TAG_POSITION = 0x42435350,  // PSCB
    CBFS_FILE_ATTR_TAG_ALIGNMENT = 0x42434c41, // ALCB
}

/// cbfs architecture types,
#[derive(PartialEq, Debug)]
#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum atype {
    CBFS_ARCHITECTURE_UNKNOWN = 0xFFFFFFFF,
    CBFS_ARCHITECTURE_X86 = 0x00000001,
    CBFS_ARCHITECTURE_ARM = 0x00000010,
}

/// cbfs header types,
#[derive(PartialEq, Debug)]
#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum htype {
    CBFS_HEADER_MAGIC = 0x4F524243,
    CBFS_HEADER_VERSION1 = 0x31313131,
    CBFS_HEADER_VERSION2 = 0x31313132,
}

/// cbfs file types
#[derive(PartialEq, Debug)]
#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum ftype {
    CBFS_TYPE_DELETED = 0x00000000,
    CBFS_TYPE_DELETED2 = 0xffffffff,
    CBFS_TYPE_STAGE = 0x10,
    CBFS_TYPE_SELF = 0x20,
    CBFS_TYPE_FIT = 0x21,
    CBFS_TYPE_OPTIONROM = 0x30,
    CBFS_TYPE_BOOTSPLASH = 0x40,
    CBFS_TYPE_RAW = 0x50,
    CBFS_TYPE_VSA = 0x51,
    CBFS_TYPE_MBI = 0x52,
    CBFS_TYPE_MICROCODE = 0x53,
    CBFS_TYPE_FSP = 0x60,
    CBFS_TYPE_MRC = 0x61,
    CBFS_TYPE_MMA = 0x62,
    CBFS_TYPE_EFI = 0x63,
    CBFS_TYPE_STRUCT = 0x70,
    CBFS_COMPONENT_CMOS_DEFAULT = 0xaa,
    CBFS_TYPE_SPD = 0xab,
    CBFS_TYPE_MRC_CACHE = 0xac,
    CBFS_COMPONENT_CMOS_LAYOUT = 0x01aa,
}

/// Payload segments types
#[derive(PartialEq, Debug)]
#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum stype {
    PAYLOAD_SEGMENT_CODE = 0x434F4445,
    PAYLOAD_SEGMENT_DATA = 0x44415441,
    PAYLOAD_SEGMENT_BSS = 0x42535320,
    PAYLOAD_SEGMENT_PARAMS = 0x50415241,
    PAYLOAD_SEGMENT_ENTRY = 0x454E5452,
    PAYLOAD_SEGMENT_DTB = 0x44544220,
    PAYLOAD_SEGMENT_BAD = 0xFFFFFFFF,
    CBFS_SEGMENT_CODE = 0x45444F43,
    CBFS_SEGMENT_DATA = 0x41544144,
    CBFS_SEGMENT_BSS = 0x20535342,
    CBFS_SEGMENT_PARAMS = 0x41524150,
    CBFS_SEGMENT_ENTRY = 0x52544E45,
}

// TODO:
// Maybe do what they suggest here? https://enodev.fr/posts/rusticity-convert-an-integer-to-an-enum.html
// I give up.
impl From<u32> for stype {
    fn from(s: u32) -> Self {
        match s {
            0x434F4445 => stype::PAYLOAD_SEGMENT_CODE,
            0x44415441 => stype::PAYLOAD_SEGMENT_DATA,
            0x42535320 => stype::PAYLOAD_SEGMENT_BSS,
            0x50415241 => stype::PAYLOAD_SEGMENT_PARAMS,
            0x454E5452 => stype::PAYLOAD_SEGMENT_ENTRY,
            0x44544220 => stype::PAYLOAD_SEGMENT_DTB,
            0x45444F43 => stype::CBFS_SEGMENT_CODE,
            0x41544144 => stype::CBFS_SEGMENT_DATA,
            0x20535342 => stype::CBFS_SEGMENT_BSS,
            0x41524150 => stype::CBFS_SEGMENT_PARAMS,
            0x52544E45 => stype::CBFS_SEGMENT_ENTRY,
            _ => stype::PAYLOAD_SEGMENT_BAD,
        }
    }
}
/// A payload. oreboot will only have payloads for anything past the romstage.
/// N.B. This struct is NOT designed to be deserialized.
// #[derive(Debug)]
pub struct Payload<'a> {
    /// Type of payload
    pub typ: ftype,
    /// Compression type
    pub compression: ctype,
    /// Offset in ROM
    pub offset: usize,
    /// Physical load address
    pub entry: usize,
    /// the dtb
    pub dtb: usize,
    /// Length in ROM
    pub rom_len: usize,
    /// Length in memory (i.e. once uncompressed)
    pub mem_len: usize,
    /// Segments
    pub segs: &'a [Segment<'a>],
}

/// A payload. oreboot will only have payloads for anything past the romstage.
/// N.B. This struct is NOT designed to be deserialized.
// #[derive(Debug)]
pub struct StreamPayload {
    /// base of rom
    pub rom: usize,
    /// Type of payload
    pub typ: ftype,
    /// Compression type
    pub compression: ctype,
    /// Offset in ROM
    pub offset: usize,
    /// Physical load address
    pub entry: usize,
    /// the dtb
    pub dtb: usize,
    /// Length in ROM
    pub rom_len: usize,
    /// Length in memory (i.e. once uncompressed)
    pub mem_len: usize,
}

// #[derive(Debug)]
pub struct Segment<'a> {
    /// Type
    pub typ: stype,
    /// Load address in memory
    pub base: usize,
    /// The data
    pub data: &'a mut dyn Driver,
}

#[derive(Deserialize, Debug)]
pub struct CBFSSeg {
    pub typ: u32,
    pub comp: u32,
    pub off: u32,
    pub load: u64,
    pub len: u32,
    pub memlen: u32,
}

// Stream payloads copy segments one at a time to memory.
// This avoids the problem in coreboot (and in Rust) where we have
// to pre-declare a fixed-size array and hope it's big enough.
// TOOD: remove all uses of non-streaming payloads.
impl StreamPayload {
    /// Load the payload in memory. Returns the entrypoint.
    pub fn load(&mut self, w: &mut print::WriteTo) {
        // TODO: how many segments are there?
        // The coreboot convention: ENTRY marks the last segment.
        // we need to ensure we create them that way too.
        let mut hdr: usize = 0;
        write!(w, "loading ...\n").unwrap();
        loop {
            write!(w, "decode header at {}\n", hdr).unwrap();
            let v = &mut [0u8; 28];
            let rom = SectionReader::new(&Memory {}, self.rom + hdr, 28);
            hdr += 28;
            write!(w, "decode header now at {}\n", hdr).unwrap();
            rom.pread(v, 0).unwrap();
            let mut seg: CBFSSeg = from_bytes(v).unwrap();
            // Better minds than mine can figure this shit out. Or when I learn more.
            let typ: stype = core::convert::From::from(seg.typ);
            match typ {
                stype::CBFS_SEGMENT_ENTRY
                | stype::CBFS_SEGMENT_CODE
                | stype::CBFS_SEGMENT_DATA
                | stype::CBFS_SEGMENT_BSS
                | stype::CBFS_SEGMENT_PARAMS => {
                    write!(w, "seg {:?}\n", seg).unwrap();
                    seg.off = seg.off.to_be();
                    seg.load = seg.load.to_be();
                    seg.len = seg.len.to_be();
                    seg.memlen = seg.memlen.to_be();
                    write!(w, "afterward seg {:?}\n", seg).unwrap();
                }
                stype::PAYLOAD_SEGMENT_BAD => {
                    panic!("seg now {:?} {:?} typ {:?}", self.rom, seg, typ);
                }
                _ => {
                    write!(w, "Seg is unchanged: {:?}\n", seg).unwrap();
                }
            }

            let mut load = seg.load as usize;

            // Copy from driver into segment.
            let mut buf = [0u8; 512];
            match typ {
                // in cbfs, this is always the LAST segment.
                // We should continue the convention.
                stype::CBFS_SEGMENT_ENTRY => {
                    write!(w, "ENTRY {}\n", load).unwrap();
                    self.entry = load;
                    return;
                }
                stype::PAYLOAD_SEGMENT_DTB => self.dtb = load,
                stype::CBFS_SEGMENT_DATA | stype::CBFS_SEGMENT_CODE => {
                    write!(w, "set up from at {:x}\n", self.rom + seg.off as usize).unwrap();
                    let data = SectionReader::new(
                        &Memory {},
                        self.rom + seg.off as usize,
                        seg.len as usize,
                    );
                    let mut i: usize = 0;
                    loop {
                        let size = match data.pread(&mut buf, i) {
                            Ok(x) => x,
                            EOF => break,
                            _ => panic!("driver error"),
                        };
                        //write!(w, "Copy to {:x} for {:x}\n", load, size).unwrap();
                        unsafe { copy(buf.as_ptr(), load as *mut u8, size) };
                        i += size;
                        load += size;
                    }
                }
                _ => panic!("fix payload loader {} {:x}", self.dtb, seg.typ),
            }
        }
    }

    /// Run the payload. This might not return.
    pub fn run(&self, w: &mut print::WriteTo) {
        // Jump to the payload.
        // See: linux/Documentation/arm/Booting
        unsafe {
            let f = transmute::<usize, EntryPoint>(self.entry);
            write!(w, "on to {:#x}", self.entry).unwrap();
            f(1, self.dtb);
        }
        // TODO: error when payload returns.
    }
}

// to be deprecated
impl<'a> Payload<'a> {
    /// Load the payload in memory. Returns the entrypoint.
    pub fn load(&mut self) {
        // Copy the segments into RAM.
        for s in self.segs {
            // Copy from driver into segment.
            let mut buf = [0u8; 512];
            let mut off = 0;
            if s.typ == stype::PAYLOAD_SEGMENT_ENTRY {
                self.entry = s.base;
            }
            if self.dtb == 0 && s.typ == stype::PAYLOAD_SEGMENT_DATA {
                self.dtb = s.base
            }
            loop {
                let size = match s.data.pread(&mut buf, off) {
                    Ok(x) => x,
                    EOF => break,
                    _ => panic!("driver error"),
                };
                // TODO: This incurs a second copy. There's probably a better way.
                unsafe { copy(buf.as_ptr(), (s.base + off) as *mut u8, size) };
                off += size;
            }
        }
    }

    /// Run the payload. This might not return.
    pub fn run(&self) {
        // Jump to the payload.
        // See: linux/Documentation/arm/Booting
        unsafe {
            let f = transmute::<usize, EntryPoint>(self.entry);
            f(1, self.dtb);
        }
        // TODO: error when payload returns.
    }
}
