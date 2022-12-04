use crate::Device;

pub const IORESOURCE_BITS: u32 = 0x0000_00ff; /* Bus-specific bits */

pub const IORESOURCE_IO: u32 = 0x0000_0100; /* Resource type */
pub const IORESOURCE_MEM: u32 = 0x0000_0200;
pub const IORESOURCE_IRQ: u32 = 0x0000_0400;
pub const IORESOURCE_DRQ: u32 = 0x0000_0800;

pub const IORESOURCE_TYPE_MASK: u32 =
    IORESOURCE_IO | IORESOURCE_MEM | IORESOURCE_IRQ | IORESOURCE_DRQ;

pub const IORESOURCE_PREFETCH: u32 = 0x0000_1000; /* No side effects */
pub const IORESOURCE_READONLY: u32 = 0x0000_2000;
pub const IORESOURCE_CACHEABLE: u32 = 0x0000_4000;
pub const IORESOURCE_RANGELENGTH: u32 = 0x0000_8000;
pub const IORESOURCE_SHADOWABLE: u32 = 0x0001_0000;
pub const IORESOURCE_BUS_HAS_VGA: u32 = 0x0002_0000;
/// This resource filters all of the unclaimed transactions to the bus below.
pub const IORESOURCE_SUBTRACTIVE: u32 = 0x0004_0000;
/// The IO resource has a bus below it.
pub const IORESOURCE_BRIDGE: u32 = 0x0008_0000;
/// This is a request to allocate resource about 4G boundary.
pub const IORESOURCE_ABOVE_4G: u32 = 0x0010_0000;
/// The resource needs to be reserved in the coreboot table
pub const IORESOURCE_RESERVE: u32 = 0x1000_0000;
/// The IO resource assignment has been stored in the device
pub const IORESOURCE_STORED: u32 = 0x2000_0000;
/// An IO resource that has been assigned a value
pub const IORESOURCE_ASSIGNED: u32 = 0x4000_0000;
/// An IO resource the allocator must not change
pub const IORESOURCE_FIXED: u32 = 0x8000_0000;

/* PCI specific resource bits (IORESOURCE_BITS) */
pub const IORESOURCE_PCI64: u32 = 1 << 0; /* 64bit long pci resource */
pub const IORESOURCE_PCI_BRIDGE: u32 = 1 << 1; /* A bridge pci resource */
pub const IORESOURCE_PCIE_RESIZABLE_BAR: u32 = 1 << 2; /* A Resizable BAR */

pub type ResourceSearch = fn(&mut dyn ResourceArg, *const Device, *const Resource);

pub trait ResourceArg: Sync {}

pub struct Resource {
    /// Base address of the resource
    pub base: u64,
    /// Size of the resource
    pub size: u64,
    /// Largest valid value base + size - 1
    pub limit: u64,
    /// Next resource in the list
    pub next: *const Resource,
    /// Descriptions of the kind of resource
    pub flags: u32,
    /// Bus specific per device resource id
    pub index: u32,
    /// Required alignment (log 2) of the resource
    pub align: u32,
    /// Granularity (log 2) of the resource
    pub gran: u32,
    /* Alignment must be >= the granularity of the resource */
}
