use crate::ddr_phy::phy_init;
use crate::mem_map::{DDR_CFG_BASE, DDR_TOP_BASE, PHYD_APB, PHYD_BASE_ADDR, TOP_BASE};
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

// plat/cv181x/ddr/ddr_sys.c
fn ddr_debug_num_write() {
    // debug_seqnum = debug_seqnum+1 ;
    // write32(4*(186 + PHY_BASE_ADDR)+CADENCE_PHYD,(debug_seqnum<<8));
    // if(debug_seqnum ==255){ ;
    // debug_seqnum1 = debug_seqnum1+1 ;
    // write32(4*(186 + PHY_BASE_ADDR)+CADENCE_PHYD,(debug_seqnum1<<8));
    // debug_seqnum = 0 ;
    // } ;
}

const DDR_PLL: usize = PHYD_APB + 0x000c;
const TX_VREF_PD: usize = PHYD_APB + 0x0028;
const ZQ_240_OPTION: usize = PHYD_APB + 0x0054;
const GPO_SETTING: usize = PHYD_APB + 0x0058;

fn cvx16_pll_init(reg_set: u32, reg_span: u32, reg_step: u32) {
    println!("cvx16_pll_init");
    // opdelay(10);
    // NOTE: macro resolves to no-op, plat/cv180x/include/ddr/ddr_sys.h
    // ddr_debug_wr32(0x00);
    ddr_debug_num_write();

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
        // TRM alpha p62
        const CLK_GEN_PLL_CTRL_BASE: usize = 0x0300_2000;
        // TRM alpha p53
        const PLL_G6_BASE: usize = CLK_GEN_PLL_CTRL_BASE + 0x0900;
        const DPLL_SSC_SYN_CTRL: usize = PLL_G6_BASE + 0x0050;
        const DPLL_SSC_SYN_SET: usize = PLL_G6_BASE + 0x0054;
        const DPLL_SSC_SYN_SPAN: usize = PLL_G6_BASE + 0x0058;
        const DPLL_SSC_SYN_STEP: usize = PLL_G6_BASE + 0x005C;

        write32(DPLL_SSC_SYN_SET, reg_set);
        // 15..0
        write32(DPLL_SSC_SYN_SPAN, reg_span);
        // 23..0
        write32(DPLL_SSC_SYN_STEP, reg_step);

        // 6: ssc_syn_fix_div
        // 5: EXTPULSE
        // 4: BYPASS
        // 3..2: MODE
        // 1: EN_SSC
        // 0: SW_UP
        let v = read32(DPLL_SSC_SYN_CTRL);
        println!("DPLL_SSC_SYN_CTRL {v:032b}");
        // negate SW_UP
        let neg_sw_up = !(v & 0x1) & 0x1;
        let v = (v & !0x1) | neg_sw_up;
        let nv = 0b010000 << 1;
        let v = (v & !(0b111111 << 2)) | nv;
        println!("DPLL_SSC_SYN_CTRL {v:032b}");
        write32(DPLL_SSC_SYN_CTRL, v);
        println!("SSC_OFF");
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
    let v = read32(0x10 + PHYD_APB);
    write32(0x10 + PHYD_APB, v & 0xffff_ff00);

    // RESETZ_DIV
    // [0]   = 1;
    write32(0x04 + PHYD_APB, 0x1);

    // DDRPLL_MAS_RSTZ_DIV
    // [7]   = 1;
    let v = read32(DDR_PLL);
    write32(DDR_PLL, v | (1 << 7));

    println!("Wait for DDR PLL LOCK=1...");
    while read32(0x10 + PHYD_APB) & (1 << 15) == 0 {}
    println!("Finished DDR PLL LOCK=1.");
    println!("PLL init done.");
}

