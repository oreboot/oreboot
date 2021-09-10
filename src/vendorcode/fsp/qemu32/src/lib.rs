#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

use uefi::EFI_GUID;

include!(concat!(env!("OUT_DIR"), "/", "bindings.rs"));

// Don't mangle as this is referenced from a linker script to place in a specific location in
// flash.
macro_rules! blob_macro {
    () => {
        include_bytes!(concat!(env!("OUT_DIR"), "/", "QEMUFSP.fd"))
    };
}
#[no_mangle]
#[used]
#[link_section = ".fspblob"]
static FSP_BLOB: [u8; blob_macro!().len()] = *blob_macro!();
