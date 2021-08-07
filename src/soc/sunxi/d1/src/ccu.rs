use crate::reg;
use consts::DeviceCtl;
use core::ops;
use model::*;

//use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};
use tock_registers::register_bitfields;
use tock_registers::registers::ReadWrite;

#[repr(C)]
pub struct RegisterBlock {
    _pad0: [u8; 0x090C],
    bgr: ReadWrite<u32, CCU_UART_BGR::Register>, /* clock something */
}

pub struct CCU {
    base: usize,
}

impl ops::Deref for CCU {
    type Target = RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

register_bitfields! [
    u32,
    CCU_UART_BGR [
        UART0_GATING OFFSET(0) NUMBITS(1) [],
        UART1_GATING OFFSET(1) NUMBITS(1) [],
        UART2_GATING OFFSET(2) NUMBITS(1) [],
        UART3_GATING OFFSET(3) NUMBITS(1) [],
        UART4_GATING OFFSET(4) NUMBITS(1) [],
        UART5_GATING OFFSET(5) NUMBITS(1) [],
        UART0_RST OFFSET(16) NUMBITS(1) [],
        UART1_RST OFFSET(17) NUMBITS(1) [],
        UART2_RST OFFSET(18) NUMBITS(1) [],
        UART3_RST OFFSET(19) NUMBITS(1) [],
        UART4_RST OFFSET(20) NUMBITS(1) [],
        UART5_RST OFFSET(21) NUMBITS(1) []
    ]
];

impl CCU {
    pub fn new() -> CCU {
        CCU {
            base: reg::CCU_BASE_ADDR as usize,
        }
    }

    /// Returns a pointer to the register block
    fn ptr(&self) -> *const RegisterBlock {
        self.base as *const _
    }
}

impl Driver for CCU {
    fn init(&mut self) -> Result<()> {
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

impl Default for CCU {
    fn default() -> Self {
        Self::new()
    }
}
