use super::*;

pub struct OneByteRead<'a> {
    buf: &'a [u8],
}

impl<'a> Driver for OneByteRead<'a> {
    fn pread(&self, data: &mut [u8], pos: usize) -> Result<usize> {
        if pos >= self.buf.len() {
            return EOF;
        }
        if data.len() == 0 {
            return Ok(0);
        }
        data[0] = self.buf[pos];
        Ok(1)
    }
    fn pwrite(&mut self, _data: &[u8], _pos: usize) -> Result<usize> {
        Err("not implemented")
    }
    fn shutdown(&mut self) {}
}

#[test]
fn pread_exact_reads_all_bytes() {
    let data: [u8; 4] = [1, 2, 3, 4];
    let drv = &mut OneByteRead { buf: &data };

    let mut got: [u8; 3] = [0, 0, 0];
    pread_exact(drv, &mut got, 0).unwrap();
    assert_eq!(data[0..3], got);
}

#[test]
fn pread_exact_reads_all_bytes_from_custom_position() {
    let data: [u8; 4] = [1, 2, 3, 4];
    let drv = &mut OneByteRead { buf: &data };

    let mut got: [u8; 3] = [0, 0, 0];
    pread_exact(drv, &mut got, 1).unwrap();
    assert_eq!(data[1..4], got);
}

#[test]
fn pread_exact_returns_ok_for_empty_data_buf() {
    let data: [u8; 4] = [1, 2, 3, 4];
    let drv = &mut OneByteRead { buf: &data };

    let mut got: [u8; 0] = [];
    pread_exact(drv, &mut got, 10).unwrap();
}

#[test]
fn pread_exact_returns_eof_when_not_enough_bytes_available() {
    let data: [u8; 4] = [1, 2, 3, 4];
    let drv = &mut OneByteRead { buf: &data };

    let mut got: [u8; 5] = [0, 0, 0, 0, 0];
    assert_eq!(pread_exact(drv, &mut got, 0).err(), Some("EOF"));
}
