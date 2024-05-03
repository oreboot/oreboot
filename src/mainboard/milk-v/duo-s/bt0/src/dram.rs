use crate::ddr_phy::phy_init;
use crate::mem_map::{
    DDR_BIST_BASE, DDR_CFG_BASE, DDR_TOP_BASE, DRAM_BASE, PHYD_APB, PHYD_BASE_ADDR, TOP_BASE,
};
use crate::util::{read32, write32};

// plat/cv181x/include/ddr/bitwise_ops.h
/*
// sets the bits of `orig` in the range of the last two args to `value`
// i.e. `(orig & val_mask) | shifted_val`
static inline uint32_t modified_bits_by_value(uint32_t orig, uint32_t value, uint32_t msb, uint32_t lsb)
{
        uint32_t bitmask = GENMASK(msb, lsb);

        orig &= ~bitmask;
        return (orig | ((value << lsb) & bitmask));
}
// extracts the bits in the range given by the two last args, shifted to last arg
static inline uint32_t get_bits_from_value(uint32_t value, uint32_t msb, uint32_t lsb)
{
        // if (msb < lsb)
        //     uartlog("%s: msb %u < lsb %u\n", __func__, msb, lsb);
        return ((value & GENMASK(msb, lsb)) >> lsb);
}
*/

// NOTE: SSC_EN is commented out in plat/cv18{0,1}x/ddr/ddr.mk
const SSC_EN: bool = false;
// NOTE: SSC_BYPASS is never set
const SSC_BYPASS: bool = false;

const DDR_PLL: usize = PHYD_APB + 0x000c;
const TX_VREF_PD: usize = PHYD_APB + 0x0028;
const ZQ_240_OPTION: usize = PHYD_APB + 0x0054;
const GPO_SETTING: usize = PHYD_APB + 0x0058;

// TRM alpha p62
const CLK_GEN_PLL_CTRL_BASE: usize = 0x0300_2000;
// TRM alpha p53
const PLL_G6_BASE: usize = CLK_GEN_PLL_CTRL_BASE + 0x0900;
const DPLL_SSC_SYN_CTRL: usize = PLL_G6_BASE + 0x0050;
const DPLL_SSC_SYN_SET: usize = PLL_G6_BASE + 0x0054;
const DPLL_SSC_SYN_SPAN: usize = PLL_G6_BASE + 0x0058;
const DPLL_SSC_SYN_STEP: usize = PLL_G6_BASE + 0x005C;

// TODO: All of this would be a build-time config.
const X16_MODE: bool = true;
const DO_BIST: bool = true;
const DBG_SHMOO: bool = false;
const DDR2: bool = false;
const DDR2_3: bool = false;
const DDR3: bool = true;
const DDR4: bool = false;
// TODO: enum
const DDR_TYPE_DDR2: u32 = 0;
const DDR_TYPE_DDR3: u32 = 1;
fn get_ddr_type() -> u32 {
    DDR_TYPE_DDR3
}

// NOTE: CTRL settings are hardcoded; for SSC, add params to this fn
fn set_dpll_ssc_syn(reg_set: u32, reg_span: u32, reg_step: u32) {
    let ctrl_cfg = 0b010000;

    write32(DPLL_SSC_SYN_SET, reg_set);
    // 15..0
    write32(DPLL_SSC_SYN_SPAN, reg_span);
    // 23..0
    write32(DPLL_SSC_SYN_STEP, reg_step);

    // 6: FIX_DIV
    // 5: EXTPULSE
    // 4: BYPASS
    // 3..2: MODE
    // 1: EN_SSC
    // 0: SW_UP
    let v = read32(DPLL_SSC_SYN_CTRL);
    println!("DPLL_SSC_SYN_CTRL {v:032b}");
    // invert SW_UP
    let neg_sw_up = !(v & 0x1) & 0x1;
    let v = (v & !(0b1111111)) | (ctrl_cfg << 1) | neg_sw_up;
    println!("DPLL_SSC_SYN_CTRL {v:032b}");
    write32(DPLL_SSC_SYN_CTRL, v);
    println!("SSC_OFF");
}

fn cvx16_pll_init(reg_set: u32, reg_span: u32, reg_step: u32) {
    println!("cvx16_pll_init");
    // opdelay(10);
    write32(TX_VREF_PD, 0x0000_0000);
    write32(ZQ_240_OPTION, 0x0008_0001);

    const DDR3: bool = true;
    let x_mem_freq_2133 = false;

    // TODO: variants
    if DDR3 {
        if x_mem_freq_2133 {
            write32(GPO_SETTING, 0x0100_0808);
        } else {
            const TX_DDR3_GPO_IN: u32 = 1 << 16;
            write32(GPO_SETTING, 0x0100_0808 | TX_DDR3_GPO_IN);
        }
    }
    /*
    #ifdef DDR2_3
    if (get_ddr_type() == DDR_TYPE_DDR3) {
        write32(GPO_SETTING, 0x0100_0808 | TX_DDR3_GPO_IN);
    }
    #endif
    */

    if SSC_EN {
        /*
        //==============================================================
        // Enable SSC
        //==============================================================
        rddata = reg_set; // TOP_REG_SSC_SET
        write32(0x54 + 0x03002900, rddata);
        rddata = get_bits_from_value(reg_span, 15, 0); // TOP_REG_SSC_SPAN
        write32(0x58 + 0x03002900, rddata);
        rddata = get_bits_from_value(reg_step, 23, 0); // TOP_REG_SSC_STEP
        write32(0x5C + 0x03002900, rddata);
        KC_MSG("reg_step = %lx\n", reg_step);

        rddata = read32(0x50 + 0x03002900);
        rddata = modified_bits_by_value(rddata, ~get_bits_from_value(rddata, 0, 0), 0, 0); // TOP_REG_SSC_SW_UP
        rddata = modified_bits_by_value(rddata, 1, 1, 1); // TOP_REG_SSC_EN_SSC
        rddata = modified_bits_by_value(rddata, 0, 3, 2); // TOP_REG_SSC_SSC_MODE
        rddata = modified_bits_by_value(rddata, 0, 4, 4); // TOP_REG_SSC_BYPASS
        rddata = modified_bits_by_value(rddata, 1, 5, 5); // extpulse
        rddata = modified_bits_by_value(rddata, 0, 6, 6); // ssc_syn_fix_div
        write32(0x50 + 0x03002900, rddata);
        */
        println!("SSC_EN");
    } else if SSC_BYPASS {
        /*
        rddata = (reg_set & 0xfc000000) + 0x04000000; // TOP_REG_SSC_SET
        write32(0x54 + 0x03002900, rddata);
        rddata = get_bits_from_value(reg_span, 15, 0); // TOP_REG_SSC_SPAN
        write32(0x58 + 0x03002900, rddata);
        rddata = get_bits_from_value(reg_step, 23, 0); // TOP_REG_SSC_STEP
        write32(0x5C + 0x03002900, rddata);
        rddata = read32(0x50 + 0x03002900);
        rddata = modified_bits_by_value(rddata, ~get_bits_from_value(rddata, 0, 0), 0, 0); // TOP_REG_SSC_SW_UP
        rddata = modified_bits_by_value(rddata, 0, 1, 1); // TOP_REG_SSC_EN_SSC
        rddata = modified_bits_by_value(rddata, 0, 3, 2); // TOP_REG_SSC_SSC_MODE
        rddata = modified_bits_by_value(rddata, 0, 4, 4); // TOP_REG_SSC_BYPASS
        rddata = modified_bits_by_value(rddata, 1, 5, 5); // TOP_REG_SSC_EXTPULSE
        rddata = modified_bits_by_value(rddata, 1, 6, 6); // ssc_syn_fix_div
        write32(0x50 + 0x03002900, rddata);
        uartlog("SSC_BYPASS\n");
        */
    } else {
        println!("SSC_EN = 0");
        set_dpll_ssc_syn(reg_set, reg_span, reg_step);
    }

    // opdelay(1000);
    // DDRPLL setting
    //[0]    = 1;      //TOP_REG_DDRPLL_EN_DLLCLK
    //[1]    = 1;      //TOP_REG_DDRPLL_EN_LCKDET
    //[2]    = 0;      //TOP_REG_DDRPLL_EN_TST
    //[5:3]  = 0b001; //TOP_REG_DDRPLL_ICTRL
    //[6]    = 0;      //TOP_REG_DDRPLL_MAS_DIV_SEL
    //[7]    = 0;      //TOP_REG_DDRPLL_MAS_RSTZ_DIV
    //[8]    = 1;      //TOP_REG_DDRPLL_SEL_4BIT
    //[10:9] = 0b01;  //TOP_REG_DDRPLL_SEL_MODE
    //[12:11]= 0b00;  //Rev
    //[13]   = 0;      //TOP_REG_DDRPLL_SEL_LOW_SPEED
    //[14]   = 0;      //TOP_REG_DDRPLL_MAS_DIV_OUT_SEL
    //[15]   = 0;      //TOP_REG_DDRPLL_PD
    let v = read32(DDR_PLL);
    write32(DDR_PLL, (v & 0xffff_0000) | 0x030b);

    // DDRPLL_TEST
    // [7:0] = 0x0;
    let v = read32(PHYD_APB + 0x10);
    write32(PHYD_APB + 0x10, v & 0xffff_ff00);

    // RESETZ_DIV
    // [0]   = 1;
    write32(PHYD_APB + 0x04, 0x1);

    // DDRPLL_MAS_RSTZ_DIV
    // [7]   = 1;
    let v = read32(DDR_PLL);
    write32(DDR_PLL, v | (1 << 7));

    println!("Wait for DDR PLL LOCK=1...");
    while read32(PHYD_APB + 0x10) & (1 << 15) == 0 {}
    println!("Finished DDR PLL LOCK=1.");
    println!("PLL init done.");
}

fn get_pll_settings(ddr_data_rate: usize) -> (u32, u32, u32) {
    let freq_in = 752;
    let mod_freq = 100;
    let dev_freq = 15;
    println!("Data rate = {ddr_data_rate}");
    let mut tar_freq = ddr_data_rate >> 4;
    if SSC_EN {
        tar_freq = (tar_freq as f32 * 0.985) as usize;
    };
    println!("tar_freq {tar_freq}");

    let reg_set = (freq_in * 67108864 / tar_freq) as u32;
    let reg_span = ((tar_freq * 250) / mod_freq) as u32;
    let reg_step = reg_set * dev_freq / (reg_span * 1000);
    println!("reg_set  {:032b} ({reg_set})", reg_set);
    println!("reg_span {:032b} ({reg_span})", reg_span);
    println!("reg_step {:032b} ({reg_step})", reg_step);

    (reg_set, reg_span, reg_step)
}

const DDR_INIT_SPEED_UP: bool = false;
const DDR_DODT: bool = false;

