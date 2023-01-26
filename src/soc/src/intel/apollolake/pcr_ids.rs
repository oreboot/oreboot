#[cfg(feature = "geminilake")]
pub const PID_GPIO_AUDIO: u16 = 0xc9;
#[cfg(feature = "geminilake")]
pub const PID_GPIO_SCC: u16 = 0xc8;

#[cfg(not(feature = "geminilake"))]
pub const PID_GPIO_SW: u16 = 0xc0;
#[cfg(not(feature = "geminilake"))]
pub const PID_GPIO_S: u16 = 0xc2;
#[cfg(not(feature = "geminilake"))]
pub const PID_GPIO_W: u16 = 0xc7;

pub const PID_GPIO_NW: u16 = 0xc4;
pub const PID_GPIO_N: u16 = 0xc5;

pub const PID_ITSS: u16 = 0xd0;
pub const PID_RTC: u16 = 0xd1;
pub const PID_LPC: u16 = 0xd2;
pub const PID_MODPHY: u16 = 0xa5;

pub const PID_AUNIT: u16 = 0x4d;
pub const PID_BUNIT: u16 = 0x4c;
pub const PID_TUNIT: u16 = 0x52;

pub const PID_PSF3: u16 = 0xc6;
pub const PID_DMI: u16 = 0x00; /* Reserved */
pub const PID_CSME0: u16 = 0x9a; /* Reserved */
