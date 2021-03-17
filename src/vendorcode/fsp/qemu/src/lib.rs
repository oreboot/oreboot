#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

// Rust types are used instead of generated ones.
pub type BOOLEAN = u8;
pub type CHAR8 = u8;
pub type CHAR16 = u16;
pub type INT8 = i8;
pub type INT16 = i16;
pub type INT32 = i32;
pub type INT64 = i64;
pub type INTN = isize;
pub type UINT8 = u8;
pub type UINT16 = u16;
pub type UINT32 = u32;
pub type UINT64 = u64;
pub type UINTN = usize;
#[repr(C)]
pub struct EFI_GUID(pub UINT32, pub UINT16, pub UINT16, pub [UINT8; 8]);
pub type GUID = EFI_GUID;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Don't mangle as this is referenced from a linker script to place in a specific location in
// flash.
macro_rules! blob_macro {
    () => {
        include_bytes!(concat!(env!("OUT_DIR"), "/QEMUFSP.fd"))
    };
}
#[no_mangle]
#[used]
#[link_section = ".fspblob"]
static FSP_BLOB: [u8; blob_macro!().len()] = *blob_macro!();
