/* SPDX-License-Identifier: GPL-2.0-only */
#![allow(non_snake_case)]

use smn::{smn_read, smn_write};

// See coreboot:src/soc/amd/common/block/smu/smu.c

pub struct MPMailbox<const message_argument_count: usize> {
    message_id_smn_address: u32,
    message_response_smn_address: u32,
    message_first_arguments_smn_address: u32,
}

impl<const message_argument_count: usize> MPMailbox<message_argument_count> {
    pub fn new(message_id_smn_address: u32, message_response_smn_address: u32, message_first_arguments_smn_address: u32) -> Self {
        Self {
            message_id_smn_address,
            message_response_smn_address,
            message_first_arguments_smn_address,
        }
    }

    fn call(&self, command: u32, arguments: &mut [u32; message_argument_count]) -> Result<(), u32> {
        //let mut response: u32 = smn_read(MESSAGE_RESPONSE_SMN);
        smn_write(self.message_response_smn_address, 0);
        for i in 0..message_argument_count {
            smn_write(self.message_first_arguments_smn_address + (i as u32) * 4, arguments[i]);
        }
        smn_write(self.message_id_smn_address, command);
        let mut response: u32 = smn_read(self.message_response_smn_address);
        while response == 0 {
            response = smn_read(self.message_response_smn_address);
        }

        if response == 1 {
            // OK
            for i in 0..message_argument_count {
                arguments[i] = smn_read(self.message_first_arguments_smn_address + (i as u32) * 4);
            }
            Ok(())
        } else {
            Err(response)
        }
    }

    pub fn call1(&self, command: u32, v: u32) -> Result<u32, u32> {
        let mut arguments: [u32; message_argument_count] = [0; message_argument_count];
        arguments[0] = v;
        let result = self.call(command, &mut arguments)?;
        Ok(arguments[0])
    }

    pub fn test(&self, v: u32) -> Result<u32, u32> {
        self.call1(1, v)
    }

    pub fn smu_version(&self) -> Result<u32, u32> {
        self.call1(2, 0)
    }
}
