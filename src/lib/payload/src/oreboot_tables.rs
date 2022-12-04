use alloc::vec::Vec;

pub const OB_GPIO_ACTIVE_LOW: usize = 0;
pub const OB_GPIO_ACTIVE_HIGH: usize = 1;
pub const OB_GPIO_MAX_NAME_LENGTH: usize = 16;
pub const OB_CMOS_MAX_NAME_LENGTH: usize = 32;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ObFramebuffer {
    tag: u32,
    size: u32,
    physical_address: u64,
    x_resolution: u32,
    y_resolution: u32,
    bytes_per_line: u32,
    bits_per_pixel: u8,
    red_mask_pos: u8,
    red_mask_size: u8,
    green_mask_pos: u8,
    green_mask_size: u8,
    blue_mask_pos: u8,
    blue_mask_size: u8,
    reserved_mask_pos: u8,
    reserved_mask_size: u8,
    orientation: u8,
}

impl ObFramebuffer {
    pub const fn new() -> Self {
        Self {
            tag: 0,
            size: 0,
            physical_address: 0,
            x_resolution: 0,
            y_resolution: 0,
            bytes_per_line: 0,
            bits_per_pixel: 0,
            red_mask_pos: 0,
            red_mask_size: 0,
            green_mask_pos: 0,
            green_mask_size: 0,
            blue_mask_pos: 0,
            blue_mask_size: 0,
            reserved_mask_pos: 0,
            reserved_mask_size: 0,
            orientation: 0,
        }
    }
}

/// Memory map windows to translate addresses between SPI flash space and host address space
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FlashMmapWindow {
    flash_base: u32,
    host_base: u32,
    size: u32,
}

impl FlashMmapWindow {
    pub const fn new() -> Self {
        Self {
            flash_base: 0,
            host_base: 0,
            size: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ObGpio {
    port: u32,
    polarity: u32,
    value: u32,
    name: [u8; OB_GPIO_MAX_NAME_LENGTH],
}

impl ObGpio {
    pub const fn new() -> Self {
        Self {
            port: 0,
            polarity: 0,
            value: 0,
            name: [0; OB_GPIO_MAX_NAME_LENGTH],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MacAddress {
    mac_addr: [u8; 6],
    // Pad it to 8 bytes to keep it simple
    pad: [u8; 2],
}

impl MacAddress {
    pub const fn new() -> Self {
        Self {
            mac_addr: [0; 6],
            pad: [0; 2],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ObCmosEntries {
    tag: u32,
    size: u32,
    bit: u32,
    length: u32,
    config: u32,
    config_id: u32,
    name: [u8; OB_CMOS_MAX_NAME_LENGTH],
}

impl ObCmosEntries {
    pub const fn new() -> Self {
        Self {
            tag: 0,
            size: 0,
            bit: 0,
            length: 0,
            config: 0,
            config_id: 0,
            name: [0; OB_CMOS_MAX_NAME_LENGTH],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct LbGpio {
    pub port: u32,
    pub polarity: u32,
    pub value: u32,
    pub name: [u8; OB_GPIO_MAX_NAME_LENGTH],
}

impl LbGpio {
    pub const fn new() -> Self {
        Self {
            port: 0,
            polarity: 0,
            value: 0,
            name: [0; OB_GPIO_MAX_NAME_LENGTH],
        }
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct LbGpios {
    pub tag: u32,
    pub size: u32,
    pub count: u32,
    pub gpios: Vec<LbGpio>,
}

impl LbGpios {
    pub const fn new() -> Self {
        Self {
            tag: 0,
            size: 0,
            count: 0,
            gpios: Vec::new(),
        }
    }
}
