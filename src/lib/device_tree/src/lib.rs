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
extern crate heapless; 

use byteorder::{BigEndian, ByteOrder};
use core::fmt;
use model::{Driver, Result};
use wrappers::SectionReader;
use heapless::{String, Vec};
use heapless::consts::*;

/// Special value that every device tree in FDT format starts with.
pub const MAGIC: u32 = 0xd00dfeed;

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

/// In-memory reader for Flattened Device Tree format.
///
/// Does not perform any sanity checks.
pub struct FdtReader<'a, D: Driver> {
    _mem_reservation_block: SectionReader<'a, D>,
    struct_block: SectionReader<'a, D>,
    strings_block: SectionReader<'a, D>,
}

fn cursor_u32(drv: &impl Driver, cursor: &mut usize) -> Result<u32> {
    let mut data = [0; 4];
    drv.pread_exact(&mut data, *cursor)?;
    *cursor += 4;
    Ok(BigEndian::read_u32(&data))
}

// Reads a string (including null-terminator). Returns bytes read.
fn cursor_string(drv: &impl Driver, cursor: &mut usize, buf: &mut [u8]) -> Result<usize> {
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

// Reads the header of the device tree.
fn read_fdt_header(drv: &impl Driver) -> Result<FdtHeader> {
    // 10 fields, each field 4 bytes.
    const HEADER_SIZE: usize = 10 * 4;

    let mut data = [0; HEADER_SIZE];
    drv.pread_exact(&mut data, 0)?;
    let mut cursor = 0;
    let mut read_u32 = || {
        cursor += 4;
        BigEndian::read_u32(&data[(cursor - 4)..cursor])
    };

    let header = FdtHeader {
        magic: read_u32(),
        total_size: read_u32(),
        off_dt_struct: read_u32(),
        off_dt_strings: read_u32(),
        off_mem_rsvmap: read_u32(),
        _version: read_u32(),
        _last_comp_version: read_u32(),
        _boot_cpuid_phys: read_u32(),
        size_dt_strings: read_u32(),
        size_dt_struct: read_u32(),
    };

    if header.magic != MAGIC {
        return Err("invalid magic in device tree header");
    }
    Ok(header)
}

impl<'a, D: Driver> FdtReader<'a, D> {
    pub fn new(drv: &'a D) -> Result<FdtReader<'a, D>> {
        let header = read_fdt_header(drv)?;

        Ok(FdtReader {
            _mem_reservation_block: SectionReader::new(
                drv,
                header.off_mem_rsvmap as usize,
                (header.total_size - header.off_dt_struct) as usize,
            ),
            struct_block: SectionReader::new(
                drv,
                header.off_dt_struct as usize,
                header.size_dt_struct as usize,
            ),
            strings_block: SectionReader::new(
                drv,
                header.off_dt_strings as usize,
                header.size_dt_strings as usize,
            ),
        })
    }

    pub fn walk(&'a self) -> FdtIterator<'a, D> {
        FdtIterator {
            dt: self,
            cursor: 0,
            name_buf: [0; MAX_NAME_SIZE],
        }
    }
}

pub struct FdtIterator<'a, D: Driver> {
    dt: &'a FdtReader<'a, D>,
    cursor: usize,
    name_buf: [u8; MAX_NAME_SIZE],
}

impl<'a, D: Driver> FdtIterator<'a, D> {
    #[allow(clippy::should_implement_trait)]
    pub fn next<'b>(
        &'b mut self,
    ) -> Result<Option<Entry<'b, SectionReader<'a, SectionReader<'a, D>>>>> {
        loop {
            let op = num_traits::cast::FromPrimitive::from_u32(cursor_u32(
                &self.dt.struct_block,
                &mut self.cursor,
            )?);
            match op {
                Some(Token::BeginNode) => {
                    let len =
                        cursor_string(&self.dt.struct_block, &mut self.cursor, &mut self.name_buf)?
                            as usize;
                    self.cursor = align4(self.cursor);
                    match core::str::from_utf8(&self.name_buf[..len]) {
                        Ok(name) => return Ok(Some(Entry::StartNode { name })),
                        Err(_) => return Err("node name is not valid utf8"),
                    }
                }
                Some(Token::EndNode) => return Ok(Some(Entry::EndNode)),
                Some(Token::Prop) => {
                    let len = cursor_u32(&self.dt.struct_block, &mut self.cursor)? as usize;
                    let mut name_off =
                        cursor_u32(&self.dt.struct_block, &mut self.cursor)? as usize;
                    let name_len = cursor_string(
                        &self.dt.strings_block,
                        &mut name_off,
                        &mut self.name_buf[..],
                    )? as usize;
                    let value = SectionReader::new(&self.dt.struct_block, self.cursor, len);
                    self.cursor = align4(self.cursor + len);
                    match core::str::from_utf8(&self.name_buf[..name_len]) {
                        Ok(name) => return Ok(Some(Entry::Property { name, value })),
                        Err(_) => return Err("property name is not valid uft8"),
                    }
                }
                Some(Token::Nop) => continue,
                Some(Token::End) => return Ok(None),
                None => return Err("unexpected token in device tree"),
            }
        }
    }

    /// Reads the whole current node skipping all entries within it.
    pub fn skip_node(&mut self) -> Result<()> {
        let mut depth = 1;
        while let Some(item) = self.next()? {
            match item {
                Entry::StartNode { name: _ } => {
                    depth += 1;
                }
                Entry::EndNode => {
                    depth -= 1;
                    if depth == 0 {
                        return Ok(());
                    }
                }
                Entry::Property { name: _, value: _ } => continue,
            }
        }
        Err("EOF")
    }
}
// Unsure about U512 as a default size. Too big?
#[derive(Default, Debug)]
pub struct Area {
    pub description: String<U512>,
    pub compatible: String<U512>,
    // If not specified, it will be automatically computed based on previous areas (if this is
    // first area, we start with 0).
    pub offset: Option<u32>,
    pub size: u32,
    pub file: Option<String<U512>>,
}

