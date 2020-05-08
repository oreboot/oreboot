#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use core::mem;

// Primitive types
pub use efi::ffi::{BOOLEAN, CHAR16, CHAR8, EFI_STATUS, INT16, INT32, INT64, INT8, UINT16, UINT32, UINT64, UINT8, UINTN};

// EFI_STATUS values
pub use efi::ffi::{
    EFI_ABORTED, EFI_CRC_ERROR, EFI_DEVICE_ERROR, EFI_INCOMPATIBLE_VERSION, EFI_INVALID_PARAMETER, EFI_NOT_FOUND, EFI_NOT_READY, EFI_OUT_OF_RESOURCES, EFI_SECURITY_VIOLATION, EFI_SUCCESS, EFI_TIMEOUT, EFI_UNSUPPORTED, EFI_VOLUME_CORRUPTED,
};

// Other types
pub use efi::ffi::{boot_services::EFI_MEMORY_TYPE, EFI_GUID, EFI_PHYSICAL_ADDRESS};

pub const FSP_FFS_INFORMATION_FILE_GUID: EFI_GUID = EFI_GUID(0x912740be, 0x2284, 0x4734, [0xb9, 0x71, 0x84, 0xb0, 0x27, 0x35, 0x3f, 0x0c]);
pub const FSP_RESERVED_MEMORY_RESOURCE_HOB_GUID: EFI_GUID = EFI_GUID(0x69a79759, 0x1373, 0x4367, [0xa6, 0xc4, 0xc7, 0xf5, 0x9e, 0xfd, 0x98, 0x6e]);
pub const FSP_NON_VOLATILE_STORAGE_HOB_GUID: EFI_GUID = EFI_GUID(0x721acf02, 0x4d77, 0x4c2a, [0xb3, 0xdc, 0x27, 0xb, 0x7b, 0xa9, 0xe4, 0xb0]);
pub const FSP_BOOTLOADER_TOLUM_HOB_GUID: EFI_GUID = EFI_GUID(0x73ff4f56, 0xaa8e, 0x4451, [0xb3, 0x16, 0x36, 0x35, 0x36, 0x67, 0xad, 0x44]);
pub const EFI_PEI_GRAPHICS_INFO_HOB_GUID: EFI_GUID = EFI_GUID(0x39f62cce, 0x6825, 0x4669, [0xbb, 0x56, 0x54, 0x1a, 0xba, 0x75, 0x3a, 0x07]);

#[repr(C)]
pub struct FSP_PATCH_TABLE {
    pub Signature: UINT32,
    pub Length: UINT16,
    pub Revision: UINT8,
    pub Reserved: UINT8,
    pub PatchEntryNum: UINT32,
}

bitfield!{
    pub struct PatchData(u32);
    pub Offset, _: 23, 0;
    pub Type, _: 27, 24;
    pub Reserved, _: 30, 28;
    pub Calculated, _: 31, 31;
}

#[repr(C)]
pub struct FSP_UPD_HEADER{
    pub Signature: UINT64,
    pub Revision: UINT8,
    pub Reserved: [UINT8; 23],
}

#[repr(C)]
pub struct FSPT_UPD{
    pub UpdHeader: FSP_UPD_HEADER,
    // Platforms specific parameters...
}

#[repr(C)]
pub struct FSPM_UPD{
    pub UpdHeader: FSP_UPD_HEADER,
    pub FspmArchUpd: FSPM_ARCH_UPD,
    // Platforms specific parameters...
}

#[repr(C)]
pub struct FSPM_ARCH_UPD {
    pub Revision: UINT8,
    pub Reserved: [UINT8; 3],
    pub NvsBufferPtr: core::ffi::c_void,
    pub StackBase: core::ffi::c_void,
    pub StackSize: UINT32,
    pub BootLoaderTolumSize: UINT32,
    pub BootMode: UINT32,
    pub Reserved1: [UINT8; 8],
}

#[repr(C)]
pub struct FSPS_UPD{
    pub UpdHeader: FSP_UPD_HEADER,
    // Platforms specific parameters...
}

