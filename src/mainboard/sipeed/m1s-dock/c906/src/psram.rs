use crate::init::glb_power_up_ldo12uhs;
use crate::util::{clear_bit, read32, set_bit, sleep, udelay, write32};

// TODO: figure out value
const EXT_TEMP_RANGE: bool = false;

const P_CLOCK_FREQUENCY: u32 = 1400;
const MEM_SIZE: u32 = 64;
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

const BASIC_INIT_EN_BIT: u32 = 0;
const BASIC_AF_EN_BIT: u32 = 1;
const BASIC_ADDRMB_MSK_OFFSET: u32 = 16;
const BASIC_ADDRMB_MSK_MASK: u32 = 0b1111_1111 << BASIC_ADDRMB_MSK_OFFSET;
const BASIC_LINEAR_BND_B_OFFSET: u32 = 28;
const BASIC_LINEAR_BND_B_MASK: u32 = 0b1111 << BASIC_LINEAR_BND_B_OFFSET;

const TIMING_CTRL: usize = CONTROLLER_BASE + 0x0030;

const DEBUG_SELECT: usize = CONTROLLER_BASE + 0x00C0;

const MANUAL_P_CLOCK_T_DIVIDER_OFFSET: u32 = 24;
const MANUAL_P_CLOCK_T_DIVIDER_MASK: u32 = 0b1111_1111 << MANUAL_P_CLOCK_T_DIVIDER_OFFSET;

const AUTO_FRESH_4_BUST_CYCLE_MASK: u32 = 0x7f;
const AUTO_FRESH_2_REFI_CYCLE_MASK: u32 = 0xffff;

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

const PHY_TIMER_1: usize = PHY_CFG_BASE + 0x34;
const PHY_TIMER_2: usize = PHY_CFG_BASE + 0x38;
const PHY_TIMER_3: usize = PHY_CFG_BASE + 0x3C;

const PHY_CFG_40: usize = PHY_CFG_BASE + 0x40;

const PHY_TIMER_4: usize = PHY_CFG_BASE + 0x44;

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

const PHY_CFG_30_WL_DQ_DIG_OFFSET: u32 = 0;
const PHY_CFG_30_WL_DQ_ANA_OFFSET: u32 = 4;
const PHY_CFG_30_WL_DIG_OFFSET: u32 = 8;
const PHY_CFG_30_WL_ANA_OFFSET: u32 = 12;
const PHY_CFG_30_RL_DIG_OFFSET: u32 = 16;
const PHY_CFG_30_RL_ANA_OFFSET: u32 = 20;
const PHY_CFG_30_OE_TIMER_OFFSET: u32 = 24;
const PHY_CFG_30_VREF_MODE_BIT: u32 = 26;
const PHY_CFG_30_OE_CTRL_HW_BIT: u32 = 27;
const PHY_CFG_30_ODT_SELECT_OFFSET: u32 = 28;
const PHY_CFG_30_OE_TIMER_MASK: u32 = 0b11 << PHY_CFG_30_OE_TIMER_OFFSET;
const PHY_CFG_30_ODT_SELECT_MASK: u32 = 0b1111 << PHY_CFG_30_ODT_SELECT_OFFSET;

const PHY_TIMER_1_DQS_START_OFFSET: u32 = 0;
const PHY_TIMER_1_DQS_ARRAY_STOP_OFFSET: u32 = 8;
const PHY_TIMER_1_ARRAY_WRITE_OFFSET: u32 = 16;
const PHY_TIMER_1_ARRAY_READ_OFFSET: u32 = 24;

const PHY_TIMER_2_AUTO_REFRESH_OFFSET: u32 = 0;
const PHY_TIMER_2_REG_WRITE_OFFSET: u32 = 8;
const PHY_TIMER_2_REG_READ_OFFSET: u32 = 16;
const PHY_TIMER_2_DQS_STOP_OFFSET: u32 = 24;

const PHY_TIMER_3_SELF_REFRESH1_IN_OFFSET: u32 = 0;
const PHY_TIMER_3_SELF_REFRESH1_EXIT_OFFSET: u32 = 8;
const PHY_TIMER_3_GLOBAL_RST_OFFSET: u32 = 16;

