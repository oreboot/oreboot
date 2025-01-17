use crate::init::glb_power_up_ldo12uhs;
use crate::util::{clear_bit, read32, set_bit, sleep, udelay, write32};

const P_CLOCK_FREQUENCY: u32 = 1400;
const PAGE_SIZE: u32 = 11;

const CONTROLLER_BASE: usize = 0x3000_F000;
const BASIC: usize = CONTROLLER_BASE;
const CMD: usize = CONTROLLER_BASE + 0x0004;
const FIFO_THRE: usize = CONTROLLER_BASE + 0x0008;
const MANUAL: usize = CONTROLLER_BASE + 0x000C;
const AUTO_FRESH_1: usize = CONTROLLER_BASE + 0x0010;
const AUTO_FRESH_2: usize = CONTROLLER_BASE + 0x0014;
const AUTO_FRESH_3: usize = CONTROLLER_BASE + 0x0018;
const AUTO_FRESH_4: usize = CONTROLLER_BASE + 0x001C;
const CONFIGURE: usize = CONTROLLER_BASE + 0x0020;
const STATUS: usize = CONTROLLER_BASE + 0x0024;

const TIMING_CTRL: usize = CONTROLLER_BASE + 0x0030;

const DEBUG_SELECT: usize = CONTROLLER_BASE + 0x00C0;

const PHY_CFG_BASE: usize = CONTROLLER_BASE + 0x0100;
const PHY_CFG_00: usize = PHY_CFG_BASE;
const PHY_CFG_04: usize = PHY_CFG_BASE + 0x04;

const PHY_CFG_DQ0: usize = PHY_CFG_BASE + 0x08;
const PHY_CFG_DQ1: usize = PHY_CFG_BASE + 0x0C;
const PHY_CFG_DQ2: usize = PHY_CFG_BASE + 0x10;
const PHY_CFG_DQ3: usize = PHY_CFG_BASE + 0x14;
const PHY_CFG_DQ4: usize = PHY_CFG_BASE + 0x18;
const PHY_CFG_DQ5: usize = PHY_CFG_BASE + 0x1C;
const PHY_CFG_DQ6: usize = PHY_CFG_BASE + 0x20;
const PHY_CFG_DQ7: usize = PHY_CFG_BASE + 0x24;

const PHY_CFG_DQS0: usize = PHY_CFG_BASE + 0x28;
const PHY_CFG_DQS1: usize = PHY_CFG_BASE + 0x2C;
const PHY_CFG_30: usize = PHY_CFG_BASE + 0x30;
const PHY_CFG_34: usize = PHY_CFG_BASE + 0x34;
const PHY_CFG_38: usize = PHY_CFG_BASE + 0x38;
const PHY_CFG_3C: usize = PHY_CFG_BASE + 0x3C;
const PHY_CFG_40: usize = PHY_CFG_BASE + 0x40;
const PHY_CFG_44: usize = PHY_CFG_BASE + 0x44;
const PHY_CFG_48: usize = PHY_CFG_BASE + 0x48;
const PHY_CFG_4C: usize = PHY_CFG_BASE + 0x4C;
const PHY_CFG_50: usize = PHY_CFG_BASE + 0x50;

const DQ_REGS: [usize; 8] = [
    PHY_CFG_DQ0,
    PHY_CFG_DQ1,
    PHY_CFG_DQ2,
    PHY_CFG_DQ3,
    PHY_CFG_DQ4,
    PHY_CFG_DQ5,
    PHY_CFG_DQ6,
    PHY_CFG_DQ7,
];

const PHY_CFG_00_CK_SR_OFFSET: u32 = 8;
const PHY_CFG_00_CK_DLY_DRV_OFFSET: u32 = 16;
const PHY_CFG_00_CEN_SR_OFFSET: u32 = 20;
const PHY_CFG_00_CEN_DLY_DRV_OFFSET: u32 = 28;

const PHY_CFG_04_DM1_SR_OFFSET: u32 = 4;
const PHY_CFG_04_DM1_DLY_DRV_OFFSET: u32 = 12;
const PHY_CFG_04_DM0_SR_OFFSET: u32 = 20;
const PHY_CFG_04_DM0_DLY_DRV_OFFSET: u32 = 28;