pub type FSP_TEMP_RAM_INIT = unsafe extern "efiapi" fn(FsptUpdDataPtr: *const FSPT_UPD) -> EFI_STATUS;
pub type FSP_MEMORY_INIT = unsafe extern "efiapi" fn(FspmUpdDataPtr: *const FSPM_UPD, FspmUpdDataPtr: *mut *mut EFI_HOB_GENERIC_HEADER) -> EFI_STATUS;
pub type FSP_TEMP_RAM_EXIT = unsafe extern "efiapi" fn() -> EFI_STATUS;
pub type FSP_SILICON_INIT = unsafe extern "efiapi" fn(FspsUpdDataPtr: *const FSPS_UPD) -> EFI_STATUS;
pub type FSP_NOTIFY_PHASE_PARAMS = unsafe extern "efiapi" fn(NotifyPhaseParamPtr: *const NOTIFY_PHASE_PARAMS) -> EFI_STATUS;

#[repr(C)]
pub struct NOTIFY_PHASE_PARAMS {
    pub Phase: FSP_INIT_PHASE,
}

pub enum FSP_INIT_PHASE {
    InitPhaseAfterPciEnumeration = 0x20,
    InitPhaseReadyToBoot = 0x40,
}

#[repr(u32)]
pub enum EFI_BOOT_MODE {
    BOOT_WITH_FULL_CONFIGURATION = 0x00,
    BOOT_WITH_MINIMAL_CONFIGURATION = 0x01,
    BOOT_ASSUMING_NO_CONFIGURATION_CHANGES = 0x02,
    BOOT_ON_S4_RESUME = 0x05,
    BOOT_ON_S3_RESUME = 0x11,
    BOOT_ON_FLASH_UPDATE = 0x12,
    BOOT_IN_RECOVERY_MODE = 0x20,
}

macro_rules! with_oem_bit_set {
    ($num:expr) => {
        (1 << ((mem::size_of::<UINTN>() * 8) - 2)) | $num
    };
}

pub const FSP_STATUS_RESET_REQUIRED_COLD: UINTN = with_oem_bit_set!(1);
pub const FSP_STATUS_RESET_REQUIRED_WARM: UINTN = with_oem_bit_set!(2);
pub const FSP_STATUS_RESET_REQUIRED_3: UINTN = with_oem_bit_set!(3);
pub const FSP_STATUS_RESET_REQUIRED_4: UINTN = with_oem_bit_set!(4);
pub const FSP_STATUS_RESET_REQUIRED_5: UINTN = with_oem_bit_set!(5);
pub const FSP_STATUS_RESET_REQUIRED_6: UINTN = with_oem_bit_set!(6);
pub const FSP_STATUS_RESET_REQUIRED_7: UINTN = with_oem_bit_set!(7);
pub const FSP_STATUS_RESET_REQUIRED_8: UINTN = with_oem_bit_set!(8);

#[repr(C)]
pub struct EFI_PEI_GRAPHICS_INFO_HOB {
    pub FrameBufferBase: EFI_PHYSICAL_ADDRESS,
    pub FrameBufferSize: UINT32,
    pub GraphicsMode: EFI_GRAPHICS_OUTPUT_MODE_INFORMATION,
}

#[repr(C)]
pub struct EFI_GRAPHICS_OUTPUT_MODE_INFORMATION {
    pub Version: UINT32,
    pub HorizontalResolution: UINT32,
    pub VerticalResolution: UINT32,
    pub PixelFormat: EFI_GRAPHICS_PIXEL_FORMAT,
    pub PixelInformation: EFI_PIXEL_BITMASK,
    pub PixelsPerScanLine: UINT32,
}

#[repr(C)]
pub enum EFI_GRAPHICS_PIXEL_FORMAT {
    PixelRedGreenBlueReserved8BitPerColor,
    PixelBlueGreenRedReserved8BitPerColor,
    PixelBitMask,
    PixelBitOnly,
    PixelFormatMask,
}

#[repr(C)]
pub struct EFI_PIXEL_BITMASK {
    pub RedMask: UINT32,
    pub GreenMask: UINT32,
    pub BlueMask: UINT32,
    pub ReservedMask: UINT32,
}

