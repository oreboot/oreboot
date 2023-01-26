/* SPDX-License-Identifier: GPL-2.0-only */
#![deny(warnings)]
#![no_std]

use cbmem::{cbmem_find, CBMEM_ID_ROMSTAGE_INFO};
use spin::rwlock::RwLock;

#[repr(C)]
pub struct RomstageHandoff {
    /// Indicate if the current boot is an S3 resume. If
    /// CONFIG_RELOCATABLE_RAMSTAGE is enabled the chipset code is
    /// responsible for initializing this variable. Otherwise, ramstage
    /// will be re-loaded from cbfs (which can be slower since it lives
    /// in flash).
    pub s3_resume: u8,
    pub reboot_required: u8,
    pub reserved: [u8; 2],
}

impl RomstageHandoff {
    pub const fn new() -> Self {
        Self {
            s3_resume: 0,
            reboot_required: 0,
            reserved: [0u8; 2],
        }
    }
}

pub fn is_resume() -> bool {
    static ONCE: RwLock<u8> = RwLock::new(0);
    static S3_RESUME: RwLock<u8> = RwLock::new(0);

    if *ONCE.read() != 0 {
        return *S3_RESUME.read() != 0;
    }

    (*ONCE.write()) = 1;
    if let Some(handoff) = cbmem_find::<RomstageHandoff>(CBMEM_ID_ROMSTAGE_INFO) {
        (*S3_RESUME.write()) = handoff.s3_resume;
        if *S3_RESUME.read() != 0 {
            //debug!("S3 Resume");
        } else {
            //debug!("Normal boot");
        }
        *S3_RESUME.read() != 0
    } else {
        false
    }
}
