/* SPDX-License-Identifier: GPL-2.0-only */
#![allow(non_snake_case)]

use pci::config32;
use pci::PciAddress;
use vcell::VolatileCell;

// See coreboot:src/soc/amd/common/block/smu/smu.c

const SMU_MESSAGE_ID_SMN: u32 = 0x3B1_0528;
const SMU_MESSAGE_RESPONSE_SMN: u32 = 0x3B1_0564;
const SMU_MESSAGE_ARGUMENT_0_SMN: u32 = 0x3B1_0998;
const SMU_MESSAGE_ARGUMENT_1_SMN: u32 = 0x3B1_099C;
const SMU_MESSAGE_ARGUMENT_2_SMN: u32 = 0x3B1_09A0;
const SMU_MESSAGE_ARGUMENT_3_SMN: u32 = 0x3B1_09A4;
const SMU_MESSAGE_ARGUMENT_4_SMN: u32 = 0x3B1_09A8;
const SMU_MESSAGE_ARGUMENT_5_SMN: u32 = 0x3B1_09AC;

pub struct SmuServiceRequest {
    command: u32,
    data: [u32; 6],
}

#[derive(Debug)]
pub struct SmuServiceResponse {
    status: u32,
    data: [u32; 6],
}

pub struct SMU<'a> {
    NB_SMN_INDEX_2: &'a VolatileCell<u32>,
    NB_SMN_DATA_2: &'a VolatileCell<u32>,
}

impl SMU<'_> {
    pub fn new() -> Self {
        SMU { NB_SMN_INDEX_2: config32(PciAddress { segment: 0, bus: 0, device: 0, function: 0, offset: 0xb8 }), NB_SMN_DATA_2: config32(PciAddress { segment: 0, bus: 0, device: 0, function: 0, offset: 0xbc }) }
    }

    fn smu_register_read(&self, a: u32) -> u32 {
        self.NB_SMN_INDEX_2.set(a);
        self.NB_SMN_DATA_2.get()
    }

    fn smu_register_write(&self, a: u32, v: u32) {
        self.NB_SMN_INDEX_2.set(a);
        self.NB_SMN_DATA_2.set(v)
    }

    fn service_call(&self, request: SmuServiceRequest) -> Result<SmuServiceResponse, u32> {
        //let mut response: u32 = self.smu_register_read(SMU_MESSAGE_RESPONSE_SMN);
        self.smu_register_write(SMU_MESSAGE_RESPONSE_SMN, 0);
        self.smu_register_write(SMU_MESSAGE_ARGUMENT_0_SMN, request.data[0]);
        self.smu_register_write(SMU_MESSAGE_ARGUMENT_1_SMN, request.data[1]);
        self.smu_register_write(SMU_MESSAGE_ARGUMENT_2_SMN, request.data[2]);
        self.smu_register_write(SMU_MESSAGE_ARGUMENT_3_SMN, request.data[3]);
        self.smu_register_write(SMU_MESSAGE_ARGUMENT_4_SMN, request.data[4]);
        self.smu_register_write(SMU_MESSAGE_ARGUMENT_5_SMN, request.data[5]);
        self.smu_register_write(SMU_MESSAGE_ID_SMN, request.command);
        let mut response: u32 = self.smu_register_read(SMU_MESSAGE_RESPONSE_SMN);
        while response == 0 {
            response = self.smu_register_read(SMU_MESSAGE_RESPONSE_SMN);
        }

        if response == 1 {
            // OK
            Ok(SmuServiceResponse {
                status: response,
                data: [
                    self.smu_register_read(SMU_MESSAGE_ARGUMENT_0_SMN),
                    self.smu_register_read(SMU_MESSAGE_ARGUMENT_1_SMN),
                    self.smu_register_read(SMU_MESSAGE_ARGUMENT_2_SMN),
                    self.smu_register_read(SMU_MESSAGE_ARGUMENT_3_SMN),
                    self.smu_register_read(SMU_MESSAGE_ARGUMENT_4_SMN),
                    self.smu_register_read(SMU_MESSAGE_ARGUMENT_5_SMN),
                ],
            })
        } else {
            Err(response)
        }
    }

    pub fn test(&self, v: u32) -> Result<u32, u32> {
        let result = self.service_call(SmuServiceRequest { command: 1, data: [v, 0, 0, 0, 0, 0] })?;
        Ok(result.data[0])
    }

    pub fn smu_version(&self) -> Result<(u32, u32), u32> {
        let result = self.service_call(SmuServiceRequest { command: 2, data: [0; 6] })?;
        Ok((result.data[0], result.data[1]))
    }

    pub fn interface_version(&self) -> Result<(u32, u32), u32> {
        let result = self.service_call(SmuServiceRequest { command: 3, data: [0; 6] })?;
        Ok((result.data[0], result.data[1]))
    }
}

impl Default for SMU<'_> {
    fn default() -> Self {
        Self::new()
    }
}