#[repr(u32)]
pub enum EFI_RESOURCE_TYPE {
    EFI_RESOURCE_SYSTEM_MEMORY = 0,
    EFI_RESOURCE_MEMORY_MAPPED_IO = 1,
    EFI_RESOURCE_IO = 2,
    EFI_RESOURCE_FIRMWARE_DEVICE = 3,
    EFI_RESOURCE_MEMORY_MAPPED_IO_PORT = 4,
    EFI_RESOURCE_MEMORY_RESERVED = 5,
    EFI_RESOURCE_IO_RESERVED = 6,
    EFI_RESOURCE_MAX_MEMORY_TYPE = 7,
}

bitflags! {
    pub struct EFI_RESOURCE_ATTRIBUTE_TYPE: UINT32 {
        const EFI_RESOURCE_ATTRIBUTE_PRESENT = 0x00000001;
        const EFI_RESOURCE_ATTRIBUTE_INITIALIZED = 0x00000002;
        const EFI_RESOURCE_ATTRIBUTE_TESTED = 0x00000004;
        const EFI_RESOURCE_ATTRIBUTE_SINGLE_BIT_ECC = 0x00000008;
        const EFI_RESOURCE_ATTRIBUTE_MULTIPLE_BIT_ECC = 0x00000010;
        const EFI_RESOURCE_ATTRIBUTE_ECC_RESERVED_1 = 0x00000020;
        const EFI_RESOURCE_ATTRIBUTE_ECC_RESERVED_2 = 0x00000040;
        const EFI_RESOURCE_ATTRIBUTE_READ_PROTECTED = 0x00000080;
        const EFI_RESOURCE_ATTRIBUTE_WRITE_PROTECTED = 0x00000100;
        const EFI_RESOURCE_ATTRIBUTE_EXECUTION_PROTECTED = 0x00000200;
        const EFI_RESOURCE_ATTRIBUTE_UNCACHEABLE = 0x00000400;
        const EFI_RESOURCE_ATTRIBUTE_WRITE_COMBINEABLE = 0x00000800;
        const EFI_RESOURCE_ATTRIBUTE_WRITE_THROUGH_CACHEABLE = 0x00001000;
        const EFI_RESOURCE_ATTRIBUTE_WRITE_BACK_CACHEABLE = 0x00002000;
        const EFI_RESOURCE_ATTRIBUTE_16_BIT_IO = 0x00004000;
        const EFI_RESOURCE_ATTRIBUTE_32_BIT_IO = 0x00008000;
        const EFI_RESOURCE_ATTRIBUTE_64_BIT_IO = 0x00010000;
        const EFI_RESOURCE_ATTRIBUTE_UNCACHED_EXPORTED = 0x00020000;
        const EFI_RESOURCE_ATTRIBUTE_READ_ONLY_PROTECTED = 0x00040000;
        const EFI_RESOURCE_ATTRIBUTE_READ_PROTECTABLE = 0x00100000;
        const EFI_RESOURCE_ATTRIBUTE_WRITE_PROTECTABLE = 0x00200000;
        const EFI_RESOURCE_ATTRIBUTE_EXECUTION_PROTECTABLE = 0x00400000;
        const EFI_RESOURCE_ATTRIBUTE_READ_ONLY_PROTECTABLE = 0x00800000;
        const EFI_RESOURCE_ATTRIBUTE_PERSISTABLE = 0x01000000;
        const EFI_RESOURCE_ATTRIBUTE_MORE_RELIABLE = 0x02000000;
    }
}

#[repr(u16)]
pub enum EFI_HOB_TYPE {
    EFI_HOB_TYPE_MEMORY_ALLOCATION = 0x0002,
    EFI_HOB_TYPE_RESOURCE_DESCRIPTOR = 0x0003,
    EFI_HOB_TYPE_GUID_EXTENSION = 0x0004,
    EFI_HOB_TYPE_UNUSED = 0xFFFE,
    EFI_HOB_TYPE_END_OF_HOB_LIST = 0xFFFF,
}

