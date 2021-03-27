/* SPDX-License-Identifier: GPL-2.0-only */
#![allow(non_snake_case)]

use smn::{smn_read, smn_write};

// See coreboot:src/soc/amd/common/block/smu/smu.c

const MESSAGE_ID_SMN: u32 = 0x3B1_0528;
const MESSAGE_RESPONSE_SMN: u32 = 0x3B1_0564;
const MESSAGE_ARGUMENT_0_SMN: u32 = 0x3B1_0998;
const MESSAGE_ARGUMENT_1_SMN: u32 = 0x3B1_099C;
const MESSAGE_ARGUMENT_2_SMN: u32 = 0x3B1_09A0;
const MESSAGE_ARGUMENT_3_SMN: u32 = 0x3B1_09A4;
const MESSAGE_ARGUMENT_4_SMN: u32 = 0x3B1_09A8;
const MESSAGE_ARGUMENT_5_SMN: u32 = 0x3B1_09AC;

struct ServiceRequest {
    command: u32,
    data: [u32; 6],
}

#[derive(Debug)]
struct ServiceResponse {
    status: u32,
    data: [u32; 6],
}

pub struct MP1;

impl MP1 {
    pub fn new() -> Self {
        Self
    }

    fn service_call(&self, request: ServiceRequest) -> Result<ServiceResponse, u32> {
        //let mut response: u32 = smn_read(MESSAGE_RESPONSE_SMN);
        smn_write(MESSAGE_RESPONSE_SMN, 0);
        smn_write(MESSAGE_ARGUMENT_0_SMN, request.data[0]);
        smn_write(MESSAGE_ARGUMENT_1_SMN, request.data[1]);
        smn_write(MESSAGE_ARGUMENT_2_SMN, request.data[2]);
        smn_write(MESSAGE_ARGUMENT_3_SMN, request.data[3]);
        smn_write(MESSAGE_ARGUMENT_4_SMN, request.data[4]);
        smn_write(MESSAGE_ARGUMENT_5_SMN, request.data[5]);
        smn_write(MESSAGE_ID_SMN, request.command);
        let mut response: u32 = smn_read(MESSAGE_RESPONSE_SMN);
        while response == 0 {
            response = smn_read(MESSAGE_RESPONSE_SMN);
        }

        if response == 1 {
            // OK
            Ok(ServiceResponse {
                status: response,
                data: [smn_read(MESSAGE_ARGUMENT_0_SMN), smn_read(MESSAGE_ARGUMENT_1_SMN), smn_read(MESSAGE_ARGUMENT_2_SMN), smn_read(MESSAGE_ARGUMENT_3_SMN), smn_read(MESSAGE_ARGUMENT_4_SMN), smn_read(MESSAGE_ARGUMENT_5_SMN)],
            })
        } else {
            Err(response)
        }
    }

    pub fn test(&self, v: u32) -> Result<u32, u32> {
        let result = self.service_call(ServiceRequest { command: 1, data: [v, 0, 0, 0, 0, 0] })?;
        Ok(result.data[0])
    }

    pub fn smu_version(&self) -> Result<(u32, u32), u32> {
        let result = self.service_call(ServiceRequest { command: 2, data: [0; 6] })?;
        Ok((result.data[0], result.data[1]))
    }
}

impl Default for MP1 {
    fn default() -> Self {
        Self::new()
    }
}
