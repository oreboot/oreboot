/* SPDX-License-Identifier: GPL-2.0-only */
#![allow(non_snake_case)]

use smn::{smn_read, smn_write};

// See coreboot:src/soc/amd/common/block/smu/smu.c

struct ServiceRequest {
    command: u32,
    data: [u32; 6],
}

#[derive(Debug)]
struct ServiceResponse {
    status: u32,
    data: [u32; 6],
}

pub struct MPMailbox {
    message_id_smn_address: u32,
    message_response_smn_address: u32,
    message_first_arguments_smn_address: u32,
    message_argument_count: usize,
}

impl MPMailbox {
    pub fn new(message_id_smn_address: u32, message_response_smn_address: u32, message_first_arguments_smn_address: u32, message_argument_count: usize) -> Self {
        Self {
            message_id_smn_address,
            message_response_smn_address,
            message_first_arguments_smn_address,
            message_argument_count,
        }
    }

    fn service_call(&self, request: ServiceRequest) -> Result<ServiceResponse, u32> {
        //let mut response: u32 = smn_read(MESSAGE_RESPONSE_SMN);
        smn_write(self.message_response_smn_address, 0);
        for i in 0..self.message_argument_count {
            smn_write(self.message_first_arguments_smn_address + (i as u32) * 4, request.data[i]);
        }
        smn_write(self.message_id_smn_address, request.command);
        let mut response: u32 = smn_read(self.message_response_smn_address);
        while response == 0 {
            response = smn_read(self.message_response_smn_address);
        }

        if response == 1 {
            // OK
            Ok(ServiceResponse {
                status: response,
                data: [smn_read(self.message_first_arguments_smn_address), smn_read(self.message_first_arguments_smn_address + 4), smn_read(self.message_first_arguments_smn_address + 4 * 2), smn_read(self.message_first_arguments_smn_address + 4 * 3), smn_read(self.message_first_arguments_smn_address + 4 * 4), smn_read(self.message_first_arguments_smn_address + 4 * 5)],
            })
        } else {
            Err(response)
        }
    }

    pub fn test(&self, v: u32) -> Result<u32, u32> {
        let result = self.service_call(ServiceRequest { command: 1, data: [v, 0, 0, 0, 0, 0] })?;
        Ok(result.data[0])
    }

    pub fn smu_version(&self) -> Result<u32, u32> {
        let result = self.service_call(ServiceRequest { command: 2, data: [0; 6] })?;
        Ok(result.data[0])
    }
}
