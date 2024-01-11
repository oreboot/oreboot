// It is not clear what CRC starfive had in mind, they wrote their own ...
fn crc32_reverse(x: u32) -> u32 {
    let mut x = x;
    x = ((x & 0x55555555) << 1) | ((x >> 1) & 0x55555555);
    x = ((x & 0x33333333) << 2) | ((x >> 2) & 0x33333333);
    x = ((x & 0x0F0F0F0F) << 4) | ((x >> 4) & 0x0F0F0F0F);
    (x << 24) | ((x & 0xFF00) << 8) | ((x >> 8) & 0xFF00) | (x >> 24)
}

fn crc32(iv: u32, sv: u32, data: Vec<u8>) -> u32 {
    let mut crc = iv;
    for v in data {
        let mut byte = crc32_reverse(v.into());
        for _x in 0..8 {
            crc = if ((crc ^ byte) & 0x80000000u32) != 0 {
                (crc << 1) ^ sv
            } else {
                crc << 1
            };
            byte <<= 1;
        }
    }

    crc
}

fn crc32_final(iv: u32) -> u32 {
    crc32_reverse(iv ^ !0u32)
}

/* version: shall be 0x01010101
 * (from https://doc-en.rvspace.org/VisionFive2/SWTRM/VisionFive2_SW_TRM/create_spl.html) */
const VERSION: u32 = 0x0101_0101;
// Offset of backup SBL from Flash info start (from input_sbl_normal.cfg)
const BACKUP: u32 = 0x20_0000;
// offset of spl header: 64+256+256 = 0x240
const HEADER_OFFSET: u32 = 0x0240;
/* Offset from HDR to SPL_IMAGE, 0x400 (00 04 00 00) currently */
const HEADER_SIZE: u32 = 0x0400;

const CRC_IV: u32 = !0;
const CRC_SV: u32 = 0x04c11db7;

// The use of a packed struct is kind of pointless. Just emit the proper things in the proper order.
// Push them into the output.
// Also, let's get real: there are no big-endian machines left. Assume LE.
//        uint32_t sofs;          /* offset of spl header: 64+256+256 = 0x240 */
//        uint32_t bofs;          /* SBL_BAK_OFFSET: Offset of backup SBL from Flash info start (from input_sbl_normal.cfg) */
//        uint8_t  zro2[636];
//        uint32_t vers;          /* version: shall be 0x01010101
//                                 * (from https://doc-en.rvspace.org/VisionFive2/SWTRM/VisionFive2_SW_TRM/create_spl.html) */
//        uint32_t fsiz;          /* u-boot-spl.bin size in bytes */
//        uint32_t res1;          /* Offset from HDR to SPL_IMAGE, 0x400 (00 04 00 00) currently */
//        uint32_t crcs;          /* CRC32 of u-boot-spl.bin */
//        uint8_t  zro3[364];

pub fn spl_create_hdr(dat: Vec<u8>) -> Vec<u8> {
    /*
        // need to find out which one to use, but it's not this one.
        // CRC-32-IEEE being the most commonly used one
        let rc32 = crc::Crc::<u32>::new(&crc::CRC_32_ISCSI);
        let mut digest = rc32.digest();
        digest.update(&dat);
        let crcout = digest.finalize();
    }
    */
    let v = crc32(CRC_IV, CRC_SV, dat.clone());
    let fv = crc32_final(v);

    let mut hdr = vec![];

    let spl_header_offset: [u8; 4] = HEADER_OFFSET.to_le_bytes();
    hdr.extend_from_slice(&spl_header_offset);

    let backup: [u8; 4] = BACKUP.to_le_bytes();
    hdr.extend_from_slice(&backup);

    hdr.resize(hdr.len() + 636, 0);

    let version: [u8; 4] = VERSION.to_le_bytes();
    hdr.extend_from_slice(&version);

    let data_len: [u8; 4] = (dat.len() as u32).to_le_bytes();
    hdr.extend_from_slice(&data_len); /* u-boot-spl.bin size in bytes */

    println!("boot blob size: {}", dat.len());

    let data_offset: [u8; 4] = HEADER_SIZE.to_le_bytes();
    hdr.extend_from_slice(&data_offset);

    /* CRC32 of dat */
    let l: [u8; 4] = fv.to_le_bytes();
    hdr.extend_from_slice(&l);
    // fill up to HEADER_SIZE
    hdr.resize(hdr.len() + 364, 0);

    assert!(
        hdr.len() == HEADER_SIZE as usize,
        "hdr is {:x} bytes, not {HEADER_SIZE}",
        hdr.len()
    );
    hdr.extend(&dat);
    hdr
}

#[test]
fn test_hdr() {
    static HOSTS: &[u8] = include_bytes!("testdata/hosts");
    static HOSTS_OUT: &[u8] = include_bytes!("testdata/hosts.normal.out");

    let out = spl_create_hdr(HOSTS.to_vec());
    assert_eq!(HOSTS_OUT.len(), out.len());
    for (x, val) in out.iter().enumerate() {
        if *val != HOSTS_OUT[x] {
            println!("at index {x:#x} got {:#x}, want {:#x}", *val, HOSTS_OUT[x]);
        }
    }
    assert_eq!(HOSTS_OUT, &out);
}
