use core::intrinsics::{copy, transmute};
use model::{Driver, EOF};

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
}

/// A payload. oreboot will only have payloads for anything past the romstage.
// #[derive(Debug)]
pub struct Payload<'a> {
    /// Type of payload
    pub typ: ftype,
    /// Compression type
    pub compression: ctype,
    /// Offset in ROM
    pub offset: u32,
    /// Physical load address
    pub load_addr: u64,
    /// Length in ROM
    pub rom_len: u32,
    /// Length in memory (i.e. once uncompressed)
    pub mem_len: u32,
    /// Segments
    pub segs: &'a [Segment<'a>],
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

impl<'a> Payload<'a> {
    /// Load the payload in memory. Returns the entrypoint.
    pub fn load(&self) {
        // Copy the segments into RAM.
        for s in self.segs {
            // Copy from driver into segment.
            let mut buf = [0u8; 512];
            let mut off = 0;
            loop {
                let size = match s.data.pread(&mut buf, off) {
                    Ok(x) => x,
                    EOF => return,
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
        // TODO: come up with a better model that is not zimage specific.
        // Find the segment containing the entrypoint.
        let entry = self.segs.iter().find(|&x| x.typ == stype::PAYLOAD_SEGMENT_ENTRY).expect("no entrypoint").base;
        // Find the segment containing the device tree.
        let dtb = self.segs.iter().find(|&x| x.typ == stype::PAYLOAD_SEGMENT_DATA).map_or(0, |x| x.base);

        // Jump to the payload.
        // See: linux/Documentation/arm/Booting

        unsafe {
            let f = transmute::<usize, unsafe extern "C" fn(r0: usize, dtb: usize)>(entry as usize);
            f(0, dtb);
        }
        // TODO: error when payload returns.
    }
}