fn ddrc_init() {
    println!("DDRC init");
    let v = read32(DDR_CFG_BASE + 0xc);
    println!("DDRC 0x000c {v:08x}");
    // "ctcq" (qctc)
    write32(DDR_CFG_BASE + 0xc, 0x63746371);
    let v = read32(DDR_CFG_BASE + 0xc);
    println!("DDRC 0x000c {v:08x}");
    // PATCH0.use_blk_ext}:0:2:=0x1
    // PATCH0.dis_auto_ref_cnt_fix:2:1:=0x0
    // PATCH0.dis_auto_ref_algn_to_8:3:1:=0x0
    // PATCH0.starve_stall_at_dfi_ctrlupd:4:1:=0x1
    // PATCH0.starve_stall_at_abr:5:1:=0x1
    // PATCH0.dis_rdwr_switch_at_abr:6:1:=0x1
    // PATCH0.dfi_wdata_same_to_axi:7:1:=0x0
    // PATCH0.pagematch_limit_threshold:8:3=0x3
    // PATCH0.qos_sel:12:2:=0x2
    // PATCH0.burst_rdwr_xpi:16:4:=0x4
    // PATCH0.always_critical_when_urgent_hpr:20:1:=0x1
    // PATCH0.always_critical_when_urgent_lpr:21:1:=0x1
    // PATCH0.always_critical_when_urgent_wr:22:1:=0x1
    // PATCH0.disable_hif_rcmd_stall_path:24:1:=0x1
    // PATCH0.disable_hif_wcmd_stall_path:25:1:=0x1
    // PATCH0.derate_sys_en:29:1:=0x1
    // PATCH0.ref_4x_sys_high_temp:30:1:=0x1
    write32(DDR_CFG_BASE + 0x44, 0x00000000);
    // PATCH1.ref_adv_stop_threshold:0:7:=0x0
    // PATCH1.ref_adv_dec_threshold:8:7:=0x0
    // PATCH1.ref_adv_max:16:7:=0x0
    write32(DDR_CFG_BASE + 0x148, 0x999F0000);
    // PATCH4.t_phyd_rden:16:6=0x0
    // PATCH4.phyd_rd_clk_stop:23:1=0x0
    // PATCH4.t_phyd_wren:24:6=0x0
    // PATCH4.phyd_wr_clk_stop:31:1=0x0
    // auto gen.
    write32(DDR_CFG_BASE + 0x0, 0x81041401);
    write32(DDR_CFG_BASE + 0x30, 0x00000000);
    write32(DDR_CFG_BASE + 0x34, 0x00930001);
    write32(DDR_CFG_BASE + 0x38, 0x00020000);
    write32(DDR_CFG_BASE + 0x50, 0x00201070);
    write32(DDR_CFG_BASE + 0x60, 0x00000000);
    write32(DDR_CFG_BASE + 0x64, 0x007100A4);
    write32(DDR_CFG_BASE + 0xc0, 0x00000000);
    write32(DDR_CFG_BASE + 0xc4, 0x00000000);
    if DDR_INIT_SPEED_UP {
        write32(DDR_CFG_BASE + 0xd0, 0x00010002);
        write32(DDR_CFG_BASE + 0xd4, 0x00020000);
    } else {
        write32(DDR_CFG_BASE + 0xd0, 0x000100E5);
        write32(DDR_CFG_BASE + 0xd4, 0x006A0000);
    }
    write32(DDR_CFG_BASE + 0xdc, 0x1F140040);
    if DDR_DODT {
        write32(DDR_CFG_BASE + 0xe0, 0x04600000);
    } else {
        write32(DDR_CFG_BASE + 0xe0, 0x00600000);
    }
    write32(DDR_CFG_BASE + 0xe4, 0x000B03BF);
    write32(DDR_CFG_BASE + 0x100, 0x0E111F10);
    write32(DDR_CFG_BASE + 0x104, 0x00030417);
    write32(DDR_CFG_BASE + 0x108, 0x0507060A);
    write32(DDR_CFG_BASE + 0x10c, 0x00002007);
    write32(DDR_CFG_BASE + 0x110, 0x07020307);
    write32(DDR_CFG_BASE + 0x114, 0x05050303);
    write32(DDR_CFG_BASE + 0x120, 0x00000907);
    write32(DDR_CFG_BASE + 0x13c, 0x00000000);
    write32(DDR_CFG_BASE + 0x180, 0xC0960026);
    write32(DDR_CFG_BASE + 0x184, 0x00000001);
    // phyd related
    write32(DDR_CFG_BASE + 0x190, 0x048a8305);
    // DFITMG0.dfi_t_ctrl_delay:24:5:=0x4
    // DFITMG0.dfi_rddata_use_dfi_phy_clk:23:1:=0x1
    // DFITMG0.dfi_t_rddata_en:16:7:=0xa
    // DFITMG0.dfi_wrdata_use_dfi_phy_clk:15:1:=0x1
    // DFITMG0.dfi_tphy_wrdata:8:6:=0x3
    // DFITMG0.dfi_tphy_wrlat:0:6:=0x5
    write32(DDR_CFG_BASE + 0x194, 0x00070202);
    // DFITMG1.dfi_t_cmd_lat:28:4:=0x0
    // DFITMG1.dfi_t_parin_lat:24:2:=0x0
    // DFITMG1.dfi_t_wrdata_delay:16:5:=0x7
    // DFITMG1.dfi_t_dram_clk_disable:8:5:=0x2
    // DFITMG1.dfi_t_dram_clk_enable:0:5:=0x2
    write32(DDR_CFG_BASE + 0x198, 0x07c13121);
    // DFILPCFG0.dfi_tlp_resp:24:5:=0x7
    // DFILPCFG0.dfi_lp_wakeup_dpd:20:4:=0xc
    // DFILPCFG0.dfi_lp_en_dpd:16:1:=0x1
    // DFILPCFG0.dfi_lp_wakeup_sr:12:4:=0x3
    // DFILPCFG0.dfi_lp_en_sr:8:1:=0x1
    // DFILPCFG0.dfi_lp_wakeup_pd:4:4:=0x2
    // DFILPCFG0.dfi_lp_en_pd:0:1:=0x1
    write32(DDR_CFG_BASE + 0x19c, 0x00000021);
    // DFILPCFG1.dfi_lp_wakeup_mpsm:4:4:=0x2
    // DFILPCFG1.dfi_lp_en_mpsm:0:1:=0x1
    // auto gen.
    write32(DDR_CFG_BASE + 0x1a0, 0xC0400018);
    write32(DDR_CFG_BASE + 0x1a4, 0x00FE00FF);
    write32(DDR_CFG_BASE + 0x1a8, 0x80000000);
    write32(DDR_CFG_BASE + 0x1b0, 0x000002C1);
    write32(DDR_CFG_BASE + 0x1c0, 0x00000001);
    write32(DDR_CFG_BASE + 0x1c4, 0x00000001);
    // address map, auto gen.
    write32(DDR_CFG_BASE + 0x200, 0x00001F1F);
    write32(DDR_CFG_BASE + 0x204, 0x00070707);
    write32(DDR_CFG_BASE + 0x208, 0x00000000);
    write32(DDR_CFG_BASE + 0x20c, 0x1F000000);
    write32(DDR_CFG_BASE + 0x210, 0x00001F1F);
    write32(DDR_CFG_BASE + 0x214, 0x060F0606);
    write32(DDR_CFG_BASE + 0x218, 0x06060606);
    write32(DDR_CFG_BASE + 0x21c, 0x00000606);
    write32(DDR_CFG_BASE + 0x220, 0x00003F3F);
    write32(DDR_CFG_BASE + 0x224, 0x06060606);
    write32(DDR_CFG_BASE + 0x228, 0x06060606);
    write32(DDR_CFG_BASE + 0x22c, 0x001F1F06);
    // auto gen.
    write32(DDR_CFG_BASE + 0x240, 0x08000610);
    if DDR_DODT {
        write32(DDR_CFG_BASE + 0x244, 0x00000001);
    } else {
        write32(DDR_CFG_BASE + 0x244, 0x00000000);
    }
    write32(DDR_CFG_BASE + 0x250, 0x00003F85);
    // SCHED.opt_vprw_sch:31:1:=0x0
    // SCHED.rdwr_idle_gap:24:7:=0x0
    // SCHED.go2critical_hysteresis:16:8:=0x0
    // SCHED.lpddr4_opt_act_timing:15:1:=0x0
    // SCHED.lpr_num_entries:8:7:=0x1f
    // SCHED.autopre_rmw:7:1:=0x1
    // SCHED.dis_opt_ntt_by_pre:6:1:=0x0
    // SCHED.dis_opt_ntt_by_act:5:1:=0x0
    // SCHED.opt_wrcam_fill_level:4:1:=0x0
    // SCHED.rdwr_switch_policy_sel:3:1:=0x0
    // SCHED.pageclose:2:1:=0x1
    // SCHED.prefer_write:1:1:=0x0
    // SCHED.dis_opt_wrecc_collision_flush:0:1:=0x1
    write32(DDR_CFG_BASE + 0x254, 0x00000000);
    // SCHED1.page_hit_limit_rd:28:3:=0x0
    // SCHED1.page_hit_limit_wr:24:3:=0x0
    // SCHED1.visible_window_limit_rd:20:3:=0x0
    // SCHED1.visible_window_limit_wr:16:3:=0x0
    // SCHED1.delay_switch_write:12:4:=0x0
    // SCHED1.pageclose_timer:0:8:=0x0
    // auto gen.
    write32(DDR_CFG_BASE + 0x25c, 0x100000F0);
    // PERFHPR1.hpr_xact_run_length:24:8:=0x20
    // PERFHPR1.hpr_max_starve:0:16:=0x6a
    write32(DDR_CFG_BASE + 0x264, 0x100000F0);
    // PERFLPR1.lpr_xact_run_length:24:8:=0x20
    // PERFLPR1.lpr_max_starve:0:16:=0x6a
    write32(DDR_CFG_BASE + 0x26c, 0x100000F0);
    // PERFWR1.w_xact_run_length:24:8:=0x20
    // PERFWR1.w_max_starve:0:16:=0x1a8
    write32(DDR_CFG_BASE + 0x300, 0x00000000);
    // DBG0.dis_max_rank_wr_opt:7:1:=0x0
    // DBG0.dis_max_rank_rd_opt:6:1:=0x0
    // DBG0.dis_collision_page_opt:4:1:=0x0
    // DBG0.dis_act_bypass:2:1:=0x0
    // DBG0.dis_rd_bypass:1:1:=0x0
    // DBG0.dis_wc:0:1:=0x0
    write32(DDR_CFG_BASE + 0x304, 0x00000000);
    // DBG1.dis_hif:1:1:=0x0
    // DBG1.dis_dq:0:1:=0x0
    write32(DDR_CFG_BASE + 0x30c, 0x00000000);
    write32(DDR_CFG_BASE + 0x320, 0x00000001);
    // SWCTL.sw_done:0:1:=0x1
    write32(DDR_CFG_BASE + 0x36c, 0x00000000);
    // POISONCFG.rd_poison_intr_clr:24:1:=0x0
    // POISONCFG.rd_poison_intr_en:20:1:=0x0
    // POISONCFG.rd_poison_slverr_en:16:1:=0x0
    // POISONCFG.wr_poison_intr_clr:8:1:=0x0
    // POISONCFG.wr_poison_intr_en:4:1:=0x0
    // POISONCFG.wr_poison_slverr_en:0:1:=0x0
    write32(DDR_CFG_BASE + 0x400, 0x00000011);
    // PCCFG.dch_density_ratio:12:2:=0x0
    // PCCFG.bl_exp_mode:8:1:=0x0
    // PCCFG.pagematch_limit:4:1:=0x1
    // PCCFG.go2critical_en:0:1:=0x1
    write32(DDR_CFG_BASE + 0x404, 0x00006000);
    // PCFGR_0.rdwr_ordered_en:16:1:=0x0
    // PCFGR_0.rd_port_pagematch_en:14:1:=0x1
    // PCFGR_0.rd_port_urgent_en:13:1:=0x1
    // PCFGR_0.rd_port_aging_en:12:1:=0x0
    // PCFGR_0.read_reorder_bypass_en:11:1:=0x0
    // PCFGR_0.rd_port_priority:0:10:=0x0
    write32(DDR_CFG_BASE + 0x408, 0x00006000);
    // PCFGW_0.wr_port_pagematch_en:14:1:=0x1
    // PCFGW_0.wr_port_urgent_en:13:1:=0x1
    // PCFGW_0.wr_port_aging_en:12:1:=0x0
    // PCFGW_0.wr_port_priority:0:10:=0x0
    write32(DDR_CFG_BASE + 0x490, 0x00000001);
    // PCTRL_0.port_en:0:1:=0x1
    write32(DDR_CFG_BASE + 0x494, 0x00000007);
    // PCFGQOS0_0.rqos_map_region2:24:8:=0x0
    // PCFGQOS0_0.rqos_map_region1:20:4:=0x0
    // PCFGQOS0_0.rqos_map_region0:16:4:=0x0
    // PCFGQOS0_0.rqos_map_level2:8:8:=0x0
    // PCFGQOS0_0.rqos_map_level1:0:8:=0x7
    write32(DDR_CFG_BASE + 0x498, 0x0000006a);
    // PCFGQOS1_0.rqos_map_timeoutr:16:16:=0x0
    // PCFGQOS1_0.rqos_map_timeoutb:0:16:=0x6a
    write32(DDR_CFG_BASE + 0x49c, 0x00000e07);
    // PCFGWQOS0_0.wqos_map_region2:24:8:=0x0
    // PCFGWQOS0_0.wqos_map_region1:20:4:=0x0
    // PCFGWQOS0_0.wqos_map_region0:16:4:=0x0
    // PCFGWQOS0_0.wqos_map_level2:8:8:=0xe
    // PCFGWQOS0_0.wqos_map_level1:0:8:=0x7
    write32(DDR_CFG_BASE + 0x4a0, 0x01a801a8);
    // PCFGWQOS1_0.wqos_map_timeout2:16:16:=0x1a8
    // PCFGWQOS1_0.wqos_map_timeout1:0:16:=0x1a8
    write32(DDR_CFG_BASE + 0x4b4, 0x00006000);
    // PCFGR_1.rdwr_ordered_en:16:1:=0x0
    // PCFGR_1.rd_port_pagematch_en:14:1:=0x1
    // PCFGR_1.rd_port_urgent_en:13:1:=0x1
    // PCFGR_1.rd_port_aging_en:12:1:=0x0
    // PCFGR_1.read_reorder_bypass_en:11:1:=0x0
    // PCFGR_1.rd_port_priority:0:10:=0x0
    write32(DDR_CFG_BASE + 0x4b8, 0x00006000);
    // PCFGW_1.wr_port_pagematch_en:14:1:=0x1
    // PCFGW_1.wr_port_urgent_en:13:1:=0x1
    // PCFGW_1.wr_port_aging_en:12:1:=0x0
    // PCFGW_1.wr_port_priority:0:10:=0x0
    write32(DDR_CFG_BASE + 0x540, 0x00000001);
    // PCTRL_1.port_en:0:1:=0x1
    write32(DDR_CFG_BASE + 0x544, 0x00000007);
    // PCFGQOS0_1.rqos_map_region2:24:8:=0x0
    // PCFGQOS0_1.rqos_map_region1:20:4:=0x0
    // PCFGQOS0_1.rqos_map_region0:16:4:=0x0
    // PCFGQOS0_1.rqos_map_level2:8:8:=0x0
    // PCFGQOS0_1.rqos_map_level1:0:8:=0x7
    write32(DDR_CFG_BASE + 0x548, 0x0000006a);
    // PCFGQOS1_1.rqos_map_timeoutr:16:16:=0x0
    // PCFGQOS1_1.rqos_map_timeoutb:0:16:=0x6a
    write32(DDR_CFG_BASE + 0x54c, 0x00000e07);
    // PCFGWQOS0_1.wqos_map_region2:24:8:=0x0
    // PCFGWQOS0_1.wqos_map_region1:20:4:=0x0
    // PCFGWQOS0_1.wqos_map_region0:16:4:=0x0
    // PCFGWQOS0_1.wqos_map_level2:8:8:=0xe
    // PCFGWQOS0_1.wqos_map_level1:0:8:=0x7
    write32(DDR_CFG_BASE + 0x550, 0x01a801a8);
    // PCFGWQOS1_1.wqos_map_timeout2:16:16:=0x1a8
    // PCFGWQOS1_1.wqos_map_timeout1:0:16:=0x1a8
    write32(DDR_CFG_BASE + 0x564, 0x00006000);
    // PCFGR_2.rdwr_ordered_en:16:1:=0x0
    // PCFGR_2.rd_port_pagematch_en:14:1:=0x1
    // PCFGR_2.rd_port_urgent_en:13:1:=0x1
    // PCFGR_2.rd_port_aging_en:12:1:=0x0
    // PCFGR_2.read_reorder_bypass_en:11:1:=0x0
    // PCFGR_2.rd_port_priority:0:10:=0x0
    write32(DDR_CFG_BASE + 0x568, 0x00006000);
    // PCFGW_2.wr_port_pagematch_en:14:1:=0x1
    // PCFGW_2.wr_port_urgent_en:13:1:=0x1
    // PCFGW_2.wr_port_aging_en:12:1:=0x0
    // PCFGW_2.wr_port_priority:0:10:=0x0
    write32(DDR_CFG_BASE + 0x5f0, 0x00000001);
    // PCTRL_2.port_en:0:1:=0x1
    write32(DDR_CFG_BASE + 0x5f4, 0x00000007);
    // PCFGQOS0_2.rqos_map_region2:24:8:=0x0
    // PCFGQOS0_2.rqos_map_region1:20:4:=0x0
    // PCFGQOS0_2.rqos_map_region0:16:4:=0x0
    // PCFGQOS0_2.rqos_map_level2:8:8:=0x0
    // PCFGQOS0_2.rqos_map_level1:0:8:=0x7
    write32(DDR_CFG_BASE + 0x5f8, 0x0000006a);
    // PCFGQOS1_2.rqos_map_timeoutr:16:16:=0x0
    // PCFGQOS1_2.rqos_map_timeoutb:0:16:=0x6a
    write32(DDR_CFG_BASE + 0x5fc, 0x00000e07);
    // PCFGWQOS0_2.wqos_map_region2:24:8:=0x0
    // PCFGWQOS0_2.wqos_map_region1:20:4:=0x0
    // PCFGWQOS0_2.wqos_map_region0:16:4:=0x0
    // PCFGWQOS0_2.wqos_map_level2:8:8:=0xe
    // PCFGWQOS0_2.wqos_map_level1:0:8:=0x7
    write32(DDR_CFG_BASE + 0x600, 0x01a801a8);
    // PCFGWQOS1_2.wqos_map_timeout2:16:16:=0x1a8
    // PCFGWQOS1_2.wqos_map_timeout1:0:16:=0x1a8
}

fn ctrl_init_low_patch() {
    // disable auto PD/SR
    write32(DDR_CFG_BASE + 0x0030, 0x00000000);
    // disable auto ctrl_upd
    write32(DDR_CFG_BASE + 0x01a0, 0xC0400018);
    // disable clock gating
    write32(DDR_CFG_BASE + 0x0014, 0x00000fff);
    // change xpi to single DDR burst
    // write32(DDR_CFG_BASE + 0x000c, 0x63746371);
}

const PHY_REG_VERSION: usize = PHYD_BASE_ADDR + 0x3000;

// plat/cv181x/ddr/ddr_sys.c
fn cvx16_setting_check() {
    println!("/ cvx16_setting_check");

    // NOTE: On Duo S, I get 20210920 - looking like year/month/day
    let phy_reg_version = read32(PHY_REG_VERSION);
    println!("  phy_reg_version {phy_reg_version:08x}");

    const DFITMG0: usize = DDR_CFG_BASE + 0x0190;
    const DFITMG1: usize = DDR_CFG_BASE + 0x0194;

    // NOTE: Those were commented out in the vendor code as well.
    // write32(DDR_CFG_BASE + 0x190, 0x048a8305);
    // write32(DDR_CFG_BASE + 0x194, 0x00070202);

    let v = read32(DFITMG0);
    let dfi_tphy_wrlat = v & 0b11111;
    let dfi_tphy_wrdata = (v >> 8) & 0b111111;
    let dfi_t_rddata_en = (v >> 16) & 0b1111111;
    let dfi_t_ctrl_delay = (v >> 24) & 0b111111;
    println!("  dfi_t_ctrl_delay {dfi_t_ctrl_delay}");
    println!("  dfi_t_rddata_en {dfi_t_rddata_en}");
    println!("  dfi_tphy_wrlat {dfi_tphy_wrlat}");
    println!("  dfi_tphy_wrdata {dfi_tphy_wrdata}");

    let v = read32(DFITMG1);
    let dfi_t_wrdata_delay = (v >> 16) & 0b11111;
    println!("  dfi_t_wrdata_delay {dfi_t_wrdata_delay}");

    // TODO: other DRAM variants
    // 1866
    if (dfi_tphy_wrlat != 0x5) {
        println!("ERR !!! dfi_tphy_wrlat not 0x5");
    }
    if (dfi_tphy_wrdata != 0x3) {
        println!("ERR !!! dfi_tphy_wrdata not 0x3");
    }
    if (dfi_t_rddata_en != 0xa) {
        println!("ERR !!! dfi_t_rddata_en not 0xa");
    }
    if (dfi_t_wrdata_delay != 0x7) {
        println!("ERR !!! dfi_t_wrdata_delay not 0x7");
    }
    println!("\\ cvx16_setting_check finish");
}

