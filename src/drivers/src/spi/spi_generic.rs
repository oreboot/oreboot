use crate::context::Context;
use crate::spi::{spi_flash::SpiFlash, Error};

#[cfg(feature = "intel")]
use southbridge::intel::common::spi::SPI_CTRLR_BUS_MAP;
use spin::rwlock::RwLock;
use util::region::Region;

pub const VENDOR_ID_ADESTO: u8 = 0x1f;
pub const VENDOR_ID_AMIC: u8 = 0x37;
pub const VENDOR_ID_ATMEL: u8 = 0x1f;
pub const VENDOR_ID_EON: u8 = 0x1c;
pub const VENDOR_ID_GIGADEVICE: u8 = 0xc8;
pub const VENDOR_ID_MACRONIX: u8 = 0xc2;
pub const VENDOR_ID_SPANSION: u8 = 0x01;
pub const VENDOR_ID_SST: u8 = 0xbf;
pub const VENDOR_ID_STMICRO: u8 = 0x20;
pub const VENDOR_ID_WINBOND: u8 = 0xef;

pub const SPI_FLASH_PAGE_ERASE_TIMEOUT_MS: u64 = 500;
pub const SPI_FLASH_PROG_TIMEOUT_MS: u64 = 200;

#[cfg(not(feature = "intel"))]
pub static SPI_CTRLR_BUS_MAP: RwLock<[SpiCtrlrBuses; 0]> = RwLock::new([]);

#[repr(C)]
pub enum SpiCntrlrDeduct {
    /// Deduct the command length from the spi_crop_chunk() calculation for
    /// sizing a transaction. If SPI_CNTRLR_DEDUCT_OPCODE_LEN is set, only
    /// the bytes after the command byte will be deducted.
    CmdLen = 1 << 0,
    /// Remove the opcode size from the command length used in the
    /// spi_crop_chunk() calculation. Controllers which have a dedicated
    /// register for the command byte would set this flag which would
    /// allow the use of the maximum transfer size.
    OpcodeLen = 1 << 1,
}

/* Controller-specific definitions: */

#[repr(C)]
pub struct SpiCtrlrBuses<'a> {
    pub bus_start: u32,
    pub bus_end: u32,
    pub ctrlr: &'a dyn SpiCtrlr,
}

impl<'a> SpiCtrlrBuses<'a> {
    pub fn setup(&self, spi: &mut SpiSlave) -> Result<(), Error> {
        spi.setup(self.bus_start, 0)
    }
}

/// Representation of SPI operation status.
#[repr(C)]
#[derive(Debug)]
pub enum SpiOpStatus {
    NotExecuted = 0,
    Success = 1,
    Failure = 2,
}

#[repr(C)]
#[derive(Debug)]
pub enum CtrlrProtType {
    ReadProtect,
    WriteProtect,
    ReadWriteProtect,
}

/// Representation of a SPI operation.
///
/// dout:	Pointer to data to send.
/// din:	Pointer to store received data.
#[repr(C)]
pub struct SpiOp<'a, 'b> {
    pub dout: &'a [u8],
    pub din: &'b mut [u8],
    pub status: SpiOpStatus,
}

/// Representation of a SPI controller. Note the xfer() and xfer_vector()
/// callbacks are meant to process full duplex transactions. If the
/// controller cannot handle these transactions then return an error when
/// din and dout are both set. See spi_xfer() below for more details.
///
/// claim_bus:		Claim SPI bus and prepare for communication.
/// release_bus:	Release SPI bus.
/// setup:		Setup given SPI device bus.
/// xfer:		Perform one SPI transfer operation.
/// xfer_vector:	Vector of SPI transfer operations.
/// xfer_dual:		(optional) Perform one SPI transfer in Dual SPI mode.
/// max_xfer_size:	Maximum transfer size supported by the controller
///			(0 = invalid,
///			 SPI_CTRLR_DEFAULT_MAX_XFER_SIZE = unlimited)
/// flags:		See SPI_CNTRLR_* enums above.
///
/// Following member is provided by specialized SPI controllers that are
/// actually SPI flash controllers.
///
/// flash_probe:	Specialized probe function provided by SPI flash
///			controllers.
/// flash_protect: Protect a region of flash using the SPI flash controller.
pub trait SpiCtrlr: Sync {
    fn claim_bus(&self) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn release_bus(&self) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn xfer(&self, _dout: &[u8], _din: &mut [u8]) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn xfer_vector(&self, _vectors: &mut [SpiOp]) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn xfer_dual(&self, _dout: &[u8], _din: &mut [u8]) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn max_xfer_size(&self) -> u32;
    fn set_max_xfer_size(&mut self, size: u32);

