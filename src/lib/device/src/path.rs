pub const DEVICE_PATH_MAX: usize = 40;
pub const BUS_PATH_MAX: usize = DEVICE_PATH_MAX + 10;

pub enum DevicePathType {
    PathNone = 0,
    Root,
    Pci,
    Pnp,
    I2c,
    Apic,
    Domain,
    CpuCluster,
    Cpu,
    CpuBus,
    Ioapic,
    Generic,
    Spi,
    Usb,
    Mmio,
    Gpio,
    /*
     * When adding path types to this table, please also update the
     * DEVICE_PATH_NAMES macro below.
     */
}

impl DevicePathType {
    pub fn to_string(&self) -> &str {
        match self {
            Self::PathNone => "DEVICE_PATH_NONE",
            Self::Root => "DEVICE_PATH_ROOT",
            Self::Pci => "DEVICE_PATH_PCI",
            Self::Pnp => "DEVICE_PATH_PNP",
            Self::I2c => "DEVICE_PATH_I2C",
            Self::Apic => "DEVICE_PATH_APIC",
            Self::Domain => "DEVICE_PATH_DOMAIN",
            Self::CpuCluster => "DEVICE_PATH_CPU_CLUSTER",
            Self::Cpu => "DEVICE_PATH_CPU",
            Self::CpuBus => "DEVICE_PATH_CPU_BUS",
            Self::Ioapic => "DEVICE_PATH_IOAPIC",
            Self::Generic => "DEVICE_PATH_GENERIC",
            Self::Spi => "DEVICE_PATH_SPI",
            Self::Usb => "DEVICE_PATH_USB",
            Self::Mmio => "DEVICE_PATH_MMIO",
            Self::Gpio => "DEVICE_PATH_GPIO",
        }
    }
}

#[derive(Clone, Copy)]
pub struct DomainPath {
    pub domain: u32,
}

#[derive(Clone, Copy)]
pub struct PCIPath {
    pub devfn: u32,
}

#[derive(Clone, Copy)]
pub struct PNPPath {
    pub port: u32,
    pub device: u32,
}

#[derive(Clone, Copy)]
pub struct I2CPath {
    pub device: u32,
    pub mode_10bit: u32,
}

#[derive(Clone, Copy)]
pub struct SPIPath {
    pub cs: u32,
}

#[derive(Clone, Copy)]
pub struct APICPath {
    pub apic_id: u32,
    pub package_id: u32,
    pub node_id: u32,
    pub core_id: u32,
    pub thread_id: u32,
}

#[derive(Clone, Copy)]
pub struct IOAPICPath {
    pub ioapic_id: u32,
}

#[derive(Clone, Copy)]
pub struct CPUClusterPath {
    pub cluster: u32,
}

#[derive(Clone, Copy)]
pub struct CPUPath {
    pub id: u32,
}

#[derive(Clone, Copy)]
pub struct CPUBusPath {
    pub id: u32,
}

#[derive(Clone, Copy)]
pub struct GenericPath {
    pub id: u32,
    pub subid: u32,
}

#[derive(Clone, Copy)]
pub struct USBPath {
    pub port_type: u32,
    pub port_id: u32,
}

#[derive(Clone, Copy)]
pub struct MMIOPath {
    pub addr: usize,
}

#[derive(Clone, Copy)]
pub struct GPIOPath {
    pub id: u32,
}

pub union DevicePathUnion {
    pub pci: PCIPath,
    pub pnp: PNPPath,
    pub i2c: I2CPath,
    pub apic: APICPath,
    pub ioapic: IOAPICPath,
    pub domain: DomainPath,
    pub cpu_cluster: CPUClusterPath,
    pub cpu: CPUPath,
    pub cpu_bus: CPUBusPath,
    pub generic: GenericPath,
    pub spi: SPIPath,
    pub usb: USBPath,
    pub mmio: MMIOPath,
    pub gpio: GPIOPath,
}

pub struct DevicePath {
    pub path_type: DevicePathType,
    pub union: DevicePathUnion,
}

impl DevicePath {
    pub const fn new() -> Self {
        Self {
            path_type: DevicePathType::PathNone,
            union: DevicePathUnion {
                pci: PCIPath { devfn: 0 },
            },
        }
    }
}
