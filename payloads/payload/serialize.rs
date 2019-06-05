// Copyright (c) 2016, Ryo Munakata
// All rights reserved.

// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:

// * Redistributions of source code must retain the above copyright notice, this
//   list of conditions and the following disclaimer.

// * Redistributions in binary form must reproduce the above copyright notice,
//   this list of conditions and the following disclaimer in the documentation
//   and/or other materials provided with the distribution.

// * Neither the name of rust-9p nor the names of its
//   contributors may be used to endorse or promote products derived from
//   this software without specific prior written permission.

// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

//! Serialize/deserialize cbfs files into/from binary.

extern crate byteorder;
//extern crate num_traits;

use self::byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
//use self::num_traits::FromPrimitive;
use crate::*;
//use std::io::{Cursor, Read, Result};
use std::mem;
use std::ops::{Shl, Shr, Try};

// macro_rules! decode {
//     ($decoder:expr) => {
//         Decodable::decode(&mut $decoder)?
//     };
//     ($typ:ident, $buf:expr) => {
//         $typ::from_bits_truncate(decode!($buf))
//     };
// }

// Create an unintialized buffer
// Safe to use only for writing data to it
fn create_buffer(size: usize) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(size);
    unsafe {
        buffer.set_len(size);
    }
    buffer
}

fn read_exact<R: Read + ?Sized>(r: &mut R, size: usize) -> Result<Vec<u8>> {
    let mut buf = create_buffer(size);
    r.read_exact(&mut buf[..]).and(Ok(buf))
}

/// A serializing specific result to overload operators on `Result`
///
/// # Overloaded operators
/// <<, >>, ?
pub struct SResult<T>(::std::io::Result<T>);

impl<U> Try for SResult<U> {
    type Ok = U;
    type Error = ::std::io::Error;

    fn into_result(self) -> ::std::result::Result<Self::Ok, Self::Error> {
        self.0
    }

    fn from_error(v: Self::Error) -> Self {
        SResult(Err(v))
    }

    fn from_ok(v: Self::Ok) -> Self {
        SResult(Ok(v))
    }
}

/// A wrapper class of WriteBytesExt to provide operator overloads
/// for serializing
///
/// Operator '<<' serializes the right hand side argument into
/// the left hand side encoder
#[derive(Clone, Debug)]
pub struct Encoder<W> {
    writer: W,
    bytes: usize,
}

impl<W: WriteBytesExt> Encoder<W> {
    pub fn new(writer: W) -> Encoder<W> {
        Encoder {
            writer: writer,
            bytes: 0,
        }
    }

    /// Return total bytes written
    pub fn bytes_written(&self) -> usize {
        self.bytes
    }

    /// Encode data, equivalent to: decoder << data
    pub fn encode<T: Encodable>(&mut self, data: &T) -> Result<usize> {
        let bytes = data.encode(&mut self.writer)?;
        self.bytes += bytes;
        Ok(bytes)
    }

    /// Get inner writer
    pub fn into_inner(self) -> W {
        self.writer
    }
}

impl<'a, T: Encodable, W: WriteBytesExt> Shl<&'a T> for Encoder<W> {
    type Output = SResult<Encoder<W>>;
    fn shl(mut self, rhs: &'a T) -> Self::Output {
        self.encode(rhs)?;
        SResult(Ok(self))
    }
}

impl<'a, T: Encodable, W: WriteBytesExt> Shl<&'a T> for SResult<Encoder<W>> {
    type Output = Self;
    fn shl(self, rhs: &'a T) -> Self::Output {
        let mut encoder = self.0?;
        encoder.encode(rhs)?;
        SResult(Ok(encoder))
    }
}

/// A wrapper class of ReadBytesExt to provide operator overloads
/// for deserializing
#[derive(Clone, Debug)]
pub struct Decoder<R> {
    reader: R,
}

impl<R: ReadBytesExt> Decoder<R> {
    pub fn new(reader: R) -> Decoder<R> {
        Decoder { reader: reader }
    }
    pub fn decode<T: Decodable>(&mut self) -> Result<T> {
        Decodable::decode(&mut self.reader)
    }
    /// Get inner reader
    pub fn into_inner(self) -> R {
        self.reader
    }
}

impl<'a, T: Decodable, R: ReadBytesExt> Shr<&'a mut T> for Decoder<R> {
    type Output = SResult<Decoder<R>>;
    fn shr(mut self, rhs: &'a mut T) -> Self::Output {
        *rhs = self.decode()?;
        SResult(Ok(self))
    }
}

impl<'a, T: Decodable, R: ReadBytesExt> Shr<&'a mut T> for SResult<Decoder<R>> {
    type Output = Self;
    fn shr(self, rhs: &'a mut T) -> Self::Output {
        let mut decoder = self.0?;
        *rhs = decoder.decode()?;
        SResult(Ok(decoder))
    }
}

/// Trait representing a type which can be serialized into binary
pub trait Encodable {
    /// Encode self to w and returns the number of bytes encoded
    fn encode<W: WriteBytesExt>(&self, w: &mut W) -> Result<usize>;
}

