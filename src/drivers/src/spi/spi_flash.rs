#![allow(non_upper_case_globals)]

#[cfg(feature = "adesto")]
use crate::spi::adesto::SPI_FLASH_VENDORS;
#[cfg(feature = "amic")]
use crate::spi::amic::SPI_FLASH_VENDORS;
#[cfg(feature = "atmel")]
use crate::spi::atmel::SPI_FLASH_VENDORS;
#[cfg(feature = "eon")]
use crate::spi::eon::SPI_FLASH_VENDORS;
#[cfg(feature = "gigadevice")]
use crate::spi::gigadevice::SPI_FLASH_VENDORS;
#[cfg(feature = "macronix")]
use crate::spi::macronix::SPI_FLASH_VENDORS;
#[cfg(feature = "spansion")]
use crate::spi::spansion::SPI_FLASH_VENDORS;
#[cfg(feature = "sst")]
use crate::spi::sst::SPI_FLASH_VENDORS;
#[cfg(feature = "stmicro")]
use crate::spi::stmicro::SPI_FLASH_VENDORS;
#[cfg(feature = "winbond")]
use crate::spi::winbond::SPI_FLASH_VENDORS;
use crate::spi::{
    spi_generic::{SpiCtrlr, SpiOp, SpiOpStatus, SpiSlave, SPI_FLASH_PAGE_ERASE_TIMEOUT_MS},
    Error, BOOT_DEVICE_SPI_FLASH_BUS,
};
use alloc::vec;
use bitfield::bitfield;
use log::{debug, error, info};
use mainboard::ROM_SIZE;
use rules::ENV_INITIAL_STAGE;
use util::{helpers::KIB, region::Region, timer::Stopwatch};

#[cfg(not(any(
    feature = "adesto",
    feature = "amic",
    feature = "atmel",
    feature = "eon",
    feature = "gigadevice",
    feature = "macronix",
    feature = "spansion",
    feature = "sst",
    feature = "stmicro",
    feature = "winbond"
)))]
pub static SPI_FLASH_VENDORS: [SpiFlashVendorInfo; 0] = [];

/* SPI Flash opcodes */
pub const SPI_OPCODE_WREN: u8 = 0x06;
pub const SPI_OPCODE_FAST_READ: u8 = 0x0b;
pub const IDCODE_LEN: usize = 5;
pub const CMD_EXIT_4BYTE_ADDR_MODE: u8 = 0xe9;
pub const CMD_READ_ID: u8 = 0x9f;
pub const CMD_READ_ARRAY_SLOW: u8 = 0x03;
pub const CMD_READ_ARRAY_FAST: u8 = 0x0b;
pub const CMD_READ_FAST_DUAL_OUTPUT: u8 = 0x3b;
pub const CMD_READ_FAST_DUAL_IO: u8 = 0xbb;
pub const CMD_READ_STATUS: u8 = 0x05;
pub const STATUS_WIP: u8 = 0x01;
pub const CMD_WRITE_ENABLE: u8 = 0x06;

#[cfg(any(
    feature = "adesto",
    feature = "amic",
    feature = "atmel",
    feature = "eon",
    feature = "gigadevice",
    feature = "macronix",
    feature = "winbond",
))]
pub const FLASH_VENDORS_LEN: usize = 1;
#[cfg(feature = "spansion")]
pub const FLASH_VENDORS_LEN: usize = 3;
#[cfg(feature = "sst")]
pub const FLASH_VENDORS_LEN: usize = 2;
#[cfg(feature = "stmicro")]
pub const FLASH_VENDORS_LEN: usize = 4;

pub const SPI_FLASH_PP_0x20_SECTOR_DESC: SpiFlashOpsDescriptor = SpiFlashOpsDescriptor {
    erase_cmd: 0x20,  /* Sector Erase */
    status_cmd: 0x05, /* Read Status */
    pp_cmd: 0x02,     /* Page Program */
    wren_cmd: 0x06,   /* Write Enable */
};

pub const SPI_FLASH_PP_0xD8_SECTOR_DESC: SpiFlashOpsDescriptor = SpiFlashOpsDescriptor {
    erase_cmd: 0xd8,  /* Sector Erase */
    status_cmd: 0x05, /* Read Status */
    pp_cmd: 0x02,     /* Page Program */
    wren_cmd: 0x06,   /* Write Enable */
};

