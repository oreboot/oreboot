/* SPDX-License-Identifier: GPL-2.0-only */
#![deny(warnings)]
#![no_std]

#[cfg(feature = "intel")]
extern crate alloc;

#[cfg(feature = "intel")]
pub mod intel;