const PHY_CFG_30_OE_TIMER_MASK: u32 = 0b11 << 24;
const PHY_CFG_30_VREF_MODE_BIT: u32 = 26;
const PHY_CFG_30_ODT_SELECT_MASK: u32 = 0b1111 << 28;

const PHY_CFG_40_UNK0_OFFSET: u32 = 16;
const PHY_CFG_40_UNK1_OFFSET: u32 = 20;
const PHY_CFG_40_DMY0_OFFSET: u32 = 8;
const PHY_CFG_40_UNK0_MASK: u32 = 0b11 << PHY_CFG_40_UNK0_OFFSET;
const PHY_CFG_40_UNK1_MASK: u32 = 0b11 << PHY_CFG_40_UNK1_OFFSET;
const PHY_CFG_40_DMY0_MASK: u32 = 0b1111_1111 << PHY_CFG_40_DMY0_OFFSET;

const PHY_CFG_48_PSRAM_TYPE_OFFSET: u32 = 8;
const PHY_CFG_48_PSRAM_TYPE_MASK: u32 = 0b11 << PHY_CFG_48_PSRAM_TYPE_OFFSET;

const PHY_CFG_4C_ODT_SEL_DLY_OFFSET: u32 = 16;
const PHY_CFG_4C_ODT_SEL_DLY_MASK: u32 = 0b1111 << PHY_CFG_4C_ODT_SEL_DLY_OFFSET;
const PHY_CFG_4C_ODT_SEL_HW_BIT: u32 = 20;

const PHY_CFG_50_DQ_OE_MID_P_MASK: u32 = 0b11 << 8;
const PHY_CFG_50_DQ_OE_MID_N_MASK: u32 = 0b11 << 12;

const PHY_CFG_50_DQ_OE_UP_P_OFFSET: u32 = 0;
const PHY_CFG_50_DQ_OE_UP_N_OFFSET: u32 = 4;
const PHY_CFG_50_DQ_OE_DN_P_OFFSET: u32 = 16;
const PHY_CFG_50_DQ_OE_DN_N_OFFSET: u32 = 20;

const PHY_CFG_DQN_SR_OFFSET: u32 = 0;
const PHY_CFG_DQN_DLY_RX_OFFSET: u32 = 8;
const PHY_CFG_DQN_DLY_DRV_OFFSET: u32 = 12;
const PHY_CFG_DQM_SR_OFFSET: u32 = 20;
const PHY_CFG_DQM_DLY_RX_OFFSET: u32 = 24;
const PHY_CFG_DQM_DLY_DRV_OFFSET: u32 = 28;

const PHY_CFG_DQS_IPP5UN_LPDDR_BIT: u32 = 0;
const PHY_CFG_DQS_EN_RX_FE_BIT: u32 = 1;
const PHY_CFG_DQS_EN_BIAS_BIT: u32 = 2;
const PHY_CFG_DQS_DLY_DRV_OFFSET: u32 = 24;
const PHY_CFG_DQS_DIFF_DLY_OFFSET: u32 = 28;

