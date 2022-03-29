// D1 user manual
// https://dl.linux-sunxi.org/D1/D1_User_Manual_V0.1_Draft_Version.pdf

// pp 1094
pub const GPIO_BASE_ADDR: u32 = 0x02000000;
pub const GPIO_PB_CFG0: u32 = GPIO_BASE_ADDR + 0x0030;
pub const GPIO_PB_CFG1: u32 = GPIO_BASE_ADDR + 0x0034;
pub const GPIO_PB_DAT: u32 = GPIO_BASE_ADDR + 0x0040;
pub const GPIO_PB_DRV0: u32 = GPIO_BASE_ADDR + 0x0044;
pub const GPIO_PB_DRV1: u32 = GPIO_BASE_ADDR + 0x0048;
pub const GPIO_PB_PULL: u32 = GPIO_BASE_ADDR + 0x0054;

// pp 900
pub const CCU_BASE_ADDR: u32 = 0x02001000;
pub const CCU_UART_BGR: u32 = CCU_BASE_ADDR + 0x090C;
