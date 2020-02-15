use core::intrinsics::{copy, transmute};
use core::fmt::Write;
use model::{Driver, EOF};
use postcard::from_bytes;
use serde::Deserialize;
use wrappers::{Memory, SectionReader};
use print;
//use serde::{Serialize, Deserialize};

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
    PAYLOAD_SEGMENT_BAD = 0xFFFFFFFF,
}

// I give up.
impl From<u32> for stype {
    fn from(s: u32) -> Self {
        match s {
            0x434F4445 => stype::PAYLOAD_SEGMENT_CODE,
            0x44415441=> stype::PAYLOAD_SEGMENT_DATA,
            0x42535320=> stype::PAYLOAD_SEGMENT_BSS,
            0x50415241=> stype::PAYLOAD_SEGMENT_PARAMS,
            0x454E5452=> stype::PAYLOAD_SEGMENT_ENTRY,
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
    pub data: &'a mut dyn Driver, // Segments
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct CBFSSeg {
    pub typ: u32,
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
        write!(w, "Greetings from the payload loader:\n").unwrap();
        // TODO: how many segments are there?
        // The coreboot convention: ENTRY marks the last segment.
        // we need to ensure we create them that way too.
        let mut hdr:usize = 0;
        loop {
            let v = &mut [0u8; 40];
            write!(w, "rom at {:#x}\n", self.rom).unwrap();
            let rom = SectionReader::new(&Memory {}, self.rom + hdr, 40);
            hdr += 40;
            match rom.pread(v, 0) {
                Ok(n) => write!(w, "{} bytes", n).unwrap(),
                Err(err) => panic!("hell  {}", err),
            }
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};

let mut rdr = Cursor::new(vec![2, 5, 3, 0]);
// Note that we use type parameters to indicate which kind of byte order
// we want!
assert_eq!(517, rdr.read_u16::<BigEndian>().unwrap());
assert_eq!(768, rdr.read_u16::<BigEndian>().unwrap());            let seg: CBFSSeg = from_bytes(v).unwrap();
            let typ: stype = core::convert::From::from(seg.typ);
            let mut load = seg.load as usize;
            write!(w, "seg is {:#x} {:#x} {:#x} {:#x} {:#x} \n", seg.typ, seg.off, seg.load, seg.len, seg.memlen).unwrap();
            // Copy from driver into segment.
            let mut buf = [0u8; 512];
            let mut off = seg.off as usize;
            
            match (self.dtb == 0, typ) {
                (_, stype::PAYLOAD_SEGMENT_ENTRY)  => {
                    self.entry = load;
                    panic!("loaded");
                },
                (true, stype::PAYLOAD_SEGMENT_DATA) => self.dtb = load,
                (false, stype::PAYLOAD_SEGMENT_DATA) => {
                    let data = SectionReader::new(&Memory {}, self.rom + off, seg.len as usize);
                    loop {
                        let size = match data.pread(&mut buf, off) {
                            
                            Ok(x) => x,
                            EOF => break,
                            _ => panic!("driver error"),
                        };
                        unsafe { copy(buf.as_ptr(), load as *mut u8, size) };
                        off += size;
                        load += size;
                    }
                },
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
            write!(w, "jump to {}\n", self.entry).unwrap();
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