// plat/cv181x/ddr/cvx16_pinmux.c
pub fn cvx16_pinmux(ddr_vendor: DramVendor) {
    println!("/ cvx16_pinmux start");
    match ddr_vendor {
        // Duo S
        DramVendor::NY4GbitDDR3 => {
            write32(0x0000 + PHYD_BASE_ADDR, 0x12141013);
            write32(0x0004 + PHYD_BASE_ADDR, 0x0C041503);
            write32(0x0008 + PHYD_BASE_ADDR, 0x06050001);
            write32(0x000C + PHYD_BASE_ADDR, 0x08070B02);
            write32(0x0010 + PHYD_BASE_ADDR, 0x0A0F0E09);
            write32(0x0014 + PHYD_BASE_ADDR, 0x0016110D);
            write32(0x0018 + PHYD_BASE_ADDR, 0x00000000);
            write32(0x001C + PHYD_BASE_ADDR, 0x00000100);
            write32(0x0020 + PHYD_BASE_ADDR, 0x02136574);
            write32(0x0024 + PHYD_BASE_ADDR, 0x00000008);
            write32(0x0028 + PHYD_BASE_ADDR, 0x76512308);
            write32(0x002C + PHYD_BASE_ADDR, 0x00000004);
        }
        DramVendor::NY2GbitDDR3 => {
            write32(0x0000 + PHYD_BASE_ADDR, 0x08070D09);
            write32(0x0004 + PHYD_BASE_ADDR, 0x0605020B);
            write32(0x0008 + PHYD_BASE_ADDR, 0x14040100);
            write32(0x000C + PHYD_BASE_ADDR, 0x15030E0C);
            write32(0x0010 + PHYD_BASE_ADDR, 0x0A0F1213);
            write32(0x0014 + PHYD_BASE_ADDR, 0x00111016);
            write32(0x0018 + PHYD_BASE_ADDR, 0x00000000);
            write32(0x001C + PHYD_BASE_ADDR, 0x00000100);
            write32(0x0020 + PHYD_BASE_ADDR, 0x82135764);
            write32(0x0024 + PHYD_BASE_ADDR, 0x00000000);
            write32(0x0028 + PHYD_BASE_ADDR, 0x67513028);
            write32(0x002C + PHYD_BASE_ADDR, 0x00000004);
        }
        DramVendor::ESMT1GbitDDR2 => {
            write32(0x0000 + PHYD_BASE_ADDR, 0x08070B09);
            write32(0x0004 + PHYD_BASE_ADDR, 0x05000206);
            write32(0x0008 + PHYD_BASE_ADDR, 0x0C04010D);
            write32(0x000C + PHYD_BASE_ADDR, 0x15030A14);
            write32(0x0010 + PHYD_BASE_ADDR, 0x10111213);
            write32(0x0014 + PHYD_BASE_ADDR, 0x000F160E);
            write32(0x0018 + PHYD_BASE_ADDR, 0x00000000);
            write32(0x001C + PHYD_BASE_ADDR, 0x00000100);
            write32(0x0020 + PHYD_BASE_ADDR, 0x31756024);
            write32(0x0024 + PHYD_BASE_ADDR, 0x00000008);
            write32(0x0028 + PHYD_BASE_ADDR, 0x26473518);
            write32(0x002C + PHYD_BASE_ADDR, 0x00000000);
        }
        DramVendor::ESMTN25512MbitDDR2 => {
            write32(0x0000 + PHYD_BASE_ADDR, 0x0C06080B);
            write32(0x0004 + PHYD_BASE_ADDR, 0x070D0904);
            write32(0x0008 + PHYD_BASE_ADDR, 0x00010502);
            write32(0x000C + PHYD_BASE_ADDR, 0x110A0E03);
            write32(0x0010 + PHYD_BASE_ADDR, 0x0F141610);
            write32(0x0014 + PHYD_BASE_ADDR, 0x00151312);
            write32(0x0018 + PHYD_BASE_ADDR, 0x00000000);
            write32(0x001C + PHYD_BASE_ADDR, 0x00000100);
            write32(0x0020 + PHYD_BASE_ADDR, 0x71840532);
            write32(0x0024 + PHYD_BASE_ADDR, 0x00000006);
            write32(0x0028 + PHYD_BASE_ADDR, 0x76103425);
            write32(0x002C + PHYD_BASE_ADDR, 0x00000008);
        }
        DramVendor::ESMT2GbitDDR3 => {
            write32(0x0000 + PHYD_BASE_ADDR, 0x080B0D06);
            write32(0x0004 + PHYD_BASE_ADDR, 0x09010407);
            write32(0x0008 + PHYD_BASE_ADDR, 0x1405020C);
            write32(0x000C + PHYD_BASE_ADDR, 0x15000E03);
            write32(0x0010 + PHYD_BASE_ADDR, 0x0A0F1213);
            write32(0x0014 + PHYD_BASE_ADDR, 0x00111016);
            write32(0x0018 + PHYD_BASE_ADDR, 0x00000000);
            write32(0x001C + PHYD_BASE_ADDR, 0x00000100);
            write32(0x0020 + PHYD_BASE_ADDR, 0x82135764);
            write32(0x0024 + PHYD_BASE_ADDR, 0x00000000);
            write32(0x0028 + PHYD_BASE_ADDR, 0x67513208);
            write32(0x002C + PHYD_BASE_ADDR, 0x00000004);
        }
        DramVendor::ETRON1Gbit => {
            write32(0x0000 + PHYD_BASE_ADDR, 0x0B060908);
            write32(0x0004 + PHYD_BASE_ADDR, 0x02000107);
            write32(0x0008 + PHYD_BASE_ADDR, 0x0C05040D);
            write32(0x000C + PHYD_BASE_ADDR, 0x13141503);
            write32(0x0010 + PHYD_BASE_ADDR, 0x160A1112);
            write32(0x0014 + PHYD_BASE_ADDR, 0x000F100E);
            write32(0x0018 + PHYD_BASE_ADDR, 0x00000000);
            write32(0x001C + PHYD_BASE_ADDR, 0x00000100);
            write32(0x0020 + PHYD_BASE_ADDR, 0x28137564);
            write32(0x0024 + PHYD_BASE_ADDR, 0x00000000);
            write32(0x0028 + PHYD_BASE_ADDR, 0x76158320);
            write32(0x002C + PHYD_BASE_ADDR, 0x00000004);
        }
        DramVendor::ESMTN251GbitDDR3 => {
            write32(0x0000 + PHYD_BASE_ADDR, 0x08060B09);
            write32(0x0004 + PHYD_BASE_ADDR, 0x02040701);
            write32(0x0008 + PHYD_BASE_ADDR, 0x0C00050D);
            write32(0x000C + PHYD_BASE_ADDR, 0x13150314);
            write32(0x0010 + PHYD_BASE_ADDR, 0x10111216);
            write32(0x0014 + PHYD_BASE_ADDR, 0x000F0A0E);
            write32(0x0018 + PHYD_BASE_ADDR, 0x00000000);
            write32(0x001C + PHYD_BASE_ADDR, 0x00000100);
            write32(0x0020 + PHYD_BASE_ADDR, 0x82135674);
            write32(0x0024 + PHYD_BASE_ADDR, 0x00000000);
            write32(0x0028 + PHYD_BASE_ADDR, 0x76153280);
            write32(0x002C + PHYD_BASE_ADDR, 0x00000004);
        }
        DramVendor::ETRON512MbitDDR2 => {
            write32(0x0000 + PHYD_BASE_ADDR, 0x070B090C);
            write32(0x0004 + PHYD_BASE_ADDR, 0x04050608);
            write32(0x0008 + PHYD_BASE_ADDR, 0x0E02030D);
            write32(0x000C + PHYD_BASE_ADDR, 0x110A0100);
            write32(0x0010 + PHYD_BASE_ADDR, 0x0F131614);
            write32(0x0014 + PHYD_BASE_ADDR, 0x00151012);
            write32(0x0018 + PHYD_BASE_ADDR, 0x00000000);
            write32(0x001C + PHYD_BASE_ADDR, 0x00000100);
            write32(0x0020 + PHYD_BASE_ADDR, 0x86014532);
            write32(0x0024 + PHYD_BASE_ADDR, 0x00000007);
            write32(0x0028 + PHYD_BASE_ADDR, 0x76012345);
            write32(0x002C + PHYD_BASE_ADDR, 0x00000008);
        }
        DramVendor::Unknown | _ => {
            println!("  DRAM vendor unknown");
        }
    }
    /*
    #ifdef ETRON_DDR2_512
        KC_MSG("pin mux X16 mode ETRON_DDR2_512 setting\n");
        //------------------------------
        //  pin mux base on PHYA
        //------------------------------
        //param_phyd_data_byte_swap_slice0    [1:     0]
        //param_phyd_data_byte_swap_slice1    [9:     8]
        rddata = 0x00000100;
        write32(0x001C + PHYD_BASE_ADDR, rddata);
        //param_phyd_swap_byte0_dq0_mux    [3:     0]
        //param_phyd_swap_byte0_dq1_mux    [7:     4]
        //param_phyd_swap_byte0_dq2_mux    [11:    8]
        //param_phyd_swap_byte0_dq3_mux    [15:   12]
        //param_phyd_swap_byte0_dq4_mux    [19:   16]
        //param_phyd_swap_byte0_dq5_mux    [23:   20]
        //param_phyd_swap_byte0_dq6_mux    [27:   24]
        //param_phyd_swap_byte0_dq7_mux    [31:   28]
        rddata = 0x86014532;
        write32(0x0020 + PHYD_BASE_ADDR, rddata);
        //param_phyd_swap_byte0_dm_mux     [3:     0]
        rddata = 0x00000007;
        write32(0x0024 + PHYD_BASE_ADDR, rddata);
        //param_phyd_swap_byte1_dq0_mux    [3:     0]
        //param_phyd_swap_byte1_dq1_mux    [7:     4]
        //param_phyd_swap_byte1_dq2_mux    [11:    8]
        //param_phyd_swap_byte1_dq3_mux    [15:   12]
        //param_phyd_swap_byte1_dq4_mux    [19:   16]
        //param_phyd_swap_byte1_dq5_mux    [23:   20]
        //param_phyd_swap_byte1_dq6_mux    [27:   24]
        //param_phyd_swap_byte1_dq7_mux    [31:   28]
        rddata = 0x76012345;
        write32(0x0028 + PHYD_BASE_ADDR, rddata);
        //param_phyd_swap_byte1_dm_mux     [3:     0]
        rddata = 0x00000008;
        write32(0x002C + PHYD_BASE_ADDR, rddata);
        //param_phyd_swap_ca0    [4:     0]
        //param_phyd_swap_ca1    [12:    8]
        //param_phyd_swap_ca2    [20:   16]
        //param_phyd_swap_ca3    [28:   24]
        rddata = 0x070B090C;
        write32(0x0000 + PHYD_BASE_ADDR, rddata);
        //param_phyd_swap_ca4    [4:     0]
        //param_phyd_swap_ca5    [12:    8]
        //param_phyd_swap_ca6    [20:   16]
        //param_phyd_swap_ca7    [28:   24]
        rddata = 0x04050608;
        write32(0x0004 + PHYD_BASE_ADDR, rddata);
        //param_phyd_swap_ca8    [4:     0]
        //param_phyd_swap_ca9    [12:    8]
        //param_phyd_swap_ca10   [20:   16]
        //param_phyd_swap_ca11   [28:   24]
        rddata = 0x0E02030D;
        write32(0x0008 + PHYD_BASE_ADDR, rddata);
        //param_phyd_swap_ca12   [4:     0]
        //param_phyd_swap_ca13   [12:    8]
        //param_phyd_swap_ca14   [20:   16]
        //param_phyd_swap_ca15   [28:   24]
        rddata = 0x110A0100;
        write32(0x000C + PHYD_BASE_ADDR, rddata);
        //param_phyd_swap_ca16   [4:     0]
        //param_phyd_swap_ca17   [12:    8]
        //param_phyd_swap_ca18   [20:   16]
        //param_phyd_swap_ca19   [28:   24]
        rddata = 0x0F131614;
        write32(0x0010 + PHYD_BASE_ADDR, rddata);
        //param_phyd_swap_ca20   [4:     0]
        //param_phyd_swap_ca21   [12:    8]
        //param_phyd_swap_ca22   [20:   16]
        rddata = 0x00151012;
        write32(0x0014 + PHYD_BASE_ADDR, rddata);
        //param_phyd_swap_cke0   [0:0]
        //param_phyd_swap_cs0    [4:4]
        rddata = 0x00000000;
        write32(0x0018 + PHYD_BASE_ADDR, rddata);
        KC_MSG("pin mux setting }\n");
    #endif
    */
    /*
    #ifdef ESMT_N25_DDR3_1G
        KC_MSG("pin mux X16 mode ESMT_N25_DDR3_1G setting\n");
        //------------------------------
        //  pin mux base on PHYA
        //------------------------------
        //param_phyd_data_byte_swap_slice0    [1:     0]
        //param_phyd_data_byte_swap_slice1    [9:     8]
        rddata = 0x00000100;
        write32(0x001C + PHYD_BASE_ADDR, rddata);
        //param_phyd_swap_byte0_dq0_mux    [3:     0]
        //param_phyd_swap_byte0_dq1_mux    [7:     4]
        //param_phyd_swap_byte0_dq2_mux    [11:    8]
        //param_phyd_swap_byte0_dq3_mux    [15:   12]
        //param_phyd_swap_byte0_dq4_mux    [19:   16]
        //param_phyd_swap_byte0_dq5_mux    [23:   20]
        //param_phyd_swap_byte0_dq6_mux    [27:   24]
        //param_phyd_swap_byte0_dq7_mux    [31:   28]
        rddata = 0x82135674;
        write32(0x0020 + PHYD_BASE_ADDR, rddata);
        //param_phyd_swap_byte0_dm_mux     [3:     0]
        rddata = 0x00000000;
        write32(0x0024 + PHYD_BASE_ADDR, rddata);
        //param_phyd_swap_byte1_dq0_mux    [3:     0]
        //param_phyd_swap_byte1_dq1_mux    [7:     4]
        //param_phyd_swap_byte1_dq2_mux    [11:    8]
        //param_phyd_swap_byte1_dq3_mux    [15:   12]
        //param_phyd_swap_byte1_dq4_mux    [19:   16]
        //param_phyd_swap_byte1_dq5_mux    [23:   20]
        //param_phyd_swap_byte1_dq6_mux    [27:   24]
        //param_phyd_swap_byte1_dq7_mux    [31:   28]
        rddata = 0x76153280;
        write32(0x0028 + PHYD_BASE_ADDR, rddata);
        //param_phyd_swap_byte1_dm_mux     [3:     0]
        rddata = 0x00000004;
        write32(0x002C + PHYD_BASE_ADDR, rddata);
        //param_phyd_swap_ca0    [4:     0]
        //param_phyd_swap_ca1    [12:    8]
        //param_phyd_swap_ca2    [20:   16]
        //param_phyd_swap_ca3    [28:   24]
        rddata = 0x08060B09;
        write32(0x0000 + PHYD_BASE_ADDR, rddata);
        //param_phyd_swap_ca4    [4:     0]
        //param_phyd_swap_ca5    [12:    8]
        //param_phyd_swap_ca6    [20:   16]
        //param_phyd_swap_ca7    [28:   24]
        rddata = 0x02040701;
        write32(0x0004 + PHYD_BASE_ADDR, rddata);
        //param_phyd_swap_ca8    [4:     0]
        //param_phyd_swap_ca9    [12:    8]
        //param_phyd_swap_ca10   [20:   16]
        //param_phyd_swap_ca11   [28:   24]
        rddata = 0x0C00050D;
        write32(0x0008 + PHYD_BASE_ADDR, rddata);
        //param_phyd_swap_ca12   [4:     0]
        //param_phyd_swap_ca13   [12:    8]
        //param_phyd_swap_ca14   [20:   16]
        //param_phyd_swap_ca15   [28:   24]
        rddata = 0x13150314;
        write32(0x000C + PHYD_BASE_ADDR, rddata);
        //param_phyd_swap_ca16   [4:     0]
        //param_phyd_swap_ca17   [12:    8]
        //param_phyd_swap_ca18   [20:   16]
        //param_phyd_swap_ca19   [28:   24]
        rddata = 0x10111216;
        write32(0x0010 + PHYD_BASE_ADDR, rddata);
        //param_phyd_swap_ca20   [4:     0]
        //param_phyd_swap_ca21   [12:    8]
        //param_phyd_swap_ca22   [20:   16]
        rddata = 0x000F0A0E;
        write32(0x0014 + PHYD_BASE_ADDR, rddata);
        //param_phyd_swap_cke0   [0:     0]
        //param_phyd_swap_cs0    [4:     4]
        rddata = 0x00000000;
        write32(0x0018 + PHYD_BASE_ADDR, rddata);
        KC_MSG("pin mux setting }\n");
    #endif
    */
    /*
    #ifdef ESMT_DDR3_2G
        KC_MSG("pin mux X16 mode ESMT_DDR3_2G setting\n");
        rddata = 0x00000100;
        write32(0x001C + PHYD_BASE_ADDR, rddata);
        rddata = 0x82135764;
        write32(0x0020 + PHYD_BASE_ADDR, rddata);
        rddata = 0x00000000;
        write32(0x0024 + PHYD_BASE_ADDR, rddata);
        rddata = 0x67513208;
        write32(0x0028 + PHYD_BASE_ADDR, rddata);
        rddata = 0x00000004;
        write32(0x002C + PHYD_BASE_ADDR, rddata);
        rddata = 0x080B0D06;
        write32(0x0000 + PHYD_BASE_ADDR, rddata);
        rddata = 0x09010407;
        write32(0x0004 + PHYD_BASE_ADDR, rddata);
        rddata = 0x1405020C;
        write32(0x0008 + PHYD_BASE_ADDR, rddata);
        rddata = 0x15000E03;
        write32(0x000C + PHYD_BASE_ADDR, rddata);
        rddata = 0x0A0F1213;
        write32(0x0010 + PHYD_BASE_ADDR, rddata);
        rddata = 0x00111016;
        write32(0x0014 + PHYD_BASE_ADDR, rddata);
        rddata = 0x00000000;
        write32(0x0018 + PHYD_BASE_ADDR, rddata);
        KC_MSG("pin mux setting }\n");
    #endif
    */
    /*
    #ifdef ETRON_DDR3_1G
        KC_MSG("pin mux X16 mode ETRON_DDR3_1G setting\n");
        rddata = 0x00000100;
        write32(0x001C + PHYD_BASE_ADDR, rddata);
        rddata = 0x28137564;
        write32(0x0020 + PHYD_BASE_ADDR, rddata);
        rddata = 0x00000000;
        write32(0x0024 + PHYD_BASE_ADDR, rddata);
        rddata = 0x76158320;
        write32(0x0028 + PHYD_BASE_ADDR, rddata);
        rddata = 0x00000004;
        write32(0x002C + PHYD_BASE_ADDR, rddata);
        rddata = 0x0B060908;
        write32(0x0000 + PHYD_BASE_ADDR, rddata);
        rddata = 0x02000107;
        write32(0x0004 + PHYD_BASE_ADDR, rddata);
        rddata = 0x0C05040D;
        write32(0x0008 + PHYD_BASE_ADDR, rddata);
        rddata = 0x13141503;
        write32(0x000C + PHYD_BASE_ADDR, rddata);
        rddata = 0x160A1112;
        write32(0x0010 + PHYD_BASE_ADDR, rddata);
        rddata = 0x000F100E;
        write32(0x0014 + PHYD_BASE_ADDR, rddata);
        rddata = 0x00000000;
        write32(0x0018 + PHYD_BASE_ADDR, rddata);
        KC_MSG("pin mux setting }\n");
    #endif
    */
    /*
    #ifdef DDR3_1G
        KC_MSG("pin mux X16 mode DDR3_1G setting\n");
        rddata = 0x00000100;
        write32(0x001C + PHYD_BASE_ADDR, rddata);
        rddata = 0x31756024;
        write32(0x0020 + PHYD_BASE_ADDR, rddata);
        rddata = 0x00000008;
        write32(0x0024 + PHYD_BASE_ADDR, rddata);
        rddata = 0x26473518;
        write32(0x0028 + PHYD_BASE_ADDR, rddata);
        rddata = 0x00000000;
        write32(0x002C + PHYD_BASE_ADDR, rddata);
        rddata = 0x08070B09;
        write32(0x0000 + PHYD_BASE_ADDR, rddata);
        rddata = 0x05000206;
        write32(0x0004 + PHYD_BASE_ADDR, rddata);
        rddata = 0x0C04010D;
        write32(0x0008 + PHYD_BASE_ADDR, rddata);
        rddata = 0x15030A14;
        write32(0x000C + PHYD_BASE_ADDR, rddata);
        rddata = 0x10111213;
        write32(0x0010 + PHYD_BASE_ADDR, rddata);
        rddata = 0x000F160E;
        write32(0x0014 + PHYD_BASE_ADDR, rddata);
        rddata = 0x00000000;
        write32(0x0018 + PHYD_BASE_ADDR, rddata);
        KC_MSG("pin mux setting }\n");
    #endif
    */
    /*
    #ifdef DDR3_2G
        KC_MSG("pin mux X16 mode DDR3_2G setting\n");
        rddata = 0x00000100;
        write32(0x001C + PHYD_BASE_ADDR, rddata);
        rddata = 0x82135764;
        write32(0x0020 + PHYD_BASE_ADDR, rddata);
        rddata = 0x00000000;
        write32(0x0024 + PHYD_BASE_ADDR, rddata);
        rddata = 0x67513028;
        write32(0x0028 + PHYD_BASE_ADDR, rddata);
        rddata = 0x00000004;
        write32(0x002C + PHYD_BASE_ADDR, rddata);
        rddata = 0x08070D09;
        write32(0x0000 + PHYD_BASE_ADDR, rddata);
        rddata = 0x0605020B;
        write32(0x0004 + PHYD_BASE_ADDR, rddata);
        rddata = 0x14040100;
        write32(0x0008 + PHYD_BASE_ADDR, rddata);
        rddata = 0x15030E0C;
        write32(0x000C + PHYD_BASE_ADDR, rddata);
        rddata = 0x0A0F1213;
        write32(0x0010 + PHYD_BASE_ADDR, rddata);
        rddata = 0x00111016;
        write32(0x0014 + PHYD_BASE_ADDR, rddata);
        rddata = 0x00000000;
        write32(0x0018 + PHYD_BASE_ADDR, rddata);
        KC_MSG("pin mux setting }\n");
    #endif
    */
    const DDR3_4G: bool = false;
    if DDR3_4G {
        println!("pin mux X16 mode DDR3_4G setting");
        write32(0x001C + PHYD_BASE_ADDR, 0x00000100);
        write32(0x0020 + PHYD_BASE_ADDR, 0x02136574);
        write32(0x0024 + PHYD_BASE_ADDR, 0x00000008);
        write32(0x0028 + PHYD_BASE_ADDR, 0x76512308);
        write32(0x002C + PHYD_BASE_ADDR, 0x00000004);
        // TODO: is the order important? All the same as above (NY4G).
        write32(0x0000 + PHYD_BASE_ADDR, 0x12141013);
        write32(0x0004 + PHYD_BASE_ADDR, 0x0C041503);
        write32(0x0008 + PHYD_BASE_ADDR, 0x06050001);
        write32(0x000C + PHYD_BASE_ADDR, 0x08070B02);
        write32(0x0010 + PHYD_BASE_ADDR, 0x0A0F0E09);
        write32(0x0014 + PHYD_BASE_ADDR, 0x0016110D);
        write32(0x0018 + PHYD_BASE_ADDR, 0x00000000);
        println!("pin mux setting");
    }
    /*
    #ifdef DDR3_DBG
        KC_MSG("pin mux X16 mode DDR3_DBG setting\n");
        rddata = 0x00000100;
        write32(0x001C + PHYD_BASE_ADDR, rddata);
        rddata = 0x30587246;
        write32(0x0020 + PHYD_BASE_ADDR, rddata);
        rddata = 0x00000001;
        write32(0x0024 + PHYD_BASE_ADDR, rddata);
        rddata = 0x26417538;
        write32(0x0028 + PHYD_BASE_ADDR, rddata);
        rddata = 0x00000000;
        write32(0x002C + PHYD_BASE_ADDR, rddata);
        rddata = 0x0002080E;
        write32(0x0000 + PHYD_BASE_ADDR, rddata);
        rddata = 0x04060D01;
        write32(0x0004 + PHYD_BASE_ADDR, rddata);
        rddata = 0x090C030B;
        write32(0x0008 + PHYD_BASE_ADDR, rddata);
        rddata = 0x05071412;
        write32(0x000C + PHYD_BASE_ADDR, rddata);
        rddata = 0x0A151013;
        write32(0x0010 + PHYD_BASE_ADDR, rddata);
        rddata = 0x0016110F;
        write32(0x0014 + PHYD_BASE_ADDR, rddata);
        rddata = 0x00000000;
        write32(0x0018 + PHYD_BASE_ADDR, rddata);
        KC_MSG("pin mux setting }\n");
    #endif
    */
    const DDR3_PINMUX: bool = false;
    if DDR3_PINMUX {
        println!("pin mux X16 mode DDR3_6mil setting");
        write32(0x001C + PHYD_BASE_ADDR, 0x00000001);
        write32(0x0020 + PHYD_BASE_ADDR, 0x40613578);
        write32(0x0024 + PHYD_BASE_ADDR, 0x00000002);
        write32(0x0028 + PHYD_BASE_ADDR, 0x03582467);
        write32(0x002C + PHYD_BASE_ADDR, 0x00000001);
        write32(0x0000 + PHYD_BASE_ADDR, 0x020E0D00);
        write32(0x0004 + PHYD_BASE_ADDR, 0x07090806);
        write32(0x0008 + PHYD_BASE_ADDR, 0x0C05010B);
        write32(0x000C + PHYD_BASE_ADDR, 0x12141503);
        write32(0x0010 + PHYD_BASE_ADDR, 0x100A0413);
        write32(0x0014 + PHYD_BASE_ADDR, 0x00160F11);
        write32(0x0018 + PHYD_BASE_ADDR, 0x00000000);
        println!("pin mux setting");
    }
    /*
    #ifdef DDR2_512
        KC_MSG("pin mux X16 mode DDR2_512 setting\n");
        rddata = 0x00000100;
        write32(0x001C + PHYD_BASE_ADDR, rddata);
        rddata = 0x60851243;
        write32(0x0020 + PHYD_BASE_ADDR, rddata);
        rddata = 0x00000007;
        write32(0x0024 + PHYD_BASE_ADDR, rddata);
        rddata = 0x67012354;
        write32(0x0028 + PHYD_BASE_ADDR, rddata);
        rddata = 0x00000008;
        write32(0x002C + PHYD_BASE_ADDR, rddata);
        rddata = 0x0C06080B;
        write32(0x0000 + PHYD_BASE_ADDR, rddata);
        rddata = 0x090D0204;
        write32(0x0004 + PHYD_BASE_ADDR, rddata);
        rddata = 0x01050700;
        write32(0x0008 + PHYD_BASE_ADDR, rddata);
        rddata = 0x160A0E03;
        write32(0x000C + PHYD_BASE_ADDR, rddata);
        rddata = 0x0F141110;
        write32(0x0010 + PHYD_BASE_ADDR, rddata);
        rddata = 0x00151312;
        write32(0x0014 + PHYD_BASE_ADDR, rddata);
        rddata = 0x00000000;
        write32(0x0018 + PHYD_BASE_ADDR, rddata);
        KC_MSG("pin mux setting }\n");
    #endif

    #ifdef N25_DDR2_512
        KC_MSG("pin mux X16 mode N25_DDR2_512 setting\n");
        rddata = 0x00000100;
        write32(0x001C + PHYD_BASE_ADDR, rddata);
        rddata = 0x71840532;
        write32(0x0020 + PHYD_BASE_ADDR, rddata);
        rddata = 0x00000006;
        write32(0x0024 + PHYD_BASE_ADDR, rddata);
        rddata = 0x76103425;
        write32(0x0028 + PHYD_BASE_ADDR, rddata);
        rddata = 0x00000008;
        write32(0x002C + PHYD_BASE_ADDR, rddata);
        rddata = 0x0C06080B;
        write32(0x0000 + PHYD_BASE_ADDR, rddata);
        rddata = 0x070D0904;
        write32(0x0004 + PHYD_BASE_ADDR, rddata);
        rddata = 0x00010502;
        write32(0x0008 + PHYD_BASE_ADDR, rddata);
        rddata = 0x110A0E03;
        write32(0x000C + PHYD_BASE_ADDR, rddata);
        rddata = 0x0F141610;
        write32(0x0010 + PHYD_BASE_ADDR, rddata);
        rddata = 0x00151312;
        write32(0x0014 + PHYD_BASE_ADDR, rddata);
        rddata = 0x00000000;
        write32(0x0018 + PHYD_BASE_ADDR, rddata);
        KC_MSG("pin mux setting }\n");
    #endif
    #ifdef DDR2_PINMUX
        KC_MSG("pin mux X16 mode DDR2 setting\n");
        rddata = 0x00000001;
        write32(0x001C + PHYD_BASE_ADDR, rddata);
        rddata = 0x40613578;
        write32(0x0020 + PHYD_BASE_ADDR, rddata);
        rddata = 0x00000002;
        write32(0x0024 + PHYD_BASE_ADDR, rddata);
        rddata = 0x03582467;
        write32(0x0028 + PHYD_BASE_ADDR, rddata);
        rddata = 0x00000001;
        write32(0x002C + PHYD_BASE_ADDR, rddata);
        rddata = 0x020E0D00;
        write32(0x0000 + PHYD_BASE_ADDR, rddata);
        rddata = 0x07090806;
        write32(0x0004 + PHYD_BASE_ADDR, rddata);
        rddata = 0x0C05010B;
        write32(0x0008 + PHYD_BASE_ADDR, rddata);
        rddata = 0x12141503;
        write32(0x000C + PHYD_BASE_ADDR, rddata);
        rddata = 0x100A0413;
        write32(0x0010 + PHYD_BASE_ADDR, rddata);
        rddata = 0x00160F11;
        write32(0x0014 + PHYD_BASE_ADDR, rddata);
        rddata = 0x00000000;
        write32(0x0018 + PHYD_BASE_ADDR, rddata);
        KC_MSG("pin mux setting }\n");
    #endif
    */
    println!("\\ cvx16_pinmux finish");
}