pub fn spi_flash_addr(addr: u32, cmd: &mut [u8]) {
    assert!(cmd.len() >= 4);
    cmd[1] = (addr >> 16) as u8;
    cmd[2] = (addr >> 8) as u8;
    cmd[3] = (addr >> 0) as u8;
}

bitfield! {
    #[derive(Clone, Copy)]
    pub struct PartIdFields(u16);
    /* Log based 2 total number of sectors. */
    pub nr_sectors_shift, set_nr_sectors_shift: 4, 0;
    pub fast_read_dual_output_support, set_fast_read_dual_output_support: 1, 4; /* 1-1-2 read */
    pub fast_read_dual_io_support, set_fast_read_dual_io_support: 1, 5; /* 1-2-2 read */
    reserved_for_flags, _: 2, 6;
    /* Block protection. Currently used by Winbond. */
    pub protection_granularity_shift, set_protection_granularity_shift: 5, 8;
    pub bp_bits, set_bp_bits: 3, 13;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SpiFlashPartId {
    /* rdid command constructs 2x 16-bit id using the following method
     * for matching after reading 5 bytes (1st byte is manuf id):
     *    id[0] = (id[1] << 8) | id[2]
     *    id[1] = (id[3] << 8) | id[4]
     */
    pub id: [u16; 2],
    /* Log based 2 total number of sectors. */
    pub fields: PartIdFields,
}

impl SpiFlashPartId {
    pub const fn create(id0: u16, nr_sectors_shift: u16) -> Self {
        Self {
            id: [id0, 0],
            fields: PartIdFields(nr_sectors_shift & 0b1111),
        }
    }

    pub const fn create_id(id: [u16; 2], nr_sectors_shift: u16) -> Self {
        Self {
            id,
            fields: PartIdFields(nr_sectors_shift & 0b1111),
        }
    }

    pub const fn create_fast_read(
        id0: u16,
        nr_sectors_shift: u16,
        fast_read_dual_output_support: u16,
        fast_read_dual_io_support: u16,
    ) -> Self {
        Self {
            id: [id0, 0],
            fields: PartIdFields(
                (nr_sectors_shift & 0b1111)
                    | ((fast_read_dual_output_support & 0b1) << 4)
                    | ((fast_read_dual_io_support & 0b1) << 5),
            ),
        }
    }

    pub const fn create_full(
        id: [u16; 2],
        nr_sectors_shift: u16,
        fast_read_dual_output_support: u16,
        fast_read_dual_io_support: u16,
        protection_granularity_shift: u16,
        bp_bits: u16,
    ) -> Self {
        Self {
            id,
            fields: PartIdFields(
                (nr_sectors_shift & 0b1111)
                    | ((fast_read_dual_output_support & 0b1) << 4)
                    | ((fast_read_dual_io_support & 0b1) << 5)
                    | ((protection_granularity_shift & 0b11111) << 8)
                    | ((bp_bits & 0b111) << 13),
            ),
        }
    }
}

bitfield! {
    pub struct VendorInfoShift(u8);
    pub page_size_shift, set_page_size_shift: 4, 0; /* if page programming oriented */
    pub sector_size_kib_shift, set_sector_size_kib_shift: 4, 4; /* Log based 2 sector size */
}

#[repr(C)]
pub struct SpiFlashOpsDescriptor {
    pub erase_cmd: u8,  /* Sector Erase */
    pub status_cmd: u8, /* Read Status Register */
    pub pp_cmd: u8,     /* Page program command, if supported */
    pub wren_cmd: u8,   /* Write Enable command */
}

#[cfg(not(feature = "sst"))]
impl SpiFlashOps for SpiFlashOpsDescriptor {}

#[repr(C)]
pub struct SpiFlashVendorInfo<'a, 'b> {
    id: u8,
    shift: VendorInfoShift,
    ids: &'a [SpiFlashPartId],
    match_id_mask: [u16; 2], /* matching bytes of the id for this set */
    desc: &'b SpiFlashOpsDescriptor,
}

impl<'a, 'b> SpiFlashVendorInfo<'a, 'b> {
    pub const fn create(
        id: u8,
        page_size_shift: u8,
        sector_size_kib_shift: u8,
        match_id_mask0: u16,
        ids: &'a [SpiFlashPartId],
        desc: &'b SpiFlashOpsDescriptor,
    ) -> Self {
        let shift =
            VendorInfoShift((page_size_shift & 0b1111) | ((sector_size_kib_shift & 0b1111) << 4));
        let match_id_mask = [match_id_mask0, 0];

        Self {
            id,
            shift,
            ids,
            match_id_mask,
            desc,
        }
    }

    pub const fn create_id(
        id: u8,
        page_size_shift: u8,
        sector_size_kib_shift: u8,
        match_id_mask: [u16; 2],
        ids: &'a [SpiFlashPartId],
        desc: &'b SpiFlashOpsDescriptor,
    ) -> Self {
        let shift =
            VendorInfoShift((page_size_shift & 0b1111) | ((sector_size_kib_shift & 0b1111) << 4));

        Self {
            id,
            shift,
            ids,
            match_id_mask,
            desc,
        }
    }

    pub fn find_part(&self, id: &[u16]) -> Option<&SpiFlashPartId> {
        let lid = [id[0] & self.match_id_mask[0], id[1] & self.match_id_mask[1]];

        for part in self.ids.iter() {
            if part.id[0] == lid[0] && part.id[1] == lid[1] {
                return Some(part);
            }
        }

        None
    }

    #[cfg(not(feature = "sst"))]
    pub fn after_probe(&self, _flash: &SpiFlash) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }
}

