#![allow(non_snake_case)]

use crate::pci::config32;
use crate::pci::PciAddress;
use vcell::VolatileCell;

const HSMP_MESSAGE_ID_SMN: u32 = 0x3B1_0534;
const HSMP_MESSAGE_RESPONSE_SMN: u32 = 0x3B1_0980;
const HSMP_MESSAGE_ARGUMENT_0_SMN: u32 = 0x3B1_09E0;
const HSMP_MESSAGE_ARGUMENT_1_SMN: u32 = 0x3B1_09E4;
const HSMP_MESSAGE_ARGUMENT_2_SMN: u32 = 0x3B1_09E8;
const HSMP_MESSAGE_ARGUMENT_3_SMN: u32 = 0x3B1_09EC;
const HSMP_MESSAGE_ARGUMENT_4_SMN: u32 = 0x3B1_09F0;
const HSMP_MESSAGE_ARGUMENT_5_SMN: u32 = 0x3B1_09F4;
const HSMP_MESSAGE_ARGUMENT_6_SMN: u32 = 0x3B1_09F8;
const HSMP_MESSAGE_ARGUMENT_7_SMN: u32 = 0x3B1_09FC;

pub struct SmuServiceRequest {
    command: u32,
    data: [u32; 8],
}

#[derive(Debug)]
pub struct SmuServiceResponse {
    status: u32,
    data: [u32; 8],
}

pub struct HSMP<'a> {
    NB_SMN_INDEX_3: &'a VolatileCell<u32>,
    NB_SMN_DATA_3: &'a VolatileCell<u32>,
}

impl HSMP<'_> {
    pub fn new(nbio: usize) -> Self {
        // TODO: Verify whether it should be 4 PER SOCKET.
        let bus = match nbio {
            0 => 0,
            1 => 0x20,
            2 => 0x40,
            3 => 0x60,
            _ => panic!("invalid NBIO"),
        };
        HSMP { NB_SMN_INDEX_3: config32(PciAddress { segment: 0, bus, device: 0, function: 0, offset: 0xc4 }), NB_SMN_DATA_3: config32(PciAddress { segment: 0, bus, device: 0, function: 0, offset: 0xc8 }) }
    }

    fn smu_register_read(&self, a: u32) -> u32 {
        self.NB_SMN_INDEX_3.set(a);
        self.NB_SMN_DATA_3.get()
    }

    fn smu_register_write(&self, a: u32, v: u32) {
        self.NB_SMN_INDEX_3.set(a);
        self.NB_SMN_DATA_3.set(v)
    }

    fn service_call(&self, request: SmuServiceRequest) -> Result<SmuServiceResponse, u32> {
        //let mut response: u32 = self.smu_register_read(HSMP_MESSAGE_RESPONSE_SMN);
        self.smu_register_write(HSMP_MESSAGE_RESPONSE_SMN, 0);
        self.smu_register_write(HSMP_MESSAGE_ARGUMENT_0_SMN, request.data[0]);
        self.smu_register_write(HSMP_MESSAGE_ARGUMENT_1_SMN, request.data[1]);
        self.smu_register_write(HSMP_MESSAGE_ARGUMENT_2_SMN, request.data[2]);
        self.smu_register_write(HSMP_MESSAGE_ARGUMENT_3_SMN, request.data[3]);
        self.smu_register_write(HSMP_MESSAGE_ARGUMENT_4_SMN, request.data[4]);
        self.smu_register_write(HSMP_MESSAGE_ARGUMENT_5_SMN, request.data[5]);
        self.smu_register_write(HSMP_MESSAGE_ARGUMENT_6_SMN, request.data[6]);
        self.smu_register_write(HSMP_MESSAGE_ARGUMENT_7_SMN, request.data[7]);
        self.smu_register_write(HSMP_MESSAGE_ID_SMN, request.command);
        let mut response: u32 = self.smu_register_read(HSMP_MESSAGE_RESPONSE_SMN);
        while response == 0 {
            response = self.smu_register_read(HSMP_MESSAGE_RESPONSE_SMN);
        }

        if response == 1 {
            // OK
            Ok(SmuServiceResponse {
                status: response,
                data: [
                    self.smu_register_read(HSMP_MESSAGE_ARGUMENT_0_SMN),
                    self.smu_register_read(HSMP_MESSAGE_ARGUMENT_1_SMN),
                    self.smu_register_read(HSMP_MESSAGE_ARGUMENT_2_SMN),
                    self.smu_register_read(HSMP_MESSAGE_ARGUMENT_3_SMN),
                    self.smu_register_read(HSMP_MESSAGE_ARGUMENT_4_SMN),
                    self.smu_register_read(HSMP_MESSAGE_ARGUMENT_5_SMN),
                    self.smu_register_read(HSMP_MESSAGE_ARGUMENT_6_SMN),
                    self.smu_register_read(HSMP_MESSAGE_ARGUMENT_7_SMN),
                ],
            })
        } else {
            Err(response)
        }
    }

    pub fn test(&self, v: u32) -> Result<u32, u32> {
        let result = self.service_call(SmuServiceRequest { command: 1, data: [v, 0, 0, 0, 0, 0, 0, 0] })?;
        Ok(result.data[0])
    }

    pub fn smu_version(&self) -> Result<(u32, u32), u32> {
        let result = self.service_call(SmuServiceRequest { command: 2, data: [0; 8] })?;
        Ok((result.data[0], result.data[1]))
    }

    pub fn interface_version(&self) -> Result<(u32, u32), u32> {
        let result = self.service_call(SmuServiceRequest { command: 3, data: [0; 8] })?;
        Ok((result.data[0], result.data[1]))
    }
}