pub fn analog_init() {
    glb_power_up_ldo12uhs();

    // disable CEn, CK, CKn
    let cfg50 = read32(PHY_CFG_50);
    let m = !(PHY_CFG_50_DQ_OE_MID_P_MASK | PHY_CFG_50_DQ_OE_MID_N_MASK);
    write32(PHY_CFG_50, (cfg50 & m));
    udelay(1);
    let cfg40 = read32(PHY_CFG_40);
    let m = !(PHY_CFG_40_UNK0_MASK | PHY_CFG_40_DMY0_MASK);
    write32(PHY_CFG_40, (cfg40 & m) | (3 << PHY_CFG_40_UNK0_OFFSET));
    udelay(1);

    // configure pads
    let v = (2 << PHY_CFG_00_CK_SR_OFFSET)
        | (11 << PHY_CFG_00_CK_DLY_DRV_OFFSET)
        | (2 << PHY_CFG_00_CEN_SR_OFFSET)
        | (8 << PHY_CFG_00_CEN_DLY_DRV_OFFSET);
    write32(PHY_CFG_00, v);
    let v = (2 << PHY_CFG_04_DM1_SR_OFFSET)
        | (6 << PHY_CFG_04_DM1_DLY_DRV_OFFSET)
        | (2 << PHY_CFG_04_DM0_SR_OFFSET)
        | (6 << PHY_CFG_04_DM0_DLY_DRV_OFFSET);
    write32(PHY_CFG_04, v);

    // DQ[0:15]
    let v = (2 << PHY_CFG_DQN_SR_OFFSET)
        | (7 << PHY_CFG_DQN_DLY_DRV_OFFSET)
        | (2 << PHY_CFG_DQM_SR_OFFSET)
        | (7 << PHY_CFG_DQM_DLY_DRV_OFFSET);
    for r in DQ_REGS {
        write32(r, v);
    }

    let v = (6 << PHY_CFG_DQS_DLY_DRV_OFFSET) | (2 << PHY_CFG_DQS_DIFF_DLY_OFFSET);
    write32(PHY_CFG_DQS0, v);
    let v = v | (1 << PHY_CFG_DQS_EN_RX_FE_BIT) | (1 << PHY_CFG_DQS_EN_BIAS_BIT);
    write32(PHY_CFG_DQS1, v);

    let m =
        !(PHY_CFG_30_OE_TIMER_MASK | (1 << PHY_CFG_30_VREF_MODE_BIT) | PHY_CFG_30_OE_TIMER_MASK);
    let cfg30 = read32(PHY_CFG_30);
    write32(
        PHY_CFG_30,
        (cfg30 & m) | (3 << 24) | (1 << PHY_CFG_30_VREF_MODE_BIT),
    );
    let cfg48 = read32(PHY_CFG_48);
    write32(
        PHY_CFG_48,
        !(PHY_CFG_48_PSRAM_TYPE_MASK) | (2 << PHY_CFG_48_PSRAM_TYPE_OFFSET),
    );
    let cfg4c = read32(PHY_CFG_4C);
    write32(
        PHY_CFG_4C,
        !(PHY_CFG_4C_ODT_SEL_DLY_MASK | PHY_CFG_4C_ODT_SEL_HW_BIT),
    );
    let v = (7 << PHY_CFG_50_DQ_OE_UP_P_OFFSET)
        | (7 << PHY_CFG_50_DQ_OE_UP_N_OFFSET)
        | (7 << PHY_CFG_50_DQ_OE_DN_P_OFFSET)
        | (7 << PHY_CFG_50_DQ_OE_DN_N_OFFSET);
    let cfg50 = read32(PHY_CFG_50);
    write32(PHY_CFG_50, cfg50 | v);
    udelay(1);

    // switch to LDO 1V2
    let cfg40 = read32(PHY_CFG_40);
    write32(PHY_CFG_40, cfg40 | (3 << PHY_CFG_40_UNK1_OFFSET));
    udelay(1);

    // reenable CEn, CK, CKn
    let cfg40 = read32(PHY_CFG_40);
    let m = !(PHY_CFG_40_UNK0_MASK | PHY_CFG_40_DMY0_MASK);
    write32(PHY_CFG_40, (cfg40 & m) | (3 << PHY_CFG_40_UNK0_OFFSET));
    udelay(1);
    let cfg50 = read32(PHY_CFG_50);
    let v = PHY_CFG_50_DQ_OE_MID_P_MASK | PHY_CFG_50_DQ_OE_MID_N_MASK;
    write32(PHY_CFG_50, cfg50 | v);
    udelay(1);
}

pub fn config_uhs_phy() {
    //
}

pub fn init() {
    //
    // TIMING_CTRL_TRFC_CYCLE
    // TIMING_CTRL_TCPHW_CYCLE
    // TIMING_CTRL_TCPHR_CYCLE
    // TIMING_CTRL_TRC_CYCLE
    let timing = (18 << 24) | (2 << 16) | 11;
    write32(TIMING_CTRL, timing);

    println!("analog init");
    analog_init();
    println!("configure ultra high-speed PHY");
    config_uhs_phy();
    udelay(150);

    // TODO

    println!("PSRAM init done :)");
}