#[cfg(not(feature = "winbond"))]
impl SpiFlashProtectionOps for SpiFlashVendorInfo<'static, 'static> {
    fn get_write(&self, _flash: &SpiFlash, _region: &Region) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn set_write(
        &self,
        _flash: &SpiFlash,
        _region: &Region,
        _status: SpiFlashStatusRegLockdown,
    ) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }
}

bitfield! {
    pub struct SpiFlashFlags(u8);
    pub dual_output, set_dual_output: 1, 0;
    pub dual_io, set_dual_io: 1, 1;
    reserved, _: 6, 2;
}

impl Clone for SpiFlashFlags {
    fn clone(&self) -> Self {
        let mut f = Self(0);
        {
            f.set_dual_output(self.dual_output());
            f.set_dual_io(self.dual_io());
            f
        }
    }
}
impl Copy for SpiFlashFlags {}

pub union SpiFlashFlagsUnion {
    pub raw: u8,
    pub flags: SpiFlashFlags,
}

/**
 * SPI write protection is enforced by locking the status register.
 * The following modes are known. It depends on the flash chip if the
 * mode is actually supported.
 *
 * PRESERVE : Keep the previous status register lock-down setting (noop)
 * NONE     : Status register isn't locked
 * PIN      : Status register is locked as long as the ~WP pin is active
 * REBOOT   : Status register is locked until power failure
 * PERMANENT: Status register is permanently locked
 */
pub enum SpiFlashStatusRegLockdown {
    WriteProtectionPreserve = -1,
    WriteProtectionNone = 0,
    WriteProtectionPin = 1,
    WriteProtectionReboot = 2,
    WriteProtectionPermanent = 3,
}

/// SpiFlash is a port of struct spi_flash from coreboot
///
/// spi_flash is a packed struct in coreboot, but is unpacked in oreboot to
/// avoid undefined behavior by making a call through an unaligned member
///
/// See issue #82523 <https://github.com/rust-lang/rust/issues/82523>
#[repr(C)]
pub struct SpiFlash {
    pub spi: SpiSlave,
    pub vendor: u8,
    pub flags: SpiFlashFlagsUnion,
    pub model: u16,
    pub size: u32,
    pub sector_size: u32,
    pub page_size: u32,
    pub erase_cmd: u8,
    pub status_cmd: u8,
    pub pp_cmd: u8,   /* Page program command */
    pub wren_cmd: u8, /* Write Enable command */
    pub part: Option<SpiFlashPartId>,
}

// FIXME: provide actual implementations, may require mainboard specific impls
impl SpiFlashOps for SpiFlash {}

// FIXME: provide actual implementations, may require mainboard specific impls
impl SpiFlashProtectionOps for SpiFlash {
    fn get_write(&self, _flash: &SpiFlash, _region: &Region) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn set_write(
        &self,
        _flash: &SpiFlash,
        _region: &Region,
        _status: SpiFlashStatusRegLockdown,
    ) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }
}