// This is a full duplicate in the vendor code:
// plat/cv181x/ddr/ddr_config/ddr_auto_x16/ddr_patch_regs.c
// plat/cv181x/ddr/ddr_config/ddr3_1866_x16/ddr_patch_regs.c
fn ddr_patch_set() {
    println!("/ ddr_patch_set start");
    const DDR3_1866: bool = true;
    if DDR3_1866 {
        // tune damp //////
        write32(0x08000150, 0x00000005);

        // CSB & CA driving
        write32(0x0800097c, 0x08080404);

        // CLK driving
        write32(0x08000980, 0x08080808);

        // DQ driving // BYTE0
        write32(0x08000a38, 0x00000606);
        // DQS driving // BYTE0
        write32(0x08000a3c, 0x06060606);
        // DQ driving // BYTE1
        write32(0x08000a78, 0x00000606);
        // DQS driving // BYTE1
        write32(0x08000a7c, 0x06060606);

        //trigger level //////
        // BYTE0
        write32(0x08000b24, 0x00100010);
        // BYTE1
        write32(0x08000b54, 0x00100010);

        //APHY TX VREFDQ rangex2 [1]
        //VREF DQ   //
        write32(0x08000410, 0x00120002);
        //APHY TX VREFCA rangex2 [1]
        //VREF CA  //
        write32(0x08000414, 0x00100002);

        // tx dline code
        //  BYTE0 DQ
        write32(0x08000a00, 0x06430643);
        write32(0x08000a04, 0x06430643);
        write32(0x08000a08, 0x06430643);
        write32(0x08000a0c, 0x06430643);
        write32(0x08000a10, 0x00000643);
        write32(0x08000a14, 0x0a7e007e);
        //  BYTE1 DQ
        write32(0x08000a40, 0x06480648);
        write32(0x08000a44, 0x06480648);
        write32(0x08000a48, 0x06480648);
        write32(0x08000a4c, 0x06480648);
        write32(0x08000a50, 0x00000648);
        write32(0x08000a54, 0x0a7e007e);

        //APHY RX TRIG rangex2[18] & disable lsmode[0]
        //f0_param_phya_reg_rx_byte0_en_lsmode[0]
        //f0_param_phya_reg_byte0_en_rec_vol_mode[12]
        //f0_param_phya_reg_rx_byte0_force_en_lvstl_odt[16]
        //f0_param_phya_reg_rx_byte0_sel_dqs_rec_vref_mode[8]
        //param_phya_reg_rx_byte0_en_trig_lvl_rangex2[18]
        // BYTE0 [0]
        write32(0x08000500, 0x00041001);
        //f0_param_phya_reg_rx_byte1_en_lsmode[0]
        //f0_param_phya_reg_byte1_en_rec_vol_mode[12]
        //f0_param_phya_reg_rx_byte0_force_en_lvstl_odt[16]
        //f0_param_phya_reg_rx_byte0_sel_dqs_rec_vref_mode[8]
        //param_phya_reg_rx_byte0_en_trig_lvl_rangex2[18]
        // BYTE1 [0]
        write32(0x08000540, 0x00041001);

        ////////  FOR U02 ///////
        /////////// U02 enable DQS voltage mode receiver
        // f0_param_phya_reg_tx_byte0_en_tx_de_dqs[20]
        write32(0x08000504, 0x00100000);
        // f0_param_phya_reg_tx_byte1_en_tx_de_dqs[20]
        write32(0x08000544, 0x00100000);
        /////////// U02 enable MASK voltage mode receiver
        // param_phya_reg_rx_sel_dqs_wo_pream_mode[2]
        write32(0x08000138, 0x00000014);

        // BYTE0 RX DQ deskew
        write32(0x08000b00, 0x00020402);
        write32(0x08000b04, 0x05020401);
        // BYTE0  DQ8 deskew [6:0] neg DQS  [15:8]  ;  pos DQS  [23:16]
        write32(0x08000b08, 0x00313902);

        // BYTE1 RX DQ deskew
        write32(0x08000b30, 0x06000100);
        write32(0x08000b34, 0x02010303);
        // BYTE1  DQ8 deskew [6:0] neg DQS  [15:8]  ;  pos DQS  [23:16]
        write32(0x08000b38, 0x00323900);

        //Read gate TX dline + shift
        // BYTE0
        write32(0x08000b0c, 0x00000a14);
        // BYTE1
        write32(0x08000b3c, 0x00000a14);

        // CKE dline + shift CKE0 [6:0]+[13:8] ; CKE1 [22:16]+[29:24]
        write32(0x08000930, 0x04000400);
        // CSB dline + shift CSB0 [6:0]+[13:8] ; CSB1 [22:16]+[29:24]
        write32(0x08000934, 0x04000400);
    }
    println!("\\ ddr_patch_set finish");
}

// plat/cv181x/ddr/ddr_sys.c
fn cvx16_en_rec_vol_mode() {
    println!("/ cvx16_en_rec_vol_mode start");
    if DDR2 {
        write32(0x0500 + PHYD_BASE_ADDR, 0x00001001);
        write32(0x0540 + PHYD_BASE_ADDR, 0x00001001);
    }
    println!("\\ cvx16_en_rec_vol_mode finish");
}

fn dfi_init() {
    // synp setting
    // phy is ready for initial dfi_init_start request
    // set umctl2 to trigger dfi_init_start
    write32(DDR_CFG_BASE + 0x00000320, 0x0);
    // dfi_init_start @ rddata[5];
    let v = read32(DDR_CFG_BASE + 0x000001b0);
    write32(DDR_CFG_BASE + 0x000001b0, v | 1 << 5);
    write32(DDR_CFG_BASE + 0x00000320, 0x1);
}