// MAX_NAME_SIZE is 64 atm. Thus v shouldn't be able to grow beyond that.
pub fn read_all(d: &dyn Driver) -> Vec<u8, U64> {
    let mut v = Vec::new();
    v.resize(MAX_NAME_SIZE, 0).expect("Tried resizing beyond v's size");
    // Safe to unwrap because SliceReader does not return an error.
    // as_mut_slice() is not implemented on heapless::Vec. However:
    // "Equivalent to &mut s[..].": https://doc.rust-lang.org/std/vec/struct.Vec.html#method.as_mut_slice
    let size = d.pread(&mut v, 0).unwrap();
    v.truncate(size);
    v
}

pub fn read_area_node<D: Driver>(iter: &mut FdtIterator<D>) -> Result<Area> {
    let mut area = Area {
        ..Default::default()
    };
    while let Some(item) = iter.next()? {
        match item {
            Entry::StartNode { name: _ } => {
                iter.skip_node()?;
            }
            Entry::EndNode => return Ok(area),
            Entry::Property { name, value } => {
                let data = read_all(&value);
                match (name, infer_type(&data[..])) {
                    ("description", Type::String(x)) => area.description = String::from(x),
                    ("compatible", Type::String(x)) => area.compatible = String::from(x),
                    ("offset", Type::U32(x)) => area.offset = Some(x),
                    ("size", Type::U32(x)) => area.size = x,
                    ("file", Type::String(x)) => area.file = Some(String::from(x)),
                    (_, _) => {}
                }
            }
        }
    }
    Ok(area)
}

/// Reads the device tree in FDT format from given driver and writes it in human readable form to
/// given writer.
pub fn print_fdt(fdt: &impl Driver, w: &mut impl core::fmt::Write) -> Result<()> {
    let reader = FdtReader::new(fdt)?;
    let mut iter = reader.walk();
    let mut depth = 0;
    while let Some(entry) = iter.next()? {
        match entry {
            Entry::StartNode { name } => {
                depth += 1;
                write!(w, "{:depth$}{}\r\n", "", name, depth = depth * 2)
                    .map_err(|_e| "failed to write")?;
            }
            Entry::EndNode {} => {
                depth -= 1;
            }
            Entry::Property { name, value: v } => {
                let buf = &mut [0; 1024];
                let len = match v.pread(buf, 0) {
                    Ok(x) => x,
                    Err("EOF") => 0,
                    Err(y) => return Err(y),
                };
                let val = infer_type(&buf[..len]);
                write!(w, "{:depth$}{} = {}\r\n", "", name, val, depth = depth * 2,)
                    .map_err(|_e| "failed to write")?;
            }
        }
    }
    Ok(())
}

pub enum Entry<'a, D: Driver> {
    StartNode { name: &'a str },
    EndNode,
    Property { name: &'a str, value: D },
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
    if data.is_empty() {
        return Type::Empty;
    }
    if let Some(i) = data.iter().position(|&c| !is_print(c)) {
        if i == data.len() - 1 && data[i] == 0 {
            match core::str::from_utf8(&data[..data.len() - 1]) {
                Ok(ret) => return Type::String(ret),
                Err(_e) => (),
            }
        }
    }
    if data.len() == 4 {
        return Type::U32(BigEndian::read_u32(data));
    }
    if data.len() == 8 {
        return Type::U64(BigEndian::read_u64(data));
    }
    Type::PropEncodedArray(data)
}

fn is_print(c: u8) -> bool {
    (0x20..0x7f).contains(&c)
}
