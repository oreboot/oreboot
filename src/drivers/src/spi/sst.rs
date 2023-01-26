use crate::spi::{
    spi_flash::{
        SPI_FLASH_PP_0x20_SECTOR_DESC, SpiFlash, SpiFlashOps, SpiFlashOpsDescriptor,
        SpiFlashPartId, SpiFlashVendorInfo, FLASH_VENDORS_LEN,
    },
    spi_generic::{SPI_FLASH_PROG_TIMEOUT_MS, VENDOR_ID_SST},
    Error,
};


pub const CMD_SST_WREN: u8 = 0x06; /* Write Enable */
pub const CMD_SST_WRDI: u8 = 0x04; /* Write Disable */
pub const CMD_SST_RDSR: u8 = 0x05; /* Read Status Register */
pub const CMD_SST_WRSR: u8 = 0x01; /* Write Status Register */
pub const CMD_SST_EWSR: u8 = 0x50; /* Enable Write Status Register */
pub const CMD_SST_READ: u8 = 0x03; /* Read Data Bytes */
pub const CMD_SST_FAST_READ: u8 = 0x0b; /* Read Data Bytes at Higher Speed */
pub const CMD_SST_BP: u8 = 0x02; /* Byte Program */
pub const CMD_SST_PP: u8 = 0x02; /* Page Program */
pub const CMD_SST_AAI_WP: u8 = 0xAD; /* Auto Address Increment Word Program */
pub const CMD_SST_SE: u8 = 0x20; /* Sector Erase */

pub const SST_SR_WIP: u8 = 1 << 0; /* Write-in-Progress */
pub const SST_SR_WEL: u8 = 1 << 1; /* Write enable */
pub const SST_SR_BP0: u8 = 1 << 2; /* Block Protection 0 */
pub const SST_SR_BP1: u8 = 1 << 3; /* Block Protection 1 */
pub const SST_SR_BP2: u8 = 1 << 4; /* Block Protection 2 */
pub const SST_SR_AAI: u8 = 1 << 6; /* Addressing mode */
pub const SST_SR_BPL: u8 = 1 << 7; /* BP bits lock */

pub const FLASH_TABLE_AI: [SpiFlashPartId; 11] = [
    /* SST25VF040B */
    SpiFlashPartId::create(0x8d, 7),
    /* SST25VF080B */
    SpiFlashPartId::create(0x8e, 8),
    /* SST25VF080 */
    SpiFlashPartId::create(0x80, 8),
    /* SST25VF016B */
    SpiFlashPartId::create(0x41, 9),
    /* SST25VF032B */
    SpiFlashPartId::create(0x4a, 10),
    /* SST25WF512 */
    SpiFlashPartId::create(0x01, 4),
    /* SST25WF010 */
    SpiFlashPartId::create(0x02, 5),
    /* SST25WF020 */
    SpiFlashPartId::create(0x03, 6),
    /* SST25WF040 */
    SpiFlashPartId::create(0x04, 7),
    /* SST25WF080 */
    SpiFlashPartId::create(0x05, 8),
    /* SST25WF080B */
    SpiFlashPartId::create(0x14, 8),
];

pub const FLASH_TABLE_PP256: [SpiFlashPartId; 1] = [SpiFlashPartId::create(0x4b, 11)]; /* SST25VF064C */

impl<'a, 'b, 'c> SpiFlash<'a, 'b, 'c> {
    pub fn sst_enable_writing(&self) -> Result<(), Error> {
        if let Err(e) = self.spi.spi_flash_cmd(CMD_SST_WREN, &mut []) {
            //error!("SF: Enabling Write failed");
            Err(e)
        } else {
            Ok(())
        }
    }

    pub fn sst_enable_writing_status(&self) -> Result<(), Error> {
        if let Err(e) = self.spi.spi_flash_cmd(CMD_SST_EWSR, &mut []) {
            //error!("SF: Enabling Write Status failed");
            Err(e)
        } else {
            Ok(())
        }
    }

    pub fn sst_disable_writing(&self) -> Result<(), Error> {
        if let Err(e) = self.spi.spi_flash_cmd(CMD_SST_WRDI, &mut []) {
            //error!("SF: Disabling Write failed");
            Err(e)
        } else {
            Ok(())
        }
    }

    pub fn sst_byte_write(&self, offset: u32, buf: &[u8]) -> Result<(), Error> {
        let cmd = [
            CMD_SST_BP,
            (offset >> 16) as u8,
            (offset >> 8) as u8,
            offset as u8,
        ];

        if cfg!(feature = "debug_spi_flash") {
            //debug!(
                "BP[{:2x}]: {} => cmd = ( 0x{:2x} 0x{:6x} )",
                self.spi.w8r8(CMD_SST_RDSR)?,
                &buf[0] as *const _ as usize,
                cmd[0],
                offset
            );
        }