const PHY_CFG_40_UNK0_OFFSET: u32 = 16;
const PHY_CFG_40_UNK1_OFFSET: u32 = 20;
const PHY_CFG_40_DMY0_OFFSET: u32 = 8;
const PHY_CFG_40_UNK0_MASK: u32 = 0b11 << PHY_CFG_40_UNK0_OFFSET;
const PHY_CFG_40_UNK1_MASK: u32 = 0b11 << PHY_CFG_40_UNK1_OFFSET;
const PHY_CFG_40_DMY0_MASK: u32 = 0b1111_1111 << PHY_CFG_40_DMY0_OFFSET;

const PHY_TIMER_4_ARRAY_READ_BUSY_OFFSET: u32 = 0;
const PHY_TIMER_4_ARRAY_WRITE_BUSY_OFFSET: u32 = 8;
const PHY_TIMER_4_REG_READ_BUSY_OFFSET: u32 = 16;
const PHY_TIMER_4_REG_WRITE_BUSY_OFFSET: u32 = 24;

const PHY_CFG_48_PSRAM_TYPE_OFFSET: u32 = 8;
const PHY_CFG_48_PSRAM_TYPE_MASK: u32 = 0b11 << PHY_CFG_48_PSRAM_TYPE_OFFSET;

const PHY_CFG_4C_ODT_SEL_DLY_OFFSET: u32 = 16;
const PHY_CFG_4C_ODT_SEL_DLY_MASK: u32 = 0b1111 << PHY_CFG_4C_ODT_SEL_DLY_OFFSET;
const PHY_CFG_4C_ODT_SEL_HW_BIT: u32 = 20;

const PHY_CFG_50_DQ_OE_MID_P_OFFSET: u32 = 8;
const PHY_CFG_50_DQ_OE_MID_N_OFFSET: u32 = 12;
const PHY_CFG_50_DQ_OE_UP_P_OFFSET: u32 = 0;
const PHY_CFG_50_DQ_OE_UP_N_OFFSET: u32 = 4;
const PHY_CFG_50_DQ_OE_DN_P_OFFSET: u32 = 16;
const PHY_CFG_50_DQ_OE_DN_N_OFFSET: u32 = 20;
const PHY_CFG_50_WL_CEN_ANA_OFFSET: u32 = 24;
const PHY_CFG_50_DQ_OE_MID_P_MASK: u32 = 0b11 << PHY_CFG_50_DQ_OE_MID_P_OFFSET;
const PHY_CFG_50_DQ_OE_MID_N_MASK: u32 = 0b11 << PHY_CFG_50_DQ_OE_MID_N_OFFSET;
const PHY_CFG_50_WL_CEN_ANA_MASK: u32 = 0b111 << PHY_CFG_50_WL_CEN_ANA_OFFSET;

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

struct PhyCfg {
    wl_dq_dig: u32,
    wl_dq_ana: u32,
    wl_dig: u32,
    wl_ana: u32,
    rl_dig: u32,
    rl_ana: u32,
    //oe_timer: u32,
    timer_dqs_start: u32,
    timer_dqs_array_stop: u32,
    timer_array_write: u32,
    timer_array_read: u32,

    timer_auto_refresh: u32,
    timer_reg_read: u32,
    timer_reg_write: u32,
    timer_dqs_stop: u32,

    timer_self_refresh1_in: u32,
    timer_self_refresh1_exit: u32,
    timer_global_rst: u32,

    timer_array_read_busy: u32,
    timer_array_write_busy: u32,
    timer_reg_read_busy: u32,
    timer_reg_write_busy: u32,

    wl_cen_ana: u32,
}

/* cfg_30..44 = 0f130010 05000101 02080108 03420909 040b0408 */
const CFG_666: PhyCfg = PhyCfg {
    wl_dq_dig: 0,
    wl_dq_ana: 1,
    wl_dig: 0,
    wl_ana: 0,
    rl_dig: 3,
    rl_ana: 1,
    //oe_timer: 15,
    timer_dqs_start: 1,
    timer_dqs_array_stop: 1,
    timer_array_write: 0,
    timer_array_read: 5,

    timer_auto_refresh: 8,
    timer_reg_read: 1,
    timer_reg_write: 8,
    timer_dqs_stop: 2,

    timer_self_refresh1_in: 9,
    timer_self_refresh1_exit: 9,
    timer_global_rst: 834,

    timer_array_read_busy: 8,
    timer_array_write_busy: 4,
    timer_reg_read_busy: 11,
    timer_reg_write_busy: 4,

    wl_cen_ana: 0,
};