impl SpiFlash {
    pub const fn new() -> Self {
        Self {
            spi: SpiSlave::new(),
            vendor: 0u8,
            flags: SpiFlashFlagsUnion {
                flags: SpiFlashFlags(0),
            },
            model: 0u16,
            size: 0u32,
            sector_size: 0u32,
            page_size: 0u32,
            erase_cmd: 0u8,
            status_cmd: 0u8,
            pp_cmd: 0u8,   /* Page program command */
            wren_cmd: 0u8, /* Write Enable command */
            part: None,
        }
    }

    pub fn cmd_read(&self, mut offset: u32, buf: &mut [u8]) -> Result<(), Error> {
        let mut cmd = [0u8; 5];
        let mut len = buf.len();

        let cmd_len: usize;

        let do_cmd = if cfg!(feature = "spi_flash_no_fast_read") {
            cmd_len = 4;
            cmd[0] = CMD_READ_ARRAY_SLOW;
            |spi: &SpiSlave, dout: &[u8], din: &mut [u8]| -> Result<(), Error> {
                spi.do_spi_flash_cmd(dout, din)
            }
        } else if unsafe { self.flags.flags.dual_io() } != 0 {
            cmd_len = 5;
            cmd[0] = CMD_READ_FAST_DUAL_IO;
            cmd[4] = 0;
            |spi: &SpiSlave, dout: &[u8], din: &mut [u8]| -> Result<(), Error> {
                spi.do_dual_io_cmd(dout, din)
            }
        } else if unsafe { self.flags.flags.dual_output() } != 0 {
            cmd_len = 5;
            cmd[0] = CMD_READ_FAST_DUAL_OUTPUT;
            cmd[4] = 0;
            |spi: &SpiSlave, dout: &[u8], din: &mut [u8]| -> Result<(), Error> {
                spi.do_dual_output_cmd(dout, din)
            }
        } else {
            cmd_len = 5;
            cmd[0] = CMD_READ_ARRAY_FAST;
            cmd[4] = 0;
            |spi: &SpiSlave, dout: &[u8], din: &mut [u8]| -> Result<(), Error> {
                spi.do_spi_flash_cmd(dout, din)
            }
        };

        let mut data = buf;

        while len > 0 {
            let xfer_len = self.spi.crop_chunk(cmd_len, len)?;
            spi_flash_addr(offset, &mut cmd);

            if let Err(ret) = do_cmd(&self.spi, &cmd[..cmd_len], &mut data[..xfer_len]) {
                debug!(
                    "SF: Failed to send read command {:2x}({:x}, {:x}): {}",
                    cmd[0], offset, xfer_len, ret as u8
                );
                return Err(ret);
            }

            offset += xfer_len as u32;
            data = &mut data[xfer_len..];
            len -= xfer_len;
        }

        Ok(())
    }

    pub fn cmd_poll_bit(&self, timeout: u64, cmd: u8, poll_bit: u8) -> Result<(), Error> {
        let spi = &self.spi;
        let mut attempt = 0;
        let mut status = [0u8; 1];
        let mut sw = Stopwatch::new();

        sw.init_msecs_expire(timeout);

        while !sw.expired() {
            attempt += 1;

            if let Err(e) = spi.do_spi_flash_cmd(&[cmd], &mut status) {
                debug!(
                    "SF: SPI command failed on attempt {} with rc {}",
                    attempt, e as u8
                );
                return Err(e);
            }

            if status[0] & poll_bit == 0 {
                return Ok(());
            }
        }

        debug!(
            "SF: timeout at {} msec after {} attempts",
            sw.duration_msecs(),
            attempt
        );

        Err(Error::Generic)
    }

    pub fn cmd_wait_ready(&self, timeout: u64) -> Result<(), Error> {
        self.cmd_poll_bit(timeout, CMD_READ_STATUS, STATUS_WIP)
    }

    pub fn cmd_erase(&self, mut offset: u32, len: usize) -> Result<(), Error> {
        let mut cmd = [0u8; 4];

        let erase_size = self.sector_size as usize;

        if offset as usize % erase_size != 0 || len % erase_size != 0 {
            error!("SF: Erase offset/length not multiple of erase size");
            return Err(Error::Generic);
        }

        if len == 0 {
            error!("SF: Erase length cannot be 0");
            return Err(Error::Generic);
        }

        cmd[0] = self.erase_cmd;
        let start = offset as usize;
        let end = start as usize + len;

        while offset < end as u32 {
            spi_flash_addr(offset, &mut cmd);
            offset += erase_size as u32;

            if cfg!(feature = "debug_spi_flash") {
                debug!(
                    "SF: erase {:2x} {:2x} {:2x} {:2x} ({:x})",
                    cmd[0], cmd[1], cmd[2], cmd[3], offset
                );
            }

            self.spi.spi_flash_cmd(CMD_WRITE_ENABLE, &mut [])?;

            self.spi.spi_flash_cmd_write(&cmd, &mut [])?;

            self.cmd_wait_ready(SPI_FLASH_PAGE_ERASE_TIMEOUT_MS)?;
        }

        debug!("SF: Successfully erased {} bytes @ {:x}", len, start);

        Ok(())
    }

