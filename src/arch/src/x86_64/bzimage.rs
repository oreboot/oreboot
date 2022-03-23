/* SPDX-License-Identifier: GPL-2.0-only */

use super::PAGE_SIZE;
use core::intrinsics::copy;
use core::mem::{size_of, transmute};
use core::ptr;
use oreboot_drivers::wrappers::{Memory, SectionReader};
use oreboot_drivers::Driver;
use oreboot_drivers::Result;
pub type EntryPoint = unsafe extern "C" fn(r0: usize, dtb: usize);

#[repr(u32)]
pub enum E820 {
    RAM = 1,
    RESERVED = 2,
    ACPI = 3,
    NVS = 4,
    UNUSABLE = 5,
    MAX = 128,
}

#[repr(C, packed)]
#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub struct E820Entry {
    addr: u64,
    size: u64,
    r#type: u32,
}

#[repr(C, packed)]
pub struct BootParams {
    screen_info: [u8; 0x040],             // 0x000
    apm_bios_info: [u8; 0x054 - 0x040],   // 0x040
    _pad2: [u8; 4],                       // 0x054
    tboot_addr: u64,                      // 0x058
    ist_info: [u8; 0x070 - 0x060],        // 0x060
    acpi_rsdp_addr: [u8; 0x078 - 0x070],  // 0x070
    _pad3: [u8; 8],                       // 0x078
    hd0_info: [u8; 0x090 - 0x080],        // 0x080 /* obsolete! */
    hd1_info: [u8; 0x0a0 - 0x090],        // 0x090 /* obsolete! */
    sys_desc_table: [u8; 0x0b0 - 0x0a0],  // 0x0a0 /* obsolete! */
    olpc_ofw_header: [u8; 0x0c0 - 0x0b0], // 0x0b0
    ext_ramdisk_image: u32,               // 0x0c0
    ext_ramdisk_size: u32,                // 0x0c4
    ext_cmd_line_ptr: u32,                // 0x0c8
    _pad4: [u8; 116],                     // 0x0cc
    edid_info: [u8; 0x1c0 - 0x140],       // 0x140
    efi_info: [u8; 0x1e0 - 0x1c0],        // 0x1c0
    alt_mem_k: u32,                       // 0x1e0
    scratch: u32,                         // 0x1e4 /* obsolete! */
    e820_entries: u8,                     // 0x1e8
    eddbuf_entries: u8,                   // 0x1e9
    edd_mbr_sig_buf_entries: u8,          // 0x1ea
    kbd_status: u8,                       // 0x1eb
    secure_boot: u8,                      // 0x1ec
    _pad5: [u8; 2],                       // 0x1ed
    sentinel: u8,                         // 0x1ef
    _pad6: [u8; 1],                       // 0x1f0
    hdr: SetupHeader,                     // 0x1f1
    _pad7: [u8; 0x290 - 0x1f1 - size_of::<SetupHeader>()],
    edd_mbr_sig_buffer: [u32; 16],               // 0x290
    e820_table: [E820Entry; E820::MAX as usize], // 0x2d0
    _pad8: [u8; 48],                             // 0xcd0
    eddbuf: [u8; 0xeec - 0xd00],                 // 0xd00
    _pad9: [u8; 276],                            // 0xeec
}

/**
 * https://www.kernel.org/doc/html/latest/x86/boot.html
*/
#[repr(C, packed)]
#[derive(Default)]
pub struct SetupHeader {
    setup_sects: u8,
    root_flags: u16,
    syssize: u32,
    ram_size: u16,
    vid_mode: u16,
    root_dev: u16,
    boot_flag: u16,
    jump: u16,
    header: u32,
    version: u16,
    realmode_swtch: u32,
    start_sys: u16,
    kernel_version: u16,
    type_of_loader: u8,
    loadflags: u8,
    setup_move_size: u16,
    code32_start: u32,
    ramdisk_image: u32,
    ramdisk_size: u32,
    bootsect_kludge: u32,
    heap_end_ptr: u16,
    ext_loader_ver: u8,
    ext_loader_type: u8,
    cmd_line_ptr: u32,
    initrd_addr_max: u32,
    kernel_alignment: u32,
    relocatable_kernel: u8,
    min_alignment: u8,
    xloadflags: u16,
    cmdline_size: u32,
    hardware_subarch: u32,
    hardware_subarch_data: u64,
    payload_offset: u32,
    payload_length: u32,
    setup_data: u64,
    pref_address: u64,
    init_size: u32,
    handover_offset: u32,
    kernel_info_offset: u32,
}

const HDRS: u32 = 0x53726448;
const MAGIC_AA55: u16 = 0xaa55;
const HEADER_OFFSET: usize = 0x01f1;
const LOW_MEM_64K: usize = 64 * 1024;
const LOW_MEM_1M: usize = 1048576;
const DTB_ADDR: usize = 0x90000; //Physical address of device tree block