/* cfg_30..44 = 0f270212 06010202 0309020d 05360e0e 050c0509 */
const CFG_1066: PhyCfg = PhyCfg {
    wl_dq_dig: 2,
    wl_dq_ana: 1,
    wl_dig: 2,
    wl_ana: 0,
    rl_dig: 7,
    rl_ana: 2,
    //oe_timer: 15,
    timer_dqs_start: 2,
    timer_dqs_array_stop: 2,
    timer_array_write: 1,
    timer_array_read: 6,

    timer_auto_refresh: 13,
    timer_reg_read: 2,
    timer_reg_write: 9,
    timer_dqs_stop: 3,

    timer_self_refresh1_in: 14,
    timer_self_refresh1_exit: 14,
    timer_global_rst: 1334,

    timer_array_read_busy: 9,
    timer_array_write_busy: 5,
    timer_reg_read_busy: 12,
    timer_reg_write_busy: 5,

    wl_cen_ana: 1,
};

/* cfg_30..44 = 0f270212 09020303 040c0313 07d11515 060f060c */
const CFG_1600: PhyCfg = PhyCfg {
    wl_dq_dig: 2,
    wl_dq_ana: 1,
    wl_dig: 2,
    wl_ana: 0,
    rl_dig: 7,
    rl_ana: 2,
    //oe_timer: 15,
    timer_dqs_start: 3,
    timer_dqs_array_stop: 3,
    timer_array_write: 2,
    timer_array_read: 9,

    timer_auto_refresh: 19,
    timer_reg_read: 3,
    timer_reg_write: 12,
    timer_dqs_stop: 4,

    timer_self_refresh1_in: 21,
    timer_self_refresh1_exit: 21,
    timer_global_rst: 2001,

    timer_array_read_busy: 12,
    timer_array_write_busy: 6,
    timer_reg_read_busy: 15,
    timer_reg_write_busy: 6,

    wl_cen_ana: 1,
};

// TODO: Use bitfield structs and Tock registers?
pub fn config_uhs_phy() {
    // NOTE: The C code panics earlier for freq >= 2200.
    let cfg = match P_CLOCK_FREQUENCY {
        1866.. => todo!(),
        1600..1866 => CFG_1600,
        1066..1600 => CFG_1066,
        800..1066 => todo!(),
        666..800 => CFG_666,
        400..666 => todo!(),
        _ => todo!(),
    };

    let cfg30 = (cfg.wl_dq_dig << PHY_CFG_30_WL_DQ_DIG_OFFSET)
        | (cfg.wl_dq_ana << PHY_CFG_30_WL_DQ_ANA_OFFSET)
        | (cfg.wl_dig << PHY_CFG_30_WL_DIG_OFFSET)
        | (cfg.wl_ana << PHY_CFG_30_WL_ANA_OFFSET)
        | (cfg.rl_dig << PHY_CFG_30_RL_DIG_OFFSET)
        | (cfg.rl_ana << PHY_CFG_30_RL_ANA_OFFSET)
        // NOTE: commented out in struct
        // | (cfg.oe_timer << PHY_CFG_30_OE_TIMER_OFFSET)
        | (3 << PHY_CFG_30_OE_TIMER_OFFSET)
        | (1 << PHY_CFG_30_VREF_MODE_BIT)
        | (1 << PHY_CFG_30_OE_CTRL_HW_BIT);
    write32(PHY_CFG_30, cfg30);

    let cfg_timer1 = (cfg.timer_dqs_start << PHY_TIMER_1_DQS_START_OFFSET)
        | (cfg.timer_dqs_array_stop << PHY_TIMER_1_DQS_ARRAY_STOP_OFFSET)
        | (cfg.timer_array_write << PHY_TIMER_1_ARRAY_WRITE_OFFSET)
        | (cfg.timer_array_read << PHY_TIMER_1_ARRAY_READ_OFFSET);
    write32(PHY_TIMER_1, cfg_timer1);

    let cfg_timer2 = (cfg.timer_auto_refresh << PHY_TIMER_2_AUTO_REFRESH_OFFSET)
        | (cfg.timer_reg_write << PHY_TIMER_2_REG_WRITE_OFFSET)
        | (cfg.timer_reg_read << PHY_TIMER_2_REG_READ_OFFSET)
        | (cfg.timer_dqs_stop << PHY_TIMER_2_DQS_STOP_OFFSET);
    write32(PHY_TIMER_2, cfg_timer2);

    let cfg_timer3 = (cfg.timer_self_refresh1_in << PHY_TIMER_3_SELF_REFRESH1_IN_OFFSET)
        | (cfg.timer_self_refresh1_exit << PHY_TIMER_3_SELF_REFRESH1_EXIT_OFFSET)
        | (cfg.timer_global_rst << PHY_TIMER_3_GLOBAL_RST_OFFSET);
    write32(PHY_TIMER_3, cfg_timer3);

    let cfg_timer4 = (cfg.timer_array_read_busy << PHY_TIMER_4_ARRAY_READ_BUSY_OFFSET)
        | (cfg.timer_array_write_busy << PHY_TIMER_4_ARRAY_WRITE_BUSY_OFFSET)
        | (cfg.timer_reg_read_busy << PHY_TIMER_4_REG_READ_BUSY_OFFSET)
        | (cfg.timer_reg_write_busy << PHY_TIMER_4_REG_WRITE_BUSY_OFFSET);
    write32(PHY_TIMER_4, cfg_timer4);

    let cfg50 = read32(PHY_CFG_50);
    let m = !(PHY_CFG_50_WL_CEN_ANA_MASK);
    write32(
        PHY_CFG_50,
        (cfg50 & m) | (cfg.wl_cen_ana << PHY_CFG_50_WL_CEN_ANA_OFFSET),
    );
}