    pub fn cmd_status(&self, reg: &mut [u8]) -> Result<(), Error> {
        self.spi.spi_flash_cmd(self.status_cmd, reg)
    }

    pub fn probe(&mut self, bus: u32, cs: u32) -> Result<(), Error> {
        if let Err(e) = self.spi.setup(bus, cs) {
            debug!("SF: Failed to set up slave");
            return Err(e);
        }

        // Try special programmer probe if any.
        let spi = self.spi.clone();
        let mut ret = spi.flash_probe(self);

        // If flash is not found, try generic spi flash probe.
        if ret.is_err() {
            let spi = self.spi.clone();
            ret = spi.flash_generic_probe(self);
        }

        // Give up -- nothing more to try if flash is not found.
        if ret.is_err() {
            debug!("SF: Unsupported manufacturer!");
            return Err(Error::Generic);
        }

        let mode_string = if unsafe { self.flags.flags.dual_io() } != 0
            && self.spi.xfer_dual(&[], &mut []).is_ok()
        {
            " (Dual I/O mode)"
        } else if unsafe { self.flags.flags.dual_output() } != 0
            && self.spi.xfer_dual(&[], &mut []).is_ok()
        {
            " (Dual Output mode)"
        } else {
            ""
        };

        info!(
            "SF: Detected {:02x} {:04x} with sector size 0x{:x}, total 0x{:x}{}",
            self.vendor, self.model, self.sector_size, self.size, mode_string
        );

        if (bus == (BOOT_DEVICE_SPI_FLASH_BUS as u32)) && self.size != ROM_SIZE {
            error!(
                "SF size 0x{:x} does not correspond to CONFIG_ROM_SIZE 0x{:x}!!",
                self.size, ROM_SIZE
            );
        }

        if cfg!(feature = "spi_flash_exit_4_byte_addr_mode") && ENV_INITIAL_STAGE != 0 {
            self.spi.spi_flash_cmd(CMD_EXIT_4BYTE_ADDR_MODE, &mut [])?;
        }

        Ok(())
    }
}

/// Representation of SPI flash operations:
/// read:	Flash read operation.
/// write:	Flash write operation.
/// erase:	Flash erase operation.
/// status:	Read flash status register.
pub trait SpiFlashOps {
    fn read(&self, _flash: &SpiFlash, _offset: u32, _buf: &mut [u8]) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn write(&self, _flash: &SpiFlash, _offset: u32, _buf: &[u8]) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn erase(&self, _flash: &SpiFlash, _offset: u32, _len: usize) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn status(&self, _flash: &SpiFlash, _reg: &mut [u8]) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }
}

/// Current code assumes all callbacks are supplied in this object.
pub trait SpiFlashProtectionOps {
    /// Returns 1 if the whole region is software write protected.
    /// Hardware write protection mechanism aren't accounted.
    /// If the write protection could be changed, due to unlocked status
    /// register for example, 0 should be returned.
    /// Returns 0 on success.
    fn get_write(&self, flash: &SpiFlash, region: &Region) -> Result<(), Error>;

    /// Enable the status register write protection, if supported on the
    /// requested region, and optionally enable status register lock-down.
    /// Returns 0 if the whole region was software write protected.
    /// Hardware write protection mechanism aren't accounted.
    /// If the status register is locked and the requested configuration
    /// doesn't match the selected one, return an error.
    /// Only a single region is supported !
    ///
    /// @return 0 on success
    fn set_write(
        &self,
        flash: &SpiFlash,
        region: &Region,
        status: SpiFlashStatusRegLockdown,
    ) -> Result<(), Error>;
}

