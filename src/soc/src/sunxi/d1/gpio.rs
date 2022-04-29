use super::reg;
use consts::DeviceCtl;
use core::ops;
use oreboot_drivers::{Driver, Result, NOT_IMPLEMENTED};

use tock_registers::interfaces::ReadWriteable;
use tock_registers::register_bitfields;
use tock_registers::registers::ReadWrite;

#[repr(C)]
pub struct RegisterBlock {
    _pad0: [u32; 12],
    /* Port B Config Register 0 */
    pbcfg0: ReadWrite<u32, GPIO_PB_CFG0::Register>,
    /* PB Config Register 1 */
    pbcfg1: ReadWrite<u32, GPIO_PB_CFG1::Register>,
    _pad1: [u32; 2],
    /* PB Data Register */
    pbdat: ReadWrite<u32, GPIO_PB_DAT::Register>,
    /* PB Drive Register 0 */
    pbdrv0: ReadWrite<u32, GPIO_PB_DRV0::Register>,
    /* PB Drive Register 1 */
    pbdrv1: ReadWrite<u32, GPIO_PB_DRV1::Register>,
    _pad2: [u32; 2],
    /* PB Pull config */
    pbpull: ReadWrite<u32, GPIO_PB_PULL::Register>,
    _pad3: [u32; 2],
    /* Port C Config Register 0 */
    pccfg0: ReadWrite<u32, GPIO_PC_CFG0::Register>,
    _pad4: [u32; 3],
    /* PC Data Register */
    pcdat: ReadWrite<u32, GPIO_PC_DAT::Register>,
}

pub struct GPIO {
    base: usize,
}

impl ops::Deref for GPIO {
    type Target = RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

register_bitfields! [
    u32,
    GPIO_PB_CFG0 [
        PB0_SELECT OFFSET(0) NUMBITS(4) [],
        PB1_SELECT OFFSET(4) NUMBITS(4) [],
        PB2_SELECT OFFSET(8) NUMBITS(4) [],
        PB3_SELECT OFFSET(12) NUMBITS(4) [],
        PB4_SELECT OFFSET(16) NUMBITS(4) [],
        PB5_SELECT OFFSET(20) NUMBITS(4) [],
        PB6_SELECT OFFSET(24) NUMBITS(4) [],
        PB7_SELECT OFFSET(28) NUMBITS(4) []
    ],
    GPIO_PB_CFG1 [
        PB8_SELECT OFFSET(0) NUMBITS(4) [],
        PB9_SELECT OFFSET(4) NUMBITS(4) [],
        PB10_SELECT OFFSET(8) NUMBITS(4) [],
        PB11_SELECT OFFSET(12) NUMBITS(4) [],
        PB12_SELECT OFFSET(16) NUMBITS(4) []
        // last 12 bits are reserved
    ],
    GPIO_PB_DAT [
        PB0_DAT OFFSET(0) NUMBITS(1) [],
        PB1_DAT OFFSET(1) NUMBITS(1) [],
        PB2_DAT OFFSET(2) NUMBITS(1) [],
        PB3_DAT OFFSET(3) NUMBITS(1) [],
        PB4_DAT OFFSET(4) NUMBITS(1) [],
        PB5_DAT OFFSET(5) NUMBITS(1) [],
        PB6_DAT OFFSET(6) NUMBITS(1) [],
        PB7_DAT OFFSET(7) NUMBITS(1) [],
        PB8_DAT OFFSET(8) NUMBITS(1) [],
        PB9_DAT OFFSET(9) NUMBITS(1) [],
        PB10_DAT OFFSET(10) NUMBITS(1) [],
        PB11_DAT OFFSET(11) NUMBITS(1) [],
        PB12_DAT OFFSET(12) NUMBITS(1) []
    ],
    GPIO_PB_DRV0 [
        // respective following 2 bits are reserved
        PB0_DRV OFFSET(0) NUMBITS(2) [],
        PB1_DRV OFFSET(4) NUMBITS(2) [],
        PB2_DRV OFFSET(8) NUMBITS(2) [],
        PB3_DRV OFFSET(12) NUMBITS(2) [],
        PB4_DRV OFFSET(16) NUMBITS(2) [],
        PB5_DRV OFFSET(20) NUMBITS(2) [],
        PB6_DRV OFFSET(24) NUMBITS(2) [],
        PB7_DRV OFFSET(28) NUMBITS(2) []
    ],
    GPIO_PB_DRV1 [
        // respective following 2 bits are reserved
        PB8_DRV OFFSET(0) NUMBITS(2) [],
        PB9_DRV OFFSET(4) NUMBITS(2) [],
        PB10_DRV OFFSET(8) NUMBITS(2) [],
        PB11_DRV OFFSET(12) NUMBITS(2) [],
        PB12_DRV OFFSET(16) NUMBITS(2) []
    ],
    GPIO_PB_PULL [
        PB0_PULL OFFSET(0) NUMBITS(2) [],
        PB1_PULL OFFSET(2) NUMBITS(2) [],
        PB2_PULL OFFSET(4) NUMBITS(2) [],
        PB3_PULL OFFSET(6) NUMBITS(2) [],
        PB4_PULL OFFSET(8) NUMBITS(2) [],
        PB5_PULL OFFSET(10) NUMBITS(2) [],
        PB6_PULL OFFSET(12) NUMBITS(2) [],
        PB7_PULL OFFSET(14) NUMBITS(2) [],
        PB8_PULL OFFSET(16) NUMBITS(2) [],
        PB9_PULL OFFSET(18) NUMBITS(2) [],
        PB10_PULL OFFSET(20) NUMBITS(2) [],
        PB11_PULL OFFSET(22) NUMBITS(2) [],
        PB12_PULL OFFSET(24) NUMBITS(2) []
        // 26-31: reserved
    ],
    GPIO_PC_CFG0 [
        PC0_SELECT OFFSET(0) NUMBITS(4) [],
        PC1_SELECT OFFSET(4) NUMBITS(4) [],
        PC2_SELECT OFFSET(8) NUMBITS(4) [],
        PC3_SELECT OFFSET(12) NUMBITS(4) [],
        PC4_SELECT OFFSET(16) NUMBITS(4) [],
        PC5_SELECT OFFSET(20) NUMBITS(4) [],
        PC6_SELECT OFFSET(24) NUMBITS(4) [],
        PC7_SELECT OFFSET(28) NUMBITS(4) []
    ],
    GPIO_PC_DAT [
        PC0_DAT OFFSET(0) NUMBITS(1) [],
        PC1_DAT OFFSET(1) NUMBITS(1) [],
        PC2_DAT OFFSET(2) NUMBITS(1) [],
        PC3_DAT OFFSET(3) NUMBITS(1) [],
        PC4_DAT OFFSET(4) NUMBITS(1) [],
        PC5_DAT OFFSET(5) NUMBITS(1) [],
        PC6_DAT OFFSET(6) NUMBITS(1) [],
        PC7_DAT OFFSET(7) NUMBITS(1) [],
    ],
];

impl GPIO {
    pub fn new() -> GPIO {
        GPIO {
            base: reg::GPIO_BASE_ADDR as usize,
        }
    }