pub fn init() {
    //
    // TIMING_CTRL_TRFC_CYCLE
    // TIMING_CTRL_TCPHW_CYCLE
    // TIMING_CTRL_TCPHR_CYCLE
    // TIMING_CTRL_TRC_CYCLE
    let timing = match P_CLOCK_FREQUENCY {
        // FIXME: guarantee at build time
        2200.. => panic!("..."),
        1800..2200 => todo!(),
        1500..1800 => todo!(),
        1400..1500 => (18 << 24) | (2 << 16) | 11,
        666..1400 => todo!(),
        _ => todo!(),
    };
    write32(TIMING_CTRL, timing);

    println!("analog init");
    analog_init();
    println!("configure ultra high-speed PHY");
    config_uhs_phy();
    udelay(150);

    // refresh parameter
    let p_clock_t_divider = match P_CLOCK_FREQUENCY {
        // NOTE: This would panic above.
        2200.. => 5,
        1800..2200 => 4,
        1500..1800 => 3,
        1400..1500 => 2,
        666..1400 => 1,
        _ => 0,
    };
    let manual = read32(MANUAL);
    let m = !MANUAL_P_CLOCK_T_DIVIDER_MASK;
    let v = (p_clock_t_divider << MANUAL_P_CLOCK_T_DIVIDER_OFFSET);
    write32(MANUAL, (manual & m) | v);

    // set refresh windows cycle count
    let (auto_refresh_1, auto_refresh_2) = if EXT_TEMP_RANGE {
        (1500000, 190)
    } else {
        (750000, 370)
    };
    write32(AUTO_FRESH_1, auto_refresh_1);
    let af2 = read32(AUTO_FRESH_2);
    let m = !AUTO_FRESH_2_REFI_CYCLE_MASK;
    write32(AUTO_FRESH_2, (af2 & m) | auto_refresh_2);

    let af4 = read32(AUTO_FRESH_4);
    let m = !AUTO_FRESH_4_BUST_CYCLE_MASK;
    write32(AUTO_FRESH_4, (af4 & m) | 5);

    let basic = read32(BASIC);
    let m = !(BASIC_ADDRMB_MSK_MASK | BASIC_LINEAR_BND_B_MASK);
    let v = ((MEM_SIZE - 1) << BASIC_ADDRMB_MSK_OFFSET)
        | (PAGE_SIZE << BASIC_LINEAR_BND_B_OFFSET)
        | BASIC_AF_EN_BIT;
    write32(BASIC, (basic & m) | v);

    let basic = read32(BASIC);
    write32(BASIC, basic | BASIC_INIT_EN_BIT);

    println!("PSRAM init done :)");
}
