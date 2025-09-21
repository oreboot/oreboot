use zerocopy::IntoBytes;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes};

// eGON.BT0 header. This header is identified by D1 ROM code
// to copy BT0 stage bootloader into SRAM memory.
// NOTE: The "real" header includes the initial jump.
// It must be 32-byte aligned. See also:
// https://github.com/u-boot/u-boot/blob/fe2ce09a0753634543c32cafe85eb87a625f76ca/include/sunxi_image.h#L80
#[derive(FromBytes, Immutable, IntoBytes, Clone, Copy, Debug)]
#[repr(C)]
struct EgonHead {
    jump_instruction: u32,
    magic: [u8; 8],
    checksum: u32,
    length: u32,
    // used by sunxi-fel
    pub_head_size: u32,
    fel_script_address: u32,
    fel_uenv_length: u32,
    // used by U-Boot's mksunxiboot
    dt_name_offset: u32,
    dram_size: u32,
    // filled by mask ROM
    boot_media: u32,
    // our remaining space
    string_pool: [u32; 13],
}

const JUMP_ARM_32: u32 = 0xea000016;
// TODO: const JUMP_RISCV_64: u32 = 0x...;

pub enum Arch {
    Riscv64,
    Arm32,
}

const STAMP_VALUE: u32 = 0x5F0A6C39;

fn align_up_to(len: usize, target_align: usize) -> usize {
    let (div, rem) = (len / target_align, len % target_align);
    if rem != 0 {
        (div + 1) * target_align
    } else {
        len
    }
}

pub fn add_header(image: &[u8], arch: Arch) -> Vec<u8> {
    let len = image.len();
    let length = align_up_to(len, 16 * 1024) as u32;
    let jump_instruction = match arch {
        Arch::Arm32 => JUMP_ARM_32,
        Arch::Riscv64 => todo!(),
    };
    // NOTE: We have to initialize the checksum with the stamp value.
    // The head itself is then used in the checksum calculcation.
    let initial_head = EgonHead {
        jump_instruction,
        magic: *b"eGON.BT0",
        checksum: STAMP_VALUE,
        length,
        pub_head_size: 0,
        fel_script_address: 0,
        fel_uenv_length: 0,
        dt_name_offset: 0,
        dram_size: 0,
        boot_media: 0,
        string_pool: [0; 13],
    };

    let pre = [initial_head.as_bytes(), &image].concat();
    let mut bin = pre.to_vec();
    bin.resize(16 * 1024, 0);

    let mut checksum: u32 = 0;
    for c in bin.chunks_exact(4).into_iter() {
        let v = u32::from_le_bytes([c[0], c[1], c[2], c[3]]);
        checksum = checksum.wrapping_add(v);
    }

    bin[12] = checksum as u8;
    bin[13] = (checksum >> 8) as u8;
    bin[14] = (checksum >> 16) as u8;
    bin[15] = (checksum >> 24) as u8;

    bin
}
