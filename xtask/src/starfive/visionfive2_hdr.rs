// offset of spl header: 64+256+256 = 0x240
const HEADER_OFFSET: u32 = 0x0240;
// Offset of backup SBL from Flash info start (from input_sbl_normal.cfg)
const BACKUP_OFFSET: u32 = 0x20_0000;
/* Offset from HDR to SPL_IMAGE, 0x400 (00 04 00 00) currently */
pub const HEADER_SIZE: u32 = 0x0400;

fn crc32(data: &Vec<u8>) -> u32 {
    let c = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
    let mut digest = c.digest();
    digest.update(&data);
    digest.finalize().to_le()
}

fn zeroes<const N: usize>() -> [u8; N] {
    [0u8; N]
}

#[repr(C)]
#[derive(Default)]
struct Version {
    major: u8,
    minor: u8,
    revision: u16,
}

#[repr(C)]
struct JH7110CommonHeader {
    // the offset to the other header
    size: u32,
    backup_offset: u32,
    _unknown1: u64,
    _skip1: [u8; 48],
    // 0x040
    ec_param_p: [u8; 32],
    ec_param_a: [u8; 32],
    ec_param_b: [u8; 32],
    ec_param_gx: [u8; 32],
    ec_param_gy: [u8; 32],
    ec_param_n: [u8; 32],
    _skip2: [u8; 64],
    // 0x140
    ec_key_1: [u8; 64],
    ec_key_2: [u8; 64],
    ec_key_3: [u8; 64],
    ec_key_4: [u8; 64],
}

impl Default for JH7110CommonHeader {
    fn default() -> Self {
        JH7110CommonHeader {
            size: HEADER_OFFSET,
            backup_offset: BACKUP_OFFSET,
            _unknown1: 0,
            _skip1: zeroes(),
            ec_param_p: zeroes(),
            ec_param_a: zeroes(),
            ec_param_b: zeroes(),
            ec_param_gx: zeroes(),
            ec_param_gy: zeroes(),
            ec_param_n: zeroes(),
            _skip2: zeroes(),
            ec_key_1: zeroes(),
            ec_key_2: zeroes(),
            ec_key_3: zeroes(),
            ec_key_4: zeroes(),
        }
    }
}

type SigPadding = [u8; 0x40];

#[repr(C)]
struct JH7110SBLHeader {
    ec_key_select: u32,
    version: Version,
    payload_size: u32,
    header_size: u32,
    checksum: u32,
    aes_iv: [u8; 0x10],
    ec_key_revoke: [u8; 0x20],
    sbl_ver_cipher: [u8; 0x10],
    // fill up to 0x180
    _rest: [u8; 0x12c],
}

impl Default for JH7110SBLHeader {
    fn default() -> Self {
        JH7110SBLHeader {
            // TODO: This needs to be a parameter for "secure boot" setup.
            // EC key to use; 0 means use none, 1 means first, 4 is maximum.
            // Those refer to the ec_key_n in JH7110CommonHeader.
            ec_key_select: 0,
            // https://doc-en.rvspace.org/VisionFive2/SWTRM/VisionFive2_SW_TRM/create_spl.html
            version: Version {
                major: 1,
                minor: 1,
                revision: 0x101,
            },
            payload_size: 0,
            header_size: HEADER_SIZE,
            checksum: 0,
            aes_iv: zeroes(),
            ec_key_revoke: zeroes(),
            sbl_ver_cipher: zeroes(),
            _rest: zeroes(),
        }
    }
}

pub fn spl_create_hdr(payload: Vec<u8>) -> Vec<u8> {
    let common_header = JH7110CommonHeader::default();
    let sig_padding: SigPadding = zeroes();
    let sbl_header = JH7110SBLHeader {
        payload_size: payload.len() as u32,
        checksum: crc32(&payload),
        ..Default::default()
    };

    // this should be easier
    let ch_size = std::mem::size_of::<JH7110CommonHeader>();
    let ch_ptr = &common_header as *const JH7110CommonHeader as *const u8;
    let ch = unsafe { std::slice::from_raw_parts(ch_ptr, ch_size) };
    let sh_size = std::mem::size_of::<JH7110SBLHeader>();
    let sh_ptr = &sbl_header as *const JH7110SBLHeader as *const u8;
    let sh = unsafe { std::slice::from_raw_parts(sh_ptr, sh_size) };

    let mut res = vec![];
    res.extend(ch);
    res.extend(&sig_padding);
    res.extend(sh);
    res.extend(&payload);
    res
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
