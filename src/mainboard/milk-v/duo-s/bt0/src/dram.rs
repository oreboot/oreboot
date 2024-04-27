use crate::util::{read32, write32};

// plat/cv181x/include/ddr/ddr_sys.h
const DDR_SYS_BASE: usize = 0x0800_0000;
const PI_BASE: usize = DDR_SYS_BASE + 0x0000;
const PHYD_BASE_ADDR: usize = DDR_SYS_BASE; // ?? used in phy_init
const PHY_BASE: usize = DDR_SYS_BASE + 0x2000;
const DDRC_BASE: usize = DDR_SYS_BASE + 0x4000;
const PHYD_BASE: usize = DDR_SYS_BASE + 0x6000;
const AXI_MON_BASE: usize = DDR_SYS_BASE + 0x8000;
const DDR_TOP_BASE: usize = DDR_SYS_BASE + 0xa000;
const DDR_BIST_BASE: usize = DDR_SYS_BASE + 0x0001_0000;

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
    // mmio_wr32(4*(186 + PHY_BASE_ADDR)+CADENCE_PHYD,(debug_seqnum<<8));
    // if(debug_seqnum ==255){ ;
    // debug_seqnum1 = debug_seqnum1+1 ;
    // mmio_wr32(4*(186 + PHY_BASE_ADDR)+CADENCE_PHYD,(debug_seqnum1<<8));
    // debug_seqnum = 0 ;
    // } ;
}

