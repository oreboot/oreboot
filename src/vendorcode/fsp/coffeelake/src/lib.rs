#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

// Rust types are used instead of generated ones.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EFI_GUID(pub UINT32, pub UINT16, pub UINT16, pub [UINT8; 8]);
pub type GUID = EFI_GUID;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
