use crate::util::{read32, write32};

// plat/cv181x/include/ddr/ddr_sys.h
const DDR_SYS_BASE: usize = 0x08000000;
const PI_BASE: usize = DDR_SYS_BASE + 0x0000;
const PHY_BASE: usize = DDR_SYS_BASE + 0x2000;
const DDRC_BASE: usize = DDR_SYS_BASE + 0x4000;
const PHYD_BASE: usize = DDR_SYS_BASE + 0x6000;
const CV_DDR_PHYD_APB: usize = DDR_SYS_BASE + 0x6000;
const AXI_MON_BASE: usize = DDR_SYS_BASE + 0x8000;
const DDR_TOP_BASE: usize = DDR_SYS_BASE + 0xa000;
const PHYD_BASE_ADDR: usize = DDR_SYS_BASE;
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

const TX_VREF_PD: usize = CV_DDR_PHYD_APB + 0x0028;
const ZQ_240_OPTION: usize = CV_DDR_PHYD_APB + 0x0054;
const GPO_SETTING: usize = CV_DDR_PHYD_APB + 0x0058;

fn cvx16_pll_init(reg_set: u32, reg_span: u32, reg_step: u32) {
    println!("cvx16_pll_init");
    // opdelay(10);
    // NOTE: macro resolves to no-op, plat/cv180x/include/ddr/ddr_sys.h
    // ddr_debug_wr32(0x00);
    ddr_debug_num_write();

    write32(TX_VREF_PD, 0x00000000);
    write32(ZQ_240_OPTION, 0x00080001);

    // TODO: variants
    // #ifdef DDR3
    /*
    #ifdef _mem_freq_2133
    // TOP_REG_TX_DDR3_GPO_IN = 0
    write32(GPO_SETTING, 0x01000808);
    #else
    */
    // TOP_REG_TX_DDR3_GPO_IN = 1
    write32(GPO_SETTING, 0x01010808);
    /*
    #endif
    #endif
    #ifdef DDR2_3
    if (get_ddr_type() == DDR_TYPE_DDR3) {
        // TOP_REG_TX_DDR3_GPO_IN = 1
        write32(GPO_SETTING, 0x01010808);
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
    // #endif // SSC_EN
    /*
        // opdelay(1000);
        // DDRPLL setting
        rddata = read32(0x0C + CV_DDR_PHYD_APB);
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
        rddata = modified_bits_by_value(rddata, 0x030b, 15, 0);
        write32(0x0C + CV_DDR_PHYD_APB, rddata);
        rddata = read32(0x10 + CV_DDR_PHYD_APB);
        //[7:0] = 0x0;   //TOP_REG_DDRPLL_TEST
        rddata = modified_bits_by_value(rddata, 0, 7, 0); // TOP_REG_DDRPLL_TEST
        write32(0x10 + CV_DDR_PHYD_APB, rddata);
        //[0]   = 1;    //TOP_REG_RESETZ_DIV
        rddata = 0x1;
        write32(0x04 + CV_DDR_PHYD_APB, rddata);
        uartlog("RSTZ_DIV=1\n");
        rddata = read32(0x0C + CV_DDR_PHYD_APB);
        //[7]   = 1;    //TOP_REG_DDRPLL_MAS_RSTZ_DIV
        rddata = modified_bits_by_value(rddata, 1, 7, 7);
        write32(0x0C + CV_DDR_PHYD_APB, rddata);
        KC_MSG("Wait for DRRPLL LOCK=1... pll init\n");

        uartlog("Start DRRPLL LOCK pll init\n");
    #ifdef REAL_LOCK
        while (1) {
            rddata = read32(0x10 + CV_DDR_PHYD_APB);
            if (get_bits_from_value(rddata, 15, 15)) {
                break;
            }
        }
    #else
        KC_MSG("check PLL lock...  pll init\n");

    #endif
    */
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

pub fn init(ddr_data_rate: usize) {
    pll_init(ddr_data_rate);
}
