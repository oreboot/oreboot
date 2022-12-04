use crate::context::Context;
use crate::spi::{spi_flash::SpiFlash, Error};
use log::error;
use util::region::Region;

/* Controller-specific definitions: */

#[repr(C)]
pub struct SpiCtrlrBuses<'a> {
    pub ctrlr: Option<&'a dyn SpiCtrlr>,
    pub bus_start: u32,
    pub bus_end: u32,
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
pub trait SpiCtrlr {
    fn claim_bus(&self, _slave: &SpiSlave) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn release_bus(&self, _slave: &SpiSlave) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn setup(&self, _slave: &SpiSlave) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn xfer(&self, _slave: &SpiSlave, _dout: &[u8], _din: &mut [u8]) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn xfer_vector(&self, _slave: &SpiSlave, _vectors: &mut [SpiOp]) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn xfer_dual(&self, _slave: &SpiSlave, _dout: &[u8], _din: &mut [u8]) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn get_max_xfer_size(&self) -> u32;
    fn set_max_xfer_size(&mut self, size: u32);

    fn get_flags(&self) -> u32;
    fn set_flags(&mut self, _flags: u32);

    fn flash_probe(&self, _slave: &SpiSlave, _flash: &SpiFlash) -> Result<(), Error> {
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
pub struct SpiSlave<'a> {
    bus: u32,
    cs: u32,
    ctrlr: Option<&'a dyn SpiCtrlr>,
}

impl<'a> SpiSlave<'a> {
    pub const fn new() -> Self {
        Self {
            bus: 0,
            cs: 0,
            ctrlr: None,
        }
    }

    pub fn clear(&mut self) {
        self.bus = 0;
        self.cs = 0;
        self.ctrlr = None;
    }

    pub fn setup(
        &mut self,
        bus: u32,
        cs: u32,
        ctrlr_map: &'a [SpiCtrlrBuses],
    ) -> Result<(), Error> {
        self.clear();

        for ctrlr in ctrlr_map.iter() {
            if ctrlr.bus_start <= bus && ctrlr.bus_end >= bus {
                self.ctrlr = ctrlr.ctrlr.clone();
                break;
            }
        }

        if let Some(ctrlr) = &self.ctrlr {
            self.bus = bus;
            self.cs = cs;

            ctrlr.setup(self)
        } else {
            error!("Can't find SPI bus {}", bus);
            Err(Error::MissingSpiBus)
        }
    }

    pub fn claim_bus(&self) -> Result<(), Error> {
        if let Some(ctrlr) = self.ctrlr {
            return ctrlr.claim_bus(&self);
        }

        Ok(())
    }

    pub fn xfer_single_op(&self, op: &mut SpiOp) -> Result<(), Error> {
        if let Some(ctrlr) = self.ctrlr {
            let ret = ctrlr.xfer(self, op.dout, op.din);
            if ret.is_err() {
                op.status = SpiOpStatus::Failure;
            } else {
                op.status = SpiOpStatus::Success;
            }
            ret
        } else {
            Err(Error::MissingSpiCtrlr)
        }
    }

    pub fn xfer_vector_default(&self, vectors: &mut [SpiOp]) -> Result<(), Error> {
        for vector in vectors.iter_mut() {
            self.xfer_single_op(vector)?;
        }

        Ok(())
    }

    pub fn xfer_vector(&self, vectors: &mut [SpiOp]) -> Result<(), Error> {
        if let Some(ctrlr) = self.ctrlr {
            ctrlr.xfer_vector(self, vectors)
        } else {
            self.xfer_vector_default(vectors)
        }
    }

    pub fn xfer(&self, req_buf: &[u8], res_buf: &mut [u8]) -> Result<(), Error> {
        if let Some(ctrlr) = self.ctrlr {
            ctrlr.xfer(&self, req_buf, res_buf)
        } else {
            Err(Error::MissingSpiCtrlr)
        }
    }

    pub fn release_bus(&self) -> Result<(), Error> {
        if let Some(ctrlr) = self.ctrlr {
            ctrlr.release_bus(self)
        } else {
            Err(Error::MissingSpiCtrlr)
        }
    }
}

impl Context for SpiSlave<'static> {}