#[repr(C)]
pub struct EFI_HOB_GENERIC_HEADER {
    pub HobType: UINT16,
    pub HobLength: UINT16,
    pub Reserved: UINT32,
}

#[repr(C)]
pub struct EFI_HOB_MEMORY_ALLOCATION_HEADER {
    pub Name: EFI_GUID,
    pub MemoryBaseAddress: EFI_PHYSICAL_ADDRESS,
    pub MemoryLength: UINT64,
    pub MemoryType: EFI_MEMORY_TYPE,
    pub Reserved: [UINT8; 4],
}

#[repr(C)]
pub struct EFI_HOB_MEMORY_ALLOCATION {
    pub Header: EFI_HOB_GENERIC_HEADER,
    pub AllocDescriptor: EFI_HOB_MEMORY_ALLOCATION_HEADER,
}

#[repr(C)]
pub struct EFI_HOB_RESOURCE_DESCRIPTOR {
    pub Header: EFI_HOB_GENERIC_HEADER,
    pub Owner: EFI_GUID,
    pub ResourceType: EFI_RESOURCE_TYPE,
    pub ResourceAttribute: EFI_RESOURCE_ATTRIBUTE_TYPE,
    pub PhysicalStart: EFI_PHYSICAL_ADDRESS,
    pub ResourceLength: UINT64,
}

#[repr(C)]
pub struct EFI_HOB_GUID_TYPE {
    pub Header: EFI_HOB_GENERIC_HEADER,
    pub Name: EFI_GUID,
}

#[repr(C)]
pub struct EFI_PEI_HOB_POINTERS {
    pub Header: *const EFI_HOB_GENERIC_HEADER,
    pub MemoryAllocation: *const EFI_HOB_MEMORY_ALLOCATION,
    pub ResourceDescriptor: *const EFI_HOB_RESOURCE_DESCRIPTOR,
    pub Guid: *const EFI_HOB_GUID_TYPE,
    pub Raw: *const UINT8,
}

type EFI_FVB_ATTRIBUTES_2 = UINT32;

#[repr(C)]
pub struct EFI_FV_BLOCK_MAP_ENTRY {
    pub NumBlocks: UINT32,
    pub Length: UINT32,
}

#[repr(C)]
pub struct EFI_FIRMWARE_VOLUME_HEADER {
    pub ZeroVector: [UINT8; 16],
    pub FileSystemGuid: EFI_GUID,
    pub FvLength: UINT64,
    pub Signature: UINT32,
    pub Attributes: EFI_FVB_ATTRIBUTES_2,
    pub HeaderLength: UINT16,
    pub Checksum: UINT16,
    pub ExtHeaderOffset: UINT16,
    pub Reserved: UINT8,
    pub Revision: UINT8,
    pub BlockMap: EFI_FV_BLOCK_MAP_ENTRY,
}

pub const EFI_FVH_SIGNATURE: [UINT8; 4] = [b'_', b'F', b'V', b'H'];

pub const EFI_FVH_REVISION: UINT8 = 0x02;

#[repr(C)]
pub struct EFI_FIRMWARE_VOLUME_EXT_HEADER {
    pub FvName: EFI_GUID,
    pub ExtHeaderSize: UINT32,
}

#[repr(C)]
pub struct EFI_FIRMWARE_VOLUME_EXT_ENTRY {
    pub ExtEntrySize: UINT16,
    pub ExtEntryType: UINT16,
}

#[repr(u16)]
pub enum EFI_FV_EXT_TYPE {
    EFI_FV_EXT_TYPE_OEM_TYPE = 1,
    EFI_FV_EXT_TYPE_GUID_TYPE = 2,
}

#[repr(C)]
pub struct EFI_FIRMWARE_VOLUME_EXT_ENTRY_OEM_TYPE {
    pub Hdr: EFI_FIRMWARE_VOLUME_EXT_ENTRY,
    pub TypeMask: UINT32,
}

#[repr(C)]
pub struct EFI_FIRMWARE_VOLUME_EXT_ENTRY_GUID_TYPE {
    pub Hdr: EFI_FIRMWARE_VOLUME_EXT_ENTRY,
    pub FormatType: EFI_GUID,
}