const TX_VREF_PD: usize = PHYD_BASE + 0x0028;
const ZQ_240_OPTION: usize = PHYD_BASE + 0x0054;
const GPO_SETTING: usize = PHYD_BASE + 0x0058;

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
    let v = read32(0x0C + PHYD_BASE);
    write32(0x0C + PHYD_BASE, (v & 0xffff_0000) | 0x030b);

    // DDRPLL_TEST
    // [7:0] = 0x0;
    let v = read32(0x10 + PHYD_BASE);
    write32(0x10 + PHYD_BASE, v & 0xffff_ff00);

    // RESETZ_DIV
    // [0]   = 1;
    write32(0x04 + PHYD_BASE, 0x1);
    println!("RSTZ_DIV=1");

    // DDRPLL_MAS_RSTZ_DIV
    // [7]   = 1;
    let v = read32(0x0C + PHYD_BASE);
    write32(0x0C + PHYD_BASE, (v & !(1 << 7)) | (1 << 7));

    println!("Wait for DDR PLL LOCK=1... pll init");

    println!("Start DDR PLL LOCK pll init");

    const REAL_LOCK: bool = true;
    if REAL_LOCK {
        while read32(0x10 + PHYD_BASE) & (1 << 15) == 0 {
            //
        }
    } else {
        println!("check PLL lock...  pll init");
    }
    println!("End DRRPLL LOCK=1... pll init");
    println!("PLL init finish !!!");
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
    let v = read32(DDRC_BASE + 0xc);
    println!("DDRC 0x000c {v:08x}");
    // "ctcq" (qctc)
    write32(DDRC_BASE + 0xc, 0x63746371);
    let v = read32(DDRC_BASE + 0xc);
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
    write32(DDRC_BASE + 0x44, 0x00000000);
    // PATCH1.ref_adv_stop_threshold:0:7:=0x0
    // PATCH1.ref_adv_dec_threshold:8:7:=0x0
    // PATCH1.ref_adv_max:16:7:=0x0
    write32(DDRC_BASE + 0x148, 0x999F0000);
    // PATCH4.t_phyd_rden:16:6=0x0
    // PATCH4.phyd_rd_clk_stop:23:1=0x0
    // PATCH4.t_phyd_wren:24:6=0x0
    // PATCH4.phyd_wr_clk_stop:31:1=0x0
    // auto gen.
    write32(DDRC_BASE + 0x0, 0x81041401);
    write32(DDRC_BASE + 0x30, 0x00000000);
    write32(DDRC_BASE + 0x34, 0x00930001);
    write32(DDRC_BASE + 0x38, 0x00020000);
    write32(DDRC_BASE + 0x50, 0x00201070);
    write32(DDRC_BASE + 0x60, 0x00000000);
    write32(DDRC_BASE + 0x64, 0x007100A4);
    write32(DDRC_BASE + 0xc0, 0x00000000);
    write32(DDRC_BASE + 0xc4, 0x00000000);
    if DDR_INIT_SPEED_UP {
        write32(DDRC_BASE + 0xd0, 0x00010002);
        write32(DDRC_BASE + 0xd4, 0x00020000);
    } else {
        write32(DDRC_BASE + 0xd0, 0x000100E5);
        write32(DDRC_BASE + 0xd4, 0x006A0000);
    }
    write32(DDRC_BASE + 0xdc, 0x1F140040);
    if DDR_DODT {
        write32(DDRC_BASE + 0xe0, 0x04600000);
    } else {
        write32(DDRC_BASE + 0xe0, 0x00600000);
    }
    write32(DDRC_BASE + 0xe4, 0x000B03BF);
    write32(DDRC_BASE + 0x100, 0x0E111F10);
    write32(DDRC_BASE + 0x104, 0x00030417);
    write32(DDRC_BASE + 0x108, 0x0507060A);
    write32(DDRC_BASE + 0x10c, 0x00002007);
    write32(DDRC_BASE + 0x110, 0x07020307);
    write32(DDRC_BASE + 0x114, 0x05050303);
    write32(DDRC_BASE + 0x120, 0x00000907);
    write32(DDRC_BASE + 0x13c, 0x00000000);
    write32(DDRC_BASE + 0x180, 0xC0960026);
    write32(DDRC_BASE + 0x184, 0x00000001);
    // phyd related
    write32(DDRC_BASE + 0x190, 0x048a8305);
    // DFITMG0.dfi_t_ctrl_delay:24:5:=0x4
    // DFITMG0.dfi_rddata_use_dfi_phy_clk:23:1:=0x1
    // DFITMG0.dfi_t_rddata_en:16:7:=0xa
    // DFITMG0.dfi_wrdata_use_dfi_phy_clk:15:1:=0x1
    // DFITMG0.dfi_tphy_wrdata:8:6:=0x3
    // DFITMG0.dfi_tphy_wrlat:0:6:=0x5
    write32(DDRC_BASE + 0x194, 0x00070202);
    // DFITMG1.dfi_t_cmd_lat:28:4:=0x0
    // DFITMG1.dfi_t_parin_lat:24:2:=0x0
    // DFITMG1.dfi_t_wrdata_delay:16:5:=0x7
    // DFITMG1.dfi_t_dram_clk_disable:8:5:=0x2
    // DFITMG1.dfi_t_dram_clk_enable:0:5:=0x2
    write32(DDRC_BASE + 0x198, 0x07c13121);
    // DFILPCFG0.dfi_tlp_resp:24:5:=0x7
    // DFILPCFG0.dfi_lp_wakeup_dpd:20:4:=0xc
    // DFILPCFG0.dfi_lp_en_dpd:16:1:=0x1
    // DFILPCFG0.dfi_lp_wakeup_sr:12:4:=0x3
    // DFILPCFG0.dfi_lp_en_sr:8:1:=0x1
    // DFILPCFG0.dfi_lp_wakeup_pd:4:4:=0x2
    // DFILPCFG0.dfi_lp_en_pd:0:1:=0x1
    write32(DDRC_BASE + 0x19c, 0x00000021);
    // DFILPCFG1.dfi_lp_wakeup_mpsm:4:4:=0x2
    // DFILPCFG1.dfi_lp_en_mpsm:0:1:=0x1
    // auto gen.
    write32(DDRC_BASE + 0x1a0, 0xC0400018);
    write32(DDRC_BASE + 0x1a4, 0x00FE00FF);
    write32(DDRC_BASE + 0x1a8, 0x80000000);
    write32(DDRC_BASE + 0x1b0, 0x000002C1);
    write32(DDRC_BASE + 0x1c0, 0x00000001);
    write32(DDRC_BASE + 0x1c4, 0x00000001);
    // address map, auto gen.
    write32(DDRC_BASE + 0x200, 0x00001F1F);
    write32(DDRC_BASE + 0x204, 0x00070707);
    write32(DDRC_BASE + 0x208, 0x00000000);
    write32(DDRC_BASE + 0x20c, 0x1F000000);
    write32(DDRC_BASE + 0x210, 0x00001F1F);
    write32(DDRC_BASE + 0x214, 0x060F0606);
    write32(DDRC_BASE + 0x218, 0x06060606);
    write32(DDRC_BASE + 0x21c, 0x00000606);
    write32(DDRC_BASE + 0x220, 0x00003F3F);
    write32(DDRC_BASE + 0x224, 0x06060606);
    write32(DDRC_BASE + 0x228, 0x06060606);
    write32(DDRC_BASE + 0x22c, 0x001F1F06);
    // auto gen.
    write32(DDRC_BASE + 0x240, 0x08000610);
    if DDR_DODT {
        write32(DDRC_BASE + 0x244, 0x00000001);
    } else {
        write32(DDRC_BASE + 0x244, 0x00000000);
    }
    write32(DDRC_BASE + 0x250, 0x00003F85);
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
    write32(DDRC_BASE + 0x254, 0x00000000);
    // SCHED1.page_hit_limit_rd:28:3:=0x0
    // SCHED1.page_hit_limit_wr:24:3:=0x0
    // SCHED1.visible_window_limit_rd:20:3:=0x0
    // SCHED1.visible_window_limit_wr:16:3:=0x0
    // SCHED1.delay_switch_write:12:4:=0x0
    // SCHED1.pageclose_timer:0:8:=0x0
    // auto gen.
    write32(DDRC_BASE + 0x25c, 0x100000F0);
    // PERFHPR1.hpr_xact_run_length:24:8:=0x20
    // PERFHPR1.hpr_max_starve:0:16:=0x6a
    write32(DDRC_BASE + 0x264, 0x100000F0);
    // PERFLPR1.lpr_xact_run_length:24:8:=0x20
    // PERFLPR1.lpr_max_starve:0:16:=0x6a
    write32(DDRC_BASE + 0x26c, 0x100000F0);
    // PERFWR1.w_xact_run_length:24:8:=0x20
    // PERFWR1.w_max_starve:0:16:=0x1a8
    write32(DDRC_BASE + 0x300, 0x00000000);
    // DBG0.dis_max_rank_wr_opt:7:1:=0x0
    // DBG0.dis_max_rank_rd_opt:6:1:=0x0
    // DBG0.dis_collision_page_opt:4:1:=0x0
    // DBG0.dis_act_bypass:2:1:=0x0
    // DBG0.dis_rd_bypass:1:1:=0x0
    // DBG0.dis_wc:0:1:=0x0
    write32(DDRC_BASE + 0x304, 0x00000000);
    // DBG1.dis_hif:1:1:=0x0
    // DBG1.dis_dq:0:1:=0x0
    write32(DDRC_BASE + 0x30c, 0x00000000);
    write32(DDRC_BASE + 0x320, 0x00000001);
    // SWCTL.sw_done:0:1:=0x1
    write32(DDRC_BASE + 0x36c, 0x00000000);
    // POISONCFG.rd_poison_intr_clr:24:1:=0x0
    // POISONCFG.rd_poison_intr_en:20:1:=0x0
    // POISONCFG.rd_poison_slverr_en:16:1:=0x0
    // POISONCFG.wr_poison_intr_clr:8:1:=0x0
    // POISONCFG.wr_poison_intr_en:4:1:=0x0
    // POISONCFG.wr_poison_slverr_en:0:1:=0x0
    write32(DDRC_BASE + 0x400, 0x00000011);
    // PCCFG.dch_density_ratio:12:2:=0x0
    // PCCFG.bl_exp_mode:8:1:=0x0
    // PCCFG.pagematch_limit:4:1:=0x1
    // PCCFG.go2critical_en:0:1:=0x1
    write32(DDRC_BASE + 0x404, 0x00006000);
    // PCFGR_0.rdwr_ordered_en:16:1:=0x0
    // PCFGR_0.rd_port_pagematch_en:14:1:=0x1
    // PCFGR_0.rd_port_urgent_en:13:1:=0x1
    // PCFGR_0.rd_port_aging_en:12:1:=0x0
    // PCFGR_0.read_reorder_bypass_en:11:1:=0x0
    // PCFGR_0.rd_port_priority:0:10:=0x0
    write32(DDRC_BASE + 0x408, 0x00006000);
    // PCFGW_0.wr_port_pagematch_en:14:1:=0x1
    // PCFGW_0.wr_port_urgent_en:13:1:=0x1
    // PCFGW_0.wr_port_aging_en:12:1:=0x0
    // PCFGW_0.wr_port_priority:0:10:=0x0
    write32(DDRC_BASE + 0x490, 0x00000001);
    // PCTRL_0.port_en:0:1:=0x1
    write32(DDRC_BASE + 0x494, 0x00000007);
    // PCFGQOS0_0.rqos_map_region2:24:8:=0x0
    // PCFGQOS0_0.rqos_map_region1:20:4:=0x0
    // PCFGQOS0_0.rqos_map_region0:16:4:=0x0
    // PCFGQOS0_0.rqos_map_level2:8:8:=0x0
    // PCFGQOS0_0.rqos_map_level1:0:8:=0x7
    write32(DDRC_BASE + 0x498, 0x0000006a);
    // PCFGQOS1_0.rqos_map_timeoutr:16:16:=0x0
    // PCFGQOS1_0.rqos_map_timeoutb:0:16:=0x6a
    write32(DDRC_BASE + 0x49c, 0x00000e07);
    // PCFGWQOS0_0.wqos_map_region2:24:8:=0x0
    // PCFGWQOS0_0.wqos_map_region1:20:4:=0x0
    // PCFGWQOS0_0.wqos_map_region0:16:4:=0x0
    // PCFGWQOS0_0.wqos_map_level2:8:8:=0xe
    // PCFGWQOS0_0.wqos_map_level1:0:8:=0x7
    write32(DDRC_BASE + 0x4a0, 0x01a801a8);
    // PCFGWQOS1_0.wqos_map_timeout2:16:16:=0x1a8
    // PCFGWQOS1_0.wqos_map_timeout1:0:16:=0x1a8
    write32(DDRC_BASE + 0x4b4, 0x00006000);
    // PCFGR_1.rdwr_ordered_en:16:1:=0x0
    // PCFGR_1.rd_port_pagematch_en:14:1:=0x1
    // PCFGR_1.rd_port_urgent_en:13:1:=0x1
    // PCFGR_1.rd_port_aging_en:12:1:=0x0
    // PCFGR_1.read_reorder_bypass_en:11:1:=0x0
    // PCFGR_1.rd_port_priority:0:10:=0x0
    write32(DDRC_BASE + 0x4b8, 0x00006000);
    // PCFGW_1.wr_port_pagematch_en:14:1:=0x1
    // PCFGW_1.wr_port_urgent_en:13:1:=0x1
    // PCFGW_1.wr_port_aging_en:12:1:=0x0
    // PCFGW_1.wr_port_priority:0:10:=0x0
    write32(DDRC_BASE + 0x540, 0x00000001);
    // PCTRL_1.port_en:0:1:=0x1
    write32(DDRC_BASE + 0x544, 0x00000007);
    // PCFGQOS0_1.rqos_map_region2:24:8:=0x0
    // PCFGQOS0_1.rqos_map_region1:20:4:=0x0
    // PCFGQOS0_1.rqos_map_region0:16:4:=0x0
    // PCFGQOS0_1.rqos_map_level2:8:8:=0x0
    // PCFGQOS0_1.rqos_map_level1:0:8:=0x7
    write32(DDRC_BASE + 0x548, 0x0000006a);
    // PCFGQOS1_1.rqos_map_timeoutr:16:16:=0x0
    // PCFGQOS1_1.rqos_map_timeoutb:0:16:=0x6a
    write32(DDRC_BASE + 0x54c, 0x00000e07);
    // PCFGWQOS0_1.wqos_map_region2:24:8:=0x0
    // PCFGWQOS0_1.wqos_map_region1:20:4:=0x0
    // PCFGWQOS0_1.wqos_map_region0:16:4:=0x0
    // PCFGWQOS0_1.wqos_map_level2:8:8:=0xe
    // PCFGWQOS0_1.wqos_map_level1:0:8:=0x7
    write32(DDRC_BASE + 0x550, 0x01a801a8);
    // PCFGWQOS1_1.wqos_map_timeout2:16:16:=0x1a8
    // PCFGWQOS1_1.wqos_map_timeout1:0:16:=0x1a8
    write32(DDRC_BASE + 0x564, 0x00006000);
    // PCFGR_2.rdwr_ordered_en:16:1:=0x0
    // PCFGR_2.rd_port_pagematch_en:14:1:=0x1
    // PCFGR_2.rd_port_urgent_en:13:1:=0x1
    // PCFGR_2.rd_port_aging_en:12:1:=0x0
    // PCFGR_2.read_reorder_bypass_en:11:1:=0x0
    // PCFGR_2.rd_port_priority:0:10:=0x0
    write32(DDRC_BASE + 0x568, 0x00006000);
    // PCFGW_2.wr_port_pagematch_en:14:1:=0x1
    // PCFGW_2.wr_port_urgent_en:13:1:=0x1
    // PCFGW_2.wr_port_aging_en:12:1:=0x0
    // PCFGW_2.wr_port_priority:0:10:=0x0
    write32(DDRC_BASE + 0x5f0, 0x00000001);
    // PCTRL_2.port_en:0:1:=0x1
    write32(DDRC_BASE + 0x5f4, 0x00000007);
    // PCFGQOS0_2.rqos_map_region2:24:8:=0x0
    // PCFGQOS0_2.rqos_map_region1:20:4:=0x0
    // PCFGQOS0_2.rqos_map_region0:16:4:=0x0
    // PCFGQOS0_2.rqos_map_level2:8:8:=0x0
    // PCFGQOS0_2.rqos_map_level1:0:8:=0x7
    write32(DDRC_BASE + 0x5f8, 0x0000006a);
    // PCFGQOS1_2.rqos_map_timeoutr:16:16:=0x0
    // PCFGQOS1_2.rqos_map_timeoutb:0:16:=0x6a
    write32(DDRC_BASE + 0x5fc, 0x00000e07);
    // PCFGWQOS0_2.wqos_map_region2:24:8:=0x0
    // PCFGWQOS0_2.wqos_map_region1:20:4:=0x0
    // PCFGWQOS0_2.wqos_map_region0:16:4:=0x0
    // PCFGWQOS0_2.wqos_map_level2:8:8:=0xe
    // PCFGWQOS0_2.wqos_map_level1:0:8:=0x7
    write32(DDRC_BASE + 0x600, 0x01a801a8);
    // PCFGWQOS1_2.wqos_map_timeout2:16:16:=0x1a8
    // PCFGWQOS1_2.wqos_map_timeout1:0:16:=0x1a8
}

pub fn init(ddr_data_rate: usize) {
    pll_init(ddr_data_rate);
    ddrc_init();
}