// DFI = DDR PHY Interface
// https://www.synopsys.com/blogs/chip-design/mastering-ddr-phy-interoperability-dfi.html
// plat/cv181x/ddr/ddr_sys.c
fn cvx16_set_dfi_init_start() {
    println!("/ cvx16_set_dfi_init start");
    dfi_init();
    println!("\\ set_dfi_init_start finish");
}

fn cvx16_ddr_phy_power_on_seq1() {
    println!("/ ddr_phy_power_on_seq1 start");
    // RESETZ/CKE PD=0
    let v = read32(0x40 + PHYD_APB);
    // TOP_REG_TX_CA_PD_CKE0
    let v = v & !(1 << 24);
    // TOP_REG_TX_CA_PD_RESETZ
    let v = v & !(1 << 30);
    write32(0x40 + PHYD_APB, v);
    println!("  RESET PD !!!");

    // CA PD=0
    // All PHYA CA PD=0
    write32(0x40 + PHYD_APB, 0);
    println!("  All PHYA CA PD=0 ...");

    // TOP_REG_TX_SEL_GPIO = 1 (DQ)
    let v = read32(0x1c + PHYD_APB);
    write32(0x1c + PHYD_APB, v | (1 << 7));
    println!("  TOP_REG_TX_SEL_GPIO = 1");

    // DQ PD=0
    // TOP_REG_TX_BYTE0_PD
    // TOP_REG_TX_BYTE1_PD
    write32(0x00 + PHYD_APB, 0);
    println!("  TX_BYTE PD=0 ...");

    // TOP_REG_TX_SEL_GPIO = 0 (DQ)
    let v = read32(0x1c + PHYD_APB);
    write32(0x1c + PHYD_APB, v & !(1 << 7));
    println!("  TOP_REG_TX_SEL_GPIO = 0");

    println!("\\ ddr_phy_power_on_seq1 finish");
}

fn cvx16_polling_dfi_init_start() {
    println!("/ first dfi_init_start");
    while read32(0x3028 + PHYD_BASE_ADDR) & (1 << 8) == 0 {}
    println!("\\ cvx16_polling_dfi_init_start finish");
}

// pass in result of reading PHYD_BASE_ADDR + 0x004c
fn get_pll_speed_change(v: u32) -> (bool, u32, u32) {
    // TOP_REG_EN_PLL_SPEED_CHG
    // <= #RD (~pwstrb_mask[0] & TOP_REG_EN_PLL_SPEED_CHG) |  pwstrb_mask_pwdata[0];
    // TOP_REG_CUR_PLL_SPEED   [1:0]
    // <= #RD (~pwstrb_mask[5:4] & TOP_REG_CUR_PLL_SPEED[1:0]) |  pwstrb_mask_pwdata[5:4];
    // TOP_REG_NEXT_PLL_SPEED  [1:0]
    // <= #RD (~pwstrb_mask[9:8] & TOP_REG_NEXT_PLL_SPEED[1:0]) |  pwstrb_mask_pwdata[9:8];
    let en_pll_speed_chg = (v & 0b1) != 0;
    let curr_pll_speed = v & (0b11 << 4);
    let next_pll_speed = v & (0b11 << 8);
    println!("  en_pll_speed_chg {en_pll_speed_chg}");
    println!("  curr_pll_speed   {curr_pll_speed}");
    println!("  next_pll_speed   {next_pll_speed}");
    (en_pll_speed_chg, curr_pll_speed, next_pll_speed)
}

fn cvx16_int_isr_08() {
    println!("/ cvx16_int_isr_08 start");
    write32(0x0118 + PHYD_BASE_ADDR, 0x0);
    let v = read32(0x4c + PHYD_APB);
    let _ = get_pll_speed_change(v);
    println!("\\ cvx16_int_isr_08 finish");
}

fn cvx16_dll_cal() {
    let v = read32(PHYD_APB + 0x4c);
    let (en_pll_speed_chg, curr_pll_speed, next_pll_speed) = get_pll_speed_change(v);

    // only do calibration and update when high speed
    if (curr_pll_speed != 0) {
        // param_phyd_dll_rx_start_cal <= int_regin[1];
        // param_phyd_dll_tx_start_cal <= int_regin[17];
        let v = read32(PHYD_BASE_ADDR + 0x0040);
        let v = v & !((1 << 17) | (1 << 1));
        write32(PHYD_BASE_ADDR + 0x0040, v);
        // param_phyd_dll_rx_start_cal <= int_regin[1];
        // param_phyd_dll_tx_start_cal <= int_regin[17];
        let v = read32(PHYD_BASE_ADDR + 0x0040);
        let v = v | (1 << 17) | (1 << 1);
        write32(PHYD_BASE_ADDR + 0x0040, v);
        while read32(0x3014 + PHYD_BASE_ADDR) & !(1 << 16) == 0 {}
        println!("  DLL lock !");
        // opdelay(1000);
        println!("  Do DLL UPD");
    // cvx16_dll_cal_status();
    } else {
        // stop calibration and update when low speed
        // param_phyd_dll_rx_start_cal <= int_regin[1];
        // param_phyd_dll_tx_start_cal <= int_regin[17];
        let v = read32(PHYD_BASE_ADDR + 0x0040);
        let v = v & !((1 << 17) | (1 << 1));
        write32(PHYD_BASE_ADDR + 0x0040, v);
    }
    println!("  DLL CAL Finish");
}

fn cvx16_ddr_phy_power_on_seq2() {
    println!("/ cvx16_ddr_phy_power_on_seq2 start");

    // OEN
    // param_phyd_sel_cke_oenz        <= `PI_SD int_regin[0];
    let v = read32(0x0154 + PHYD_BASE_ADDR);
    write32(0x0154 + PHYD_BASE_ADDR, v & !(1));
    // param_phyd_tx_ca_oenz          <= `PI_SD int_regin[0];
    // param_phyd_tx_ca_clk0_oenz     <= `PI_SD int_regin[8];
    // param_phyd_tx_ca_clk1_oenz     <= `PI_SD int_regin[16];
    write32(0x0130 + PHYD_BASE_ADDR, 0x00000000);

    println!("  DLL calibration if necessary ...");
    cvx16_dll_cal();
    println!("  DLL calibration done");

    println!("  ZQCAL if necessary ...");
    const DO_ZQ_CAL: bool = false;
    if DO_ZQ_CAL {
        // zqcal hw mode, bit0: offset_cal, bit1:pl_en, bit2:step2_en
        // cvx16_ddr_zqcal_hw_isr8(0x7);
        println!("  ZQCAL done");
    } else {
        println!("  cv181x without ZQ Calibration ...");
    }

    const DO_ZQ240_CAL: bool = false;
    println!("  ZQ240 calibration if necessary ...");
    if DO_ZQ240_CAL {
        // cvx16_ddr_zq240_cal();//zq240_cal
        println!("  ZQ240 cal done");
    } else {
        println!("  cv181x without ZQ240 Calibration ...");
    }

    const DO_ZQ_CAL_VAR: bool = false;
    if DO_ZQ_CAL_VAR {
        //  zq_cal_var();
    } else {
        println!("  ZQ calculate variation not run");
    }

    // CA PD =0
    // All PHYA CA PD=0
    write32(0x40 + PHYD_APB, 0x80000000);
    println!("  All PHYA CA PD = 0 ...");
    // BYTE PD =0
    write32(0x00 + PHYD_APB, 0x00000000);
    println!("  TX_BYTE PD = 0 ...");
    println!("\\ cvx16_ddr_phy_power_on_seq2 finish");
}

fn cvx16_set_dfi_init_complete() {
    println!("/ cvx16_set_dfi_init_complete start");
    // opdelay(20000);
    // HACK
    for _ in 0..20000 {
        read32(PHYD_BASE_ADDR + 0x0118);
    }
    // rddata[8] = 1;
    write32(PHYD_BASE_ADDR + 0x0120, 0x00000010);
    println!("  set init_complete = 1 ...");
    // param_phyd_clkctrl_init_complete   <= int_regin[0];
    write32(PHYD_BASE_ADDR + 0x0118, 0x1);
    println!("\\ cvx16_set_dfi_init_complete finish");
}

fn cvx16_clk_div40() {
    println!("  clk_div40");
    // TOP_REG_DDRPLL_SEL_LOW_SPEED = 1
    let v = read32(PHYD_APB + 0x0c);
    write32(PHYD_APB + 0x0c, v | (1 << 13));
}

fn cvx16_clk_div2() {
    // TOP_REG_DDRPLL_MAS_DIV_OUT_SEL 1
    println!("  clk_div2");
    let v = read32(PHYD_APB + 0x0c);
    write32(PHYD_APB + 0x0c, v | (1 << 14));
}

fn cvx16_clk_normal(reg_set: u32, reg_span: u32, reg_step: u32) {
    println!("  clk_normal");
    let v = read32(PHYD_APB + 0x0c);
    // TOP_REG_DDRPLL_SEL_LOW_SPEED 0
    // TOP_REG_DDRPLL_MAS_DIV_OUT_SEL 0
    write32(PHYD_APB + 0x0c, v & !((1 << 13) | (1 << 14)));

    // NOTE: similar to cvx16_pll_init
    if SSC_EN {
        /*
        write32(0x54 + 0x03002900, reg_set);
        // TOP_REG_SSC_SPAN
        rddata = get_bits_from_value(reg_span, 15, 0);
        write32(0x58 + 0x03002900, rddata);
        // TOP_REG_SSC_STEP
        rddata = get_bits_from_value(reg_step, 23, 0);
        write32(0x5C + 0x03002900, rddata);
        rddata = read32(0x50 + 0x03002900);
        // TOP_REG_SSC_SW_UP
        rddata = modified_bits_by_value(rddata, ~get_bits_from_value(rddata, 0, 0), 0, 0);
        // TOP_REG_SSC_EN_SSC
        rddata = modified_bits_by_value(rddata, 1, 1, 1);
        // TOP_REG_SSC_SSC_MODE
        rddata = modified_bits_by_value(rddata, 0, 3, 2);
        // TOP_REG_SSC_BYPASS
        rddata = modified_bits_by_value(rddata, 0, 4, 4);
        // extpulse
        rddata = modified_bits_by_value(rddata, 1, 5, 5);
        // ssc_syn_fix_div
        rddata = modified_bits_by_value(rddata, 0, 6, 6);
        write32(0x50 + 0x03002900, rddata);
        */
        println!("SSC_EN");
    }
    if SSC_BYPASS {
        /*
        // TOP_REG_SSC_SET
        rddata = (reg_set & 0xfc000000) + 0x04000000;
        write32(0x54 + 0x03002900, rddata);
        // TOP_REG_SSC_SPAN
        rddata = get_bits_from_value(reg_span, 15, 0);
        write32(0x58 + 0x03002900, rddata);
        // TOP_REG_SSC_STEP
        rddata = get_bits_from_value(reg_step, 23, 0);
        write32(0x5C + 0x03002900, rddata);
        rddata = read32(0x50 + 0x03002900);
        // TOP_REG_SSC_SW_UP
        rddata = modified_bits_by_value(rddata, ~get_bits_from_value(rddata, 0, 0), 0, 0);
        // TOP_REG_SSC_EN_SSC
        rddata = modified_bits_by_value(rddata, 0, 1, 1);
        // TOP_REG_SSC_SSC_MODE
        rddata = modified_bits_by_value(rddata, 0, 3, 2);
        // TOP_REG_SSC_BYPASS
        rddata = modified_bits_by_value(rddata, 0, 4, 4);
        // TOP_REG_SSC_EXTPULSE
        rddata = modified_bits_by_value(rddata, 1, 5, 5);
        // ssc_syn_fix_div
        rddata = modified_bits_by_value(rddata, 1, 6, 6);
        */
        println!("  SSC_BYPASS");
    } else {
        set_dpll_ssc_syn(reg_set, reg_span, reg_step);
        println!("  SSC_OFF");
    }
    println!("  back to original frequency");
}

fn change_pll_freq(reg_set: u32, reg_span: u32, reg_step: u32) {
    println!("/ change_pll_freq start");
    println!("  Change PLL frequency if necessary ...");
    // TOP_REG_RESETZ_DIV = 0
    write32(0x04 + PHYD_APB, 0);
    // TOP_REG_RESETZ_DQS = 0
    write32(0x08 + PHYD_APB, 0);
    // TOP_REG_DDRPLL_MAS_RSTZ_DIV  =0
    let v = read32(0x0C + PHYD_APB);
    write32(0x0C + PHYD_APB, v & !(1 << 7));
    println!("  RSTZ_DIV = 0");

    // NOTE: Reading a register may have meaning in hardware.
    // Yes, the vendor code reads this 6x. It _may_ have an effect.
    read32(0x4c + PHYD_APB);
    read32(0x4c + PHYD_APB);
    read32(0x4c + PHYD_APB);
    read32(0x4c + PHYD_APB);
    read32(0x4c + PHYD_APB);
    let v = read32(0x4c + PHYD_APB);
    let (en_chg, curr_speed, next_speed) = get_pll_speed_change(v);

    let v = (v & (0b11 << 8)) | curr_speed << 8;
    let v = (v & (0b11 << 4)) | next_speed << 4;
    if (en_chg) {
        match next_speed {
            0 => {
                // next clk_div40
                write32(0x4c + PHYD_APB, v);
                cvx16_clk_div40();
            }
            0x1 => {
                // next clk normal div_2
                write32(0x4c + PHYD_APB, v);
                cvx16_clk_div2();
            }
            0x2 => {
                // next clk normal
                write32(0x4c + PHYD_APB, v);
                cvx16_clk_normal(reg_set, reg_span, reg_step);
            }
            _ => {}
        }
        // opdelay(100000);  //  1000ns
    }

    // NOTE: similar to cvx16_pll_init
    // TOP_REG_RESETZ_DIV = 1
    write32(PHYD_APB + 0x04, 0x1);
    // TOP_REG_DDRPLL_MAS_RSTZ_DIV
    let v = read32(0x0C + PHYD_APB);
    write32(PHYD_APB + 0x0c, v | (1 << 7));
    println!("  RSTZ_DIV = 1");
    // TOP_REG_RESETZ_DQS
    write32(PHYD_APB + 0x08, 0x1);
    println!("  TOP_REG_RESETZ_DQS");

    println!("  Wait for DDR PLL_SLV_LOCK = 1...");
    while read32(PHYD_APB + 0x10) & (1 << 15) == 0 {
        // opdelay(200);
        // HACK
        for _ in 0..200 {
            read32(PHYD_APB + 0x10);
        }
    }

    println!("\\ change_pll_freq finish");
}

fn cvx16_ddr_phy_power_on_seq3() {
    println!("/ ddr_phy_power_on_seq3 start");
    // RESETYZ/CKE OENZ
    // param_phyd_sel_cke_oenz        <= `PI_SD int_regin[0];
    let v = read32(PHYD_BASE_ADDR + 0x0154);
    write32(PHYD_BASE_ADDR + 0x0154, v & !(0x1));
    // param_phyd_tx_ca_oenz          <= `PI_SD int_regin[0];
    // param_phyd_tx_ca_clk0_oenz     <= `PI_SD int_regin[8];
    // param_phyd_tx_ca_clk1_oenz     <= `PI_SD int_regin[16];
    write32(PHYD_BASE_ADDR + 0x0130, 0x0);
    println!("  --> ca_oenz  ca_clk_oenz !!!");

    // clock gated for power save
    // param_phya_reg_tx_byte0_en_extend_oenz_gated_dline <= `PI_SD int_regin[0];
    // param_phya_reg_tx_byte1_en_extend_oenz_gated_dline <= `PI_SD int_regin[1];
    // param_phya_reg_tx_byte2_en_extend_oenz_gated_dline <= `PI_SD int_regin[2];
    // param_phya_reg_tx_byte3_en_extend_oenz_gated_dline <= `PI_SD int_regin[3];
    let v = read32(PHYD_BASE_ADDR + 0x0204);
    write32(PHYD_BASE_ADDR + 0x0204, v | 1 << 18);
    let v = read32(PHYD_BASE_ADDR + 0x0224);
    write32(PHYD_BASE_ADDR + 0x0224, v | 1 << 18);
    println!("  --> en clock gated for power save !!!");
    println!("\\ ddr_phy_power_on_seq3 finish");
}

fn cvx16_wait_for_dfi_init_complete() {
    println!("/ wait_for_dfi_init_complete start");
    while read32(DDR_CFG_BASE + 0x01bc) & 0x1 == 0 {}
    dfi_init();
    println!("\\ wait_for_dfi_init_complete finish");
}

fn cvx16_polling_synp_normal_mode() {
    println!("/ polling_synp_normal_mode start");
    // synp ctrl operating_mode
    while let v = read32(DDR_CFG_BASE + 0x0004) & 0b111 {
        println!("  operating_mode {v}");
        if v == 1 {
            break;
        }
    }
    println!("\\ polling_synp_normal_mode finish");
}

