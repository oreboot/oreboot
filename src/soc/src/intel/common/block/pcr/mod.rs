use crate::intel::Error;
#[cfg(target_arch = "x86_64")]
use arch::x86_64::mmio::{read32, write32};
use core::mem::size_of;
use log::error;
use payload::drivers::pci_map_bus_ops::{
    pci_read_config16, pci_read_config32, pci_write_config16, pci_write_config32,
};
use types::{pci_def::PCI_VENDOR_ID, pci_type::PciDevFnT};
use util::{
    helpers::{align_down, is_aligned},
    timer::Stopwatch,
};

mod gpmr;
pub use gpmr::*;

// FIXME: needs proper Kconfig
pub const CONFIG_PCR_BASE_ADDRESS: usize = 0xd0000000;
pub const PCR_PORTID_SHIFT: usize = 16;
// 10 ms
pub const PCR_SBI_CMD_TIMEOUT: u32 = 10;
pub const P2SB_CR_SBI_STATUS: u32 = 0xd8;
pub const P2SB_CR_SBI_STATUS_BUSY: u32 = 1;
pub const P2SB_CR_SBI_STATUS_NOT_SUPPORTED: u32 = 1;
pub const P2SB_CR_SBI_ADDR: u32 = 0xd0;
pub const P2SB_CR_SBI_DESTID: u32 = 24;
pub const P2SB_CR_SBI_EXT_ADDR: u32 = 0xdc;
pub const P2SB_CR_SBI_MASK: u32 = 0x7;
pub const P2SB_CR_SBI_FBE_MASK: u32 = 0xf;
pub const P2SB_CR_SBI_FID_MASK: u32 = 0xff;
pub const P2SB_CR_SBI_OPCODE_MASK: u32 = 0xff00;
pub const P2SB_CR_SBI_POSTED_MASK: u32 = 0x0080;
pub const P2SB_CR_SBI_STATUS_MASK: u32 = 0x0006;
pub const P2SB_CR_SBI_STATUS_SUCCESS: u32 = 0;
pub const P2SB_CR_SBI_BAR: u32 = 8;
pub const P2SB_CR_SBI_DATA: u32 = 0xd4;
pub const P2SB_CR_SBI_FBE: u32 = 12;
pub const P2SB_CR_SBI_OPCODE: u32 = 8;
pub const P2SB_CR_SBI_POSTED: u32 = 7;
pub const P2SB_CR_SBI_ROUTE_IDEN: u32 = 0xda;

#[repr(C)]
#[derive(Debug)]
pub enum PcrSbiOpcode {
    MemRead = 0x00,
    MemWrite = 0x01,
    PciConfigRead = 0x04,
    PciConfigWrite = 0x05,
    PcrRead = 0x06,
    PcrWrite = 0x07,
    GpioLockUnlock = 0x13,
}

pub struct PcrSbiMsg {
    /// 0x00 - Port ID of the SBI message
    pub pid: u8,
    /// 0x01 - Register offset of the SBI message
    pub offset: u32,
    /// 0x05 - Opcode
    pub opcode: PcrSbiOpcode,
    /// 0x06 - Posted message
    pub is_posted: bool,
    /// 0x07 - First Byte Enable
    pub fast_byte_enable: u16,
    /// 0x09 - Base address
    pub bar: u16,
    /// 0x0B - Function ID
    pub fid: u16,
}

pub fn __pcr_reg_address(pid: u8, offset: u16) -> usize {
    let mut reg_addr = CONFIG_PCR_BASE_ADDRESS;
    reg_addr += (pid as usize) << PCR_PORTID_SHIFT;
    reg_addr += offset as usize;
    reg_addr
}

pub fn pcr_read32(pid: u8, offset: u16) -> u32 {
    assert!(is_aligned(offset as u32, size_of::<u32>() as u32));

    unsafe { read32(__pcr_reg_address(pid, offset) as usize) }
}

pub fn pcr_write32(pid: u8, offset: u16, indata: u32) {
    // Ensure the PCR offst is correctly aligned
    assert!(is_aligned(offset as u32, size_of::<u32>() as u32));

    unsafe { write32(__pcr_reg_address(pid, offset), indata) };
    // Ensure the writes complete
    unsafe { write_completion(pid, offset) };
}

pub fn pcr_wait_for_completion(dev: PciDevFnT) -> Result<(), Error> {
    let mut sw = Stopwatch::new();
    sw.init_msecs_expire(PCR_SBI_CMD_TIMEOUT as u64);

    while !sw.expired() {
        if pci_read_config16(dev, P2SB_CR_SBI_STATUS as u16) & P2SB_CR_SBI_STATUS_BUSY as u16 == 0 {
            return Ok(());
        }
    }

    Err(Error::PcrTimeout)
}

pub unsafe fn write_completion(pid: u8, offset: u16) {
    read32(__pcr_reg_address(
        pid,
        align_down(offset as u32, size_of::<u32>() as u32) as u16,
    ) as usize);
}

pub fn pcr_rmw32(pid: u8, offset: u16, anddata: u32, ordata: u32) {
    let mut data32 = pcr_read32(pid, offset);
    data32 &= anddata;
    data32 |= ordata;
    pcr_write32(pid, offset, data32);
}

pub fn pcr_or32(pid: u8, offset: u16, ordata: u32) {
    let mut data32 = pcr_read32(pid, offset);
    data32 |= ordata;
    pcr_write32(pid, offset, data32);
}

