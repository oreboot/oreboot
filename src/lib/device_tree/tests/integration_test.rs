extern crate device_tree;

use device_tree::{Entry, FdtReader};
use model::Driver;
use std::io::Write;
use std::process::{Command, Stdio};
use wrappers::SliceReader;

fn assert_node<D: Driver>(entry: Entry<D>, name: &str, depth: usize) {
    if let Entry::Node { path } = entry {
        assert_eq!(path.name(), name);
        assert_eq!(path.depth(), depth);
    } else {
        panic!("Expected Node!")
    }
}

fn assert_property<D: Driver>(entry: Entry<D>, expected_name: &str, expected_depth: usize, expected_value: &[u8]) {
    if let Entry::Property { path, value } = entry {
        assert_eq!(path.name(), expected_name);
        assert_eq!(path.depth(), expected_depth);
        assert_eq!(read_all(&value), expected_value);
    } else {
        panic!("Expected Node!")
    }
}

fn dts_to_dtb(dts: &str) -> std::vec::Vec<u8> {
    let mut dtc = Command::new("dtc").arg("-O").arg("dtb").stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    dtc.stdin.as_mut().unwrap().write_all(dts.as_bytes()).unwrap();
    let output = dtc.wait_with_output().unwrap();
    if output.status.success() {
        return output.stdout;
    }
    panic!("dtc command failed: {:?}", String::from_utf8_lossy(&output.stderr))
}

fn read_all(d: &impl Driver) -> std::vec::Vec<u8> {
    let mut data = [0; 500];
    let size = match d.pread(&mut data, 0) {
        Ok(x) => Ok(x),
        Err("EOF") => Ok(0),
        Err(other) => Err(other),
    }
    .unwrap();
    let mut result = std::vec::Vec::new();
    result.extend_from_slice(&data[0..size]);
    return result;
}

#[test]
fn test_reads_empty_device_tree() {
    let data = dts_to_dtb("/dts-v1/; / { };");
    let slice_reader = &SliceReader::new(&data);
    let reader = FdtReader::new(slice_reader).unwrap();
    let mut it = reader.walk();

    assert_node(it.next().unwrap(), "", 1);
    assert!(it.next().is_none());
}

#[test]
fn test_reads_properties() {
    let data = dts_to_dtb(
        r#"
/dts-v1/;
/ { 
    #address-cells = <1>;
};"#,
    );
    let slice_reader = &SliceReader::new(&data);
    let reader = FdtReader::new(slice_reader).unwrap();
    let mut it = reader.walk();

    assert_node(it.next().unwrap(), "", 1);
    assert_property(it.next().unwrap(), "#address-cells", 2, &vec![0, 0, 0, 1]);
    assert!(it.next().is_none());
}

#[test]
fn test_reads_empty_properties() {
    let data = dts_to_dtb(
        r#"
/dts-v1/;
/ { 
    #address-cells;
};"#,
    );
    let slice_reader = &SliceReader::new(&data);
    let reader = FdtReader::new(slice_reader).unwrap();
    let mut it = reader.walk();

    assert_node(it.next().unwrap(), "", 1);
    assert_property(it.next().unwrap(), "#address-cells", 2, &vec![]);
    assert!(it.next().is_none());
}

#[test]
fn test_reads_nested_nodes() {
    let data = dts_to_dtb(
        r#"
/dts-v1/;
/ {
    node1 {
        #address-cells = "ok";
        node2 {
        };
    };
    node3 {
    };
};"#,
    );
    let slice_reader = &SliceReader::new(&data);
    let reader = FdtReader::new(slice_reader).unwrap();
    let mut it = reader.walk();

    assert_node(it.next().unwrap(), "", 1);
    assert_node(it.next().unwrap(), "node1", 2);
    assert_property(it.next().unwrap(), "#address-cells", 3, &vec![111, 107, 0]);
    assert_node(it.next().unwrap(), "node2", 3);
    assert_node(it.next().unwrap(), "node3", 2);
    assert!(it.next().is_none());
}

#[test]
fn test_returns_error_when_magic_is_invalid() {
    let mut data = dts_to_dtb(
        r#"
/dts-v1/;
/ {
    #address-cells = "ok";
};"#,
    );
    // Magic is a first number
    data[0] = 1;
    let slice_reader = &SliceReader::new(&data);
    assert_eq!(FdtReader::new(slice_reader).err(), Some("invalid magic in device tree header"));
}

#[test]
fn test_returns_error_when_header_is_too_short() {
    let data = dts_to_dtb(
        r#"
/dts-v1/;
/ {
    #address-cells = "ok";
};"#,
    );
    let slice_reader = &SliceReader::new(&data[0..4]);
    assert_eq!(FdtReader::new(slice_reader).err(), Some("not enough data to read device tree header"));
}