// plat/cv181x/include/ddr/ddr_pkg_info.h
#[derive(Debug)]
#[repr(u32)]
pub enum DramVendor {
    Unknown = 0,
    NY4GbitDDR3 = 1,
    NY2GbitDDR3 = 2,
    ESMT1GbitDDR2 = 3,
    ESMTN25512MbitDDR2 = 4,
    ETRON1Gbit = 5,
    ESMT2GbitDDR3 = 6,
    PM2G = 7,
    PM1G = 8,
    ETRON512MbitDDR2 = 9,
    ESMTN251GbitDDR3 = 10,
}

fn pwrctl_init() -> (u32, u32, u32, u32) {
    // Write 0 to PCTRL_n.port_en, without port 0
    // port number = 0,1,2,3
    for i in 1..4 {
        write32(DDR_CFG_BASE + 0x490 + 0xb0 * i, 0x0);
    }

    // Poll PSTAT.rd_port_busy_n = 0
    // Poll PSTAT.wr_port_busy_n = 0
    while read32(DDR_CFG_BASE + 0x3fc) != 0 {
        println!("  Poll PSTAT.rd_port_busy_n = 0");
    }

    // disable PWRCTL.powerdown_en, PWRCTL.selfref_en
    let v = read32(DDR_CFG_BASE + 0x30);
    // save for later
    let selfref_sw = (v >> 5) & 0b1;
    let en_dfi_dram_clk_disable = (v >> 3) & 0b1;
    let powerdown_en = (v >> 1) & 0b1;
    let selfref_en = v & 0b1;
    // PWRCTL.selfref_sw
    let v = v & !(1 << 5);
    // PWRCTL.en_dfi_dram_clk_disable
    let v = v & !(1 << 3);
    // PWRCTL.deeppowerdown_en, non-mDDR/non-LPDDR2/non-LPDDR3,
    // v = v & !(1 << 2);
    // this register must not be set to 1
    // PWRCTL.powerdown_en
    let v = v & !(1 << 1);
    // PWRCTL.selfref_en
    let v = v & !1;
    write32(DDR_CFG_BASE + 0x30, v);
    (
        selfref_sw,
        en_dfi_dram_clk_disable,
        powerdown_en,
        selfref_en,
    )
}

fn pwrctl_restore(
    selfref_sw: u32,
    en_dfi_dram_clk_disable: u32,
    powerdown_en: u32,
    selfref_en: u32,
) {
    // RFSHCTL3.dis_auto_refresh = 0
    // let v = read32(DDR_CFG_BASE + 0x60);
    // let v = v & !(0b1);
    // write32(DDR_CFG_BASE + 0x60, v);
    // restore PWRCTL.powerdown_en, PWRCTL.selfref_en
    let v = read32(DDR_CFG_BASE + 0x30);
    // PWRCTL.selfref_sw
    let v = v & !(1 << 5) | (selfref_sw << 5);
    // PWRCTL.en_dfi_dram_clk_disable
    let v = v & !(1 << 3) | (en_dfi_dram_clk_disable << 3);
    // PWRCTL.deeppowerdown_en, non-mDDR/non-LPDDR2/non-LPDDR3,
    // let v = v & !(1 << 2);
    // this register must not be set to 1
    // PWRCTL.powerdown_en
    let v = v & !(1 << 1) | (powerdown_en << 1);
    // PWRCTL.selfref_en
    let v = v & !(1) | selfref_en;
    write32(DDR_CFG_BASE + 0x30, v);

    // Write 1 to PCTRL_n.port_en
    for i in 1..4 {
        write32(DDR_CFG_BASE + 0x490 + 0xb0 * i, 0x1);
    }
}

fn bist_x_init_finish() {
    // specified DDR space
    write32(DDR_BIST_BASE + 0x10, 0x00000000);
    write32(DDR_BIST_BASE + 0x14, 0x000fffff);
    // specified AXI address step
    let v = if X16_MODE { 0x00000004 } else { 0x00000008 };
    write32(DDR_BIST_BASE + 0x18, v);
}

const BIST_OP_WRITE: u32 = 1 << 30;
const BIST_OP_READ: u32 = 2 << 30;
const BIST_OP_GOTO: u32 = 3 << 30;

// command queue: 6 registers
// 31..30: op code; 1: write, 2: read
// 29..21: start
// 20..12: stop
// 8: DQ invert
// 7: DM invert
// 6..4: DQ rotate
// 3..0: repetitions

fn cvx16_bist_wr_prbs_init() {
    println!("    bist_wr_prbs_init");
    // bist clock enable
    write32(DDR_BIST_BASE + 0x0, 0x00060006);

    let base_cmd = (0 << 21) | (511 << 12) | (0b0101 << 9);
    // W  1~17  prbs  repeat0
    let cmd1 = BIST_OP_WRITE | base_cmd;
    // R  1~17  prbs  repeat0
    let cmd2 = BIST_OP_READ | base_cmd;
    // write cmd queue
    write32(DDR_BIST_BASE + 0x40, cmd1);
    write32(DDR_BIST_BASE + 0x44, cmd2);
    // NOP
    for i in 0..4 {
        write32(DDR_BIST_BASE + 0x48 + i * 4, 0);
    }

    bist_x_init_finish();
    println!("    bist_wr_prbs_init done");
}

fn cvx16_bist_wrlvl_init() {
    println!("    bist_wrlvl_init");
    // bist clock enable
    write32(DDR_BIST_BASE + 0x0, 0x00060006);

    let cmd = BIST_OP_WRITE | (0b0101 << 9);
    write32(DDR_BIST_BASE + 0x40, cmd);
    // NOP
    for i in 0..5 {
        write32(DDR_BIST_BASE + 0x44 + i * 4, 0);
    }

    bist_x_init_finish();
    println!("     bist_wrlvl_init done");
}

fn cvx16_bist_rdglvl_init() {
    println!("    bist_rdglvl_init");
    // bist clock enable
    write32(DDR_BIST_BASE + 0x0, 0x00060006);

    let cmd = BIST_OP_READ | (0 << 21) | (3 << 12) | (0b0101 << 9);
    write32(DDR_BIST_BASE + 0x40, cmd);
    // NOP
    for i in 0..5 {
        write32(DDR_BIST_BASE + 0x44 + i * 4, 0);
    }

    bist_x_init_finish();
    println!("     bist_rdglvl_init done");
}

fn cvx16_bist_wdmlvl_init() {
    println!("    bist_wdmlvl_init");
    // bist clock enable
    write32(DDR_BIST_BASE + 0x0, 0x00060006);

    let fmax = 15;
    let fmin = 5;
    let fdiff = (fmax - fmin + 1);
    // 8*f/4 -1
    let sram_sp = 9 * (fmin + fmax) * fdiff / 2 / 4 + fdiff;
    println!("      sram_sp = {sram_sp:08x}");

    // bist sso_period
    write32(DDR_BIST_BASE + 0x24, (fmax << 8) + fmin);

    let cmd1 = BIST_OP_WRITE | (sram_sp << 12) | (0b0011 << 9);
    let cmd2 = BIST_OP_WRITE | (sram_sp << 12) | (0b0111 << 9);
    let cmd3 = BIST_OP_READ | (sram_sp << 12) | (0b0111 << 9);
    write32(DDR_BIST_BASE + 0x40, cmd1);
    write32(DDR_BIST_BASE + 0x44, cmd2);
    write32(DDR_BIST_BASE + 0x48, cmd3);
    // NOP
    for i in 0..5 {
        write32(DDR_BIST_BASE + 0x4c + i * 4, 0);
    }

    bist_x_init_finish();
    println!("    bist_wdmlvl_init done");
}

fn cvx16_bist_wdqlvl_init(mode: u32) {
    println!("    bist_wdqlvl_init");
    // bist clock enable
    write32(DDR_BIST_BASE + 0x0, 0x00060006);

    if mode == 0 {
        // phyd pattern
        let base_cmd = (0 << 21) | (3 << 12) | (0b0101 << 9);
        let cmd1 = BIST_OP_WRITE | base_cmd;
        let cmd2 = BIST_OP_READ | base_cmd;
        write32(DDR_BIST_BASE + 0x40, cmd1);
        write32(DDR_BIST_BASE + 0x44, cmd2);
        // NOP
        for i in 0..4 {
            write32(DDR_BIST_BASE + 0x48 + i * 4, 0);
        }
    } else if mode == 0x1 {
        // bist write/read
        let fmin = 5;
        let fmax = 15;
        let fdiff = fmax - fmin + 1;
        // 8*f/4 -1
        let sram_sp = 9 * (fmin + fmax) * fdiff / 2 / 4 + fdiff;
        println!("      sram_sp = {sram_sp:08x}");

        // bist sso_period
        write32(DDR_BIST_BASE + 0x24, (fmax << 8) + fmin);
        let base1 = (511 << 12) | (0b0101 << 9);
        let base2 = (sram_sp << 12) | (0b0110 << 9);
        write32(DDR_BIST_BASE + 0x40, BIST_OP_WRITE | base1);
        write32(DDR_BIST_BASE + 0x44, BIST_OP_READ | base1);
        write32(DDR_BIST_BASE + 0x48, BIST_OP_WRITE | base2);
        write32(DDR_BIST_BASE + 0x4c, BIST_OP_READ | base2);
        //       GOTO      addr_not_reset loop_cnt
        write32(DDR_BIST_BASE + 0x50, BIST_OP_GOTO | (0 << 20) | (1 << 0));
        // NOP
        write32(DDR_BIST_BASE + 0x54, 0);
    } else if (mode == 0x11) {
        // bist write/read
        // TODO
    } else if (mode == 0x12) {
        // bist write/read
        // TODO
    } else {
        // TODO
    }

    bist_x_init_finish();
    println!("    bist_wdqlvl_init done");
}

fn cvx16_bist_wr_sram_init() {
    // TODO
}

fn cvx16_rdlvl_req(x: u32) {
    //
}

fn cvx16_rdlvl_sw_req(x: u32) {
    //
}

fn cvx16_rdglvl_req() {
    // NOTE: training need ctrl_low_patch first
    let (
        selfref_sw,
        en_dfi_dram_clk_disable,
        powerdown_en,
        selfref_en, //
    ) = pwrctl_init();

    cvx16_clk_gating_disable();

    // RFSHCTL3.dis_auto_refresh = 1
    // let v = read32(DDR_CFG_BASE + 0x60);
    // write32(DDR_CFG_BASE + 0x60, v | 1);

    let ddr3 = DDR3 || (DDR2_3 && (get_ddr_type() == DDR_TYPE_DDR3));
    let ddr3_mpr_mode = read32(PHYD_BASE_ADDR + 0x0184) & (1 << 4) != 0;

    if ddr3 && ddr3_mpr_mode {
        // RFSHCTL3.dis_auto_refresh =1
        let v = read32(DDR_CFG_BASE + 0x60);
        write32(DDR_CFG_BASE + 0x60, v | 0x1);
        // MR3
        let v = read32(DDR_CFG_BASE + 0xe0);
        // Dataflow from MPR
        let v = v | (1 << 2);
        cvx16_synp_mrw(0x3, v & 0xffff);
    }

    // bist setting for dfi rdglvl
    cvx16_bist_rdglvl_init();

    // param_phyd_dfi_rdglvl_req
    let v = read32(PHYD_BASE_ADDR + 0x0184);
    write32(PHYD_BASE_ADDR + 0x0184, v | 1);

    println!("wait retraining finish ...");
    //[0] param_phyd_dfi_wrlvl_done
    //[1] param_phyd_dfi_rdglvl_done
    //[2] param_phyd_dfi_rdlvl_done
    //[3] param_phyd_dfi_wdqlvl_done
    while read32(PHYD_BASE_ADDR + 0x3444) & (1 << 1) == 0 {}
    // BIST clock disable
    write32(DDR_BIST_BASE + 0x0, 0x00040000);

    if ddr3 && ddr3_mpr_mode {
        // MR3
        let v = read32(DDR_CFG_BASE + 0xe0);
        // Normal operation
        let v = v & !(1 << 2);
        cvx16_synp_mrw(0x3, v & 0xffff);
        // RFSHCTL3.dis_auto_refresh = 0
        let v = read32(DDR_CFG_BASE + 0x60);
        write32(DDR_CFG_BASE + 0x60, v & !1);
    }

    pwrctl_restore(
        selfref_sw,
        en_dfi_dram_clk_disable,
        powerdown_en,
        selfref_en,
    );

    // cvx16_rdglvl_status();
    cvx16_clk_gating_enable();
}

fn cvx16_clk_gating_enable() {
    // TOP_REG_CG_EN_PHYD_TOP      0
    // TOP_REG_CG_EN_CALVL         1
    // TOP_REG_CG_EN_WRLVL         2
    // N/A                         3
    // TOP_REG_CG_EN_WRDQ          4
    // TOP_REG_CG_EN_RDDQ          5
    // TOP_REG_CG_EN_PIGTLVL       6
    // TOP_REG_CG_EN_RGTRACK       7
    // TOP_REG_CG_EN_DQSOSC        8
    // TOP_REG_CG_EN_LB            9
    // TOP_REG_CG_EN_DLL_SLAVE     10 //0:a-on
    // TOP_REG_CG_EN_DLL_MST       11 //0:a-on
    // TOP_REG_CG_EN_ZQ            12
    // TOP_REG_CG_EN_PHY_PARAM     13 //0:a-on
    // 0b10110010000001
    write32(0x44 + PHYD_APB, 0x00002C81);
    // #ifdef _mem_freq_1333
    // #ifdef DDR2
    let v = read32(DDR_CFG_BASE + 0x190);
    let v = v & !(0b11111 << 24) | (6 << 24);
    write32(DDR_CFG_BASE + 0x190, v);
    // #endif
    // PHYD_SHIFT_GATING_EN
    write32(0x00F4 + PHYD_BASE_ADDR, 0x00030033);
    // phyd_stop_clk
    let v = read32(DDR_CFG_BASE + 0x30);
    write32(DDR_CFG_BASE + 0x30, v | 1 << 9);
    // dfi read/write clock gatting
    let v = read32(DDR_CFG_BASE + 0x148);
    let v = v | (1 << 23) | (1 << 31);
    write32(DDR_CFG_BASE + 0x148, v);
    println!("clk_gating_enable");

    // disable clock gating
    // write32(0x0800_a000 + 0x14 , 0x00000fff);
    // println!("axi disable clock gating");
}

fn cvx16_clk_gating_disable() {
    // TOP_REG_CG_EN_PHYD_TOP      0
    // TOP_REG_CG_EN_CALVL         1
    // TOP_REG_CG_EN_WRLVL         2
    // N/A                         3
    // TOP_REG_CG_EN_WRDQ          4
    // TOP_REG_CG_EN_RDDQ          5
    // TOP_REG_CG_EN_PIGTLVL       6
    // TOP_REG_CG_EN_RGTRACK       7
    // TOP_REG_CG_EN_DQSOSC        8
    // TOP_REG_CG_EN_LB            9
    // TOP_REG_CG_EN_DLL_SLAVE     10 //0:a-on
    // TOP_REG_CG_EN_DLL_MST       11 //0:a-on
    // TOP_REG_CG_EN_ZQ            12
    // TOP_REG_CG_EN_PHY_PARAM     13 //0:a-on
    // 0b01001011110101
    write32(0x44 + PHYD_APB, 0x000012F5);
    // PHYD_SHIFT_GATING_EN
    write32(0x00F4 + PHYD_BASE_ADDR, 0x00000000);
    // phyd_stop_clk
    let v = read32(DDR_CFG_BASE + 0x30);
    let v = v & !(1 << 9);
    write32(DDR_CFG_BASE + 0x30, v);
    // dfi read/write clock gatting
    let v = read32(DDR_CFG_BASE + 0x148);
    let v = v & !((1 << 23) | (1 << 31));
    write32(DDR_CFG_BASE + 0x148, v);
    println!("  clk_gating_disable");

    // disable clock gating
    // write32(0x0800_a000 + 0x14 , 0x00000fff);
    // println!("axi disable clock gating");
}

fn cvx16_synp_mrw(addr: u32, data: u32) {
    // ZQCTL0.dis_auto_zq to 1.
    let v = read32(DDR_CFG_BASE + 0x180);
    let init_dis_auto_zq = if v >> 31 == 0 {
        write32(DDR_CFG_BASE + 0x180, v | (1 << 31));
        println!("    non-lp4 Write ZQCTL0.dis_auto_zq to 1");
        // opdelay(256);
        println!("  Wait tzqcs = 128 cycles");
        true
    } else {
        false
    };
    // Poll MRSTAT.mr_wr_busy until it is 0
    println!("  Poll MRSTAT.mr_wr_busy until it is 0");
    while read32(DDR_CFG_BASE + 0x18) & (1 << 0) != 0 {}
    println!("    non-lp4 Poll MRSTAT.mr_wr_busy finish");
    // Write the MRCTRL0.mr_type, MRCTRL0.mr_addr, MRCTRL0.mr_rank
    // and (for MRWs) MRCTRL1.mr_data
    // rddata[31:0]  = 0;
    // rddata[0]     = 0;       // mr_type  0:write   1:read
    // rddata[5:4]   = 1;       // mr_rank
    // rddata[15:12] = addr;    // mr_addr
    let v = (0b01 << 4) | ((addr & 0b1111) << 12);
    write32(DDR_CFG_BASE + 0x10, v);
    println!("    non-lp4 Write the MRCTRL0");
    // rddata[31:0] = 0;
    // rddata[15:0] = data;     // mr_data
    write32(DDR_CFG_BASE + 0x14, data & 0xffff);
    println!("    non-lp4 Write the MRCTRL1");
    // Write MRCTRL0.mr_wr to 1
    let v = read32(DDR_CFG_BASE + 0x10);
    write32(DDR_CFG_BASE + 0x10, v | (1 << 31));
    println!("    non-lp4 Write MRCTRL0.mr_wr to 1");
    if init_dis_auto_zq {
        // ZQCTL0.dis_auto_zq to 0.
        let v = read32(DDR_CFG_BASE + 0x180);
        write32(DDR_CFG_BASE + 0x180, v & !(1 << 31));
        println!("    non-lp4 Write ZQCTL0.dis_auto_zq to 0");
    }
}

