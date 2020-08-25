extern crate wrappers;

use model::Driver;
use wrappers::SliceReader;

fn check_pread(device: &impl Driver, pos: usize, expected: &[u8]) {
    let mut buf = std::vec::Vec::new();
    buf.resize(expected.len(), 0);
    let size = device.pread(buf.as_mut_slice(), pos).unwrap();
    assert_eq!(size, expected.len());
    assert_eq!(buf, expected);
}

#[test]
fn test_slice_reader() {
    let data = [1, 2, 3, 4, 5];
    let driver = SliceReader::new(&data);

    check_pread(&driver, 0, &[1]);
    check_pread(&driver, 0, &[1, 2, 3, 4, 5]);
    check_pread(&driver, 1, &[2, 3, 4, 5]);
    check_pread(&driver, 4, &[5]);
}

#[test]
fn test_slice_reader_returns_zero_for_empty_buf() {
    let data = [1, 2, 3, 4, 5];
    let driver = SliceReader::new(&data);
    let mut buf = [0; 0];

    assert_eq!(0, driver.pread(&mut buf, 1).unwrap());
}

#[test]
#[should_panic]
fn test_slice_reader_returns_eof() {
    let data = [1, 2, 3, 4, 5];
    let driver = SliceReader::new(&data);
    let mut buf = [0; 5];

    driver.pread(&mut buf, 5).unwrap();
}

#[test]
#[should_panic]
fn test_slice_reader_returns_eof_with_empty_buf() {
    let data = [1, 2, 3, 4, 5];
    let driver = SliceReader::new(&data);
    let mut buf = [0; 0];

    driver.pread(&mut buf, 5).unwrap();
}
