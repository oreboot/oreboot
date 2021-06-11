#![no_std]

use core::fmt;
use core::mem::size_of;
use core::num::Wrapping;

pub use efi::EFI_GUID as GUID;
use fsp_qemu_sys as efi;

#[derive(PartialEq, Eq)]
pub enum FvTraverseError {
    InvalidFvChecksum {
        fv_base: usize,
        checksum: u16,
    },
    InvalidFfsSize {
        ffs_base: usize,
    },
    InvalidFfsHeaderChecksum {
        ffs_base: usize,
        checksum: u8,
    },
    InvalidFfsDataChecksum {
        ffs_base: usize,
        got_checksum: u8,
        want_checksum: u8,
    },
    OutOfBound {
        index: usize,
        limit: usize,
    },
}

use FvTraverseError::*;

impl fmt::Display for FvTraverseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvalidFvChecksum { fv_base, checksum } => write!(
                f,
                "FV@{:#x} has invalid checksum {:#x}, expected 0x0",
                fv_base, checksum
            ),
            InvalidFfsSize { ffs_base } => {
                write!(f, "FFS@{:#x} has invalid extended size", ffs_base)
            }
            InvalidFfsHeaderChecksum { ffs_base, checksum } => write!(
                f,
                "FFS@{:#x} ffs has invalid checksum {:#x}, expected 0x0",
                ffs_base, checksum
            ),
            InvalidFfsDataChecksum {
                ffs_base,
                got_checksum,
                want_checksum,
            } => write!(
                f,
                "FFS@{:#x} ffs has invalid checksum {:#x}, expected {:#x}",
                ffs_base, got_checksum, want_checksum
            ),
            OutOfBound { index, limit } => write!(
                f,
                "Parsing firmare volume resulted in out-of-bounds access ({:#x} > {:#x})",
                index, limit
            ),
        }
    }
}

const EFI_FIRMWARE_FILE_SYSTEM2_GUID: GUID = GUID(
    0x8c8ce578,
    0x8a3d,
    0x4f1c,
    [0x99, 0x35, 0x89, 0x61, 0x85, 0xc3, 0x2d, 0xd3],
);
const EFI_FIRMWARE_FILE_SYSTEM3_GUID: GUID = GUID(
    0x5473c07a,
    0x3dcb,
    0x4dca,
    [0xbd, 0x6f, 0x1e, 0x96, 0x89, 0xe7, 0x34, 0x9a],
);
const EFI_FVH_SIGNATURE: u32 = 0x4856465f; // "FVH_"

#[derive(Debug, PartialEq, Eq)]
pub struct SectionInfo {
    pub fv_base: usize,
    pub ffs_base: usize,
    pub ffs_guid: GUID,
    pub ffs_type: u32,
    pub sec_base: usize,
    pub sec_type: u32,
}

// The data is kept outside the SectionInfo struct to make it easy to debug print the SectionInfo
// without printing thousands of lines of data.
pub type SectionData<'a> = &'a [u8];