    fn flags(&self) -> u32;
    fn set_flags(&mut self, _flags: u32);

    fn flash_probe(&self, _flash: &mut SpiFlash) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn flash_protect(
        &self,
        _flash: &SpiFlash,
        _region: &Region,
        _type: CtrlrProtType,
    ) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }
}

/// Representation of a SPI slave, i.e. what we're communicating with.
///
///   bus:	ID of the bus that the slave is attached to.
///   cs:	ID of the chip select connected to the slave.
///   ctrlr:	Pointer to SPI controller structure.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SpiSlave {
    pub bus: u32,
    pub cs: u32,
}

impl SpiCtrlr for SpiSlave {
    fn xfer_vector(&self, vectors: &mut [SpiOp]) -> Result<(), Error> {
        self.xfer_vector_default(vectors)
    }

    fn max_xfer_size(&self) -> u32 {
        0
    }

    fn set_max_xfer_size(&mut self, _size: u32) {}

    fn flags(&self) -> u32 {
        0
    }

    fn set_flags(&mut self, _flags: u32) {}
}

impl SpiSlave {
    pub const fn new() -> Self {
        Self { bus: 0, cs: 0 }
    }

    pub fn clear(&mut self) {
        self.bus = 0;
        self.cs = 0;
    }

    pub fn setup(&mut self, bus: u32, cs: u32) -> Result<(), Error> {
        self.clear();
        let ctrlr_map = &*SPI_CTRLR_BUS_MAP.read();
        let mut ctrlr_bus = None;

        for ctrlr in ctrlr_map.iter() {
            if ctrlr.bus_start <= bus && ctrlr.bus_end >= bus {
                ctrlr_bus = Some(ctrlr);
                break;
            }
        }

        if let Some(c) = ctrlr_bus {
            self.bus = bus;
            self.cs = cs;

            c.setup(self)
        } else {
            //error!("Can't find SPI bus {}", bus);
            Err(Error::MissingSpiBus)
        }
    }

    pub fn xfer_single_op(&self, op: &mut SpiOp) -> Result<(), Error> {
        let ret = self.xfer(op.dout, op.din);
        if ret.is_err() {
            op.status = SpiOpStatus::Failure;
        } else {
            op.status = SpiOpStatus::Success;
        }
        ret
    }

    pub fn xfer_vector_default(&self, vectors: &mut [SpiOp]) -> Result<(), Error> {
        for vector in vectors.iter_mut() {
            self.xfer_single_op(vector)?;
        }

        Ok(())
    }

    pub fn crop_chunk(&self, mut cmd_len: usize, buf_len: usize) -> Result<usize, Error> {
        let deduct_cmd_len = !!(self.flags() & SpiCntrlrDeduct::CmdLen as u32);
        let deduct_opcode_len = !!(self.flags() & SpiCntrlrDeduct::OpcodeLen as u32);
        let mut ctrlr_max = self.max_xfer_size() as usize;

        assert!(ctrlr_max != 0);

        /* Assume opcode is always one byte and deduct it from the cmd_len
        as the hardware has a separate register for the opcode. */
        if deduct_opcode_len != 0 {
            cmd_len -= 1;
        }

        /* Subtract command length from useable buffer size. If
        deduct_opcode_len is set, only subtract the number command bytes
        after the opcode. If the adjusted cmd_len is larger than ctrlr_max
        return 0 to inidicate an error. */
        if deduct_cmd_len != 0 {
            if ctrlr_max >= cmd_len {
                ctrlr_max -= cmd_len;
            } else {
                ctrlr_max = 0;
                //debug!("crop_chunk: Command longer than buffer");
            }
        }

        Ok(core::cmp::min(ctrlr_max, buf_len))
    }

    pub fn w8r8(&self, byte: u8) -> Result<u8, Error> {
        let dout = [byte, 0u8];
        let mut din = [0u8; 2];

        self.xfer(&dout, &mut din)?;

        Ok(din[1])
    }
}

impl Context for SpiSlave {}
