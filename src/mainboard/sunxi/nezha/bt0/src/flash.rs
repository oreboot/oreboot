use oreboot_soc::sunxi::d1::spi::{Instance, Spi};

mod consts {
    #![allow(unused)]

    pub(super) const CMD_GET_FEATURE: u8 = 0x0f;
    pub(super) const CMD_READ_ID: u8 = 0x9f;
    pub(super) const CMD_READ_PAGE: u8 = 0x13;
    pub(super) const CMD_READ_CACHE: u8 = 0x03;
    pub(super) const FEAT_STATUS: u8 = 0xc0;
    pub(super) const LEN_PAGE_BITS: u32 = 11;
    pub(super) const LEN_PAGE: u32 = 1 << LEN_PAGE_BITS;
    pub(super) const LEN_PAGE_MASK: u32 = LEN_PAGE - 1;

    pub(super) const CMD_NOR_WRSR: u8 = 0x01;
    pub(super) const CMD_NOR_PROG: u8 = 0x02;
    pub(super) const CMD_NOR_READ: u8 = 0x03;
    pub(super) const CMD_NOR_READ_FAST: u8 = 0x0b;
    pub(super) const CMD_NOR_READ_STATUS_REG1: u8 = 0x05;
    pub(super) const CMD_NOR_READ_STATUS_REG2: u8 = 0x35;
    pub(super) const CMD_NOR_READ_STATUS_REG3: u8 = 0x15;
    pub(super) const CMD_NOR_WRITE_ENABLE: u8 = 0x06;
    pub(super) const CMD_NOR_E4K: u8 = 0x20;
    pub(super) const CMD_NOR_E32K: u8 = 0x52;
    pub(super) const CMD_NOR_SFDP: u8 = 0x5a;
    pub(super) const CMD_NOR_READ_ID: u8 = 0x9f;
    pub(super) const CMD_NOR_ENTER_4B: u8 = 0xb7;
    pub(super) const CMD_NOR_E64K: u8 = 0xd8;
    pub(super) const CMD_NOR_EXIT_4B: u8 = 0xe9;
}

use consts::*;

/// NAND Flash with SPI.
#[cfg(feature = "nand")]
pub struct SpiNand<SPI: Instance, PINS>(Spi<SPI, PINS>);

#[cfg(feature = "nand")]
impl<SPI: Instance, PINS> SpiNand<SPI, PINS> {
    #[inline]
    pub fn new(inner: Spi<SPI, PINS>) -> Self {
        Self(inner)
    }
    #[allow(unused)] // FIXME: remove when SpiNand is moved to a seperate crate
    #[inline]
    pub fn free(self) -> Spi<SPI, PINS> {
        self.0
    }
}

#[cfg(feature = "nand")]
impl<SPI: Instance, PINS> SpiNand<SPI, PINS> {
    /// Reads hardware ID.
    #[inline]
    pub fn read_id(&self) -> [u8; 3] {
        let mut buf = [0u8; 3];
        self.wait();
        self.0.transfer([CMD_READ_ID], 1, &mut buf);
        buf
    }

    /// Copies bytes from `base` address to `buf`.
    #[inline]
    pub fn copy_into(&mut self, mut base: u32, mut buf: &mut [u8]) {
        while !buf.is_empty() {
            let mut cmd = u32::to_be_bytes(base >> LEN_PAGE_BITS);
            cmd[0] = CMD_READ_PAGE;
            self.wait();
            self.0.transfer(cmd, 0, []);

            let ca = base & LEN_PAGE_MASK;
            let (head, tail) = buf.split_at_mut(buf.len().min((LEN_PAGE - ca) as _));
            base += head.len() as u32;
            buf = tail;

            let mut cmd = u32::to_be_bytes(ca);
            cmd[1] = CMD_READ_CACHE;
            self.wait();
            self.0.transfer(&cmd[1..], 1, head);
        }
    }
}

#[cfg(feature = "nand")]
impl<SPI: Instance, PINS> SpiNand<SPI, PINS> {
    #[inline]
    fn get_feature(&self, key: u8) -> u8 {
        let mut feature = 0u8;

        self.0.transfer(
            [CMD_GET_FEATURE, key],
            0,
            core::slice::from_mut(&mut feature),
        );

        feature
    }

    /// 等待忙状态结束。
    #[inline]
    fn wait(&self) {
        // SPI NOR QPI: C0 P7..P0 is for setting read parameters
        while self.get_feature(FEAT_STATUS) & 1 == 1 {
            core::hint::spin_loop();
        }
    }
}

/// NOR Flash with SPI.
#[cfg(feature = "nor")]
pub struct SpiNor<SPI: Instance, PINS>(Spi<SPI, PINS>);

#[cfg(feature = "nor")]
impl<SPI: Instance, PINS> SpiNor<SPI, PINS> {
    #[inline]
    pub fn new(inner: Spi<SPI, PINS>) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn free(self) -> Spi<SPI, PINS> {
        self.0
    }
}

#[cfg(feature = "nor")]
impl<SPI: Instance, PINS> SpiNor<SPI, PINS> {
    /// Reads hardware ID.
    #[inline]
    pub fn read_id(&self) -> [u8; 3] {
        let mut buf = [0u8; 3];
        self.0.transfer([CMD_READ_ID], 0, &mut buf);
        buf
    }

    /// Copies bytes from address `addr` to `buf`.
    #[inline]
    pub fn copy_into(&mut self, addr: [u8; 3]) -> [u8; 64] {
        let cmd = [CMD_NOR_READ, addr[0], addr[1], addr[2]];
        let mut buf = [0u8; 64];
        self.0.transfer(cmd, 0, &mut buf);
        buf
    }
}