#[repr(C)]
pub struct EFI_FFS_INTEGRITY_CHECK {
    pub Checksum16: UINT16,
}

pub const FFS_FIXED_CHECKSUM: UINT16 = 0xAA;

#[repr(u8)]
pub enum EFI_FV_FILETYPE {
    EFI_FV_FILETYPE_FREEFORM = 0x02,
}

bitflags! {
    pub struct EFI_FFS_FILE_ATTRIBUTES: UINT8 {
        const FFS_ATTRIB_LARGE_FILE = 0x01;
        const FFS_ATTRIB_FIXED = 0x04;
        const FFS_ATTRIB_DATA_ALIGNMENT = 0x38;
        const FFS_ATTRIB_CHECKSUM = 0x40;
    }
}

bitflags! {
    pub struct EFI_FFS_FILE_STATE: UINT8 {
        const EFI_FILE_HEADER_CONSTRUCTION = 0x01;
        const EFI_FILE_HEADER_VALID = 0x02;
        const EFI_FILE_DATA_VALID = 0x04;
        const EFI_FILE_MARKED_FOR_UPDATE = 0x08;
        const EFI_FILE_DELETED = 0x10;
        const EFI_FILE_HEADER_INVALID = 0x20;
    }
}

#[repr(C)]
pub struct EFI_FFS_FILE_HEADER {
    pub Name: EFI_GUID,
    pub IntegrityCheck: EFI_FFS_INTEGRITY_CHECK,
    pub Type: EFI_FV_FILETYPE,
    pub Attributes: EFI_FFS_FILE_ATTRIBUTES,
    pub Size: [UINT8; 3],
    pub State: EFI_FFS_FILE_STATE,
}

impl EFI_FFS_FILE_HEADER {
    pub fn is_file2(self: &EFI_FFS_FILE_HEADER) -> bool {
        self.Attributes.contains(EFI_FFS_FILE_ATTRIBUTES::FFS_ATTRIB_LARGE_FILE)
    }

    pub fn size(self: &EFI_FFS_FILE_HEADER) -> u32 {
        ((self.Size[2] as u32) << 16) | ((self.Size[1] as u32) << 8) | (self.Size[0] as u32)
    }
}

#[repr(C)]
pub struct EFI_FFS_FILE_HEADER2 {
    pub Name: EFI_GUID,
    pub IntegrityCheck: EFI_FFS_INTEGRITY_CHECK,
    pub Type: EFI_FV_FILETYPE,
    pub Attributes: EFI_FFS_FILE_ATTRIBUTES,
    pub Size: [UINT8; 3],
    pub State: EFI_FFS_FILE_STATE,
    pub ExtendedSize: UINT32,
}

impl EFI_FFS_FILE_HEADER2 {
    pub fn size(self: &EFI_FFS_FILE_HEADER2) -> u32 {
        self.ExtendedSize
    }
}

#[repr(u8)]
pub enum EFI_SECTION_TYPE {
    EFI_SECTION_RAW = 0x19,
}

#[repr(C)]
pub struct EFI_COMMON_SECTION_HEADER {
    pub Size: [UINT8; 3],
    pub Type: EFI_SECTION_TYPE,
}

impl EFI_COMMON_SECTION_HEADER {
    pub fn is_section2(self: &EFI_COMMON_SECTION_HEADER) -> bool {
        self.size() == 0xffffff
    }

    pub fn size(self: &EFI_COMMON_SECTION_HEADER) -> u32 {
        ((self.Size[2] as u32) << 16) | ((self.Size[1] as u32) << 8) | (self.Size[0] as u32)
    }
}

#[repr(C)]
pub struct EFI_COMMON_SECTION_HEADER2 {
    pub Size: [UINT8; 3],
    pub Type: EFI_SECTION_TYPE,
    pub ExtendedSize: UINT32,
}

impl EFI_COMMON_SECTION_HEADER2 {
    pub fn size(self: &EFI_COMMON_SECTION_HEADER2) -> u32 {
        self.ExtendedSize
    }
}
