//! `device_tree` is a library for operating on the Device Tree data structure.
//!
//! Device tree is a data structure for describing the hardware. Useful abbreviations:
//!
//!   * FDT - Flattened Device Tree format (also called Device Tree Blob (DTB) format) is a compact
//!           binary encoding of the device tree structure.
//!   * DTB - Device Tree Blob format (also called Flattened Device Tree (FDT) format) is a compact
//!           binary encoding of the device tree structure.
//!   * DTS - Device Tree Syntax is a human friendly text representation of the device tree.
//!   * DTC - Device Tree Compiler is a tool that can convert device tree between different
//!           formats.
//!
//! The code in this library is heapless (no heap allocated data structure is used) so that it can
//! be used in bootloader.
//!
//! For more information about Device Tree Standard, see https://www.devicetree.org/.

#![no_std]
#![deny(warnings)]

#[macro_use]
extern crate num_derive;

use byteorder::{BigEndian, ByteOrder};
use core::fmt;
use model::{Driver, Result};
use wrappers::SectionReader;

/// Special value that every device tree in FDT format starts with.
pub const MAGIC: u32 = 0xd00dfeed;

/// The largest nesting level of the device tree supported by this library. It is a fixed value
/// because this library does not use heap so we need to know how much memory to allocate in
/// advance. Value 16 is trying to strike a good balance between supporting
/// reasonable amount of nesting and not allocating too much memory.
pub const MAX_DEPTH: usize = 16;

/// Maximum length of a node or property name.
///
/// According to standard, the name has format `node-name@unit-address`:
///
/// * the `node-name` part is required to be between 1-31 characters (bytes)
/// * `unit-address` is optional, but if present, should contain address in hex format.
///   We are reserving 32 characters to encode 128 bit address.
/// * We are reserving 1 byte for null terminator.
///
/// An error is returned for longer names.
///
/// TODO: Reserve 1 byte for "@" character.
/// TODO: Do we need to encode 128 bit addresses, or would it be enough to just support
/// 32 and 64 bit addresses?
pub const MAX_NAME_SIZE: usize = 31 + 1 + 32;

struct FdtHeader {
    magic: u32,
    total_size: u32,
    off_dt_struct: u32,
    off_dt_strings: u32,
    off_mem_rsvmap: u32,
    _version: u32,
    _last_comp_version: u32,
    _boot_cpuid_phys: u32,
    size_dt_strings: u32,
    size_dt_struct: u32,
}

#[derive(FromPrimitive)]
#[repr(u32)]
enum Token {
    BeginNode = 0x1,
    EndNode = 0x2,
    Prop = 0x3,
    Nop = 0x4,
    End = 0x9,
}

/// Represents a path to device tree node.
/// It is also being used by properties, in which case the last component of the path stores the
/// name of the property.
pub struct Path {
    depth: usize,
    len: [usize; MAX_DEPTH],
    buf: [[u8; MAX_NAME_SIZE]; MAX_DEPTH],
}

impl Path {
    /// Returns the number of components in the path.
    ///
    /// Returns 1 for root path `/`.
    /// Returns node.depth() + 1 when used for properties.
    pub fn depth(&self) -> usize {
        self.depth
    }

    /// Returns value of the i-th component of the path.
    /// It has to be lower than `depth()`.
    pub fn at(&self, i: usize) -> &[u8] {
        &self.buf[i][..self.len[i]]
    }

    /// Returns the last component of the path.
    /// When used for properties, returns the property name.
    pub fn name(&self) -> &str {
        let name = self.at(self.depth - 1);
        unsafe { core::str::from_utf8_unchecked(name) }
    }
}

/// In-memory reader for Flattened Device Tree format.
///
/// Does not perform any sanity checks.
pub struct FdtReader<'a> {
    _mem_reservation_block: SectionReader<'a>,
    struct_block: SectionReader<'a>,
    strings_block: SectionReader<'a>,
}

fn read_u32(drv: &dyn Driver, offset: usize) -> Result<u32> {
    let mut data = [0; 4];
    drv.pread(&mut data, offset)?;
    Ok(BigEndian::read_u32(&data))
}

fn cursor_u32(drv: &dyn Driver, cursor: &mut usize) -> Result<u32> {
    let mut data = [0; 4];
    *cursor += drv.pread(&mut data, *cursor)?;
    Ok(BigEndian::read_u32(&data))
}

// Reads a string (including null-terminator). Returns bytes read.
fn cursor_string(drv: &dyn Driver, cursor: &mut usize, buf: &mut [u8]) -> Result<usize> {
    for i in 0..buf.len() {
        *cursor += drv.pread(&mut buf[i..i + 1], *cursor)?;
        if buf[i] == 0 {
            return Ok(i);
        }
    }
    Err("name too long")
}

fn align4(x: usize) -> usize {
    (x + 3) & !3
}

