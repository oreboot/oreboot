#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Types from the ffi crate are used instead of the generated ones.
#[allow(unused_imports)]
use efi::ffi::{BOOLEAN, CHAR16, CHAR8, EFI_GUID, EFI_GUID as GUID, INT16, INT32, INT64, INT8, UINT16, UINT32, UINT64, UINT8, UINTN};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
