use core::fmt::Write;
use x86_64::instructions::{rdmsr, wrmsr};

fn one(w: &mut print::WriteTo, a: u32, v: u64, ro: bool) {
    write!(w, "{:x} ", a).unwrap();
    let rv = rdmsr(a);
    let mut d = "";
    if rv != v {
        d = "DIFF";
    }
    write!(w, "{} tried to chagne it {:x} got {:x}\r\n", d, v, rv,).unwrap();
    if !ro {
        unsafe {
            wrmsr(a, v);
        }
        let nv = rdmsr(a);
        write!(w, "{} {:x} got {:x}; \r\n", d, v, nv).unwrap();
    }
}
pub fn msrs(w: &mut print::WriteTo) {
    one(w, 0xc0000080, 0xd01, false);
    one(w, 0xc0000081, 0x23001000000000, false);
    one(w, 0xc0000082, 0xffffffff99a00000, false);
    one(w, 0xc0000083, 0xffffffff99a01230, false);
    one(w, 0xc0000084, 0x47700, false);
    one(w, 0xc00000e7, 0x16497beaa820, false);
    one(w, 0xc00000e8, 0x1fafceec9a6e, false);
    one(w, 0xc0000100, 0x5b6c50, false);
    one(w, 0xc0000101, 0xffffffff9a031000, false);
    one(w, 0xc0000104, 0x100000000, false);
    one(w, 0xc0000200, 0x800, false);
    one(w, 0xc0000201, 0x800, false);
    one(w, 0xc0000202, 0x800, false);
    one(w, 0xc0000203, 0x800, false);
    one(w, 0xc0000204, 0x800, false);
    one(w, 0xc0000205, 0x800, false);
    one(w, 0xc0000206, 0x800, false);
    one(w, 0xc0000207, 0x800, false);
    one(w, 0xc0000208, 0x800, false);
    one(w, 0xc0000209, 0x800, false);
    one(w, 0xc000020a, 0x800, false);
    one(w, 0xc000020b, 0x800, false);
    one(w, 0xc000020c, 0x800, false);
    one(w, 0xc000020d, 0x800, false);
    one(w, 0xc000020e, 0x800, false);
    one(w, 0xc000020f, 0x800, false);
    one(w, 0xc0000410, 0x1001028, false);
    one(w, 0xc0002004, 0x70000007d, false);
    one(w, 0xc0002005, 0xb000000000, false);
    one(w, 0xc0002014, 0x300000079, false);
    one(w, 0xc0002015, 0x100b000000000, false);
    one(w, 0xc0002024, 0x50000007f, false);
    one(w, 0xc0002025, 0x200b000000000, false);
    one(w, 0xc0002034, 0x300000079, false);
    one(w, 0xc0002035, 0x300b000000000, false);
    one(w, 0xc0002054, 0x300000079, false);
    one(w, 0xc0002055, 0x500b000000000, false);
    one(w, 0xc0002064, 0x300000079, false);
    one(w, 0xc0002065, 0x600b000000000, false);
    one(w, 0xc0002074, 0x50000007f, false);
    one(w, 0xc0002075, 0x700b020350000, false);
    one(w, 0xc0002084, 0x50000007f, false);
    one(w, 0xc0002085, 0x700b020350100, false);
    one(w, 0xc0002094, 0x50000007f, false);
    one(w, 0xc0002095, 0x700b020350200, false);
    one(w, 0xc00020a4, 0x50000007f, false);
    one(w, 0xc00020a4, 0x700b020350300, false);
    one(w, 0xc00020b4, 0x50000007f, false);
    one(w, 0xc00020b5, 0x700b020750000, false);
    one(w, 0xc00020c4, 0x50000007f, false);
    one(w, 0xc00020c5, 0x700b020750100, false);
    one(w, 0xc00020d4, 0x50000007f, false);
    one(w, 0xc00020d5, 0x700b020750200, false);
    one(w, 0xc00020e4, 0x50000007f, false);
    one(w, 0xc00020e5, 0x700b020750300, false);
    one(w, 0xc00020f4, 0x300000079, false);
    one(w, 0xc00020f5, 0x2000130430400, false);
    one(w, 0xc0002104, 0x10000007b, false);
    one(w, 0xc0002105, 0x530082900, false);
    one(w, 0xc0002114, 0x70000007d, false);
    one(w, 0xc0002115, 0x9600050f00, false);
    one(w, 0xc0002134, 0x50000007f, false);
    one(w, 0xc0002135, 0x2002e00000001, false);
    one(w, 0xc0002144, 0x50000007f, false);
    one(w, 0xc0002145, 0x2002e00000101, false);
    one(w, 0xc0002164, 0x70000007d, false);
    one(w, 0xc0002165, 0x1813b17000, false);
    one(w, 0xc0002174, 0x70000007d, false);
    one(w, 0xc0002175, 0x46115c0000, false);
    one(w, 0xc0002184, 0x300000079, false);
    one(w, 0xc0002185, 0x1000103b30400, false);
    one(w, 0xc0002194, 0x300000079, false);
    one(w, 0xc0002195, 0x100ff03830400, false);
    one(w, 0xc00021a4, 0x10000007b, false);
    one(w, 0xc00021a5, 0x50005e100, false);
    one(w, 0xc00021b4, 0x70000007d, false);
    one(w, 0xc00021b5, 0x1002e00001e01, false);

    // ?
    // one(w, 0xc0010010, 0xf40000, false);
    one(w, 0xc0010015, 0x0000_0000_0900_0010, false);

    // tom and tom2
    one(w, 0xc001001a, 0x80000000, false);
    one(w, 0xc001001d, 0x450000000, false);

    // one(w, 0xc0010022, 0x200, false); no mce
    // what's your name?
    one(w, 0xc0010030, 0x4359504520444d41, false);
    one(w, 0xc0010031, 0x3233203235343720, false);
    one(w, 0xc0010032, 0x72502065726f432d, false);
    one(w, 0xc0010033, 0x20726f737365636f, false);
    one(w, 0xc0010034, 0x2020202020202020, false);
    one(w, 0xc0010035, 0x20202020202020, false);

    one(w, 0xc0010056, 0x28000b2, false);
    one(w, 0xc0010058, 0xe0000021, false);
    //   one(w, 0xc0010061, 0x20, false); error on write.
    one(w, 0xc0010064, 0x8000000045d2085e, false);
    one(w, 0xc0010065, 0x8000000045160a64, false);
    one(w, 0xc0010066, 0x8000000043da0c5a, false);
    one(w, 0xc0010073, 0x813, false);
    one(w, 0xc0010074, 0x289, false);
    one(w, 0xc0010111, 0xafba2000, false);
    one(w, 0xc0010112, 0xac000000, false);
    one(w, 0xc0010113, 0xfffffc006003, false);
    one(w, 0xc001020b, 0xffff, false);
    one(w, 0xc0010292, 0x40b8012, false);
    one(w, 0xc0010293, 0x104886, false);
    one(w, 0xc0010294, 0xf8e847f00008912, false);
    one(w, 0xc0010296, 0x484848, false);
    one(w, 0xc0010297, 0x380000fc000, false);
    one(w, 0xc0010299, 0xa1003, false);
    one(w, 0xc001029a, 0x9731905d, false);
    one(w, 0xc001029b, 0x95073877, false);
    one(w, 0xc00102b3, 0xfff0, false);
    one(w, 0xc00102f0, 0x1, false);
    one(w, 0xc0010400, 0x600, false);
    one(w, 0xc0010401, 0x2c00, false);
    one(w, 0xc0010402, 0x8, false);
    one(w, 0xc0010406, 0x40, false);
    one(w, 0xc0010407, 0x80, false);
    one(w, 0xc0010408, 0x80, false);
    one(w, 0xc0010409, 0x80, false);
    one(w, 0xc001040a, 0x80, false);
    one(w, 0xc001040b, 0x80, false);
    one(w, 0xc001040c, 0x80, false);
    one(w, 0xc001040d, 0x80, false);
    one(w, 0xc001040e, 0x80, false);
    one(w, 0xc0010413, 0x2, false);
    one(w, 0xc0010414, 0x2, false);
    one(w, 0xc0010416, 0x6, false);
    one(w, 0xc0010419, 0x3c0, false);
    one(w, 0xc0011000, 0x8000, false);
    one(w, 0xc0011002, 0x219c91a9, false);
    one(w, 0xc0011003, 0x1, false);
    if true {
        // boots ok, does not fix apic timer verification
        one(w, 0xc0011004, 0x7ed8320b178bfbff, false);
        one(w, 0xc0011005, 0x75c237ff2fd3fbff, false);
        one(w, 0xc001100c, 0xff711b00, false);
        one(w, 0xc0011020, 0x6404000000000, false);
        one(w, 0xc0011021, 0x2000000, false);
        one(w, 0xc0011022, 0xc000000002500000, false);
        one(w, 0xc0011023, 0x2000000000020, false);
        one(w, 0xc0011028, 0x200248000d4, false);
        one(w, 0xc0011029, 0x3000310e08002, false);
        one(w, 0xc001102a, 0x38080, false);
        one(w, 0xc001102b, 0x2008cc17, false);
        one(w, 0xc001102c, 0x309c70000000000, false);
        one(w, 0xc001102d, 0x101c00000010, false);
        one(w, 0xc001102e, 0x12024000000000, false);
    }
    if true {
        // boots ok, does not fix apic timer
        one(w, 0xc001103a, 0x100, true); // gpf on write
        one(w, 0xc0011074, 0xa000000000000000, false);
        one(w, 0xc0011076, 0x14, false);
        one(w, 0xc0011077, 0x6d00000000000000, false);
        one(w, 0xc0011083, 0x38d6b5ad1bc6b5ad, false);
        one(w, 0xc0011092, 0x57840a05, false);
        one(w, 0xc0011093, 0x6071f9fc, false);
        one(w, 0xc0011094, 0x110c, false);
        one(w, 0xc0011097, 0x5dbf, false);
    }
    if true {
        // boots ok, does not fix apic timer
        one(w, 0xc0011098, 0xa, false);
        one(w, 0xc00110a2, 0xc9000000, false);
        one(w, 0xc00110dc, 0x3030018cf757, false);
    }
    if true {
        // if these next four are enabled, we get past apic verification as failure
        one(w, 0xc00110dd, 0x13bcff, false);
        one(w, 0xc00110e1, 0x410e50400c2cb4e0, false);
    }
    if true {
        // if one of these two are disabled, apic fails.
        one(w, 0xc00110e2, 0x2afa00082018, false); // It's this one. It's undocumented.
    }
    if true {
        one(w, 0xc00110e3, 0x1, false);
    }

    ////one(w, 0x10, 0x6780a9b73d4, true);
    ////one(w, 0x1b, 0xfee00900, true);
    ////one(w, 0x8b, 0x8301038, true);
    ////one(w, 0xe7, 0x322248bbed, false);
    ////one(w, 0xe8, 0x3a9d34b62b, false);
    ////one(w, 0xfe, 0x508, false);
    ////one(w, 0x179, 0x11c, false);
    one(w, 0x200, 0x6, false);
    one(w, 0x201, 0xffff80000800, false);
    one(w, 0x202, 0x80000006, false);
    one(w, 0x203, 0xffffe0000800, false);
    one(w, 0x204, 0xa0000006, false);
    one(w, 0x205, 0xfffff0000800, false);
    one(w, 0x206, 0xff000005, false);
    one(w, 0x207, 0xffffff000800, false);
    one(w, 0x208, 0xac000000, false);
    one(w, 0x209, 0xfffffc000800, false);
    one(w, 0x20a, 0xa2fa0000, false);
    one(w, 0x20b, 0xffffffff0800, false);
    one(w, 0x250, 0x606060606060606, false);
    one(w, 0x258, 0x606060606060606, false);
    one(w, 0x259, 0x404040404040404, false);
    one(w, 0x268, 0x505050505050505, false);
    one(w, 0x269, 0x505050505050505, false);
    one(w, 0x26a, 0x505050505050505, false);
    one(w, 0x26b, 0x505050505050505, false);
    one(w, 0x26c, 0x505050505050505, false);
    one(w, 0x26d, 0x505050505050505, false);
    one(w, 0x26e, 0x505050505050505, false);
    one(w, 0x26f, 0x505050505050505, false);
    one(w, 0x277, 0x7040600070406, false);
    one(w, 0x2ff, 0xc00, false);
}