fn cvx16_wrlvl_req() {
    // NOTE: training need ctrl_low_patch first
    // wrlvl response only DQ0
    write32(0x005C + PHYD_BASE_ADDR, 0x00FE0000);

    // Note: training need ctrl_low_patch first
    let (
        selfref_sw,
        en_dfi_dram_clk_disable,
        powerdown_en,
        selfref_en, //
    ) = pwrctl_init();

    cvx16_clk_gating_disable();

    // save ctrl wr_odt_en
    let v = read32(DDR_CFG_BASE + 0x244);
    let wr_odt_en = v & 0b1;

    // bist setting for dfi wrlvl
    cvx16_bist_wrlvl_init();

    // RFSHCTL3.dis_auto_refresh = 1
    // let v = read32(DDR_CFG_BASE + 0x60);
    // write32(DDR_CFG_BASE + 0x60, v | 1);

    let ddr3 = DDR3 || (DDR2_3 && (get_ddr_type() == DDR_TYPE_DDR3));
    if ddr3 {
        let mut rtt_nom = 0;
        if (wr_odt_en == 1) {
            println!("wr_odt_en = 1 ...");

            let v = read32(DDR_CFG_BASE + 0xe0);
            // save rtt_wr bits 26..25
            let rtt_wr = (v >> 25) & 0b11;
            if (rtt_wr != 0x0) {
                // disable rtt_wr
                let v = v & !(0b11 << 25);
                // MR2
                cvx16_synp_mrw(0x2, v >> 16);
                // set rtt_nom
                rtt_nom = read32(DDR_CFG_BASE + 0xdc);
                // rtt_nom[2]=0
                rtt_nom = rtt_nom & !(1 << 9);
                // rtt_nom[1]=rtt_wr[1]
                let b = (rtt_wr >> 1) & 0b1;
                rtt_nom = rtt_nom & !(1 << 6) | (b << 6);
                // rtt_nom[1]=rtt_wr[0]
                let b = rtt_wr & 0b1;
                rtt_nom = rtt_nom & !(1 << 2) | (b << 2);
                println!("dodt for wrlvl setting");
            }
        } else {
            println!("rtt_nom for wrlvl setting");
            println!("wr_odt_en = 0 ...");

            // set rtt_nom = 120ohm
            rtt_nom = read32(DDR_CFG_BASE + 0xdc);
            // rtt_nom[2]=0
            rtt_nom = rtt_nom & !(1 << 9);
            // rtt_nom[1]=1
            rtt_nom = rtt_nom | (1 << 6);
            // rtt_nom[1]=0
            rtt_nom = rtt_nom & !(1 << 2);
            cvx16_synp_mrw(0x1, rtt_nom & 0xffff);
        }
        // Write leveling enable
        rtt_nom = rtt_nom | (1 << 7);
        cvx16_synp_mrw(0x1, rtt_nom & 0xffff);
        println!("DDR3 MRS rtt_nom ...");
    }

    if DDR4 {
        let v = read32(DDR_CFG_BASE + 0xdc);
        // Write leveling enable
        let v = v | (1 << 7);
        cvx16_synp_mrw(0x1, v & 0xffff);
    }

    let v = read32(PHYD_BASE_ADDR + 0x0180);
    // param_phyd_dfi_wrlvl_req
    let v = v | 1;
    // param_phyd_dfi_wrlvl_odt_en
    let v = v & !(1 << 4) | (wr_odt_en << 4);
    write32(PHYD_BASE_ADDR + 0x0180, v);
    println!("wait retraining finish ...");

    //[0] param_phyd_dfi_wrlvl_done
    //[1] param_phyd_dfi_rdglvl_done
    //[2] param_phyd_dfi_rdlvl_done
    //[3] param_phyd_dfi_wdqlvl_done
    while read32(PHYD_BASE_ADDR + 0x3444) & (1 << 0) == 0 {}
    // BIST clock disable
    write32(DDR_BIST_BASE + 0x0, 0x00040000);

    // RFSHCTL3.dis_auto_refresh =0
    let v = read32(DDR_CFG_BASE + 0x60);
    write32(DDR_CFG_BASE + 0x60, v & !(0b1));

    if ddr3 {
        let v = read32(DDR_CFG_BASE + 0xdc);
        // let v = v & !(1 << 7);
        // Write leveling disable
        cvx16_synp_mrw(0x1, v & 0xffff);
        let v = read32(DDR_CFG_BASE + 0xe0);
        // MR2
        cvx16_synp_mrw(0x2, v >> 16);
    }

    if DDR4 {
        let v = read32(DDR_CFG_BASE + 0xdc);
        // let v = v & !(1 << 7);
        // Write leveling disable
        cvx16_synp_mrw(0x1, v & 0xffff);
    }

    pwrctl_restore(
        selfref_sw,
        en_dfi_dram_clk_disable,
        powerdown_en,
        selfref_en,
    );

    // cvx16_wrlvl_status();
    cvx16_clk_gating_enable();
}

fn cvx16_dfi_ca_park_prbs(cap_enable: bool) {
    // param_phyd_sw_dfi_phyupd_req =1
    write32(PHYD_BASE_ADDR + 0x0174, 0x1);
    // param_phyd_to_reg_dfi_phyupd_req  8   8
    // param_phyd_to_reg_dfi_phyupd_ack  9   9
    while (read32(PHYD_BASE_ADDR + 0x3030) >> 8) & 0b11 != 0b11 {}

    // DDR3
    //   cfg_det_en = 0b1;
    //   cfg_cs_det_en = 0b1;
    //   cap_prbs_en = 0b1;
    //   cfg_cs_polarity = 0b1;
    //   cap_prbs_1t = 0b0;
    //   cfg_ca_reference = {0b0,0x0_ffff,0x7,0x0,0b1,0b0,0b1,0b1};
    //   cfg_cs_retain_cycle = 0b0000_0001;
    //   cfg_ca_retain_cycle = 0b0000_0000;
    //   cfg_ca_park_value = 0x3fff_ffff;

    let dfi_ca_park_misc = if cap_enable { 0x1B } else { 0 };
    write32(DDR_TOP_BASE + 0x00, dfi_ca_park_misc);
    println!("    dfi_ca_park_prbs enable = {cap_enable}");

    // dfi_ca_park_retain_cycle;
    write32(DDR_TOP_BASE + 0x04, 0x1);
    // dfi_ca_park_ca_ref
    write32(DDR_TOP_BASE + 0x08, 0x1ffffcb);
    // dfi_ca_park_ca_park
    write32(DDR_TOP_BASE + 0x0c, 0x3fffffff);

    // param_phyd_sw_dfi_phyupd_req_clr =1
    write32(PHYD_BASE_ADDR + 0x0174, 0x00000010);
}

enum LvlMode {
    WdmLvl,
    WdqLvl,
    WdqAndWdmLvl,
}

fn cvx16_wdqlvl_req(data_mode: u32, lvl_mode: LvlMode) {
    // NOTE: training need ctrl_low_patch first
    let (
        selfref_sw,
        en_dfi_dram_clk_disable,
        powerdown_en,
        selfref_en, //
    ) = pwrctl_init();

    cvx16_clk_gating_disable();
    println!("   cvx16_dfi_ca_park_prbs  start");
    cvx16_dfi_ca_park_prbs(true);
    println!("   cvx16_dfi_ca_park_prbs  done");

    // param_phyd_piwdqlvl_dq_mode
    // <= #RD (~pwstrb_mask[12] & param_phyd_piwdqlvl_dq_mode) | pwstrb_mask_pwdata[12];
    // param_phyd_piwdqlvl_dm_mode
    // <= #RD (~pwstrb_mask[13] & param_phyd_piwdqlvl_dm_mode) | pwstrb_mask_pwdata[13];
    // 13: param_phyd_piwdqlvl_dm_mode
    // 12: param_phyd_piwdqlvl_dq_mode
    let bb = match lvl_mode {
        LvlMode::WdmLvl => 1 << 13,
        LvlMode::WdqLvl => 1 << 12,
        LvlMode::WdqAndWdmLvl => (1 << 13) | (1 << 12),
    };
    let v = read32(0x00BC + PHYD_BASE_ADDR);
    write32(0x00BC + PHYD_BASE_ADDR, v & !(0b11 << 12) | bb);

    match lvl_mode {
        LvlMode::WdmLvl => {
            let v = read32(DDR_CFG_BASE + 0xC);
            write32(DDR_CFG_BASE + 0xC, v | (1 << 17));
            // cvx16_bist_wdmlvl_init(sram_sp);
            cvx16_bist_wdmlvl_init();
        }
        _ => {
            // bist setting for dfi rdglvl
            // data_mode = 0x0 : phyd pattern
            // data_mode = 0x1 : bist read/write
            // data_mode = 0x11: with Error enject,  multi- bist write/read
            // data_mode = 0x12: with Error enject,  multi- bist write/read
            // cvx16_bist_wdqlvl_init(data_mode, sram_sp);
            cvx16_bist_wdqlvl_init(data_mode);
        }
    }

    // param_phyd_dfi_wdqlvl
    let v = read32(PHYD_BASE_ADDR + 0x018C);
    println!("      phyd_dfi_wdqlvl {v:08x}");
    // req
    let v = v | 0b1;
    let vref_train_en = match lvl_mode {
        LvlMode::WdmLvl => 0,
        _ => 1,
    };
    let bist_data_en = match data_mode {
        0x1 | 0x11 | 0x12 => 1,
        _ => 0,
    };
    let clr_mask = !((1 << 10) | (1 << 4));
    let v = (v & clr_mask) | (vref_train_en << 10) | (bist_data_en << 4);
    write32(PHYD_BASE_ADDR + 0x018C, v);
    println!("      phyd_dfi_wdqlvl {v:08x}");

    println!("    wait retraining finish ...");
    //[0] param_phyd_dfi_wrlvl_done
    //[1] param_phyd_dfi_rdglvl_done
    //[2] param_phyd_dfi_rdlvl_done
    //[3] param_phyd_dfi_wdqlvl_done
    while read32(PHYD_BASE_ADDR + 0x3444) & (1 << 3) == 0 {}

    let v = read32(DDR_CFG_BASE + 0xC);
    let v = v & !(1 << 7);
    write32(DDR_CFG_BASE + 0xC, v);
    // BIST clock disable
    write32(DDR_BIST_BASE + 0x0, 0x00040000);

    cvx16_dfi_ca_park_prbs(false);

    pwrctl_restore(
        selfref_sw,
        en_dfi_dram_clk_disable,
        powerdown_en,
        selfref_en,
    );

    // cvx16_wdqlvl_status();
    cvx16_clk_gating_enable();
}

fn cvx16_wdqlvl_sw_req(x: u32, y: u32) {
    //
}

fn ctrl_init_high_patch() {
    // enable auto PD/SR
    write32(0x08004000 + 0x30, 0x00000002);
    // enable auto ctrl_upd
    write32(0x08004000 + 0x1a0, 0x00400018);
    // enable clock gating
    write32(0x0800a000 + 0x14, 0x00000000);
    // change xpi to multi DDR burst
    // write32(0x08004000 + 0xc, 0x63786370);
}

fn ctrl_init_detect_dram_size() -> u32 {
    let mut cap_in_mbyte = 4;

    if DDR3 || DDR2_3 && get_ddr_type() == DDR_TYPE_DDR3 {
        fn bist_poll() -> u32 {
            // bist_enable
            write32(DDR_BIST_BASE + 0x0, 0x00010001);
            // poll for BIST done
            let res = loop {
                let r = read32(DDR_BIST_BASE + 0x80);
                if r & (1 << 2) != 0 {
                    break r;
                }
            };
            // bist disable
            write32(DDR_BIST_BASE + 0x0, 0x00010000);
            println!("          BIST poll: {res:08x}");
            res
        }

        // Axsize = 3, axlen = 4, cgen
        write32(DDR_BIST_BASE + 0x0, 0x000e0006);

        // DDR space
        write32(DDR_BIST_BASE + 0x10, 0x00000000);
        write32(DDR_BIST_BASE + 0x14, 0xffffffff);

        // specified AXI address step
        write32(DDR_BIST_BASE + 0x18, 0x00000004);

        // write PRBS to 0x0 as background {{{
        let cmd = BIST_OP_WRITE | (3 << 12) | (0b0101 << 9);
        write32(DDR_BIST_BASE + 0x40, cmd);
        // NOP
        for i in 0..5 {
            write32(DDR_BIST_BASE + 0x44 + i * 4, 0);
        }

        let mut res = bist_poll();

        // (get_bits_from_value(rddata, 3, 3) == 0) &&
        // BIST may fail stop the loop (?)
        while (res & (1 << 3) == 0 && cap_in_mbyte < 15) {
            cap_in_mbyte += 1;
            println!("    cap_in_mbyte = {cap_in_mbyte}");

            // DDR space
            write32(DDR_BIST_BASE + 0x10, 1 << (cap_in_mbyte + 20 - 4));

            // write ~PRBS to (0x1 << *dram_cap_in_mbyte) {{{

            // write 16 UI~prbs
            let cmd = BIST_OP_WRITE | (3 << 12) | (0b0101 << 9) | (1 << 8);
            write32(DDR_BIST_BASE + 0x40, cmd);
            // NOP
            for i in 0..5 {
                write32(DDR_BIST_BASE + 0x44 + i * 4, 0);
            }

            res = bist_poll();

            // check PRBS at 0x0 {{{
            // read 16 UI prbs
            let cmd = BIST_OP_READ | (3 << 12) | (0b0101 << 9);
            write32(DDR_BIST_BASE + 0x40, cmd);
            // NOP
            for i in 0..5 {
                write32(DDR_BIST_BASE + 0x44 + i * 4, 0);
            }

            res = bist_poll();
        }
    }

    if DDR2 || DDR2_3 && get_ddr_type() == DDR_TYPE_DDR2 {
        cap_in_mbyte = 6;
    }

    // save dram_cap_in_mbyte
    write32(PHYD_BASE_ADDR + 0x0208, cap_in_mbyte);

    // clock gen: BIST clock disable
    write32(DDR_BIST_BASE + 0x0, 0x00040000);

    cap_in_mbyte
}

fn ctrl_init_update_by_dram_size(size: u32) {
    let v = read32(0x08004000 + 0x0);
    let s1 = (v >> 12) & 0b11;
    let s2 = (v >> 30) & 0b11;
    println!("   DRAM cap shift vals: x16 {s1}, dev {s2}");
    // DRAM cap in megabytes per cap
    let dram_cap_in_mbyte = size;
    // change sys cap to x16 cap
    let dram_cap_in_mbyte = dram_cap_in_mbyte >> (1 - s1);
    // change x16 cap to device cap
    let dram_cap_in_mbyte = dram_cap_in_mbyte >> (2 - s2);
    /*
    dram_cap_in_mbyte_per_dev >>= (1 - get_bits_from_value(rddata, 13, 12));
    dram_cap_in_mbyte_per_dev >>= (2 - get_bits_from_value(rddata, 31, 30));
    */
    println!("   DRAM cap in MB per dev: {dram_cap_in_mbyte}");
    match dram_cap_in_mbyte {
        6 => {
            write32(0x08004000 + 0x64, 0x0071002A);
            write32(0x08004000 + 0x120, 0x00000903);
        }
        7 => {
            write32(0x08004000 + 0x64, 0x00710034);
            write32(0x08004000 + 0x120, 0x00000903);
        }
        8 => {
            write32(0x08004000 + 0x64, 0x0071004B);
            write32(0x08004000 + 0x120, 0x00000904);
        }
        9 => {
            write32(0x08004000 + 0x64, 0x0071007A);
            write32(0x08004000 + 0x120, 0x00000905);
        }
        10 => {
            write32(0x08004000 + 0x64, 0x007100A4);
            write32(0x08004000 + 0x120, 0x00000907);
        }
        _ => {
            // not supposed to happen, but you never know...
            println!("  WARNING: unsupported DRAM cap in MB per dev");
        }
    }
    // toggle refresh_update_level
    write32(0x08004000 + 0x60, 0x00000002);
    write32(0x08004000 + 0x60, 0x00000000);
}

fn cvx16_dram_cap_check(size: u32) {
    // TODO
}