impl<'a> FdtReader<'a> {
    pub fn new(drv: &'a dyn Driver) -> Result<FdtReader<'a>> {
        let header = FdtHeader {
            magic: read_u32(drv, 0x0)?,
            total_size: read_u32(drv, 0x4)?,
            off_dt_struct: read_u32(drv, 0x8)?,
            off_dt_strings: read_u32(drv, 0xc)?,
            off_mem_rsvmap: read_u32(drv, 0x10)?,
            _version: read_u32(drv, 0x14)?,
            _last_comp_version: read_u32(drv, 0x18)?,
            _boot_cpuid_phys: read_u32(drv, 0x1c)?,
            size_dt_strings: read_u32(drv, 0x20)?,
            size_dt_struct: read_u32(drv, 0x24)?,
        };

        if header.magic != MAGIC {
            return Err("invalid magic");
        }

        Ok(FdtReader::<'a> {
            _mem_reservation_block: SectionReader::new(drv, header.off_mem_rsvmap as usize, (header.total_size - header.off_dt_struct) as usize),
            struct_block: SectionReader::new(drv, header.off_dt_struct as usize, header.size_dt_struct as usize),
            strings_block: SectionReader::new(drv, header.off_dt_strings as usize, header.size_dt_strings as usize),
        })
    }

    pub fn walk(&'a self) -> FdtIterator<'a> {
        FdtIterator { dt: self, cursor: 0, depth: 0, len_buf: [0; MAX_DEPTH], path_buf: [[0; MAX_NAME_SIZE]; MAX_DEPTH] }
    }
}

/// Iterator used to walk through device tree representation.
pub struct FdtIterator<'a> {
    dt: &'a FdtReader<'a>,
    cursor: usize,
    depth: usize,
    len_buf: [usize; MAX_DEPTH],
    path_buf: [[u8; MAX_NAME_SIZE]; MAX_DEPTH],
}

// TODO: how to return errors from an iterator?
impl<'a> Iterator for FdtIterator<'a> {
    type Item = Entry<'a>;

    fn next(&mut self) -> Option<Entry<'a>> {
        loop {
            let op = num_traits::cast::FromPrimitive::from_u32(cursor_u32(&self.dt.struct_block, &mut self.cursor).unwrap());
            match op {
                Some(Token::BeginNode) => {
                    if self.depth == MAX_DEPTH {
                        panic!("max depth");
                    }
                    self.len_buf[self.depth] = cursor_string(&self.dt.struct_block, &mut self.cursor, &mut self.path_buf[self.depth]).unwrap() as usize;
                    self.cursor = align4(self.cursor);
                    self.depth += 1;
                    // Note: This performs a copy of the whole path_buf array!
                    return Some(Entry::Node { path: Path { depth: self.depth, len: self.len_buf, buf: self.path_buf } });
                }
                Some(Token::EndNode) => {
                    self.depth -= 1;
                }
                Some(Token::Prop) => {
                    if self.depth == MAX_DEPTH {
                        panic!("max depth");
                    }
                    let len = cursor_u32(&self.dt.struct_block, &mut self.cursor).unwrap() as usize;
                    let mut nameoff = cursor_u32(&self.dt.struct_block, &mut self.cursor).unwrap() as usize;
                    self.len_buf[self.depth] = cursor_string(&self.dt.strings_block, &mut nameoff, &mut self.path_buf[self.depth][..]).unwrap() as usize;
                    let value = SectionReader::new(&self.dt.struct_block, self.cursor, len);
                    self.cursor = align4(self.cursor + len);
                    // Note: This performs a copy of the whole path_buf array!
                    return Some(Entry::Property::<'a> { path: Path { depth: self.depth + 1, len: self.len_buf, buf: self.path_buf }, value: value });
                }
                Some(Token::Nop) => continue,
                Some(Token::End) => return None,
                None => panic!("unexpected token"),
            }
        }
    }
}

pub enum Entry<'a> {
    Node { path: Path },
    Property { path: Path, value: SectionReader<'a> },
}

/// Typed value of device tree properties.
pub enum Type<'a> {
    Empty,
    String(&'a str),
    U32(u32),
    U64(u64),
    PropEncodedArray(&'a [u8]),
}

impl<'a> core::fmt::Display for Type<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Empty => write!(f, "<>"),
            Type::String(s) => write!(f, "{:?}", s),
            Type::U32(x) => write!(f, "{:#x}", x),
            Type::U64(x) => write!(f, "{:#x}", x),
            Type::PropEncodedArray(x) => write!(f, "{:?}", x),
        }
    }
}

/// Guesses the type of the property value and returns parsed type.
///
/// Sometimes it is impossible to unambiguously infer the type. For example, ['f', 'o', 'o', 0]
/// bytes could represent a "foo" string, but could also represent u32 number or just a sequence of
/// bytes. Because of this unambiguity, this method should only be used to print the content of the
/// data to user.
pub fn infer_type(data: &[u8]) -> Type {
    if data.len() == 0 {
        return Type::Empty;
    }
    if let Some(i) = data.iter().position(|&c| !is_print(c)) {
        if i == data.len() - 1 && data[i] == 0 {
            match core::str::from_utf8(&data[..data.len() - 1]) {
                Ok(ret) => return Type::String(ret),
                Err(_e) => {}
            }
        }
    }
    if data.len() == 4 {
        return Type::U32(BigEndian::read_u32(&data));
    }
    if data.len() == 8 {
        return Type::U64(BigEndian::read_u64(&data));
    }
    Type::PropEncodedArray(data)
}

fn is_print(c: u8) -> bool {
    0x20 <= c && c < 0x7f
}