        self.sst_enable_writing()?;

        self.spi.spi_flash_cmd_write(&cmd, &buf[..1])?;

        self.cmd_wait_ready(SPI_FLASH_PROG_TIMEOUT_MS)
    }

    pub fn sst_write_ai(&self, mut offset: u32, buf: &[u8]) -> Result<(), Error> {
        let done =
            |ret: Result<(), Error>, len: usize, offset: u32, actual: usize| -> Result<(), Error> {
                if cfg!(feature = "debug_spi_flash") {
                    //debug!(
                        "SF: SST: program {} {} bytes @ 0x{:x}",
                        if ret.is_err() { "failure" } else { "success" },
                        len,
                        offset - actual as u32
                    );
                }

                ret
            };

        let len = buf.len();
        let mut cmd = [0u8; 4];

        /* If the data is not word aligned, write out leading single byte */
        let mut actual = (offset % 2) as usize;

        if actual != 0 {
            if let Err(e) = self.sst_byte_write(offset, buf) {
                return done(Err(e), len, offset, actual);
            }
        }
        offset += actual as u32;

        if let Err(e) = self.sst_enable_writing() {
            return done(Err(e), len, offset, actual);
        }

        cmd[0] = CMD_SST_AAI_WP;
        cmd[1] = (offset >> 16) as u8;
        cmd[2] = (offset >> 8) as u8;
        cmd[3] = offset as u8;

        let mut ret = Ok(());

        while actual < len - 1 {
            if cfg!(feature = "debug_spi_flash") {
                //debug!(
                    "WP[{:2x}]: {} => cmd = ( 0x{:2x} 0x{:6x} )",
                    self.spi.w8r8(CMD_SST_RDSR)?,
                    &buf[actual] as *const _ as usize,
                    cmd[0],
                    offset
                );
            }

            if let Err(e) = self.spi.spi_flash_cmd_write(&cmd, &buf[actual..actual + 2]) {
                //error!("SF: SST word program failed");
                ret = Err(e);
                break;
            }

            ret = self.cmd_wait_ready(SPI_FLASH_PROG_TIMEOUT_MS);

            if ret.is_err() {
                break;
            }

            offset += 2;
            actual += 2;
        }

        if ret.is_ok() {
            ret = self.sst_disable_writing();
        }

        /* If there is a single trailing byte, write it out */
        if ret.is_ok() && actual != len {
            ret = self.sst_byte_write(offset, &buf[actual..]);
        }

        done(ret, len, offset, actual)
    }

    pub fn sst_unlock(&self) -> Result<(), Error> {
        self.sst_enable_writing_status()?;

        let mut status = [0; 1];
        self.spi.spi_flash_cmd_write(&[CMD_SST_WRSR], &mut status)?;

        //debug!("SF: SST: status = {:x}", self.spi.w8r8(CMD_SST_RDSR)?);

        Ok(())
    }
}

impl<'a, 'b> SpiFlashVendorInfo<'a, 'b> {
    pub fn after_probe(&self, flash: &SpiFlash) -> Result<(), Error> {
        flash.sst_unlock()
    }
}

impl SpiFlashOps for SpiFlashOpsDescriptor {
    fn read(&self, flash: &SpiFlash, offset: u32, buf: &mut [u8]) -> Result<(), Error> {
        flash.cmd_read(offset, buf)
    }

    fn write(&self, flash: &SpiFlash, offset: u32, buf: &[u8]) -> Result<(), Error> {
        flash.sst_write_ai(offset, buf)
    }

    fn erase(&self, flash: &SpiFlash, offset: u32, len: usize) -> Result<(), Error> {
        flash.cmd_erase(offset, len)
    }

    fn status(&self, flash: &SpiFlash, reg: &mut [u8]) -> Result<(), Error> {
        flash.cmd_status(reg)
    }
}

pub const DESCAI: SpiFlashOpsDescriptor = SpiFlashOpsDescriptor {
    erase_cmd: CMD_SST_SE,
    status_cmd: CMD_SST_RDSR,
    pp_cmd: 0,
    wren_cmd: CMD_SST_WREN,
};

pub const SPI_FLASH_SST_AI_VI: SpiFlashVendorInfo =
    SpiFlashVendorInfo::create(VENDOR_ID_SST, 0, 2, 0xff, &FLASH_TABLE_AI, &DESCAI);

pub const SPI_FLASH_SST_VI: SpiFlashVendorInfo = SpiFlashVendorInfo::create(
    VENDOR_ID_SST,
    8,
    2,
    0xff,
    &FLASH_TABLE_PP256,
    &SPI_FLASH_PP_0x20_SECTOR_DESC,
);

pub static SPI_FLASH_VENDORS: [SpiFlashVendorInfo; FLASH_VENDORS_LEN] =
    [SPI_FLASH_SST_AI_VI, SPI_FLASH_SST_VI];
