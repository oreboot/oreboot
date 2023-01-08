pub const DEVICE_PATH_MAX: usize = 40;
pub const BUS_PATH_MAX: usize = DEVICE_PATH_MAX + 10;

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
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
    Mdio,
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
            Self::Mdio => "DEVICE_PATH_MDIO",
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

#[derive(Clone, Copy)]
pub struct MDIOPath {
    pub addr: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
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
    pub mdio: MDIOPath,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DevicePath {
    pub path_type: DevicePathType,
    pub union: DevicePathUnion,
}

impl PartialEq for DevicePath {
    fn eq(&self, oth: &Self) -> bool {
        if self.path_type != oth.path_type {
            return false;
        }

        let (u1, u2) = (&self.union, &oth.union);
        match self.path_type {
            DevicePathType::PathNone => false,
            DevicePathType::Root => true,
            DevicePathType::Pci => unsafe { u1.pci.devfn == u2.pci.devfn },
            DevicePathType::Pnp => unsafe { u1.pnp.port == u2.pnp.port && u1.pnp.device == u2.pnp.device },
            DevicePathType::I2c => unsafe { u1.i2c.device == u2.i2c.device && u1.i2c.mode_10bit == u2.i2c.mode_10bit },
            DevicePathType::Apic => unsafe { u1.apic.apic_id == u2.apic.apic_id },
            DevicePathType::Domain => unsafe { u1.domain.domain == u2.domain.domain },
            DevicePathType::CpuCluster => unsafe { u1.cpu_cluster.cluster == u2.cpu_cluster.cluster },
            DevicePathType::Cpu => unsafe { u1.cpu.id == u2.cpu.id },
            DevicePathType::CpuBus => unsafe { u1.cpu_bus.id == u2.cpu_bus.id },
            DevicePathType::Generic => unsafe { u1.generic.id == u2.generic.id && u1.generic.subid == u2.generic.subid },
            DevicePathType::Spi => unsafe { u1.spi.cs == u2.spi.cs },
            DevicePathType::Usb => unsafe { u1.usb.port_type == u2.usb.port_type && u1.usb.port_id == u2.usb.port_id },
            DevicePathType::Mmio => unsafe { u1.mmio.addr == u2.mmio.addr },
            DevicePathType::Gpio => unsafe { u1.gpio.id == u2.gpio.id },
            DevicePathType::Mdio => unsafe { u1.mdio.addr == u2.mdio.addr },
            _ => unreachable!(),
        }
    }
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