impl SpiSlave {
    pub fn do_spi_flash_cmd(&self, dout: &[u8], din: &mut [u8]) -> Result<(), Error> {
        let in_len = din.len();
        /*
         * SPI flash requires command-response kind of behavior. Thus, two
         * separate SPI vectors are required -- first to transmit dout and other
         * to receive in din. If some specialized SPI flash controllers
         * (e.g. x86) can perform both command and response together, it should
         * be handled at SPI flash controller driver level.
         */
        let mut vectors = [
            SpiOp {
                dout,
                din: &mut [],
                status: SpiOpStatus::NotExecuted,
            },
            SpiOp {
                dout: &[],
                din,
                status: SpiOpStatus::NotExecuted,
            },
        ];

        let count = if in_len == 0 { 1 } else { vectors.len() };

        self.claim_bus()?;

        let ret = self.xfer_vector(&mut vectors[..count]);

        self.release_bus()?;

        ret
    }

    pub fn do_dual_output_cmd(&self, dout: &[u8], din: &mut [u8]) -> Result<(), Error> {
        /*
         * spi_xfer_vector() will automatically fall back to .xfer() if
         * .xfer_vector() is unimplemented. So using vector API here is more
         * flexible, even though a controller that implements .xfer_vector()
         * and (the non-vector based) .xfer_dual() but not .xfer() would be
         * pretty odd.
         */
        let mut vector = [SpiOp {
            dout,
            din: &mut [],
            status: SpiOpStatus::NotExecuted,
        }];

        self.claim_bus()?;

        let mut ret = self.xfer_vector(&mut vector);
        if ret.is_ok() {
            ret = self.xfer_dual(&[], din);
        }

        self.release_bus()?;

        ret
    }

    pub fn do_dual_io_cmd(&self, dout: &[u8], din: &mut [u8]) -> Result<(), Error> {
        /* Only the very first byte (opcode) is transferred in "single" mode. */
        let mut vector = [SpiOp {
            dout: &dout[..1],
            din: &mut [],
            status: SpiOpStatus::NotExecuted,
        }];
        self.claim_bus()?;

        let mut ret = self.xfer_vector(&mut vector);

        if ret.is_ok() {
            ret = self.xfer_dual(&dout[1..], &mut []);
        }

        if ret.is_ok() {
            ret = self.xfer_dual(&[], din);
        }

        self.release_bus()?;

        ret
    }

    pub fn spi_flash_cmd(&self, cmd: u8, response: &mut [u8]) -> Result<(), Error> {
        let ret = self.do_spi_flash_cmd(&[cmd], response);

        if let Err(e) = &ret {
            debug!("SF: Failed to send command {:02x}: {}", cmd, *e as i32);
        }

        ret
    }

    pub fn spi_flash_vector_helper(
        &self,
        vectors: &mut [SpiOp],
        func: fn(&SpiSlave, &[u8], &mut [u8]) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let count = vectors.len();
        if count < 1 || count > 2 {
            return Err(Error::Generic);
        }

        /* SPI flash commands always have a command first... */
        if vectors[0].dout.len() == 0 {
            return Err(Error::Generic);
        }

        /* And not read any data during the command. */
        if vectors[0].din.len() != 0 {
            return Err(Error::Generic);
        }

        let din = if count == 2 {
            /* If response bytes requested ensure the buffer is valid. */
            if vectors[1].din.len() == 0 {
                return Err(Error::Generic);
            }

            /* No sends can accompany a receive. */
            if vectors[1].dout.len() != 0 {
                return Err(Error::Generic);
            }

            &mut *vectors[1].din
        } else {
            &mut [0u8; 0]
        };

        let ret = func(self, vectors[0].dout, din);

        if ret.is_err() {
            vectors[0].status = SpiOpStatus::Failure;
            if count == 2 {
                vectors[1].status = SpiOpStatus::Failure;
            }
        } else {
            vectors[0].status = SpiOpStatus::Success;
            if count == 2 {
                vectors[1].status = SpiOpStatus::Success;
            }
        }

        ret
    }

