#![no_std]

#[macro_use]
extern crate num_derive;

use byteorder::{BigEndian, ByteOrder};
use core::fmt;
use drivers::model::{Driver, Result};
use drivers::wrappers::SectionReader;

const MAGIC: u32 = 0xd00dfeed;
const MAX_DEPTH: usize = 16;
// Maximum length of a node or property name. This is 31 for the name, 32 for the unit name and 1
// for the null terminator. An error is returned for longer names.
const MAX_NAME_SIZE: usize = 31 + 32 + 1;

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

pub struct Path {
    depth: usize,
    len: [usize; MAX_DEPTH],
    buf: [[u8; MAX_NAME_SIZE]; MAX_DEPTH],
}

impl Path {
    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn at(&self, i: usize) -> &[u8] {
        &self.buf[i][..self.len[i]]
    }

    pub fn name(&self) -> &str {
        let name = self.at(self.depth - 1);
        unsafe { core::str::from_utf8_unchecked(name) }
    }
}

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

/// In-memory device tree traversal.
/// Does not perform any sanity checks.
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

pub fn infer_type(data: &[u8]) -> Type {
    if data.len() == 0 {
        return Type::Empty;
    }
    if let Some(i) = data.iter().position(|&c| !is_print(c)) {
        if i == data.len() - 1 && data[i] == 0 {
            return Type::String(unsafe { core::str::from_utf8_unchecked(&data[..data.len() - 1]) });
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
    0x20 < c && c < 0xff
}