    /// Returns a pointer to the register block
    fn ptr(&self) -> *const RegisterBlock {
        self.base as *const _
    }
}

impl Driver for GPIO {
    fn init(&mut self) -> Result<()> {
        // set port B GPIO5 high
        self.pbcfg0.modify(GPIO_PB_CFG0::PB5_SELECT.val(1)); // output / LED
        self.pbdat.modify(GPIO_PB_DAT::PB5_DAT.val(1)); // high

        // set port C GPIO1 high
        self.pccfg0.modify(GPIO_PC_CFG0::PC1_SELECT.val(1)); // output / LED
        self.pcdat.modify(GPIO_PC_DAT::PC1_DAT.val(1)); // high

        // Config GPIOB8 and GPIOB9 to txd0 and rxd0
        self.pbcfg1.modify(GPIO_PB_CFG1::PB8_SELECT.val(6)); // 0110: UART0 TX
        self.pbcfg1.modify(GPIO_PB_CFG1::PB9_SELECT.val(6)); // 0110: UART0 RX

        // enable pull-ups
        self.pbpull.modify(GPIO_PB_PULL::PB8_PULL.val(1)); // 01: pull-up
        self.pbpull.modify(GPIO_PB_PULL::PB9_PULL.val(1)); // 01: pull-up
        Ok(())
    }

    fn pread(&self, data: &mut [u8], _offset: usize) -> Result<usize> {
        Ok(data.len())
    }

    fn pwrite(&mut self, data: &[u8], _offset: usize) -> Result<usize> {
        /*
        'outer: for (sent_count, &c) in data.iter().enumerate() {
            return Ok(sent_count);
        }
        */
        Ok(data.len())
    }

    fn ctl(&mut self, __d: DeviceCtl) -> Result<usize> {
        NOT_IMPLEMENTED
    }

    fn stat(&self, _data: &mut [u8]) -> Result<usize> {
        NOT_IMPLEMENTED
    }

    fn shutdown(&mut self) {}
}

impl Default for GPIO {
    fn default() -> Self {
        Self::new()
    }
}
