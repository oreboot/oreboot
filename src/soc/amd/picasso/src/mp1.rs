/* SPDX-License-Identifier: GPL-2.0-only */
#![allow(non_snake_case)]

use pci::config32;
use pci::PciAddress;
use vcell::VolatileCell;

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

pub struct MP1<'a> {
    NB_SMN_INDEX_2: &'a VolatileCell<u32>,
    NB_SMN_DATA_2: &'a VolatileCell<u32>,
}

impl MP1<'_> {
    pub fn new() -> Self {
        Self { NB_SMN_INDEX_2: config32(PciAddress { segment: 0, bus: 0, device: 0, function: 0, offset: 0xb8 }), NB_SMN_DATA_2: config32(PciAddress { segment: 0, bus: 0, device: 0, function: 0, offset: 0xbc }) }
    }

    fn register_read_smn(&self, a: u32) -> u32 {
        self.NB_SMN_INDEX_2.set(a);
        self.NB_SMN_DATA_2.get()
    }

    fn register_write_smn(&self, a: u32, v: u32) {
        self.NB_SMN_INDEX_2.set(a);
        self.NB_SMN_DATA_2.set(v)
    }

    fn service_call(&self, request: ServiceRequest) -> Result<ServiceResponse, u32> {
        //let mut response: u32 = self.register_read_smn(MESSAGE_RESPONSE_SMN);
        self.register_write_smn(MESSAGE_RESPONSE_SMN, 0);
        self.register_write_smn(MESSAGE_ARGUMENT_0_SMN, request.data[0]);
        self.register_write_smn(MESSAGE_ARGUMENT_1_SMN, request.data[1]);
        self.register_write_smn(MESSAGE_ARGUMENT_2_SMN, request.data[2]);
        self.register_write_smn(MESSAGE_ARGUMENT_3_SMN, request.data[3]);
        self.register_write_smn(MESSAGE_ARGUMENT_4_SMN, request.data[4]);
        self.register_write_smn(MESSAGE_ARGUMENT_5_SMN, request.data[5]);
        self.register_write_smn(MESSAGE_ID_SMN, request.command);
        let mut response: u32 = self.register_read_smn(MESSAGE_RESPONSE_SMN);
        while response == 0 {
            response = self.register_read_smn(MESSAGE_RESPONSE_SMN);
        }

        if response == 1 {
            // OK
            Ok(ServiceResponse {
                status: response,
                data: [
                    self.register_read_smn(MESSAGE_ARGUMENT_0_SMN),
                    self.register_read_smn(MESSAGE_ARGUMENT_1_SMN),
                    self.register_read_smn(MESSAGE_ARGUMENT_2_SMN),
                    self.register_read_smn(MESSAGE_ARGUMENT_3_SMN),
                    self.register_read_smn(MESSAGE_ARGUMENT_4_SMN),
                    self.register_read_smn(MESSAGE_ARGUMENT_5_SMN),
                ],
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

    pub fn interface_version(&self) -> Result<(u32, u32), u32> {
        let result = self.service_call(ServiceRequest { command: 3, data: [0; 6] })?;
        Ok((result.data[0], result.data[1]))
    }
}

impl Default for MP1<'_> {
    fn default() -> Self {
        Self::new()
    }
}
