// NOTE: This is mostly generic code and to be moved to our arch crate.

use aarch64_cpu::registers::*;

// NOTE: Assumes the clock to run at 24MHz.
pub fn udelay(us: u64) {
    let t0 = CNTPCT_EL0.get();
    let t1 = t0 + us * 0x18;
    while CNTPCT_EL0.get() < t1 {}
}

pub fn print_cpuinfo() {
    let v = MIDR_EL1.extract();
    println!("MIDR EL1: 0x{v:08x?}");

    let arch = match MIDR_EL1.read_as_enum(MIDR_EL1::Architecture) {
        Some(MIDR_EL1::Architecture::Value::Individual) => "individual",
        _ => "other",
        None => "N/A",
    };
    println!("Architecture: {arch}");

    let implementer = MIDR_EL1.read_as_enum(MIDR_EL1::Implementer);
    let imp = match implementer {
        Some(MIDR_EL1::Implementer::Value::Arm) => "Arm",
        _ => "[unknown]",
        None => "N/A",
    };
    println!("Implementer: {imp}");

    // NOTE: Needs distinguishing; this is only for Implementer == Arm
    // In theory, part numbers could overlap between vendors.
    // In practice, they might be unique. TODO: What does the spec say?
    // https://github.com/bp0/armids/blob/master/arm.ids
    let part_num = MIDR_EL1.read(MIDR_EL1::PartNum);
    let par = match part_num {
        0xd01 => "Cortex-A32",
        0xd03 => "Cortex-A53",
        0xd04 => "Cortex-A35",
        0xd05 => "Cortex-A55",
        0xd07 => "Cortex-A57",
        0xd08 => "Cortex-A72",
        0xd09 => "Cortex-A73",
        0xd0a => "Cortex-A75",
        0xd0b => "Cortex-A76",
        _ => "[unknown]",
    };
    println!("Part number: {par} ({part_num:03x})");

    let rev = MIDR_EL1.read(MIDR_EL1::Revision);
    println!("Revision: {rev}");

    let var = MIDR_EL1.read(MIDR_EL1::Variant);
    println!("Variant: {var}");

    let rndr = match ID_AA64ISAR0_EL1.read_as_enum(ID_AA64ISAR0_EL1::RNDR) {
        Some(ID_AA64ISAR0_EL1::RNDR::Value::Supported) => "yes",
        Some(ID_AA64ISAR0_EL1::RNDR::Value::NotSupported) => "no",
        None => "N/A",
    };
    println!("ID_AA64ISAR0_EL1\n  RNDR: {rndr}");

    let tgran4 = match ID_AA64MMFR0_EL1.read_as_enum(ID_AA64MMFR0_EL1::TGran4) {
        Some(ID_AA64MMFR0_EL1::TGran4::Value::Supported) => "yes",
        Some(ID_AA64MMFR0_EL1::TGran4::Value::NotSupported) => "no",
        None => "N/A",
    };
    println!("ID_AA64MMFR0_EL1");
    println!("  TGran4: {tgran4}");

    let twed = match ID_AA64MMFR1_EL1.read_as_enum(ID_AA64MMFR1_EL1::TWED) {
        Some(ID_AA64MMFR1_EL1::TWED::Value::Supported) => "yes",
        Some(ID_AA64MMFR1_EL1::TWED::Value::Unsupported) => "no",
        _ => "?",
        None => "N/A",
    };
    println!("ID_AA64MMFR1_EL1");
    println!("  TWED: {twed}");

    let bbm = match ID_AA64MMFR2_EL1.read_as_enum(ID_AA64MMFR2_EL1::BBM) {
        Some(ID_AA64MMFR2_EL1::BBM::Value::Level0) => "Level 0",
        Some(ID_AA64MMFR2_EL1::BBM::Value::Level1) => "Level 1",
        Some(ID_AA64MMFR2_EL1::BBM::Value::Level2) => "Level 2",
        _ => "?",
        None => "N/A",
    };
    println!("ID_AA64MMFR2_EL1");
    println!("  BBM: {bbm}");
}