fn bist() -> Result<(), ()> {
    // bist enable
    let v = if X16_MODE { 0x00030003 } else { 0x00010001 };
    write32(DDR_BIST_BASE + 0x0, v);
    println!(">> BIST start");
    let res = loop {
        let r = read32(DDR_BIST_BASE + 0x0080);
        if r & (1 << 2) != 0 {
            break r;
        }
    };
    let success = res & (1 << 3) == 0;
    let (odd, even) = if success {
        // read err_data
        let ol = read32(DDR_BIST_BASE + 0x88) as u64;
        let oh = read32(DDR_BIST_BASE + 0x8c) as u64;
        let el = read32(DDR_BIST_BASE + 0x90) as u64;
        let eh = read32(DDR_BIST_BASE + 0x94) as u64;
        (oh << 32 | ol, eh << 32 | el)
    } else {
        (0, 0)
    };
    // bist disable
    write32(DDR_BIST_BASE + 0x0, 0x00050000);

    if success {
        println!("-  BIST success");
        Ok(())
    } else {
        println!("-  BIST err_data_odd  {odd:016x}");
        println!("-  BIST err_data_even {even:016x}");
        Err(())
    }
}

use crate::mem_map::AXI_MON_BASE;

const REMAPPING_BASE: usize = 0;
const AXIMON_M1_WRITE: usize = REMAPPING_BASE + 0x0;
const AXIMON_M1_READ: usize = REMAPPING_BASE + 0x80;
const AXIMON_M2_WRITE: usize = REMAPPING_BASE + 0x100;
const AXIMON_M2_READ: usize = REMAPPING_BASE + 0x180;
const AXIMON_M3_WRITE: usize = REMAPPING_BASE + 0x200;
const AXIMON_M3_READ: usize = REMAPPING_BASE + 0x280;
const AXIMON_M4_WRITE: usize = REMAPPING_BASE + 0x300;
const AXIMON_M4_READ: usize = REMAPPING_BASE + 0x380;
const AXIMON_M5_WRITE: usize = REMAPPING_BASE + 0x400;
const AXIMON_M5_READ: usize = REMAPPING_BASE + 0x480;
const AXIMON_M6_WRITE: usize = REMAPPING_BASE + 0x500;
const AXIMON_M6_READ: usize = REMAPPING_BASE + 0x580;

const AXIMON_OFFSET_LAT_BIN_SIZE_SEL: usize = 0x50;

fn axi_mon_latency_setting(lat_bin_size_sel: u32) {
    // for ddr3 1866: bin_size_sel=0d'5
    write32(
        (AXI_MON_BASE + AXIMON_M1_WRITE + AXIMON_OFFSET_LAT_BIN_SIZE_SEL),
        lat_bin_size_sel,
    );
    write32(
        (AXI_MON_BASE + AXIMON_M1_READ + AXIMON_OFFSET_LAT_BIN_SIZE_SEL),
        lat_bin_size_sel,
    );

    // input clk sel
    write32(AXI_MON_BASE + AXIMON_M1_WRITE + 0x00, 0x01000100);
    // hit sel setting
    let rdata = read32(AXI_MON_BASE + AXIMON_M1_WRITE + 0x04);
    write32(AXI_MON_BASE + AXIMON_M1_WRITE + 0x04, rdata & 0xfffffc00);

    write32(AXI_MON_BASE + AXIMON_M1_READ + 0x00, 0x01000100);
    let rdata = read32(AXI_MON_BASE + AXIMON_M1_READ + 0x04);
    write32(AXI_MON_BASE + AXIMON_M1_READ + 0x04, rdata & 0xfffffc00);

    write32(
        AXI_MON_BASE + AXIMON_M5_WRITE + AXIMON_OFFSET_LAT_BIN_SIZE_SEL,
        lat_bin_size_sel,
    );
    write32(
        AXI_MON_BASE + AXIMON_M5_READ + AXIMON_OFFSET_LAT_BIN_SIZE_SEL,
        lat_bin_size_sel,
    );

    write32(AXI_MON_BASE + AXIMON_M5_WRITE + 0x00, 0x01000100);
    let rdata = read32(AXI_MON_BASE + AXIMON_M5_WRITE + 0x04);
    write32(AXI_MON_BASE + AXIMON_M5_WRITE + 0x04, rdata & 0xfffffc00);

    write32(AXI_MON_BASE + AXIMON_M5_READ + 0x00, 0x01000100);
    let rdata = read32(AXI_MON_BASE + AXIMON_M5_READ + 0x04);
    write32(AXI_MON_BASE + AXIMON_M5_READ + 0x04, rdata & 0xfffffc00);

    // ERROR("mon cg en.\n");
    let rdata = read32(DDR_TOP_BASE + 0x14);
    write32((DDR_TOP_BASE + 0x14), rdata | 0x00000100);
}

const AXIMON_START_REGVALUE: u32 = 0x30001;
fn axi_mon_start(b: usize) {
    write32(AXI_MON_BASE + b, AXIMON_START_REGVALUE);
}

fn axi_mon_start_all() {
    axi_mon_start(AXIMON_M1_WRITE);
    axi_mon_start(AXIMON_M1_READ);
    axi_mon_start(AXIMON_M2_WRITE);
    axi_mon_start(AXIMON_M2_READ);
    axi_mon_start(AXIMON_M3_WRITE);
    axi_mon_start(AXIMON_M3_READ);
    axi_mon_start(AXIMON_M4_WRITE);
    axi_mon_start(AXIMON_M4_READ);
    axi_mon_start(AXIMON_M5_WRITE);
    axi_mon_start(AXIMON_M5_READ);
    axi_mon_start(AXIMON_M6_WRITE);
    axi_mon_start(AXIMON_M6_READ);
}

// fsbl plat/cv181x/ddr/ddr_sys_bring_up.c ddr_sys_bring_up
pub fn init(ddr_data_rate: usize, dram_vendor: u32) {
    let (reg_set, reg_span, reg_step) = get_pll_settings(ddr_data_rate);
    cvx16_pll_init(reg_set, reg_span, reg_step);
    ddrc_init();

    // release ddrc soft reset
    println!("Release DDR controller from reset");
    write32(DDR_TOP_BASE + 0x20, 0x0);

    // set axi QOS
    // M1 = 0xA (VIP realtime)
    // M2 = 0x8 (VIP offline)
    // M3 = 0x7 (CPU)
    // M4 = 0x0 (TPU)
    // M5 = 0x9 (Video codec)
    // M6 = 0x2 (high speed peri)
    write32(TOP_BASE + 0x01D8, 0x007788aa);
    write32(TOP_BASE + 0x01DC, 0x00002299);

    phy_init();
    cvx16_setting_check();
    cvx16_pinmux(unsafe { core::mem::transmute(dram_vendor) });
    ddr_patch_set();
    cvx16_en_rec_vol_mode();
    cvx16_set_dfi_init_start();
    cvx16_ddr_phy_power_on_seq1();
    cvx16_polling_dfi_init_start();
    cvx16_int_isr_08();
    cvx16_ddr_phy_power_on_seq2();
    cvx16_set_dfi_init_complete();
    change_pll_freq(reg_set, reg_span, reg_step);
    cvx16_ddr_phy_power_on_seq3();
    cvx16_wait_for_dfi_init_complete();
    cvx16_polling_synp_normal_mode();

    const DRAM_TEST: bool = false;
    // a very simple write+read check
    if DRAM_TEST {
        let offset1 = 0x3000;
        let offset2 = 0x6000;
        for i in (0..128).step_by(4) {
            let a = DRAM_BASE + offset1 + i as usize;
            let v = 0xff00_0000 | i as u32;
            write32(a, v);
            let a = DRAM_BASE + offset2 + i as usize;
            let v = 0x0f0f_0000 | i as u32;
            write32(a, v);
        }
        for i in (0..128).step_by(4) {
            let a = DRAM_BASE + offset1 + i as usize;
            let e = 0xff00_0000 | i as u32;
            let v = read32(a);
            if v != e {
                panic!("@{a:08x} expected {e:08x} got {v:08x}");
            }
            let a = DRAM_BASE + offset2 + i as usize;
            let e = 0x0f0f_0000 | i as u32;
            let v = read32(a);
            if v != e {
                panic!("@{a:08x} expected {e:08x} got {v:08x}");
            }
        }
    }

    if DO_BIST {
        cvx16_bist_wr_prbs_init();
        if let Err(()) = bist() {
            panic!("ERROR bist_fail");
        }
    }

    ctrl_init_low_patch();
    println!("ctrl_low_patch finish");

    if !DDR2 {
        cvx16_wrlvl_req();
        println!("cvx16_wrlvl_req finish");
    }

    if DO_BIST {
        cvx16_bist_wr_prbs_init();
        if let Err(()) = bist() {
            panic!("ERROR bist_fail");
        }
    }

    cvx16_rdglvl_req();
    println!("cvx16_rdglvl_req finish");

    if DO_BIST {
        cvx16_bist_wr_prbs_init();
        if let Err(()) = bist() {
            panic!("ERROR bist_fail");
        }
    }

    //ERROR("AXI mon setting for latency histogram.\n");
    //axi_mon_set_lat_bin_size(0x5);

    if DBG_SHMOO {
        /*
        // DPHY WDQ
        // param_phyd_dfi_wdqlvl_vref_start [6:0]
        // param_phyd_dfi_wdqlvl_vref_end [14:8]
        // param_phyd_dfi_wdqlvl_vref_step [19:16]
        write32(0x08000190, 0x00021E02);
        // param_phyd_piwdqlvl_dly_step[23:20]
        write32(0x080000a4, 0x01220504);
        // write start   shift = 5  /  dline = 78
        write32(0x080000a0, 0x0d400578);
        //write
        println!("wdqlvl_M1_ALL_DQ_DM\n");
        // data_mode = 'h0 : phyd pattern
        // data_mode = 'h1 : bist read/write
        // data_mode = 'h11: with Error enject,  multi- bist write/read
        // data_mode = 'h12: with Error enject,  multi- bist write/read
        // lvl_mode  = 'h0 : wdmlvl
        // lvl_mode  = 'h1 : wdqlvl
        // lvl_mode  = 'h2 : wdqlvl and wdmlvl
        // cvx16_wdqlvl_req(data_mode, lvl_mode)
        println!("cvx16_wdqlvl_sw_req dq/dm");
        // console_getc();
        cvx16_wdqlvl_sw_req(1, 2);
        // cvx16_wdqlvl_status();
        println!("cvx16_wdqlvl_req dq/dm finish");

        println!("cvx16_wdqlvl_sw_req dq");
        // console_getc();
        cvx16_wdqlvl_sw_req(1, 1);
        // cvx16_wdqlvl_status();
        println!("cvx16_wdqlvl_req dq finish");

        println!("cvx16_wdqlvl_sw_req dm");
        // console_getc();
        cvx16_wdqlvl_sw_req(1, 0);
        // cvx16_wdqlvl_status();
        */
        println!("cvx16_wdqlvl_req dm finish");
    } else {
        // cvx16_wdqlvl_req
        println!(" wdqlvl_M1_ALL_DQ_DM");
        // sso_8x1_c(5, 15, 0, 1, &sram_sp);
        // mode = write, input int fmin = 5, input int fmax = 15,
        // input int sram_st = 0, output int sram_sp

        // data_mode = 'h0 : phyd pattern
        // data_mode = 'h1 : bist read/write
        // data_mode = 'h11: with Error enject,  multi- bist write/read
        // data_mode = 'h12: with Error enject,  multi- bist write/read
        // lvl_mode  = 'h0 : wdmlvl
        // lvl_mode  = 'h1 : wdqlvl
        // lvl_mode  = 'h2 : wdqlvl and wdmlvl
        cvx16_wdqlvl_req(1, LvlMode::WdqAndWdmLvl);
        println!("  cvx16_wdqlvl_req dq/dm finish");
        cvx16_wdqlvl_req(1, LvlMode::WdqLvl);
        println!("  cvx16_wdqlvl_req dq finish");
        cvx16_wdqlvl_req(1, LvlMode::WdmLvl);
        println!("  cvx16_wdqlvl_req dm finish");
        if DO_BIST {
            cvx16_bist_wr_prbs_init();
            if let Err(()) = bist() {
                panic!("-- ERROR bist_fail");
            }
        }
    }

    /*
    if DBG_SHMOO {
        // param_phyd_pirdlvl_dly_step [3:0]
        // param_phyd_pirdlvl_vref_step [11:8]
        write32(0x08000088, 0x0A010212);

        //read
        println!("cvx16_rdlvl_req start");
        // console_getc();
        println!("SW mode 1, sram write/read continuous goto");
        cvx16_rdlvl_sw_req(1);
        // cvx16_rdlvl_status();
        println!("cvx16_rdlvl_req finish");
    } else {
        // cvx16_rdlvl_req
        // mode = 'h0  : MPR mode, DDR3 only.
        // mode = 'h1  : sram write/read continuous goto
        // mode = 'h2  : multi- bist write/read
        // mode = 'h10 : with Error enject,  multi- bist write/read
        // mode = 'h12 : with Error enject,  multi- bist write/read
        let v = read32(PHYD_BASE_ADDR + 0x008c);
        // param_phyd_pirdlvl_capture_cnt
        let v = (v & (0b1111 << 4)) | 0x1;
        write32(PHYD_BASE_ADDR + 0x008c + PHYD_BASE_ADDR, v);

        println!("mode multi- bist write/read");
        // mode multi- PRBS bist write/read
        // cvx16_rdlvl_req(2);
        // mode multi- SRAM bist write/read
        cvx16_rdlvl_req(1);
        println!("cvx16_rdlvl_req finish");

        if DO_BIST {
            cvx16_bist_wr_prbs_init();
            if let Err(()) = bist() {
                panic!("ERROR bist_fail");
            }
        }
    }
    */

    /*
    if DBG_SHMOO_CA {
        //CA training
        NOTICE("\n===== calvl_req =====\n"); console_getc();
        // sso_8x1_c(5, 15, sram_sp, 1, &sram_sp_1);
        calvl_req(cap);
    }

    if DBG_SHMOO_CS {
        //CS training
        NOTICE("\n===== cslvl_req =====\n"); console_getc();
        // sso_8x1_c(5, 15, sram_sp, 1, &sram_sp_1);
        cslvl_req(cap);
    }
    */

    if DBG_SHMOO {
        /*
        cvx16_dll_cal_status();
        cvx16_wrlvl_status();
        cvx16_rdglvl_status();
        cvx16_rdlvl_status();
        cvx16_wdqlvl_status();
        */
    }

    ctrl_init_high_patch();

    let dram_cap_in_mbyte = ctrl_init_detect_dram_size();
    println!("dram_cap_in_mbyte: {dram_cap_in_mbyte}");
    ctrl_init_update_by_dram_size(dram_cap_in_mbyte);
    println!("ctrl_init_update_by_dram_size finish");
    println!("dram_cap_in_mbyte: {dram_cap_in_mbyte}");
    cvx16_dram_cap_check(dram_cap_in_mbyte);
    println!("cvx16_dram_cap_check finish");

    // clk_gating_enable
    cvx16_clk_gating_enable();
    println!("cvx16_clk_gating_enable finish");

    if DO_BIST {
        cvx16_bist_wr_prbs_init();
        if let Err(()) = bist() {
            println!("ERROR prbs bist_fail");
            panic!("DDR BIST FAIL");
        }
        /*
        cvx16_bist_wr_sram_init();
        if let Err(()) = bist() {
            println!("ERROR sram bist_fail");
            panic!("ERROR bist_fail");
        }
        */
        println!("DDR BIST PASS");
    }

    /*
    #ifdef FULL_MEM_BIST
        //full memory
        // sso_8x1_c(5, 15, 0, 1, &sram_sp);
        // sso_8x1_c(5, 15, sram_sp, 1, &sram_sp);

        NOTICE("====FULL_MEM_BIST====\n");
        bist_result = bist_all_dram(0, cap);
        if (bist_result == 0) {
            NOTICE("bist_all_dram(prbs): ERROR bist_fail\n");
        } else {
            NOTICE("bist_all_dram(prbs): BIST PASS\n");
        }

        bist_result = bist_all_dram(1, cap);
        if (bist_result == 0) {
            NOTICE("bist_all_dram(sram): ERROR bist_fail\n");
        } else {
            NOTICE("bist_all_dram(sram): BIST PASS\n");
        }

        bist_result = bist_all_dram(2, cap);
        if (bist_result == 0) {
            NOTICE("bist_all_dram(01): ERROR bist_fail\n");
        } else {
            NOTICE("bist_all_dram(01): BIST PASS\n");
        }

        NOTICE("===== BIST END ======\n");
    #endif //FULL_MEM_BIST
    */

    /*
    if FULL_MEM_BIST_FOREVER {
        println!("Start DRAM stress test");
        bist_all_dram_forever(cap);
    }
    */

    // ERROR("AXI mon setting for latency histogram.\n");
    axi_mon_latency_setting(0x5);

    // ERROR("AXI mon 0 register dump before start.\n");
    // dump_axi_mon_reg(AXIMON_M1_WRITE);
    // ERROR("AXI mon 1 register dump before start.\n");
    // dump_axi_mon_reg(AXIMON_M1_READ);

    axi_mon_start_all();
}