    pub fn flash_generic_probe(&self, flash: &mut SpiFlash) -> Result<(), Error> {
        let mut idcode = [0u8; IDCODE_LEN];
        let mut id = [0u16; 2];

        /* Read the ID codes */
        self.spi_flash_cmd(CMD_READ_ID, &mut idcode)?;

        if cfg!(DEBUG_SPI_FLASH) {
            debug!("SF: Got idcode: ");
            for b in idcode.iter() {
                debug!("{:2x}", b);
            }
        }

        let mut manuf_id = idcode[0];

        debug!("Manufacturer: {:2x}", manuf_id);

        /* If no result from RDID command and STMicro parts are enabled attempt
        to wake the part from deep sleep and obtain alternative id info. */
        if cfg!(feature = "stmicro") && manuf_id == 0xff {
            self.stmicro_release_deep_sleep_identify(&mut idcode)?;
            manuf_id = idcode[0];
        }

        id[0] = ((idcode[1] as u16) << 8) | idcode[2] as u16;
        id[1] = ((idcode[3] as u16) << 8) | idcode[4] as u16;

        self.find_match(flash, manuf_id, &id)
    }

    pub fn spi_flash_generic_probe(&self, flash: &mut SpiFlash) -> Result<(), Error> {
        let mut idcode = [0u8; IDCODE_LEN];
        let mut id = [0u16; 2];

        /* Read the ID codes */
        self.spi_flash_cmd(CMD_READ_ID, &mut idcode)?;

        if cfg!(DEBUG_SPI_FLASH) {
            debug!("SF: Got idcode: ");
            for b in idcode.iter() {
                debug!("{:2x}", b);
            }
        }

        let mut manuf_id = idcode[0];

        debug!("Manufacturer: {:2x}", manuf_id);

        /* If no result from RDID command and STMicro parts are enabled attempt
        to wake the part from deep sleep and obtain alternative id info. */
        if cfg!(feature = "stmicro") && manuf_id == 0xff {
            self.stmicro_release_deep_sleep_identify(&mut idcode)?;
            manuf_id = idcode[0];
        }

        id[0] = ((idcode[1] as u16) << 8) | idcode[2] as u16;
        id[1] = ((idcode[3] as u16) << 8) | idcode[4] as u16;

        self.find_match(flash, manuf_id, &id)
    }

    pub fn find_match(&self, flash: &mut SpiFlash, manuf_id: u8, id: &[u16]) -> Result<(), Error> {
        for vendor in SPI_FLASH_VENDORS.iter() {
            if manuf_id != vendor.id {
                continue;
            }

            if let Some(part) = vendor.find_part(id) {
                return self.fill_spi_flash(flash, vendor, part);
            } else {
                continue;
            }
        }

        Err(Error::Generic)
    }

    pub fn fill_spi_flash(
        &self,
        flash: &mut SpiFlash,
        vi: &SpiFlashVendorInfo,
        part: &SpiFlashPartId,
    ) -> Result<(), Error> {
        flash.spi = *self;
        flash.vendor = vi.id;
        flash.model = part.id[0];

        flash.page_size = 1 << vi.shift.page_size_shift();
        flash.sector_size = ((1 << vi.shift.sector_size_kib_shift()) * KIB) as u32;
        flash.size = flash.sector_size * (1 << part.fields.nr_sectors_shift());

        flash.erase_cmd = vi.desc.erase_cmd;
        flash.status_cmd = vi.desc.status_cmd;
        flash.pp_cmd = vi.desc.pp_cmd;
        flash.wren_cmd = vi.desc.wren_cmd;

        unsafe {
            flash
                .flags
                .flags
                .set_dual_output(part.fields.fast_read_dual_output_support() as u8);
            flash
                .flags
                .flags
                .set_dual_io(part.fields.fast_read_dual_io_support() as u8);
        }

        // prot_ops (SpiFlashProtectionOps) is as trait in oreboot
        flash.part = Some(*part);

        if let Err(e) = vi.after_probe(flash) {
            if e == Error::Unimplemented {
                Ok(())
            } else {
                Err(e)
            }
        } else {
            Ok(())
        }
    }

    #[cfg(not(feature = "stmicro"))]
    pub fn stmicro_release_deep_sleep_identify(&self, _idcode: &mut [u8]) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    pub fn spi_flash_cmd_write(&self, cmd: &[u8], data: &[u8]) -> Result<(), Error> {
        let cmd_len = cmd.len();
        let data_len = data.len();
        let mut buf = vec![0u8; cmd_len + data_len];
        buf[..cmd_len].copy_from_slice(cmd);
        buf[cmd_len..].copy_from_slice(data);

        if let Err(e) = self.do_spi_flash_cmd(&buf, &mut []) {
            debug!(
                "SF: Failed to send write command ({} bytes): {}",
                data_len, e as u8
            );
            Err(e)
        } else {
            Ok(())
        }
    }
}