// The implementation of load_linux64 is inspired by
// https://github.com/akaros/akaros/blob/master/user/vmm/memory.c and
// https://github.com/machyve/xhyve/blob/master/src/firmware/kexec.c
pub struct BzImage {
    pub low_mem_size: u64,
    pub high_mem_start: u64,
    pub high_mem_size: u64,
    pub rom_base: usize,
    pub rom_size: usize,
    pub load: usize,
    pub entry: usize,
}

impl Default for BootParams {
    fn default() -> Self {
        BootParams {
            screen_info: [0u8; 0x040],             // 0x000
            apm_bios_info: [0u8; 0x054 - 0x040],   // 0x040
            _pad2: [0u8; 4],                       // 0x054
            tboot_addr: 0u64,                      // 0x058
            ist_info: [0u8; 0x070 - 0x060],        // 0x060
            acpi_rsdp_addr: [0u8; 0x078 - 0x070],  // 0x070
            _pad3: [0u8; 8],                       // 0x078
            hd0_info: [0u8; 0x090 - 0x080],        // 0x080 /* obsolete! */
            hd1_info: [0u8; 0x0a0 - 0x090],        // 0x090 /* obsolete! */
            sys_desc_table: [0u8; 0x0b0 - 0x0a0],  // 0x0a0 /* obsolete! */
            olpc_ofw_header: [0u8; 0x0c0 - 0x0b0], // 0x0b0
            ext_ramdisk_image: 0u32,               // 0x0c0
            ext_ramdisk_size: 0u32,                // 0x0c4
            ext_cmd_line_ptr: 0u32,                // 0x0c8
            _pad4: [0u8; 116],                     // 0x0cc
            edid_info: [0u8; 0x1c0 - 0x140],       // 0x140
            efi_info: [0u8; 0x1e0 - 0x1c0],        // 0x1c0
            alt_mem_k: 0u32,                       // 0x1e0
            scratch: 0u32,                         // 0x1e4 /* obsolete! */
            e820_entries: 0u8,                     // 0x1e8
            eddbuf_entries: 0u8,                   // 0x1e9
            edd_mbr_sig_buf_entries: 0u8,          // 0x1ea
            kbd_status: 0u8,                       // 0x1eb
            secure_boot: 0u8,                      // 0x1ec
            _pad5: [0u8; 2],                       // 0x1ed
            sentinel: 0u8,                         // 0x1ef
            _pad6: [0u8; 1],                       // 0x1f0
            hdr: SetupHeader::default(),           // 0x1f1
            _pad7: [0u8; 0x290 - 0x1f1 - size_of::<SetupHeader>()],
            edd_mbr_sig_buffer: [0u32; 16], // 0x290
            e820_table: [E820Entry::default(); E820::MAX as usize], // 0x2d0
            _pad8: [0u8; 48],               // 0xcd0
            eddbuf: [0u8; 0xeec - 0xd00],   // 0xd00
            _pad9: [0u8; 276],              // 0xeec
        }
    }
}

impl BzImage {
    pub fn load(&mut self, w: &mut impl core::fmt::Write) -> Result<usize> {
        // The raw pointer shit is too painful.
        let rom = SectionReader::new(&Memory {}, self.rom_base, self.rom_size);
        let mut header: SetupHeader = {
            let mut buff = [0u8; size_of::<SetupHeader>()];
            rom.pread(&mut buff, HEADER_OFFSET).unwrap();
            //TODO: Use safe_transmute
            unsafe { transmute(buff) }
        };

        // first we make sure that the kernel is not too old
        // from https://www.kernel.org/doc/Documentation/x86/boot.txt:
        // For backwards compatibility, if the setup_sects field contains 0, the
        // real value is 4.
        if header.setup_sects == 0 {
            header.setup_sects = 4;
        }
        // carriage returns used due to very simple terminal
        // check magic numbers
        if header.boot_flag != MAGIC_AA55 {
            let i = header.boot_flag;
            let p = ptr::addr_of!(header.boot_flag) as u16;
            writeln!(
                w,
                "boot flag is {:x}, not {:x}, at {:x}, \r\n",
                i, MAGIC_AA55, p
            )
            .unwrap();
            return Err("magic number missing: header.boot_flag != 0xaa55");
        }
        if header.header != HDRS {
            let i = header.header;
            let p = ptr::addr_of!(header.header) as u32;
            writeln!(w, "header.header is {:x}, not {:x} @ {:x} \r\n", i, HDRS, p).unwrap();
            return Err("magic number missing: header.header != 0x53726448");
        }
        // only accept version >= 2.12
        if header.version < 0x020c {
            let version = header.version;
            writeln!(w, "kernel version too old: 0x{:04x}", version).unwrap();
            return Err("kernel version too old");
        }

        let mut bp = BootParams::default();
        bp = BootParams { hdr: header, ..bp };

        const SECTOR_SIZE: usize = 512;
        // load kernel
        let mut kernel_offset = (bp.hdr.setup_sects as usize + 1) * SECTOR_SIZE;

        write!(w, "Kernel offset is {:x}\r\n", kernel_offset).unwrap();

        // Copy from driver into segment.
        let mut buf = [0u8; SECTOR_SIZE];
        let mut amt: usize = 0;
        writeln!(
            w,
            "Copy {:x} to {:x} for {:x}",
            kernel_offset + self.rom_base,
            self.load,
            0x300000
        )
        .unwrap();
        let mut load = self.load;
        // Read payload from rom into memory
        loop {
            if amt > 0x300000 {
                break;
            }
            let size = match rom.pread(&mut buf, kernel_offset) {
                Ok(x) => x,
                _x => break,
            };
            unsafe { copy(buf.as_ptr(), load as *mut u8, size) };
            kernel_offset += size;
            load += size;
            amt += size;
        }

        self.setup_e820_tables(&mut bp);

        unsafe {
            copy(&bp, DTB_ADDR as *mut BootParams, 1);
        }

        Ok(0)
    }