fn pll_init(ddr_data_rate: usize) {
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

    cvx16_pll_init(reg_set, reg_span, reg_step);
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

const PHY_REG_VERSION: usize = PHYD_BASE_ADDR + 0x3000;

// plat/cv181x/ddr/ddr_sys.c
fn cvx16_setting_check() {
    println!("| cvx16_setting_check");

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
    println!("| cvx16_setting_check finish");
}

// plat/cv181x/ddr/cvx16_pinmux.c
pub fn cvx16_pinmux(ddr_vendor: DramVendor) {
    println!("| cvx16_pinmux start");
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
    println!("| cvx16_pinmux finish");
}

// This is a full duplicate in the vendor code:
// plat/cv181x/ddr/ddr_config/ddr_auto_x16/ddr_patch_regs.c
// plat/cv181x/ddr/ddr_config/ddr3_1866_x16/ddr_patch_regs.c
fn ddr_patch_set() {
    println!("| ddr_patch_set start");
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
    println!("| ddr_patch_set finish");
}

// plat/cv181x/ddr/ddr_sys.c
fn cvx16_en_rec_vol_mode() {
    println!("| cvx16_en_rec_vol_mode start");
    const DDR2: bool = false;
    if DDR2 {
        write32(0x0500 + PHYD_BASE_ADDR, 0x00001001);
        write32(0x0540 + PHYD_BASE_ADDR, 0x00001001);
    }
    println!("| cvx16_en_rec_vol_mode finish");
}

// DFI = DDR PHY Interface
// https://www.synopsys.com/blogs/chip-design/mastering-ddr-phy-interoperability-dfi.html
// plat/cv181x/ddr/ddr_sys.c
fn cvx16_set_dfi_init_start() {
    // synp setting
    // phy is ready for initial dfi_init_start request
    // set umctl2 to tigger dfi_init_start
    println!("| cvx16_set_dfi_init start");
    write32(DDR_CFG_BASE + 0x00000320, 0x0);
    // dfi_init_start @ rddata[5];
    let v = read32(DDR_CFG_BASE + 0x000001b0);
    write32(DDR_CFG_BASE + 0x000001b0, v | 1 << 5);
    write32(DDR_CFG_BASE + 0x00000320, 0x1);
    println!("| set_dfi_init_start finish");
}

fn cvx16_ddr_phy_power_on_seq1() {
    println!("| ddr_phy_power_on_seq1 start");
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

    println!("| ddr_phy_power_on_seq1 finish");
}

fn cvx16_polling_dfi_init_start() {
    println!("| first dfi_init_start");
    while (read32(0x3028 + PHYD_BASE_ADDR) >> 8) & 0x1 == 0 {}
    println!("| cvx16_polling_dfi_init_start finish");
}

fn cvx16_INT_ISR_08() {
    //
    println!("| cvx16_INT_ISR_08 finish");
}

fn cvx16_ddr_phy_power_on_seq3() {
    //
    println!("| ddr_phy_power_on_seq3 finish");
}

fn cvx16_wait_for_dfi_init_complete() {
    //
    println!("| wait_for_dfi_init_complete finish");
}

fn cvx16_polling_synp_normal_mode() {
    //
    println!("| polling_synp_normal_mode finish");
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

// fsbl plat/cv181x/ddr/ddr_sys_bring_up.c ddr_sys_bring_up
pub fn init(ddr_data_rate: usize, dram_vendor: u32) {
    pll_init(ddr_data_rate);
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

    cvx16_INT_ISR_08();

    cvx16_ddr_phy_power_on_seq3();

    cvx16_wait_for_dfi_init_complete();

    cvx16_polling_synp_normal_mode();

    /*
    #ifdef DO_BIST
        cvx16_bist_wr_prbs_init();
        cvx16_bist_start_check(&bist_result, &err_data_odd, &err_data_even);
        KC_MSG(", bist_result = %x, err_data_odd = %lx, err_data_even = %lx\n", bist_result, err_data_odd,
               err_data_even);
        if (bist_result == 0) {
            ERROR("ERROR bist_fail\n");
            ERROR("bist_result = %x, err_data_odd = %lx, err_data_even = %lx\n", bist_result, err_data_odd,
                  err_data_even);
        }
    #endif

        ctrl_init_low_patch();
        KC_MSG("ctrl_low_patch finish\n");

        // cvx16_wrlvl_req
    #ifndef DDR2
    #ifdef DDR2_3
        if (get_ddr_type() == DDR_TYPE_DDR3) {
            cvx16_wrlvl_req();
            KC_MSG("cvx16_wrlvl_req finish\n");
        }
    #else
        cvx16_wrlvl_req();
        KC_MSG("cvx16_wrlvl_req finish\n");
    #endif
    #endif

    #ifdef DO_BIST
        cvx16_bist_wr_prbs_init();
        cvx16_bist_start_check(&bist_result, &err_data_odd, &err_data_even);
        KC_MSG(", bist_result = %x, err_data_odd = %lx, err_data_even = %lx\n", bist_result, err_data_odd,
               err_data_even);
        if (bist_result == 0) {
            ERROR("ERROR bist_fail\n");
            ERROR("bist_result = %x, err_data_odd = %lx, err_data_even = %lx\n", bist_result, err_data_odd,
                  err_data_even);
        }
    #endif
        // cvx16_rdglvl_req
        cvx16_rdglvl_req();
        KC_MSG("cvx16_rdglvl_req finish\n");
    #ifdef DO_BIST
        cvx16_bist_wr_prbs_init();
        cvx16_bist_start_check(&bist_result, &err_data_odd, &err_data_even);
        KC_MSG(", bist_result = %x, err_data_odd = %lx, err_data_even = %lx\n", bist_result, err_data_odd,
               err_data_even);
        if (bist_result == 0) {
            ERROR("ERROR bist_fail\n");
            ERROR("bist_result = %x, err_data_odd = %lx, err_data_even = %lx\n", bist_result, err_data_odd,
                  err_data_even);
        }
    #endif

        //ERROR("AXI mon setting for latency histogram.\n");
        //axi_mon_set_lat_bin_size(0x5);

    #ifdef DBG_SHMOO
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
        KC_MSG("wdqlvl_M1_ALL_DQ_DM\n");
        // data_mode = 'h0 : phyd pattern
        // data_mode = 'h1 : bist read/write
        // data_mode = 'h11: with Error enject,  multi- bist write/read
        // data_mode = 'h12: with Error enject,  multi- bist write/read
        // lvl_mode  = 'h0 : wdmlvl
        // lvl_mode  = 'h1 : wdqlvl
        // lvl_mode  = 'h2 : wdqlvl and wdmlvl
        // cvx16_wdqlvl_req(data_mode, lvl_mode)
        NOTICE("cvx16_wdqlvl_sw_req dq/dm\n"); console_getc();
        cvx16_wdqlvl_sw_req(1, 2);
        // cvx16_wdqlvl_status();
        KC_MSG("cvx16_wdqlvl_req dq/dm finish\n");

        NOTICE("cvx16_wdqlvl_sw_req dq\n"); console_getc();
        cvx16_wdqlvl_sw_req(1, 1);
        // cvx16_wdqlvl_status();
        KC_MSG("cvx16_wdqlvl_req dq finish\n");

        NOTICE("cvx16_wdqlvl_sw_req dm\n"); console_getc();
        cvx16_wdqlvl_sw_req(1, 0);
        // cvx16_wdqlvl_status();
        NOTICE("cvx16_wdqlvl_req dm finish\n");
    #else //DBG_SHMOO
        // cvx16_wdqlvl_req
        KC_MSG("wdqlvl_M1_ALL_DQ_DM\n");
        // sso_8x1_c(5, 15, 0, 1, &sram_sp);//mode = write, input int fmin = 5, input int fmax = 15,
                            //input int sram_st = 0, output int sram_sp

        // data_mode = 'h0 : phyd pattern
        // data_mode = 'h1 : bist read/write
        // data_mode = 'h11: with Error enject,  multi- bist write/read
        // data_mode = 'h12: with Error enject,  multi- bist write/read
        // lvl_mode  = 'h0 : wdmlvl
        // lvl_mode  = 'h1 : wdqlvl
        // lvl_mode  = 'h2 : wdqlvl and wdmlvl
        // cvx16_wdqlvl_req(data_mode, lvl_mode);
        cvx16_wdqlvl_req(1, 2);
        KC_MSG("cvx16_wdqlvl_req dq/dm finish\n");

        cvx16_wdqlvl_req(1, 1);
        KC_MSG("cvx16_wdqlvl_req dq finish\n");

        cvx16_wdqlvl_req(1, 0);
        KC_MSG("cvx16_wdqlvl_req dm finish\n");

    #ifdef DO_BIST
        cvx16_bist_wr_prbs_init();
        cvx16_bist_start_check(&bist_result, &err_data_odd, &err_data_even);
        KC_MSG(", bist_result = %x, err_data_odd = %lx, err_data_even = %lx\n", bist_result, err_data_odd,
               err_data_even);
        if (bist_result == 0) {
            KC_MSG("ERROR bist_fail\n");
        }
    #endif
    #endif //!DBG_SHMOO

    #ifdef DBG_SHMOO
        // param_phyd_pirdlvl_dly_step [3:0]
        // param_phyd_pirdlvl_vref_step [11:8]
        write32(0x08000088, 0x0A010212);

        //read
        NOTICE("cvx16_rdlvl_req start\n"); console_getc();
        NOTICE("SW mode 1, sram write/read continuous goto\n");
        cvx16_rdlvl_sw_req(1);
        // cvx16_rdlvl_status();
        NOTICE("cvx16_rdlvl_req finish\n");
    #else //DBG_SHMOO
        // cvx16_rdlvl_req
        // mode = 'h0  : MPR mode, DDR3 only.
        // mode = 'h1  : sram write/read continuous goto
        // mode = 'h2  : multi- bist write/read
        // mode = 'h10 : with Error enject,  multi- bist write/read
        // mode = 'h12 : with Error enject,  multi- bist write/read
        rddata = read32(0x008c + PHYD_BASE_ADDR);
        rddata = modified_bits_by_value(rddata, 1, 7, 4); // param_phyd_pirdlvl_capture_cnt
        write32(0x008c + PHYD_BASE_ADDR, rddata);

        KC_MSG("mode multi- bist write/read\n");
        // cvx16_rdlvl_req(2); // mode multi- PRBS bist write/read
        cvx16_rdlvl_req(1); // mode multi- SRAM bist write/read
        KC_MSG("cvx16_rdlvl_req finish\n");
    #ifdef DO_BIST
        cvx16_bist_wr_prbs_init();
        cvx16_bist_start_check(&bist_result, &err_data_odd, &err_data_even);
        KC_MSG(", bist_result = %x, err_data_odd = %lx, err_data_even = %lx\n", bist_result, err_data_odd,
               err_data_even);
        if (bist_result == 0) {
            KC_MSG("ERROR bist_fail\n");
        }
    #endif
    #endif //!DBG_SHMOO

    #ifdef DBG_SHMOO_CA
        //CA training
        NOTICE("\n===== calvl_req =====\n"); console_getc();
        // sso_8x1_c(5, 15, sram_sp, 1, &sram_sp_1);
        calvl_req(cap);
    #endif //DBG_SHMOO_CA

    #ifdef DBG_SHMOO_CS
        //CS training
        NOTICE("\n===== cslvl_req =====\n"); console_getc();
        // sso_8x1_c(5, 15, sram_sp, 1, &sram_sp_1);
        cslvl_req(cap);
    #endif // DBG_SHMOO_CS

    #ifdef DBG_SHMOO
        cvx16_dll_cal_status();
        cvx16_wrlvl_status();
        cvx16_rdglvl_status();
        cvx16_rdlvl_status();
        cvx16_wdqlvl_status();
    #endif // DBG_SHMOO

        // ctrl_high_patch
        ctrl_init_high_patch();

        ctrl_init_detect_dram_size(&dram_cap_in_mbyte);
        KC_MSG("ctrl_init_detect_dram_size finish\n");

        ctrl_init_update_by_dram_size(dram_cap_in_mbyte);
        KC_MSG("ctrl_init_update_by_dram_size finish\n");

        KC_MSG("dram_cap_in_mbyte = %x\n", dram_cap_in_mbyte);
        cvx16_dram_cap_check(dram_cap_in_mbyte);
        KC_MSG("cvx16_dram_cap_check finish\n");

        // clk_gating_enable
        cvx16_clk_gating_enable();
        KC_MSG("cvx16_clk_gating_enable finish\n");

    #ifdef DO_BIST
        cvx16_bist_wr_prbs_init();
        cvx16_bist_start_check(&bist_result, &err_data_odd, &err_data_even);
        KC_MSG(", bist_result = %x, err_data_odd = %lx, err_data_even = %lx\n", bist_result, err_data_odd,
               err_data_even);
        if (bist_result == 0) {
            KC_MSG("ERROR prbs bist_fail\n");
            NOTICE("DDR BIST FAIL\n");
            while (1) {
            }
        }

        cvx16_bist_wr_sram_init();
        cvx16_bist_start_check(&bist_result, &err_data_odd, &err_data_even);
        KC_MSG(", bist_result = %x, err_data_odd = %lx, err_data_even = %lx\n", bist_result, err_data_odd,
               err_data_even);
        if (bist_result == 0) {
            KC_MSG("ERROR sram bist_fail\n");
            NOTICE("DDR BIST FAIL\n");
            while (1) {
            }
        }
        NOTICE("DDR BIST PASS\n");
    #endif

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

    #ifdef FULL_MEM_BIST_FOREVER
        NOTICE("Press any key to start stress test\n"); console_getc();
        bist_all_dram_forever(cap);
    #endif //FULL_MEM_BIST_FOREVER

        //ERROR("AXI mon setting for latency histogram.\n");
        axi_mon_latency_setting(0x5);

        //ERROR("AXI mon 0 register dump before start.\n");
        //dump_axi_mon_reg(AXIMON_M1_WRITE);
        //ERROR("AXI mon 1 register dump before start.\n");
        //dump_axi_mon_reg(AXIMON_M1_READ);

        axi_mon_start_all();
    */
}
