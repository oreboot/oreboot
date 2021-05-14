/* SPDX-License-Identifier: GPL-2.0-only */
#![allow(non_snake_case)]

use smn::{smn_read, smn_write};

#[derive(Debug, Clone)]
pub struct MPMailboxCallError(u32);

pub type Result<T> = core::result::Result<T, MPMailboxCallError>;

// See coreboot:src/soc/amd/common/block/smu/smu.c

pub struct MPMailbox<const MESSAGE_ARGUMENT_COUNT: usize> {
    message_id_smn_address: u32,
    message_response_smn_address: u32,
    message_first_argument_smn_address: u32,
}

impl<const MESSAGE_ARGUMENT_COUNT: usize> MPMailbox<MESSAGE_ARGUMENT_COUNT> {
    pub fn new(message_id_smn_address: u32, message_response_smn_address: u32, message_first_argument_smn_address: u32) -> Self {
        Self { message_id_smn_address, message_response_smn_address, message_first_argument_smn_address }
    }

    fn call(&self, command: u32, arguments: &mut [u32; MESSAGE_ARGUMENT_COUNT]) -> Result<()> {
        //let mut response: u32 = smn_read(MESSAGE_RESPONSE_SMN);
        smn_write(self.message_response_smn_address, 0);
        for (i, argument) in arguments.iter().enumerate().take(MESSAGE_ARGUMENT_COUNT) {
            smn_write(self.message_first_argument_smn_address + (i as u32) * 4, *argument);
        }
        smn_write(self.message_id_smn_address, command);
        let mut response: u32 = smn_read(self.message_response_smn_address);
        while response == 0 {
            response = smn_read(self.message_response_smn_address);
        }

        if response == 1 {
            // OK
            for (i, argument) in arguments.iter_mut().enumerate().take(MESSAGE_ARGUMENT_COUNT) {
                *argument = smn_read(self.message_first_argument_smn_address + (i as u32) * 4);
            }
            Ok(())
        } else {
            Err(MPMailboxCallError(response))
        }
    }

    pub fn call1(&self, command: u32, v: u32) -> Result<u32> {
        let mut arguments: [u32; MESSAGE_ARGUMENT_COUNT] = [0; MESSAGE_ARGUMENT_COUNT];
        arguments[0] = v;
        self.call(command, &mut arguments)?;
        Ok(arguments[0])
    }

    pub fn test(&self, v: u32) -> Result<u32> {
        self.call1(1, v)
    }

    pub fn smu_version(&self) -> Result<u32> {
        self.call1(2, 0)
    }
}