impl Encodable for u8 {
    fn encode<W: WriteBytesExt>(&self, w: &mut W) -> Result<usize> {
        w.write_u8(*self).and(Ok(mem::size_of::<Self>()))
    }
}

impl Encodable for u16 {
    fn encode<W: WriteBytesExt>(&self, w: &mut W) -> Result<usize> {
        w.write_u16::<LittleEndian>(*self)
            .and(Ok(mem::size_of::<Self>()))
    }
}

impl Encodable for u32 {
    fn encode<W: WriteBytesExt>(&self, w: &mut W) -> Result<usize> {
        w.write_u32::<LittleEndian>(*self)
            .and(Ok(mem::size_of::<Self>()))
    }
}

impl Encodable for u64 {
    fn encode<W: WriteBytesExt>(&self, w: &mut W) -> Result<usize> {
        w.write_u64::<LittleEndian>(*self)
            .and(Ok(mem::size_of::<Self>()))
    }
}

impl Encodable for String {
    fn encode<W: WriteBytesExt>(&self, w: &mut W) -> Result<usize> {
        let mut bytes = (self.len() as u16).encode(w)?;
        bytes += w.write_all(self.as_bytes()).and(Ok(self.len()))?;
        Ok(bytes)
    }
}

impl Encodable for Payload {
    fn encode<W: WriteBytesExt>(&self, w: &mut W) -> Result<usize> {
        Ok((Encoder::new(w)
            << &self.typ
            << &self.compression
            << &self.offset
            << &self.load_addr
            << &self.rom_len
            << &self.mem_len
            << &self.data)?
            .bytes_written()
        )
    }
}

/// Trait representing a type which can be deserialized from binary
pub trait Decodable: Sized {
    fn decode<R: ReadBytesExt>(r: &mut R) -> Result<Self>;
}

impl Decodable for u8 {
    fn decode<R: ReadBytesExt>(r: &mut R) -> Result<Self> {
        r.read_u8()
    }
}

impl Decodable for u16 {
    fn decode<R: ReadBytesExt>(r: &mut R) -> Result<Self> {
        r.read_u16::<LittleEndian>()
    }
}

impl Decodable for u32 {
    fn decode<R: ReadBytesExt>(r: &mut R) -> Result<Self> {
        r.read_u32::<LittleEndian>()
    }
}

impl Decodable for u64 {
    fn decode<R: ReadBytesExt>(r: &mut R) -> Result<Self> {
        r.read_u64::<LittleEndian>()
    }
}

impl Decodable for Payload {
    fn decode<R: ReadBytesExt>(r: &mut R) -> Result<Self> {
        Ok(Payload {
            typ: Decodable::decode(r)?,
            compression: Decodable::decode(r)?,
            offset: Decodable::decode(r)?,
            compresskion: Decodable::decode(r)?,
            offset: Decodable::decode(r)?,
            load_addr: Decodable::decode(r)?,
            rom_len: Decodable::decode(r)?,
            mem_len: Decodable::decode(r)?,
            dsata: Decodable::decode(r)?,
        })
    }
}

// /// Helper function to read cbfs from a byte-oriented stream
// pub fn read_msg<R: ReadBytesExt>(r: &mut R) -> Result<Msg> {
//     Decodable::decode(r)
// }

// /// Helper function to write cbfs message into a byte-oriented stream
// pub fn write_msg<W: WriteBytesExt>(w: &mut W, msg: &Msg) -> Result<usize> {
//     msg.encode(w)
// }

#[test]
fn encoder_test1() {
    let expected: Vec<u8> = (0..10).collect();
    let mut encoder = Vec::new();
    for i in 0..10 {
        (&(i as u8)).encode(&mut encoder).unwrap();
    }
    assert_eq!(expected, encoder);
}

#[test]
fn decoder_test1() {
    let expected: Vec<u8> = (0..10).collect();
    let mut decoder = Cursor::new(expected.clone());
    let mut actual: Vec<u8> = Vec::new();
    loop {
        match Decodable::decode(&mut decoder) {
            Ok(i) => actual.push(i),
            Err(_) => break,
        }
    }
    assert_eq!(expected, actual);
}

#[test]
fn msg_encode_decode1() {
    let expected = Msg {
        tag: 0xdead,
        body: Fcall::Rversion {
            msize: 40,
            version: P92000L.to_owned(),
        },
    };
    let mut buf = Vec::new();
    let _ = expected.encode(&mut buf);

    let mut readbuf = Cursor::new(buf);
    let actual = Decodable::decode(&mut readbuf);

    assert_eq!(expected, actual.unwrap());
}

    #[test]
fn payload_encode_decode1() {
    let expected = Payload{
        typ: 0,
        compression: 0,
        offset: 0,
        load_addr: 0,
        rom_len: 0,
        mem_len: 0,
        data: (0..10).collect(),
    };
    let mut buf = Vec::new();
    let _ = expected.encode(&mut buf);

    let mut readbuf = Cursor::new(buf);
    let actual = Decodable::decode(&mut readbuf);

    assert_eq!(expected, actual.unwrap());
}