    pub fn setup_e820_tables(&self, bp: &mut BootParams) {
        let mut bp = bp;
        // The first page is always reserved.
        let entry_first_page = E820Entry {
            addr: 0,
            size: PAGE_SIZE as u64,
            r#type: E820::RESERVED as u32,
        };
        // a tiny bit of low memory for trampoline
        let entry_low = E820Entry {
            addr: PAGE_SIZE as u64,
            size: (LOW_MEM_64K - PAGE_SIZE) as u64,
            r#type: E820::RAM as u32,
        };
        // memory from 64K to LOW_MEM_1M is reserved
        let entry_reserved = E820Entry {
            addr: LOW_MEM_64K as u64,
            size: (LOW_MEM_1M - LOW_MEM_64K) as u64,
            r#type: E820::RESERVED as u32,
        };
        // LOW_MEM_1M to low_mem_size for ramdisk and multiboot
        let entry_low_main = E820Entry {
            addr: LOW_MEM_1M as u64,
            size: (self.low_mem_size - LOW_MEM_1M as u64),
            r#type: E820::RAM as u32,
        };
        // Mark 0xB000_0000 to top of 4G address space as reserved.
        // MMIO configuration space lives here. Linux complains if it is not reserved.
        let config_reserved = E820Entry {
            addr: 0xB000_0000u64,
            size: 0x5000_0000u64,
            r#type: E820::RESERVED as u32,
        };
        // Memory between low_mem_size and high_mem is used for PCI address space
        // main memory above 4GB
        let entry_main = E820Entry {
            addr: self.high_mem_start as u64,
            size: self.high_mem_size as u64,
            r#type: E820::RAM as u32,
        };
        bp.e820_table[0] = entry_first_page;
        bp.e820_table[1] = entry_low;
        bp.e820_table[2] = entry_reserved;
        bp.e820_table[3] = entry_low_main;
        bp.e820_table[4] = config_reserved;
        bp.e820_table[5] = entry_main;
        bp.e820_entries = 6;
    }

    /// Run the payload. This might not return.
    pub fn run(&self, w: &mut impl core::fmt::Write) {
        // Jump to the payload.
        // See: linux/Documentation/arm/Booting
        unsafe {
            let f = transmute::<usize, EntryPoint>(self.entry);
            write!(w, "on to {:#x} ", self.entry).unwrap();
            f(1, DTB_ADDR);
        }
        // TODO: error when payload returns.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup_e820_tables_correctly_sets_table() {
        let test_image = BzImage {
            low_mem_size: 0xFFFFFF,
            high_mem_start: 0xFFFF,
            high_mem_size: 0,
            rom_base: 0xFF00_0000,
            rom_size: 0xAAAA,
            load: 0xAAAA,
            entry: 0xAAAA,
        };
        let mut test_bp = BootParams::default();
        let test_hdr = SetupHeader::default();
        test_bp.hdr = test_hdr;
        test_image.setup_e820_tables(&mut test_bp);

        //Set up E820Entries for test
        let entry_first_page = E820Entry {
            addr: 0,
            size: PAGE_SIZE as u64,
            r#type: E820::RESERVED as u32,
        };
        let entry_low = E820Entry {
            addr: PAGE_SIZE as u64,
            size: (LOW_MEM_64K - PAGE_SIZE) as u64,
            r#type: E820::RAM as u32,
        };
        let entry_reserved = E820Entry {
            addr: LOW_MEM_64K as u64,
            size: (LOW_MEM_1M - LOW_MEM_64K) as u64,
            r#type: E820::RESERVED as u32,
        };
        let entry_low_main = E820Entry {
            addr: LOW_MEM_1M as u64,
            size: (test_image.low_mem_size - LOW_MEM_1M as u64),
            r#type: E820::RAM as u32,
        };
        let entry_main = E820Entry {
            addr: test_image.high_mem_start as u64,
            size: test_image.high_mem_size as u64,
            r#type: E820::RAM as u32,
        };

        assert_eq!(test_bp.e820_table[0], entry_first_page);
        assert_eq!(test_bp.e820_table[1], entry_low);
        assert_eq!(test_bp.e820_table[2], entry_reserved);
        assert_eq!(test_bp.e820_table[3], entry_low_main);
        assert_eq!(test_bp.e820_table[4], entry_main);
        assert_eq!(test_bp.e820_entries, 5);
    }
}