// Supports ffs2 and ffs3. All other firmware volumes are skipped.
pub fn fv_traverse<'a, F>(data: &'a [u8], mut visitor: F) -> Result<(), FvTraverseError>
where
    F: FnMut(SectionInfo, SectionData<'a>),
{
    // This procedure is defined in the "Platform Initialization Specification, Vol. 3".
    let mut index = 0;

    // Helper function to check bounds when reading from data.
    let slice_data = |start, end, limit| {
        if start > data.len() {
            return Err(OutOfBound {
                index: start,
                limit: data.len(),
            });
        }
        if end > data.len() {
            return Err(OutOfBound {
                index: end,
                limit: data.len(),
            });
        }
        if start > limit {
            return Err(OutOfBound {
                index: start,
                limit,
            });
        }
        if end > limit {
            return Err(OutOfBound { index: end, limit });
        }
        Ok(&data[start..end])
    };

    while index < data.len() {
        let fv_base = index;
        let fv_bytes = slice_data(
            fv_base,
            fv_base + size_of::<efi::EFI_FIRMWARE_VOLUME_HEADER>(),
            data.len(),
        )?;
        let fv: efi::EFI_FIRMWARE_VOLUME_HEADER =
            unsafe { core::ptr::read(fv_bytes.as_ptr() as *const _) };

        // Check FV header signature.
        if fv.Signature != EFI_FVH_SIGNATURE {
            break;
        }

        // Check FV header checksum.
        let checksum = fv_bytes.chunks_exact(2).fold(0u16, |sum, chunk| {
            let word = (chunk[0] as u16) | ((chunk[1] as u16) << 8);
            (Wrapping(sum) + Wrapping(word)).0
        });
        if checksum != 0 {
            return Err(InvalidFvChecksum { fv_base, checksum });
        }

        // Skip headers.
        let fv_end = index + fv.FvLength as usize;
        if fv.ExtHeaderOffset == 0 {
            index += fv.HeaderLength as usize;
        } else {
            index += fv.ExtHeaderOffset as usize;
            let fveh_bytes = slice_data(
                index,
                index + size_of::<efi::EFI_FIRMWARE_VOLUME_EXT_HEADER>(),
                fv_end,
            )?;
            let fveh: efi::EFI_FIRMWARE_VOLUME_EXT_HEADER =
                unsafe { core::ptr::read(fveh_bytes.as_ptr() as *const _) };
            index += fveh.ExtHeaderSize as usize;
        }

        // Check FV header GUID.
        if fv.FileSystemGuid != EFI_FIRMWARE_FILE_SYSTEM2_GUID
            && fv.FileSystemGuid != EFI_FIRMWARE_FILE_SYSTEM3_GUID
        {
            // Skip to the next FV.
            index = fv_end;
            continue;
        }

        // Iterate through files.
        while {
            index = (index + 7) & !7; // align to 8 bytes
            index < fv_end
        } {
            let ffs_base = index;

            // Check if the header is empty.
            let erase_polarity = if fv.Attributes & efi::EFI_FVB2_ERASE_POLARITY == 0 {
                0x00
            } else {
                0xff
            };
            let ffs_bytes = slice_data(
                ffs_base,
                ffs_base + size_of::<efi::EFI_FFS_FILE_HEADER>(),
                fv_end,
            )?;
            if ffs_bytes.iter().all(|&x| x == erase_polarity) {
                break; // Reached empty space in FV.
            }

            // Determine the file sizes.
            let ffs: efi::EFI_FFS_FILE_HEADER =
                unsafe { core::ptr::read(ffs_bytes.as_ptr() as *const _) };
            let (ffs_header_size, ffs_size) =
                if ffs.Attributes & (efi::FFS_ATTRIB_LARGE_FILE as u8) == 0 {
                    (
                        size_of::<efi::EFI_FFS_FILE_HEADER>(),
                        little_endian3(ffs.Size),
                    )
                } else {
                    if little_endian3(ffs.Size) != 0 {
                        return Err(InvalidFfsSize { ffs_base });
                    }
                    let ffs2_bytes = slice_data(
                        ffs_base,
                        ffs_base + size_of::<efi::EFI_FFS_FILE_HEADER2>(),
                        fv_end,
                    )?;
                    (
                        ffs2_bytes.len(),
                        unsafe {
                            core::ptr::read(ffs2_bytes.as_ptr() as *const efi::EFI_FFS_FILE_HEADER2)
                        }
                        .ExtendedSize as usize,
                    )
                };
            let ffs_end = ffs_base + ffs_size;

            // Check the FFS header checksum.
            let file_checksum = unsafe { ffs.IntegrityCheck.Checksum.File };
            let bytes = slice_data(ffs_base, ffs_base + ffs_header_size, fv_end)?;
            let checksum = (bytes
                .iter()
                .fold(Wrapping(0u8), |sum, &val| sum + Wrapping(val))
                - Wrapping(ffs.State)
                - Wrapping(file_checksum))
            .0;
            if checksum != 0 {
                return Err(InvalidFfsHeaderChecksum { ffs_base, checksum });
            }

            // Check the FFS file checksum.
            if ffs.Attributes & (efi::FFS_ATTRIB_CHECKSUM as u8) == 0 {
                if file_checksum != 0xaa {
                    return Err(InvalidFfsDataChecksum {
                        ffs_base,
                        got_checksum: file_checksum,
                        want_checksum: 0xaa,
                    });
                }
            } else {
                let bytes = slice_data(ffs_base + ffs_header_size, ffs_end, fv_end)?;
                let checksum = bytes
                    .iter()
                    .fold(0u8, |sum, &val| (Wrapping(sum) + Wrapping(val)).0);
                if checksum != file_checksum {
                    return Err(InvalidFfsDataChecksum {
                        ffs_base,
                        got_checksum: checksum,
                        want_checksum: file_checksum,
                    });
                }
            }

            // Only some file types contain sections.
            let filetypes_with_sections = &[
                efi::EFI_FV_FILETYPE_APPLICATION,
                efi::EFI_FV_FILETYPE_COMBINED_MM_DXE,
                efi::EFI_FV_FILETYPE_COMBINED_PEIM_DRIVER,
                efi::EFI_FV_FILETYPE_DRIVER,
                efi::EFI_FV_FILETYPE_DXE_CORE,
                efi::EFI_FV_FILETYPE_FIRMWARE_VOLUME_IMAGE,
                efi::EFI_FV_FILETYPE_FREEFORM,
                efi::EFI_FV_FILETYPE_MM,
                efi::EFI_FV_FILETYPE_MM_CORE,
                efi::EFI_FV_FILETYPE_MM_CORE_STANDALONE,
                efi::EFI_FV_FILETYPE_MM_STANDALONE,
                efi::EFI_FV_FILETYPE_PEIM,
                efi::EFI_FV_FILETYPE_PEI_CORE,
                efi::EFI_FV_FILETYPE_SECURITY_CORE,
            ];
            if filetypes_with_sections
                .iter()
                .all(|&x| x != ffs.Type as u32)
            {
                // Skip to the next file.
                index = ffs_end;
                continue;
            }

            // Skip the FFS header.
            index += ffs_header_size;

            // Iterate through sections.
            while {
                index = (index + 3) & !3; // align to 4 bytes
                index < ffs_end
            } {
                let sec_base = index;
                let sec_common: efi::EFI_COMMON_SECTION_HEADER =
                    unsafe { core::ptr::read(data[sec_base..].as_ptr() as *const _) };

                // Determine section sizes.
                let (sec_header_size, sec_size) = match little_endian3(sec_common.Size) {
                    0xffffff => (
                        size_of::<efi::EFI_COMMON_SECTION_HEADER2>(),
                        unsafe {
                            core::ptr::read(
                                data[sec_base..].as_ptr() as *const efi::EFI_COMMON_SECTION_HEADER2
                            )
                        }
                        .ExtendedSize as usize,
                    ),
                    x => (size_of::<efi::EFI_COMMON_SECTION_HEADER>(), x),
                };
                let sec_end = sec_base + sec_size;

                // Apply visitor.
                visitor(
                    SectionInfo {
                        fv_base,
                        ffs_base,
                        ffs_guid: ffs.Name,
                        ffs_type: ffs.Type as u32,
                        sec_base,
                        sec_type: sec_common.Type as u32,
                    },
                    slice_data(sec_base + sec_header_size, sec_end, sec_end)?,
                );

                // Skip to next section.
                index = sec_end;
            }

            // Skip to next file.
            index = ffs_end;
        }

        // Skip to next FV.
        index = fv_end;
    }
    Ok(())
}

/// Read a 3-byte little endian value.
fn little_endian3(x: [u8; 3]) -> usize {
    ((x[2] as usize) << 16) | ((x[1] as usize) << 8) | (x[0] as usize)
}