/// API to perform sideband communication
///
/// Input:
/// struct pcr_sbi_msg
/// data - read/write for sbi message
/// response -
/// 0 - successful
/// 1 - unsuccessful
/// 2 - powered down
/// 3 - multi-cast mixed
///
/// Output:
/// 0: SBI message is successfully completed
/// -1: SBI message failure
pub fn pcr_execute_sideband_msg(
    dev: PciDevFnT,
    msg: &mut PcrSbiMsg,
    data: &mut u32,
    response: &mut u8,
) -> Result<(), Error> {
    match msg.opcode {
        PcrSbiOpcode::MemWrite
        | PcrSbiOpcode::PciConfigRead
        | PcrSbiOpcode::PciConfigWrite
        | PcrSbiOpcode::PcrRead
        | PcrSbiOpcode::PcrWrite
        | PcrSbiOpcode::GpioLockUnlock => (),
        _ => {
            error!("SBI Failure: Wrong Input = {}!", msg.opcode as u16);
            return Err(Error::SbiFailure);
        }
    }

    if pci_read_config16(dev, PCI_VENDOR_ID) == 0xffff {
        error!("SBI Failure: P2SB device Hidden!");
        return Err(Error::SbiFailure);
    }

    /*
     * BWG Section 2.2.1
     * 1. Poll P2SB PCI offset D8h[0] = 0b
     * Make sure the previous operation is completed.
     */
    if pcr_wait_for_completion(dev).is_err() {
        error!("SBI Failure: Time Out!");
        return Err(Error::SbiFailure);
    }

    /* Initial Response status */
    *response = P2SB_CR_SBI_STATUS_NOT_SUPPORTED as u8;

    /*
     * 2. Write P2SB PCI offset D0h[31:0] with Address
     * and Destination Port ID
     */
    pci_write_config32(
        dev,
        P2SB_CR_SBI_ADDR as u16,
        ((msg.pid as u32) << P2SB_CR_SBI_DESTID) as u32 | msg.offset,
    );

    /*
     * 3. Write P2SB PCI offset DCh[31:0] with extended address,
     * which is expected to be 0
     */
    pci_write_config32(dev, P2SB_CR_SBI_EXT_ADDR as u16, msg.offset >> 16);

    /*
     * 4. Set P2SB PCI offset D8h[15:8] = 00000110b for read
     *    Set P2SB PCI offset D8h[15:8] = 00000111b for write
     *
     * Set SBISTAT[15:8] to the opcode passed in
     * Set SBISTAT[7] to the posted passed in
     */
    let mut sbi_status = pci_read_config16(dev, P2SB_CR_SBI_STATUS as u16);
    sbi_status &= !(P2SB_CR_SBI_OPCODE_MASK | P2SB_CR_SBI_POSTED_MASK) as u16;
    sbi_status |= ((msg.opcode as u16) << P2SB_CR_SBI_OPCODE)
        | ((msg.is_posted as u16) << P2SB_CR_SBI_POSTED);
    pci_write_config16(dev, P2SB_CR_SBI_STATUS as u16, sbi_status);

    /*
     * 5. Write P2SB PCI offset DAh[15:0] = F000h
     *
     * Set RID[15:0] = Fbe << 12 | Bar << 8 | Fid
     */
    let sbi_rid = (((msg.fast_byte_enable as u16) & P2SB_CR_SBI_FBE_MASK as u16)
        << P2SB_CR_SBI_FBE)
        | (((msg.bar as u16) & P2SB_CR_SBI_MASK as u16) << P2SB_CR_SBI_BAR)
        | (msg.fid as u16 & P2SB_CR_SBI_FID_MASK as u16);
    pci_write_config16(dev, P2SB_CR_SBI_ROUTE_IDEN as u16, sbi_rid);

    match msg.opcode {
        PcrSbiOpcode::MemWrite
        | PcrSbiOpcode::PciConfigWrite
        | PcrSbiOpcode::PcrWrite
        | PcrSbiOpcode::GpioLockUnlock => {
            /*
             * 6. Write P2SB PCI offset D4h[31:0] with the
             * intended data accordingly
             */
            let sbi_data = *data;
            pci_write_config32(dev, P2SB_CR_SBI_DATA as u16, sbi_data);
        }
        /* 6. Write P2SB PCI offset D4h[31:0] with dummy data */
        _ => pci_write_config32(dev, P2SB_CR_SBI_DATA as u16, 0),
    };

    /*
     * 7. Set P2SB PCI offset D8h[0] = 1b, Poll P2SB PCI offset D8h[0] = 0b
     *
     * Set SBISTAT[0] = 1b, trigger the SBI operation
     */
    let mut sbi_status = pci_read_config16(dev, P2SB_CR_SBI_STATUS as u16);
    sbi_status |= P2SB_CR_SBI_STATUS_BUSY as u16;
    pci_write_config16(dev, P2SB_CR_SBI_STATUS as u16, sbi_status);

    /* Poll SBISTAT[0] = 0b, Polling for Busy bit */
    if pcr_wait_for_completion(dev).is_err() {
        error!("SBI Failure: Time Out!");
        return Err(Error::SbiFailure);
    }

    /*
     * 8. Check if P2SB PCI offset D8h[2:1] = 00b for
     * successful transaction
     */
    *response = ((sbi_status & P2SB_CR_SBI_STATUS_MASK as u16) >> 1) as u8;
    if *response as u32 == P2SB_CR_SBI_STATUS_SUCCESS {
        match msg.opcode {
            PcrSbiOpcode::MemRead | PcrSbiOpcode::PciConfigRead | PcrSbiOpcode::PcrRead => {
                let sbi_data = pci_read_config32(dev, P2SB_CR_SBI_DATA as u16);
                *data = sbi_data;
            }
            _ => (),
        };
        return Ok(());
    }

    error!("SBI Failure: Transaction Status = {:x}", *response);
    Err(Error::SbiFailure)
}
