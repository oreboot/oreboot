use crate::util::{read32, udelay, write32};

// This DRAM init code for the K230D is adapted from vendor SDK>
// <https://github.com/kendryte/k230_linux_sdk>
// It is based on Buildroot with a U-Boot overlay in:
// `buildroot-overlay/boot/uboot/u-boot-2022.10-overlay`
//
// Looking through the files, there is `board/canaan/k230d_canmv/Kconfig`, which
// contains `config SIPLP4_2667`; that in turn is found again in
// `board/canaan/common/Makefile`:
// ```
// obj-$(CONFIG_SIPLP4_2667) += gen_siplp4_2667.o
// ```
//
// Find `gen_siplp4_2667` further down, a target generating the C file:
// ```
// $(obj)/gen_siplp4_2667.c: $(src)/sip_lpddr4_init_32_swap_2667.c
//  ($(srctree)/arch/riscv/cpu/k230/ddr.sh $< $@ 0x50000 0x53fff 0x54000 0x5433d  || exit 1)
// ```
// So to only get this file, in `board/canaan/common`, run:
// ```
// sh ../../../arch/riscv/cpu/k230/ddr.sh \
//   sip_lpddr4_init_32_swap_2667.c \
//   gen_siplp4_2667.c \
//   0x50000 0x53fff 0x54000 0x5433d
// ```
// reference code lands in `gen_siplp4_2667.c`

const DEBUG: bool = false;

const CMU_BASE: usize = 0x9110_0000;
const DDR_CLK_CFG: usize = CMU_BASE + 0x0060;

const BOOT_BASE: usize = 0x9110_2000;
const PLL0_CFG0: usize = BOOT_BASE + 0x0;
const PLL0_CFG1: usize = BOOT_BASE + 0x4;
const PLL0_CTL: usize = BOOT_BASE + 0x8;
const PLL0_STAT: usize = BOOT_BASE + 0xC;
const PLL1_CFG0: usize = BOOT_BASE + 0x10;
const PLL1_CFG1: usize = BOOT_BASE + 0x14;
const PLL1_CTL: usize = BOOT_BASE + 0x18;
const PLL1_STAT: usize = BOOT_BASE + 0x1C;
const PLL2_CFG0: usize = BOOT_BASE + 0x20;
const PLL2_CFG1: usize = BOOT_BASE + 0x24;
const PLL2_CTL: usize = BOOT_BASE + 0x28;
const PLL2_STAT: usize = BOOT_BASE + 0x2C;
const PLL3_CFG0: usize = BOOT_BASE + 0x30;
const PLL3_CFG1: usize = BOOT_BASE + 0x34;
const PLL3_CTL: usize = BOOT_BASE + 0x38;
const PLL3_STAT: usize = BOOT_BASE + 0x3C;

const PWR_BASE: usize = 0x9110_3000;
const MEM_CTL_POWER_LPI_CTL: usize = PWR_BASE + 0x009c;

const DDR_BASE: usize = 0x9800_0000;
const DDR_PHY: usize = DDR_BASE + 0x0200_0000;

fn pd_pll(pll_ctl: usize, pll_stat: usize) {
    write32(pll_ctl, 0x0001_0001);
    while read32(pll_stat) & 0x30 != 0x0 {}
}

fn init_pll(pll_ctl: usize, pll_stat: usize) {
    write32(pll_ctl, 0x0002_0002);
    while read32(pll_stat) & 0x30 != 0x20 {}
}

fn cfg_pll(
    fb_div: u32,
    ref_div: u32,
    out_div: u32,
    pllx_cfg0: usize,
    pllx_cfg1: usize,
    pllx_ctl: usize,
    pllx_stat: usize,
) {
    pd_pll(pllx_ctl, pllx_stat);
    // for minimum long term jitter
    write32(pllx_cfg1, (fb_div / 4) | 0x20000);
    write32(
        pllx_cfg0,
        (fb_div & 0x1fff) | ((ref_div & 0x3f) << 16) | ((out_div & 0xf) << 24),
    );
    init_pll(pllx_ctl, pllx_stat);
}

fn change_pll_2660() {
    /* enable cache */
    // NOTE: The recommended value for BWADJ is FBK_DIV/2.Valid values range from 0 to 0xFFF.
    // To minimize long-term jitter, using NB=NF/4 is better. NB = BWADJ[11:0] + 1,
    // So, BWADJ=(NB-1)=[NF/2 -1] or (NF/4 -1)--minimize long term jitter

    // 1860Mhz
    let fb_div = 110; // NF - 1
    let ref_div = 0; // NR - 1
    let out_div = 0; // OD - 1
    cfg_pll(
        fb_div, ref_div, out_div, PLL2_CFG0, PLL2_CFG1, PLL2_CTL, PLL2_STAT,
    );

    // switch source to pll2 div4
    write32(DDR_CLK_CFG, 0x43fc | (1 << 31) | (1 << 1));

    udelay(50);
}

fn init_phy() {
    // write32(DDR_PHY + 4 * 0x1005f, 0x1ff);
    // write32(DDR_PHY + 4 * 0x1015f, 0x1ff);
    // write32(DDR_PHY + 4 * 0x1105f, 0x1ff);
    // write32(DDR_PHY + 4 * 0x1115f, 0x1ff);
    // write32(DDR_PHY + 4 * 0x1205f, 0x1ff);
    // write32(DDR_PHY + 4 * 0x1215f, 0x1ff);
    // write32(DDR_PHY + 4 * 0x1305f, 0x1ff);
    // write32(DDR_PHY + 4 * 0x1315f, 0x1ff);
    // write32(DDR_PHY + 4 * 0x00055, 0x1ff);
    // write32(DDR_PHY + 4 * 0x01055, 0x1ff);
    // write32(DDR_PHY + 4 * 0x02055, 0x1ff);
    // write32(DDR_PHY + 4 * 0x03055, 0x1ff);
    // write32(DDR_PHY + 4 * 0x04055, 0x1ff);
    // write32(DDR_PHY + 4 * 0x05055, 0x1ff);
    // write32(DDR_PHY + 4 * 0x06055, 0x1ff);
    // write32(DDR_PHY + 4 * 0x07055, 0x1ff);
    // write32(DDR_PHY + 4 * 0x08055, 0x1ff);
    // write32(DDR_PHY + 4 * 0x09055, 0x1ff);
    // write32(DDR_PHY + 4 * 0x200c5, 0x019);

    write32(DDR_PHY + 4 * 0x0002002e, 0x00000002);
    write32(DDR_PHY + 4 * 0x00090204, 0x00000000);
    write32(DDR_PHY + 4 * 0x00020024, 0x000000a3);
    write32(DDR_PHY + 4 * 0x0002003a, 0x00000002);
    write32(DDR_PHY + 4 * 0x0002007d, 0x00000212);
    write32(DDR_PHY + 4 * 0x0002007c, 0x00000061);
    write32(DDR_PHY + 4 * 0x00020056, 0x00000003);

    // iteration place
    // PHY RX ODT
    // 0010_00 00_0000 0x208 0x200  Down:120
    // 0010_10 00_0000 0x28a 0x280  Down:80
    // 0110_00 00_0000 0x618 0x600  Down:60
    // 0110_10 00_0000 0x69a 0x680  Down:48
    // 1110_00 00_0000 0xe38 0xe00  Down:40
    // 1110_10 00_0000 0xeba 0xe80  Down:34.3
    write32(DDR_PHY + 4 * 0x0001004d, 0x00000600);
    write32(DDR_PHY + 4 * 0x0001014d, 0x00000600);
    write32(DDR_PHY + 4 * 0x0001104d, 0x00000600);
    write32(DDR_PHY + 4 * 0x0001114d, 0x00000600);
    write32(DDR_PHY + 4 * 0x0001204d, 0x00000600);
    write32(DDR_PHY + 4 * 0x0001214d, 0x00000600);
    write32(DDR_PHY + 4 * 0x0001304d, 0x00000600);
    write32(DDR_PHY + 4 * 0x0001314d, 0x00000600);

    // iteration place
    // PHY TX output impedence
    // 0010_00 00_1000 0x208 Pullup/Down:120
    // 0010_10 00_1010 0x28a Pullup/Down:80
    // 0110_00 01_1000 0x618 Pullup/Down:60
    // 0110_10 01_1010 0x69a Pullup/Down:48
    // 1110_00 11_1000 0xe38 Pullup/Down:40
    // 1110_10 11_1010 0xeba Pullup/Down:34.3
    write32(DDR_PHY + 4 * 0x00010049, 0x00000e38);
    write32(DDR_PHY + 4 * 0x00010149, 0x00000e38);
    write32(DDR_PHY + 4 * 0x00011049, 0x00000e38);
    write32(DDR_PHY + 4 * 0x00011149, 0x00000e38);
    write32(DDR_PHY + 4 * 0x00012049, 0x00000e38);
    write32(DDR_PHY + 4 * 0x00012149, 0x00000e38);
    write32(DDR_PHY + 4 * 0x00013049, 0x00000e38);
    write32(DDR_PHY + 4 * 0x00013149, 0x00000e38);

    // iteration
    // PHY AC/CLK output  impedence
    // 00000_00000  0x0    120
    // 00001_00001  0x21   60
    // 00011_00011  0x63   40
    // 00111_00111  0xe7   30
    // 01111_01111  0x1ef  24
    // 11111_11111  0x3ff  20

    // [phyinit_C_initPhyConfig] Programming ATxImpedance::ADrvStrenP to 0x1
    // [phyinit_C_initPhyConfig] Programming ATxImpedance::ADrvStrenN to 0x1
    write32(DDR_PHY + 4 * 0x00000043, 0x000003ff);
    write32(DDR_PHY + 4 * 0x00001043, 0x000003ff);
    write32(DDR_PHY + 4 * 0x00002043, 0x000003ff);
    write32(DDR_PHY + 4 * 0x00003043, 0x000003ff);
    write32(DDR_PHY + 4 * 0x00004043, 0x000003ff);
    write32(DDR_PHY + 4 * 0x00005043, 0x000003ff);
    write32(DDR_PHY + 4 * 0x00006043, 0x000003ff);
    write32(DDR_PHY + 4 * 0x00007043, 0x000003ff);
    write32(DDR_PHY + 4 * 0x00008043, 0x000003ff);
    write32(DDR_PHY + 4 * 0x00009043, 0x000003ff);

    write32(DDR_PHY + 4 * 0x00020018, 0x00000001);
    write32(DDR_PHY + 4 * 0x00020075, 0x00000004);
    write32(DDR_PHY + 4 * 0x00020050, 0x00000000);
    write32(DDR_PHY + 4 * 0x00020008, 0x0000029b);
    write32(DDR_PHY + 4 * 0x00020088, 0x00000009);

    // iteration place
    // PHY VERF
    // 0x104 15% (0x14)
    // 0x14c 20% (0x1a)
    // 0x19c 25% (0x20)
    // 0x1e4 30% (0x26)
    // 0x284 40% (0x33)

    // [phyinit_C_initPhyConfig] Pstate=0, Programming VrefInGlobal::GlobalVrefInDAC to 0x51
    // [phyinit_C_initPhyConfig] Pstate=0, Programming VrefInGlobal to 0x288
    write32(DDR_PHY + 4 * 0x000200b2, 0x0000014c);

    write32(DDR_PHY + 4 * 0x00010043, 0x000005a1);
    write32(DDR_PHY + 4 * 0x00010143, 0x000005a1);
    write32(DDR_PHY + 4 * 0x00011043, 0x000005a1);
    write32(DDR_PHY + 4 * 0x00011143, 0x000005a1);
    write32(DDR_PHY + 4 * 0x00012043, 0x000005a1);
    write32(DDR_PHY + 4 * 0x00012143, 0x000005a1);
    write32(DDR_PHY + 4 * 0x00013043, 0x000005a1);
    write32(DDR_PHY + 4 * 0x00013143, 0x000005a1);
    write32(DDR_PHY + 4 * 0x000200fa, 0x00000001);
    write32(DDR_PHY + 4 * 0x00020019, 0x00000001);
    write32(DDR_PHY + 4 * 0x000200f0, 0x00000000);
    write32(DDR_PHY + 4 * 0x000200f1, 0x00000000);
    write32(DDR_PHY + 4 * 0x000200f2, 0x00004444);
    write32(DDR_PHY + 4 * 0x000200f3, 0x00008888);
    write32(DDR_PHY + 4 * 0x000200f4, 0x00005555);
    write32(DDR_PHY + 4 * 0x000200f5, 0x00000000);
    write32(DDR_PHY + 4 * 0x000200f6, 0x00000000);
    write32(DDR_PHY + 4 * 0x000200f7, 0x0000f000);
    write32(DDR_PHY + 4 * 0x0001004a, 0x00000500);

    write32(DDR_PHY + 4 * 0x0001104a, 0x00000500);
    write32(DDR_PHY + 4 * 0x00012000, 0x00000004);
    write32(DDR_PHY + 4 * 0x0001204a, 0x000007ff);
    write32(DDR_PHY + 4 * 0x00013000, 0x00000004);
    write32(DDR_PHY + 4 * 0x0001304a, 0x000007ff);

    write32(DDR_PHY + 4 * 0x00020025, 0x00000000);
    write32(DDR_PHY + 4 * 0x0002002d, 0x00000000);
    write32(DDR_PHY + 4 * 0x0002002c, 0x00000000);
    write32(DDR_PHY + 4 * 0x00010020, 0x00000006);
    write32(DDR_PHY + 4 * 0x00011020, 0x00000006);
    write32(DDR_PHY + 4 * 0x00012020, 0x00000006);
    write32(DDR_PHY + 4 * 0x00013020, 0x00000006);
    write32(DDR_PHY + 4 * 0x00020020, 0x00000006);
    write32(DDR_PHY + 4 * 0x000100d0, 0x00000100);
    write32(DDR_PHY + 4 * 0x000101d0, 0x00000100);
    write32(DDR_PHY + 4 * 0x000110d0, 0x00000100);
    write32(DDR_PHY + 4 * 0x000111d0, 0x00000100);
    write32(DDR_PHY + 4 * 0x000120d0, 0x00000100);
    write32(DDR_PHY + 4 * 0x000121d0, 0x00000100);
    write32(DDR_PHY + 4 * 0x000130d0, 0x00000100);
    write32(DDR_PHY + 4 * 0x000131d0, 0x00000100);
    write32(DDR_PHY + 4 * 0x000100c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000101c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000102c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000103c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000104c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000105c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000106c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000107c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000108c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000110c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000111c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000112c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000113c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000114c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000115c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000116c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000117c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000118c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000120c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000121c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000122c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000123c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000124c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000125c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000126c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000127c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000128c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000130c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000131c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000132c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000133c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000134c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000135c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000136c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000137c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x000138c0, 0x00000057);
    write32(DDR_PHY + 4 * 0x00010080, 0x00000318);
    write32(DDR_PHY + 4 * 0x00010180, 0x00000318);
    write32(DDR_PHY + 4 * 0x00011080, 0x00000318);
    write32(DDR_PHY + 4 * 0x00011180, 0x00000318);
    write32(DDR_PHY + 4 * 0x00012080, 0x00000318);
    write32(DDR_PHY + 4 * 0x00012180, 0x00000318);
    write32(DDR_PHY + 4 * 0x00013080, 0x00000318);
    write32(DDR_PHY + 4 * 0x00013180, 0x00000318);
    write32(DDR_PHY + 4 * 0x00090201, 0x00001600);
    write32(DDR_PHY + 4 * 0x00090202, 0x0000000a);
    write32(DDR_PHY + 4 * 0x00090203, 0x00002200);
    write32(DDR_PHY + 4 * 0x00020072, 0x00000001);
    write32(DDR_PHY + 4 * 0x00020073, 0x00000001);
    write32(DDR_PHY + 4 * 0x000100ae, 0x00000027);
    write32(DDR_PHY + 4 * 0x000110ae, 0x00000027);
    write32(DDR_PHY + 4 * 0x000120ae, 0x00000027);
    write32(DDR_PHY + 4 * 0x000130ae, 0x00000027);
    write32(DDR_PHY + 4 * 0x000100af, 0x00000027);
    write32(DDR_PHY + 4 * 0x000110af, 0x00000027);
    write32(DDR_PHY + 4 * 0x000120af, 0x00000027);
    write32(DDR_PHY + 4 * 0x000130af, 0x00000027);
    write32(DDR_PHY + 4 * 0x000100aa, 0x00000501);
    write32(DDR_PHY + 4 * 0x000110aa, 0x0000050d);
    write32(DDR_PHY + 4 * 0x000120aa, 0x00000501);
    write32(DDR_PHY + 4 * 0x000130aa, 0x0000050d);
    write32(DDR_PHY + 4 * 0x00020077, 0x00000034);
    write32(DDR_PHY + 4 * 0x0002007c, 0x00000054);
    write32(DDR_PHY + 4 * 0x0002007d, 0x000004b2);
    write32(DDR_PHY + 4 * 0x000400c0, 0x0000010f);
    write32(DDR_PHY + 4 * 0x000200cb, 0x000061f0);
    write32(DDR_PHY + 4 * 0x00020060, 0x00000002);

    // swap

    write32(DDR_PHY + 4 * 0x100a0, 0x4); //lndq =1
    write32(DDR_PHY + 4 * 0x100a1, 0x5); //CA1 =0
    write32(DDR_PHY + 4 * 0x100a2, 0x7); //CA1 =0
    write32(DDR_PHY + 4 * 0x100a3, 0x6); //CA1 =0
    write32(DDR_PHY + 4 * 0x100a4, 0x0); //CA1 =0
    write32(DDR_PHY + 4 * 0x100a5, 0x2); //CA1 =0
    write32(DDR_PHY + 4 * 0x100a6, 0x3); //CA1 =0
    write32(DDR_PHY + 4 * 0x100a7, 0x1); //CA1 =0

    write32(DDR_PHY + 4 * 0x110a0, 0x0); //lndq =1
    write32(DDR_PHY + 4 * 0x110a1, 0x1); //CA1 =0
    write32(DDR_PHY + 4 * 0x110a2, 0x3); //CA1 =0
    write32(DDR_PHY + 4 * 0x110a3, 0x2); //CA1 =0
    write32(DDR_PHY + 4 * 0x110a4, 0x4); //CA1 =0
    write32(DDR_PHY + 4 * 0x110a5, 0x7); //CA1 =0
    write32(DDR_PHY + 4 * 0x110a6, 0x6); //CA1 =0
    write32(DDR_PHY + 4 * 0x110a7, 0x5); //CA1 =0

    write32(DDR_PHY + 4 * 0xd0000, 0x0);

    let base = DDR_PHY + 0x50000 * 4;
    for (i, v) in crate::ddr_data::G_DDR_INST.iter().enumerate() {
        write32(base + i * 4, *v as u32);
    }

    write32(DDR_PHY + 4 * 0xd0000, 0x1);
    write32(DDR_PHY + 4 * 0xd0000, 0x0);

    let base = DDR_PHY + 0x54000 * 4;
    for (i, v) in crate::ddr_data::G_DDR_DATA.iter().enumerate() {
        write32(base + i * 4, *v as u32);
    }
    write32(DDR_PHY + 4 * 0xd0000, 0x1);

    write32(DDR_PHY + 4 * 0x000d0000, 0x00000001);
    write32(DDR_PHY + 4 * 0x000d0099, 0x00000009);
    write32(DDR_PHY + 4 * 0x000d0099, 0x00000001);
    write32(DDR_PHY + 4 * 0x000d0099, 0x00000000);

    let mut train_data = 0;
    while (train_data & 0x7) != 0x07 {
        while read32(DDR_PHY + 4 * 0x000d0004) & 0x1 != 0x0 {}

        train_data = read32(DDR_PHY + 4 * 0x000d0032);
        train_message(train_data);
        if DEBUG && train_data == 0x07 {
            debug_ca_dq_dbytes();
        }

        write32(DDR_PHY + 4 * 0x000d0031, 0x00000000);
        while (read32(DDR_PHY + 4 * 0x000d0004) & 0x1 == 0x0) {}
        write32(DDR_PHY + 4 * 0x000d0031, 0x00000001);
    }

    write32(DDR_PHY + 4 * 0x000d0099, 0x00000001);
    write32(DDR_PHY + 4 * 0x000d0000, 0x00000000);
    // write32(DDR_PHY + 4 * 0x000d0000, 0x00000001);

    // lp4_ddr2667_2d();

    write32(DDR_PHY + 4 * 0x000d0000, 0x00000000);

    write32(DDR_PHY + 4 * 0x00090000, 0x00000010);
    write32(DDR_PHY + 4 * 0x00090001, 0x00000400);
    write32(DDR_PHY + 4 * 0x00090002, 0x0000010e);
    write32(DDR_PHY + 4 * 0x00090003, 0x00000000);
    write32(DDR_PHY + 4 * 0x00090004, 0x00000000);
    write32(DDR_PHY + 4 * 0x00090005, 0x00000008);
    write32(DDR_PHY + 4 * 0x00090029, 0x0000000b);
    write32(DDR_PHY + 4 * 0x0009002a, 0x00000480);
    write32(DDR_PHY + 4 * 0x0009002b, 0x00000109);
    write32(DDR_PHY + 4 * 0x0009002c, 0x00000008);
    write32(DDR_PHY + 4 * 0x0009002d, 0x00000448);
    write32(DDR_PHY + 4 * 0x0009002e, 0x00000139);
    write32(DDR_PHY + 4 * 0x0009002f, 0x00000008);
    write32(DDR_PHY + 4 * 0x00090030, 0x00000478);
    write32(DDR_PHY + 4 * 0x00090031, 0x00000109);
    write32(DDR_PHY + 4 * 0x00090032, 0x00000000);
    write32(DDR_PHY + 4 * 0x00090033, 0x000000e8);
    write32(DDR_PHY + 4 * 0x00090034, 0x00000109);
    write32(DDR_PHY + 4 * 0x00090035, 0x00000002);
    write32(DDR_PHY + 4 * 0x00090036, 0x00000010);
    write32(DDR_PHY + 4 * 0x00090037, 0x00000139);
    write32(DDR_PHY + 4 * 0x00090038, 0x0000000b);
    write32(DDR_PHY + 4 * 0x00090039, 0x000007c0);
    write32(DDR_PHY + 4 * 0x0009003a, 0x00000139);
    write32(DDR_PHY + 4 * 0x0009003b, 0x00000044);
    write32(DDR_PHY + 4 * 0x0009003c, 0x00000633);
    write32(DDR_PHY + 4 * 0x0009003d, 0x00000159);
    write32(DDR_PHY + 4 * 0x0009003e, 0x0000014f);
    write32(DDR_PHY + 4 * 0x0009003f, 0x00000630);
    write32(DDR_PHY + 4 * 0x00090040, 0x00000159);
    write32(DDR_PHY + 4 * 0x00090041, 0x00000047);
    write32(DDR_PHY + 4 * 0x00090042, 0x00000633);
    write32(DDR_PHY + 4 * 0x00090043, 0x00000149);
    write32(DDR_PHY + 4 * 0x00090044, 0x0000004f);
    write32(DDR_PHY + 4 * 0x00090045, 0x00000633);
    write32(DDR_PHY + 4 * 0x00090046, 0x00000179);
    write32(DDR_PHY + 4 * 0x00090047, 0x00000008);
    write32(DDR_PHY + 4 * 0x00090048, 0x000000e0);
    write32(DDR_PHY + 4 * 0x00090049, 0x00000109);
    write32(DDR_PHY + 4 * 0x0009004a, 0x00000000);
    write32(DDR_PHY + 4 * 0x0009004b, 0x000007c8);
    write32(DDR_PHY + 4 * 0x0009004c, 0x00000109);
    write32(DDR_PHY + 4 * 0x0009004d, 0x00000000);
    write32(DDR_PHY + 4 * 0x0009004e, 0x00000001);
    write32(DDR_PHY + 4 * 0x0009004f, 0x00000008);
    write32(DDR_PHY + 4 * 0x00090050, 0x00000000);
    write32(DDR_PHY + 4 * 0x00090051, 0x0000045a);
    write32(DDR_PHY + 4 * 0x00090052, 0x00000009);
    write32(DDR_PHY + 4 * 0x00090053, 0x00000000);
    write32(DDR_PHY + 4 * 0x00090054, 0x00000448);
    write32(DDR_PHY + 4 * 0x00090055, 0x00000109);
    write32(DDR_PHY + 4 * 0x00090056, 0x00000040);
    write32(DDR_PHY + 4 * 0x00090057, 0x00000633);
    write32(DDR_PHY + 4 * 0x00090058, 0x00000179);
    write32(DDR_PHY + 4 * 0x00090059, 0x00000001);
    write32(DDR_PHY + 4 * 0x0009005a, 0x00000618);
    write32(DDR_PHY + 4 * 0x0009005b, 0x00000109);
    write32(DDR_PHY + 4 * 0x0009005c, 0x000040c0);
    write32(DDR_PHY + 4 * 0x0009005d, 0x00000633);
    write32(DDR_PHY + 4 * 0x0009005e, 0x00000149);
    write32(DDR_PHY + 4 * 0x0009005f, 0x00000008);
    write32(DDR_PHY + 4 * 0x00090060, 0x00000004);
    write32(DDR_PHY + 4 * 0x00090061, 0x00000048);
    write32(DDR_PHY + 4 * 0x00090062, 0x00004040);
    write32(DDR_PHY + 4 * 0x00090063, 0x00000633);
    write32(DDR_PHY + 4 * 0x00090064, 0x00000149);
    write32(DDR_PHY + 4 * 0x00090065, 0x00000000);
    write32(DDR_PHY + 4 * 0x00090066, 0x00000004);
    write32(DDR_PHY + 4 * 0x00090067, 0x00000048);
    write32(DDR_PHY + 4 * 0x00090068, 0x00000040);
    write32(DDR_PHY + 4 * 0x00090069, 0x00000633);
    write32(DDR_PHY + 4 * 0x0009006a, 0x00000149);
    write32(DDR_PHY + 4 * 0x0009006b, 0x00000010);
    write32(DDR_PHY + 4 * 0x0009006c, 0x00000004);
    write32(DDR_PHY + 4 * 0x0009006d, 0x00000018);
    write32(DDR_PHY + 4 * 0x0009006e, 0x00000000);
    write32(DDR_PHY + 4 * 0x0009006f, 0x00000004);
    write32(DDR_PHY + 4 * 0x00090070, 0x00000078);
    write32(DDR_PHY + 4 * 0x00090071, 0x00000549);
    write32(DDR_PHY + 4 * 0x00090072, 0x00000633);
    write32(DDR_PHY + 4 * 0x00090073, 0x00000159);
    write32(DDR_PHY + 4 * 0x00090074, 0x00000d49);
    write32(DDR_PHY + 4 * 0x00090075, 0x00000633);
    write32(DDR_PHY + 4 * 0x00090076, 0x00000159);
    write32(DDR_PHY + 4 * 0x00090077, 0x0000094a);
    write32(DDR_PHY + 4 * 0x00090078, 0x00000633);
    write32(DDR_PHY + 4 * 0x00090079, 0x00000159);
    write32(DDR_PHY + 4 * 0x0009007a, 0x00000441);
    write32(DDR_PHY + 4 * 0x0009007b, 0x00000633);
    write32(DDR_PHY + 4 * 0x0009007c, 0x00000149);
    write32(DDR_PHY + 4 * 0x0009007d, 0x00000042);
    write32(DDR_PHY + 4 * 0x0009007e, 0x00000633);
    write32(DDR_PHY + 4 * 0x0009007f, 0x00000149);
    write32(DDR_PHY + 4 * 0x00090080, 0x00000001);
    write32(DDR_PHY + 4 * 0x00090081, 0x00000633);
    write32(DDR_PHY + 4 * 0x00090082, 0x00000149);
    write32(DDR_PHY + 4 * 0x00090083, 0x00000000);
    write32(DDR_PHY + 4 * 0x00090084, 0x000000e0);
    write32(DDR_PHY + 4 * 0x00090085, 0x00000109);
    write32(DDR_PHY + 4 * 0x00090086, 0x0000000a);
    write32(DDR_PHY + 4 * 0x00090087, 0x00000010);
    write32(DDR_PHY + 4 * 0x00090088, 0x00000109);
    write32(DDR_PHY + 4 * 0x00090089, 0x00000009);
    write32(DDR_PHY + 4 * 0x0009008a, 0x000003c0);
    write32(DDR_PHY + 4 * 0x0009008b, 0x00000149);
    write32(DDR_PHY + 4 * 0x0009008c, 0x00000009);
    write32(DDR_PHY + 4 * 0x0009008d, 0x000003c0);
    write32(DDR_PHY + 4 * 0x0009008e, 0x00000159);
    write32(DDR_PHY + 4 * 0x0009008f, 0x00000018);
    write32(DDR_PHY + 4 * 0x00090090, 0x00000010);
    write32(DDR_PHY + 4 * 0x00090091, 0x00000109);
    write32(DDR_PHY + 4 * 0x00090092, 0x00000000);
    write32(DDR_PHY + 4 * 0x00090093, 0x000003c0);
    write32(DDR_PHY + 4 * 0x00090094, 0x00000109);
    write32(DDR_PHY + 4 * 0x00090095, 0x00000018);
    write32(DDR_PHY + 4 * 0x00090096, 0x00000004);
    write32(DDR_PHY + 4 * 0x00090097, 0x00000048);
    write32(DDR_PHY + 4 * 0x00090098, 0x00000018);
    write32(DDR_PHY + 4 * 0x00090099, 0x00000004);
    write32(DDR_PHY + 4 * 0x0009009a, 0x00000058);
    write32(DDR_PHY + 4 * 0x0009009b, 0x0000000b);
    write32(DDR_PHY + 4 * 0x0009009c, 0x00000010);
    write32(DDR_PHY + 4 * 0x0009009d, 0x00000109);
    write32(DDR_PHY + 4 * 0x0009009e, 0x00000001);
    write32(DDR_PHY + 4 * 0x0009009f, 0x00000010);
    write32(DDR_PHY + 4 * 0x000900a0, 0x00000109);
    write32(DDR_PHY + 4 * 0x000900a1, 0x00000005);
    write32(DDR_PHY + 4 * 0x000900a2, 0x000007c0);
    write32(DDR_PHY + 4 * 0x000900a3, 0x00000109);
    write32(DDR_PHY + 4 * 0x00040000, 0x00000811);
    write32(DDR_PHY + 4 * 0x00040020, 0x00000880);
    write32(DDR_PHY + 4 * 0x00040040, 0x00000000);
    write32(DDR_PHY + 4 * 0x00040060, 0x00000000);
    write32(DDR_PHY + 4 * 0x00040001, 0x00004008);
    write32(DDR_PHY + 4 * 0x00040021, 0x00000083);
    write32(DDR_PHY + 4 * 0x00040041, 0x0000004f);
    write32(DDR_PHY + 4 * 0x00040061, 0x00000000);
    write32(DDR_PHY + 4 * 0x00040002, 0x00004040);
    write32(DDR_PHY + 4 * 0x00040022, 0x00000083);
    write32(DDR_PHY + 4 * 0x00040042, 0x00000051);
    write32(DDR_PHY + 4 * 0x00040062, 0x00000000);
    write32(DDR_PHY + 4 * 0x00040003, 0x00000811);
    write32(DDR_PHY + 4 * 0x00040023, 0x00000880);
    write32(DDR_PHY + 4 * 0x00040043, 0x00000000);
    write32(DDR_PHY + 4 * 0x00040063, 0x00000000);
    write32(DDR_PHY + 4 * 0x00040004, 0x00000720);
    write32(DDR_PHY + 4 * 0x00040024, 0x0000000f);
    write32(DDR_PHY + 4 * 0x00040044, 0x00001740);
    write32(DDR_PHY + 4 * 0x00040064, 0x00000000);
    write32(DDR_PHY + 4 * 0x00040005, 0x00000016);
    write32(DDR_PHY + 4 * 0x00040025, 0x00000083);
    write32(DDR_PHY + 4 * 0x00040045, 0x0000004b);
    write32(DDR_PHY + 4 * 0x00040065, 0x00000000);
    write32(DDR_PHY + 4 * 0x00040006, 0x00000716);
    write32(DDR_PHY + 4 * 0x00040026, 0x0000000f);
    write32(DDR_PHY + 4 * 0x00040046, 0x00002001);
    write32(DDR_PHY + 4 * 0x00040066, 0x00000000);
    write32(DDR_PHY + 4 * 0x00040007, 0x00000716);
    write32(DDR_PHY + 4 * 0x00040027, 0x0000000f);
    write32(DDR_PHY + 4 * 0x00040047, 0x00002800);
    write32(DDR_PHY + 4 * 0x00040067, 0x00000000);
    write32(DDR_PHY + 4 * 0x00040008, 0x00000716);
    write32(DDR_PHY + 4 * 0x00040028, 0x0000000f);
    write32(DDR_PHY + 4 * 0x00040048, 0x00000f00);
    write32(DDR_PHY + 4 * 0x00040068, 0x00000000);
    write32(DDR_PHY + 4 * 0x00040009, 0x00000720);
    write32(DDR_PHY + 4 * 0x00040029, 0x0000000f);
    write32(DDR_PHY + 4 * 0x00040049, 0x00001400);
    write32(DDR_PHY + 4 * 0x00040069, 0x00000000);
    write32(DDR_PHY + 4 * 0x0004000a, 0x00000e08);
    write32(DDR_PHY + 4 * 0x0004002a, 0x00000c15);
    write32(DDR_PHY + 4 * 0x0004004a, 0x00000000);
    write32(DDR_PHY + 4 * 0x0004006a, 0x00000000);
    write32(DDR_PHY + 4 * 0x0004000b, 0x00000625);
    write32(DDR_PHY + 4 * 0x0004002b, 0x00000015);
    write32(DDR_PHY + 4 * 0x0004004b, 0x00000000);
    write32(DDR_PHY + 4 * 0x0004006b, 0x00000000);
    write32(DDR_PHY + 4 * 0x0004000c, 0x00004028);
    write32(DDR_PHY + 4 * 0x0004002c, 0x00000080);
    write32(DDR_PHY + 4 * 0x0004004c, 0x00000000);
    write32(DDR_PHY + 4 * 0x0004006c, 0x00000000);
    write32(DDR_PHY + 4 * 0x0004000d, 0x00000e08);
    write32(DDR_PHY + 4 * 0x0004002d, 0x00000c1a);
    write32(DDR_PHY + 4 * 0x0004004d, 0x00000000);
    write32(DDR_PHY + 4 * 0x0004006d, 0x00000000);
    write32(DDR_PHY + 4 * 0x0004000e, 0x00000625);
    write32(DDR_PHY + 4 * 0x0004002e, 0x0000001a);
    write32(DDR_PHY + 4 * 0x0004004e, 0x00000000);
    write32(DDR_PHY + 4 * 0x0004006e, 0x00000000);
    write32(DDR_PHY + 4 * 0x0004000f, 0x00004040);
    write32(DDR_PHY + 4 * 0x0004002f, 0x00000080);
    write32(DDR_PHY + 4 * 0x0004004f, 0x00000000);
    write32(DDR_PHY + 4 * 0x0004006f, 0x00000000);
    write32(DDR_PHY + 4 * 0x00040010, 0x00002604);
    write32(DDR_PHY + 4 * 0x00040030, 0x00000015);
    write32(DDR_PHY + 4 * 0x00040050, 0x00000000);
    write32(DDR_PHY + 4 * 0x00040070, 0x00000000);
    write32(DDR_PHY + 4 * 0x00040011, 0x00000708);
    write32(DDR_PHY + 4 * 0x00040031, 0x00000005);
    write32(DDR_PHY + 4 * 0x00040051, 0x00000000);
    write32(DDR_PHY + 4 * 0x00040071, 0x00002002);
    write32(DDR_PHY + 4 * 0x00040012, 0x00000008);
    write32(DDR_PHY + 4 * 0x00040032, 0x00000080);
    write32(DDR_PHY + 4 * 0x00040052, 0x00000000);
    write32(DDR_PHY + 4 * 0x00040072, 0x00000000);
    write32(DDR_PHY + 4 * 0x00040013, 0x00002604);
    write32(DDR_PHY + 4 * 0x00040033, 0x0000001a);
    write32(DDR_PHY + 4 * 0x00040053, 0x00000000);
    write32(DDR_PHY + 4 * 0x00040073, 0x00000000);
    write32(DDR_PHY + 4 * 0x00040014, 0x00000708);
    write32(DDR_PHY + 4 * 0x00040034, 0x0000000a);
    write32(DDR_PHY + 4 * 0x00040054, 0x00000000);
    write32(DDR_PHY + 4 * 0x00040074, 0x00002002);
    write32(DDR_PHY + 4 * 0x00040015, 0x00004040);
    write32(DDR_PHY + 4 * 0x00040035, 0x00000080);
    write32(DDR_PHY + 4 * 0x00040055, 0x00000000);
    write32(DDR_PHY + 4 * 0x00040075, 0x00000000);
    write32(DDR_PHY + 4 * 0x00040016, 0x0000060a);
    write32(DDR_PHY + 4 * 0x00040036, 0x00000015);
    write32(DDR_PHY + 4 * 0x00040056, 0x00001200);
    write32(DDR_PHY + 4 * 0x00040076, 0x00000000);
    write32(DDR_PHY + 4 * 0x00040017, 0x0000061a);
    write32(DDR_PHY + 4 * 0x00040037, 0x00000015);
    write32(DDR_PHY + 4 * 0x00040057, 0x00001300);
    write32(DDR_PHY + 4 * 0x00040077, 0x00000000);
    write32(DDR_PHY + 4 * 0x00040018, 0x0000060a);
    write32(DDR_PHY + 4 * 0x00040038, 0x0000001a);
    write32(DDR_PHY + 4 * 0x00040058, 0x00001200);
    write32(DDR_PHY + 4 * 0x00040078, 0x00000000);
    write32(DDR_PHY + 4 * 0x00040019, 0x00000642);
    write32(DDR_PHY + 4 * 0x00040039, 0x0000001a);
    write32(DDR_PHY + 4 * 0x00040059, 0x00001300);
    write32(DDR_PHY + 4 * 0x00040079, 0x00000000);
    write32(DDR_PHY + 4 * 0x0004001a, 0x00004808);
    write32(DDR_PHY + 4 * 0x0004003a, 0x00000880);
    write32(DDR_PHY + 4 * 0x0004005a, 0x00000000);
    write32(DDR_PHY + 4 * 0x0004007a, 0x00000000);
    write32(DDR_PHY + 4 * 0x000900a4, 0x00000000);
    write32(DDR_PHY + 4 * 0x000900a5, 0x00000790);
    write32(DDR_PHY + 4 * 0x000900a6, 0x0000011a);
    write32(DDR_PHY + 4 * 0x000900a7, 0x00000008);
    write32(DDR_PHY + 4 * 0x000900a8, 0x000007aa);
    write32(DDR_PHY + 4 * 0x000900a9, 0x0000002a);
    write32(DDR_PHY + 4 * 0x000900aa, 0x00000010);
    write32(DDR_PHY + 4 * 0x000900ab, 0x000007b2);
    write32(DDR_PHY + 4 * 0x000900ac, 0x0000002a);
    write32(DDR_PHY + 4 * 0x000900ad, 0x00000000);
    write32(DDR_PHY + 4 * 0x000900ae, 0x000007c8);
    write32(DDR_PHY + 4 * 0x000900af, 0x00000109);
    write32(DDR_PHY + 4 * 0x000900b0, 0x00000010);
    write32(DDR_PHY + 4 * 0x000900b1, 0x00000010);
    write32(DDR_PHY + 4 * 0x000900b2, 0x00000109);
    write32(DDR_PHY + 4 * 0x000900b3, 0x00000010);
    write32(DDR_PHY + 4 * 0x000900b4, 0x000002a8);
    write32(DDR_PHY + 4 * 0x000900b5, 0x00000129);
    write32(DDR_PHY + 4 * 0x000900b6, 0x00000008);
    write32(DDR_PHY + 4 * 0x000900b7, 0x00000370);
    write32(DDR_PHY + 4 * 0x000900b8, 0x00000129);
    write32(DDR_PHY + 4 * 0x000900b9, 0x0000000a);
    write32(DDR_PHY + 4 * 0x000900ba, 0x000003c8);
    write32(DDR_PHY + 4 * 0x000900bb, 0x000001a9);
    write32(DDR_PHY + 4 * 0x000900bc, 0x0000000c);
    write32(DDR_PHY + 4 * 0x000900bd, 0x00000408);
    write32(DDR_PHY + 4 * 0x000900be, 0x00000199);
    write32(DDR_PHY + 4 * 0x000900bf, 0x00000014);
    write32(DDR_PHY + 4 * 0x000900c0, 0x00000790);
    write32(DDR_PHY + 4 * 0x000900c1, 0x0000011a);
    write32(DDR_PHY + 4 * 0x000900c2, 0x00000008);
    write32(DDR_PHY + 4 * 0x000900c3, 0x00000004);
    write32(DDR_PHY + 4 * 0x000900c4, 0x00000018);
    write32(DDR_PHY + 4 * 0x000900c5, 0x0000000e);
    write32(DDR_PHY + 4 * 0x000900c6, 0x00000408);
    write32(DDR_PHY + 4 * 0x000900c7, 0x00000199);
    write32(DDR_PHY + 4 * 0x000900c8, 0x00000008);
    write32(DDR_PHY + 4 * 0x000900c9, 0x00008568);
    write32(DDR_PHY + 4 * 0x000900ca, 0x00000108);
    write32(DDR_PHY + 4 * 0x000900cb, 0x00000018);
    write32(DDR_PHY + 4 * 0x000900cc, 0x00000790);
    write32(DDR_PHY + 4 * 0x000900cd, 0x0000016a);
    write32(DDR_PHY + 4 * 0x000900ce, 0x00000008);
    write32(DDR_PHY + 4 * 0x000900cf, 0x000001d8);
    write32(DDR_PHY + 4 * 0x000900d0, 0x00000169);
    write32(DDR_PHY + 4 * 0x000900d1, 0x00000010);
    write32(DDR_PHY + 4 * 0x000900d2, 0x00008558);
    write32(DDR_PHY + 4 * 0x000900d3, 0x00000168);
    write32(DDR_PHY + 4 * 0x000900d4, 0x00000070);
    write32(DDR_PHY + 4 * 0x000900d5, 0x00000788);
    write32(DDR_PHY + 4 * 0x000900d6, 0x0000016a);
    write32(DDR_PHY + 4 * 0x000900d7, 0x00001ff8);
    write32(DDR_PHY + 4 * 0x000900d8, 0x000085a8);
    write32(DDR_PHY + 4 * 0x000900d9, 0x000001e8);
    write32(DDR_PHY + 4 * 0x000900da, 0x00000050);
    write32(DDR_PHY + 4 * 0x000900db, 0x00000798);
    write32(DDR_PHY + 4 * 0x000900dc, 0x0000016a);
    write32(DDR_PHY + 4 * 0x000900dd, 0x00000060);
    write32(DDR_PHY + 4 * 0x000900de, 0x000007a0);
    write32(DDR_PHY + 4 * 0x000900df, 0x0000016a);
    write32(DDR_PHY + 4 * 0x000900e0, 0x00000008);
    write32(DDR_PHY + 4 * 0x000900e1, 0x00008310);
    write32(DDR_PHY + 4 * 0x000900e2, 0x00000168);
    write32(DDR_PHY + 4 * 0x000900e3, 0x00000008);
    write32(DDR_PHY + 4 * 0x000900e4, 0x0000a310);
    write32(DDR_PHY + 4 * 0x000900e5, 0x00000168);
    write32(DDR_PHY + 4 * 0x000900e6, 0x0000000a);
    write32(DDR_PHY + 4 * 0x000900e7, 0x00000408);
    write32(DDR_PHY + 4 * 0x000900e8, 0x00000169);
    write32(DDR_PHY + 4 * 0x000900e9, 0x0000006e);
    write32(DDR_PHY + 4 * 0x000900ea, 0x00000000);
    write32(DDR_PHY + 4 * 0x000900eb, 0x00000068);
    write32(DDR_PHY + 4 * 0x000900ec, 0x00000000);
    write32(DDR_PHY + 4 * 0x000900ed, 0x00000408);
    write32(DDR_PHY + 4 * 0x000900ee, 0x00000169);
    write32(DDR_PHY + 4 * 0x000900ef, 0x00000000);
    write32(DDR_PHY + 4 * 0x000900f0, 0x00008310);
    write32(DDR_PHY + 4 * 0x000900f1, 0x00000168);
    write32(DDR_PHY + 4 * 0x000900f2, 0x00000000);
    write32(DDR_PHY + 4 * 0x000900f3, 0x0000a310);
    write32(DDR_PHY + 4 * 0x000900f4, 0x00000168);
    write32(DDR_PHY + 4 * 0x000900f5, 0x00001ff8);
    write32(DDR_PHY + 4 * 0x000900f6, 0x000085a8);
    write32(DDR_PHY + 4 * 0x000900f7, 0x000001e8);
    write32(DDR_PHY + 4 * 0x000900f8, 0x00000068);
    write32(DDR_PHY + 4 * 0x000900f9, 0x00000798);
    write32(DDR_PHY + 4 * 0x000900fa, 0x0000016a);
    write32(DDR_PHY + 4 * 0x000900fb, 0x00000078);
    write32(DDR_PHY + 4 * 0x000900fc, 0x000007a0);
    write32(DDR_PHY + 4 * 0x000900fd, 0x0000016a);
    write32(DDR_PHY + 4 * 0x000900fe, 0x00000068);
    write32(DDR_PHY + 4 * 0x000900ff, 0x00000790);
    write32(DDR_PHY + 4 * 0x00090100, 0x0000016a);
    write32(DDR_PHY + 4 * 0x00090101, 0x00000008);
    write32(DDR_PHY + 4 * 0x00090102, 0x00008b10);
    write32(DDR_PHY + 4 * 0x00090103, 0x00000168);
    write32(DDR_PHY + 4 * 0x00090104, 0x00000008);
    write32(DDR_PHY + 4 * 0x00090105, 0x0000ab10);
    write32(DDR_PHY + 4 * 0x00090106, 0x00000168);
    write32(DDR_PHY + 4 * 0x00090107, 0x0000000a);
    write32(DDR_PHY + 4 * 0x00090108, 0x00000408);
    write32(DDR_PHY + 4 * 0x00090109, 0x00000169);
    write32(DDR_PHY + 4 * 0x0009010a, 0x00000058);
    write32(DDR_PHY + 4 * 0x0009010b, 0x00000000);
    write32(DDR_PHY + 4 * 0x0009010c, 0x00000068);
    write32(DDR_PHY + 4 * 0x0009010d, 0x00000000);
    write32(DDR_PHY + 4 * 0x0009010e, 0x00000408);
    write32(DDR_PHY + 4 * 0x0009010f, 0x00000169);
    write32(DDR_PHY + 4 * 0x00090110, 0x00000000);
    write32(DDR_PHY + 4 * 0x00090111, 0x00008b10);
    write32(DDR_PHY + 4 * 0x00090112, 0x00000168);
    write32(DDR_PHY + 4 * 0x00090113, 0x00000001);
    write32(DDR_PHY + 4 * 0x00090114, 0x0000ab10);
    write32(DDR_PHY + 4 * 0x00090115, 0x00000168);
    write32(DDR_PHY + 4 * 0x00090116, 0x00000000);
    write32(DDR_PHY + 4 * 0x00090117, 0x000001d8);
    write32(DDR_PHY + 4 * 0x00090118, 0x00000169);
    write32(DDR_PHY + 4 * 0x00090119, 0x00000080);
    write32(DDR_PHY + 4 * 0x0009011a, 0x00000790);
    write32(DDR_PHY + 4 * 0x0009011b, 0x0000016a);
    write32(DDR_PHY + 4 * 0x0009011c, 0x00000018);
    write32(DDR_PHY + 4 * 0x0009011d, 0x000007aa);
    write32(DDR_PHY + 4 * 0x0009011e, 0x0000006a);
    write32(DDR_PHY + 4 * 0x0009011f, 0x0000000a);
    write32(DDR_PHY + 4 * 0x00090120, 0x00000000);
    write32(DDR_PHY + 4 * 0x00090121, 0x000001e9);
    write32(DDR_PHY + 4 * 0x00090122, 0x00000008);
    write32(DDR_PHY + 4 * 0x00090123, 0x00008080);
    write32(DDR_PHY + 4 * 0x00090124, 0x00000108);
    write32(DDR_PHY + 4 * 0x00090125, 0x0000000f);
    write32(DDR_PHY + 4 * 0x00090126, 0x00000408);
    write32(DDR_PHY + 4 * 0x00090127, 0x00000169);
    write32(DDR_PHY + 4 * 0x00090128, 0x0000000c);
    write32(DDR_PHY + 4 * 0x00090129, 0x00000000);
    write32(DDR_PHY + 4 * 0x0009012a, 0x00000068);
    write32(DDR_PHY + 4 * 0x0009012b, 0x00000009);
    write32(DDR_PHY + 4 * 0x0009012c, 0x00000000);
    write32(DDR_PHY + 4 * 0x0009012d, 0x000001a9);
    write32(DDR_PHY + 4 * 0x0009012e, 0x00000000);
    write32(DDR_PHY + 4 * 0x0009012f, 0x00000408);
    write32(DDR_PHY + 4 * 0x00090130, 0x00000169);
    write32(DDR_PHY + 4 * 0x00090131, 0x00000000);
    write32(DDR_PHY + 4 * 0x00090132, 0x00008080);
    write32(DDR_PHY + 4 * 0x00090133, 0x00000108);
    write32(DDR_PHY + 4 * 0x00090134, 0x00000008);
    write32(DDR_PHY + 4 * 0x00090135, 0x000007aa);
    write32(DDR_PHY + 4 * 0x00090136, 0x0000006a);
    write32(DDR_PHY + 4 * 0x00090137, 0x00000000);
    write32(DDR_PHY + 4 * 0x00090138, 0x00008568);
    write32(DDR_PHY + 4 * 0x00090139, 0x00000108);
    write32(DDR_PHY + 4 * 0x0009013a, 0x000000b7);
    write32(DDR_PHY + 4 * 0x0009013b, 0x00000790);
    write32(DDR_PHY + 4 * 0x0009013c, 0x0000016a);
    write32(DDR_PHY + 4 * 0x0009013d, 0x0000001f);
    write32(DDR_PHY + 4 * 0x0009013e, 0x00000000);
    write32(DDR_PHY + 4 * 0x0009013f, 0x00000068);
    write32(DDR_PHY + 4 * 0x00090140, 0x00000008);
    write32(DDR_PHY + 4 * 0x00090141, 0x00008558);
    write32(DDR_PHY + 4 * 0x00090142, 0x00000168);
    write32(DDR_PHY + 4 * 0x00090143, 0x0000000f);
    write32(DDR_PHY + 4 * 0x00090144, 0x00000408);
    write32(DDR_PHY + 4 * 0x00090145, 0x00000169);
    write32(DDR_PHY + 4 * 0x00090146, 0x0000000d);
    write32(DDR_PHY + 4 * 0x00090147, 0x00000000);
    write32(DDR_PHY + 4 * 0x00090148, 0x00000068);
    write32(DDR_PHY + 4 * 0x00090149, 0x00000000);
    write32(DDR_PHY + 4 * 0x0009014a, 0x00000408);
    write32(DDR_PHY + 4 * 0x0009014b, 0x00000169);
    write32(DDR_PHY + 4 * 0x0009014c, 0x00000000);
    write32(DDR_PHY + 4 * 0x0009014d, 0x00008558);
    write32(DDR_PHY + 4 * 0x0009014e, 0x00000168);
    write32(DDR_PHY + 4 * 0x0009014f, 0x00000008);
    write32(DDR_PHY + 4 * 0x00090150, 0x000003c8);
    write32(DDR_PHY + 4 * 0x00090151, 0x000001a9);
    write32(DDR_PHY + 4 * 0x00090152, 0x00000003);
    write32(DDR_PHY + 4 * 0x00090153, 0x00000370);
    write32(DDR_PHY + 4 * 0x00090154, 0x00000129);
    write32(DDR_PHY + 4 * 0x00090155, 0x00000020);
    write32(DDR_PHY + 4 * 0x00090156, 0x000002aa);
    write32(DDR_PHY + 4 * 0x00090157, 0x00000009);
    write32(DDR_PHY + 4 * 0x00090158, 0x00000008);
    write32(DDR_PHY + 4 * 0x00090159, 0x000000e8);
    write32(DDR_PHY + 4 * 0x0009015a, 0x00000109);
    write32(DDR_PHY + 4 * 0x0009015b, 0x00000000);
    write32(DDR_PHY + 4 * 0x0009015c, 0x00008140);
    write32(DDR_PHY + 4 * 0x0009015d, 0x0000010c);
    write32(DDR_PHY + 4 * 0x0009015e, 0x00000010);
    write32(DDR_PHY + 4 * 0x0009015f, 0x00008138);
    write32(DDR_PHY + 4 * 0x00090160, 0x00000104);
    write32(DDR_PHY + 4 * 0x00090161, 0x00000008);
    write32(DDR_PHY + 4 * 0x00090162, 0x00000448);
    write32(DDR_PHY + 4 * 0x00090163, 0x00000109);
    write32(DDR_PHY + 4 * 0x00090164, 0x0000000f);
    write32(DDR_PHY + 4 * 0x00090165, 0x000007c0);
    write32(DDR_PHY + 4 * 0x00090166, 0x00000109);
    write32(DDR_PHY + 4 * 0x00090167, 0x00000000);
    write32(DDR_PHY + 4 * 0x00090168, 0x000000e8);
    write32(DDR_PHY + 4 * 0x00090169, 0x00000109);
    write32(DDR_PHY + 4 * 0x0009016a, 0x00000047);
    write32(DDR_PHY + 4 * 0x0009016b, 0x00000630);
    write32(DDR_PHY + 4 * 0x0009016c, 0x00000109);
    write32(DDR_PHY + 4 * 0x0009016d, 0x00000008);
    write32(DDR_PHY + 4 * 0x0009016e, 0x00000618);
    write32(DDR_PHY + 4 * 0x0009016f, 0x00000109);
    write32(DDR_PHY + 4 * 0x00090170, 0x00000008);
    write32(DDR_PHY + 4 * 0x00090171, 0x000000e0);
    write32(DDR_PHY + 4 * 0x00090172, 0x00000109);
    write32(DDR_PHY + 4 * 0x00090173, 0x00000000);
    write32(DDR_PHY + 4 * 0x00090174, 0x000007c8);
    write32(DDR_PHY + 4 * 0x00090175, 0x00000109);
    write32(DDR_PHY + 4 * 0x00090176, 0x00000008);
    write32(DDR_PHY + 4 * 0x00090177, 0x00008140);
    write32(DDR_PHY + 4 * 0x00090178, 0x0000010c);
    write32(DDR_PHY + 4 * 0x00090179, 0x00000000);
    write32(DDR_PHY + 4 * 0x0009017a, 0x00000478);
    write32(DDR_PHY + 4 * 0x0009017b, 0x00000109);
    write32(DDR_PHY + 4 * 0x0009017c, 0x00000000);
    write32(DDR_PHY + 4 * 0x0009017d, 0x00000001);
    write32(DDR_PHY + 4 * 0x0009017e, 0x00000008);
    write32(DDR_PHY + 4 * 0x0009017f, 0x00000008);
    write32(DDR_PHY + 4 * 0x00090180, 0x00000004);
    write32(DDR_PHY + 4 * 0x00090181, 0x00000000);
    write32(DDR_PHY + 4 * 0x00090006, 0x00000008);
    write32(DDR_PHY + 4 * 0x00090007, 0x000007c8);
    write32(DDR_PHY + 4 * 0x00090008, 0x00000109);
    write32(DDR_PHY + 4 * 0x00090009, 0x00000000);
    write32(DDR_PHY + 4 * 0x0009000a, 0x00000400);
    write32(DDR_PHY + 4 * 0x0009000b, 0x00000106);
    write32(DDR_PHY + 4 * 0x000d00e7, 0x00000400);
    write32(DDR_PHY + 4 * 0x00090017, 0x00000000);
    write32(DDR_PHY + 4 * 0x0009001f, 0x00000029);
    write32(DDR_PHY + 4 * 0x00090026, 0x00000068);
    write32(DDR_PHY + 4 * 0x000400d0, 0x00000000);
    write32(DDR_PHY + 4 * 0x000400d1, 0x00000101);
    write32(DDR_PHY + 4 * 0x000400d2, 0x00000105);
    write32(DDR_PHY + 4 * 0x000400d3, 0x00000107);
    write32(DDR_PHY + 4 * 0x000400d4, 0x0000010f);
    write32(DDR_PHY + 4 * 0x000400d5, 0x00000202);
    write32(DDR_PHY + 4 * 0x000400d6, 0x0000020a);
    write32(DDR_PHY + 4 * 0x000400d7, 0x0000020b);
    write32(DDR_PHY + 4 * 0x0002003a, 0x00000002);
    write32(DDR_PHY + 4 * 0x000200be, 0x00000003);
    write32(DDR_PHY + 4 * 0x0002000b, 0x00000053);
    write32(DDR_PHY + 4 * 0x0002000c, 0x000000a6);
    write32(DDR_PHY + 4 * 0x0002000d, 0x00000682);
    write32(DDR_PHY + 4 * 0x0002000e, 0x0000002c);
    write32(DDR_PHY + 4 * 0x0009000c, 0x00000000);
    write32(DDR_PHY + 4 * 0x0009000d, 0x00000173);
    write32(DDR_PHY + 4 * 0x0009000e, 0x00000060);
    write32(DDR_PHY + 4 * 0x0009000f, 0x00006110);
    write32(DDR_PHY + 4 * 0x00090010, 0x00002152);
    write32(DDR_PHY + 4 * 0x00090011, 0x0000dfbd);
    write32(DDR_PHY + 4 * 0x00090012, 0x00002060);
    write32(DDR_PHY + 4 * 0x00090013, 0x00006152);
    write32(DDR_PHY + 4 * 0x00020010, 0x0000005a);
    write32(DDR_PHY + 4 * 0x00020011, 0x00000003);
    write32(DDR_PHY + 4 * 0x00040080, 0x000000e0);
    write32(DDR_PHY + 4 * 0x00040081, 0x00000012);
    write32(DDR_PHY + 4 * 0x00040082, 0x000000e0);
    write32(DDR_PHY + 4 * 0x00040083, 0x00000012);
    write32(DDR_PHY + 4 * 0x00040084, 0x000000e0);
    write32(DDR_PHY + 4 * 0x00040085, 0x00000012);
    write32(DDR_PHY + 4 * 0x000400fd, 0x0000000f);
    write32(DDR_PHY + 4 * 0x00010011, 0x00000001);
    write32(DDR_PHY + 4 * 0x00010012, 0x00000001);
    write32(DDR_PHY + 4 * 0x00010013, 0x00000180);
    write32(DDR_PHY + 4 * 0x00010018, 0x00000001);
    write32(DDR_PHY + 4 * 0x00010002, 0x00006209);
    write32(DDR_PHY + 4 * 0x000100b2, 0x00000001);
    write32(DDR_PHY + 4 * 0x000101b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x000102b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x000103b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x000104b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x000105b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x000106b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x000107b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x000108b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x00011011, 0x00000001);
    write32(DDR_PHY + 4 * 0x00011012, 0x00000001);
    write32(DDR_PHY + 4 * 0x00011013, 0x00000180);
    write32(DDR_PHY + 4 * 0x00011018, 0x00000001);
    write32(DDR_PHY + 4 * 0x00011002, 0x00006209);
    write32(DDR_PHY + 4 * 0x000110b2, 0x00000001);
    write32(DDR_PHY + 4 * 0x000111b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x000112b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x000113b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x000114b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x000115b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x000116b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x000117b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x000118b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x00012011, 0x00000001);
    write32(DDR_PHY + 4 * 0x00012012, 0x00000001);
    write32(DDR_PHY + 4 * 0x00012013, 0x00000180);
    write32(DDR_PHY + 4 * 0x00012018, 0x00000001);
    write32(DDR_PHY + 4 * 0x00012002, 0x00006209);
    write32(DDR_PHY + 4 * 0x000120b2, 0x00000001);
    write32(DDR_PHY + 4 * 0x000121b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x000122b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x000123b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x000124b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x000125b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x000126b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x000127b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x000128b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x00013011, 0x00000001);
    write32(DDR_PHY + 4 * 0x00013012, 0x00000001);
    write32(DDR_PHY + 4 * 0x00013013, 0x00000180);
    write32(DDR_PHY + 4 * 0x00013018, 0x00000001);
    write32(DDR_PHY + 4 * 0x00013002, 0x00006209);
    write32(DDR_PHY + 4 * 0x000130b2, 0x00000001);
    write32(DDR_PHY + 4 * 0x000131b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x000132b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x000133b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x000134b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x000135b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x000136b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x000137b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x000138b4, 0x00000001);
    write32(DDR_PHY + 4 * 0x00020089, 0x00000001);
    write32(DDR_PHY + 4 * 0x00020088, 0x00000019);
    write32(DDR_PHY + 4 * 0x000c0080, 0x00000002);
    write32(DDR_PHY + 4 * 0x000d0000, 0x00000001);
    write32(DDR_PHY + 4 * 0x000d0000, 0x00000000);

    read32(DDR_PHY + 4 * 0x0002001d);
    write32(DDR_PHY + 4 * 0x0002001d, 0x00000001);
    read32(DDR_PHY + 4 * 0x00020097);
}

pub fn init() {
    change_pll_2660();

    // const MCTL_CLOCK_SWITCH: usize = PWR_BASE + 0x00a0;
    // let v = read32(MCTL_CLOCK_SWITCH);
    // DFI1 enabled
    // write32(MCTL_CLOCK_SWITCH, v | (1 << 4));

    write32(DDR_BASE + 0x00000304, 0x00000001);
    write32(DDR_BASE + 0x00000030, 0x00000001);
    read32(DDR_BASE + 0x00000004);
    write32(DDR_BASE + 0x00000000, 0x01081020);
    write32(DDR_BASE + 0x00000010, 0x0000b030);
    write32(DDR_BASE + 0x00000014, 0x0002ec4d);
    write32(DDR_BASE + 0x00000020, 0x00000202);
    write32(DDR_BASE + 0x00000024, 0xf491fa51);
    write32(DDR_BASE + 0x00000028, 0x00000001);
    write32(DDR_BASE + 0x0000002c, 0x00000001);
    write32(DDR_BASE + 0x00000030, 0x00000020);
    write32(DDR_BASE + 0x00000034, 0x00408a04);
    write32(DDR_BASE + 0x00000038, 0x0e0e0002);
    write32(DDR_BASE + 0x0000003c, 0x00000060);
    write32(DDR_BASE + 0x00000050, 0x98210000);
    write32(DDR_BASE + 0x00000054, 0x004b0043);
    write32(DDR_BASE + 0x00000060, 0x00000001);
    write32(DDR_BASE + 0x00000064, 0x00510057);
    write32(DDR_BASE + 0x00000068, 0x00280000);
    write32(DDR_BASE + 0x000000c0, 0x00000000);
    write32(DDR_BASE + 0x000000d0, 0xc0020002);
    write32(DDR_BASE + 0x000000d4, 0x00010002);
    write32(DDR_BASE + 0x000000d8, 0x00001600);
    write32(DDR_BASE + 0x000000dc, 0x00440024);
    write32(DDR_BASE + 0x000000e0, 0x00310008);
    write32(DDR_BASE + 0x000000e4, 0x00040008);
    write32(DDR_BASE + 0x000000e8, 0x0000004d);
    write32(DDR_BASE + 0x000000ec, 0x0000004d);
    write32(DDR_BASE + 0x000000f0, 0x00000000);
    write32(DDR_BASE + 0x000000f4, 0x0000032f);
    write32(DDR_BASE + 0x000000f8, 0x00000004);
    write32(DDR_BASE + 0x00000100, 0x171b161c);
    write32(DDR_BASE + 0x00000104, 0x00050528);
    write32(DDR_BASE + 0x00000108, 0x060c0e12);
    write32(DDR_BASE + 0x0000010c, 0x00a0a006);
    write32(DDR_BASE + 0x00000110, 0x0c04070c);
    write32(DDR_BASE + 0x00000114, 0x02040a0a);
    write32(DDR_BASE + 0x00000118, 0x01010006);
    write32(DDR_BASE + 0x0000011c, 0x00000402);
    write32(DDR_BASE + 0x00000120, 0x00000101);
    write32(DDR_BASE + 0x00000130, 0x00020000);
    write32(DDR_BASE + 0x00000134, 0x0b100002);
    write32(DDR_BASE + 0x00000138, 0x0000005c);
    write32(DDR_BASE + 0x0000013c, 0x80000000);
    write32(DDR_BASE + 0x00000144, 0x00860043);
    write32(DDR_BASE + 0x00000180, 0xc29b0014);
    write32(DDR_BASE + 0x00000184, 0x0227c42a);
    write32(DDR_BASE + 0x00000188, 0x00000000);
    write32(DDR_BASE + 0x00000190, 0x03938208);
    write32(DDR_BASE + 0x00000194, 0x00090202);
    write32(DDR_BASE + 0x00000198, 0x0710a120);
    write32(DDR_BASE + 0x000001a0, 0xe0400018);
    write32(DDR_BASE + 0x000001a4, 0x00020035);
    write32(DDR_BASE + 0x000001a8, 0x00000000);
    write32(DDR_BASE + 0x000001b0, 0x00000015);
    write32(DDR_BASE + 0x000001b4, 0x00001308);
    write32(DDR_BASE + 0x000001c0, 0x00000001);
    write32(DDR_BASE + 0x000001c4, 0xd1000000);
    write32(DDR_BASE + 0x00000200, 0x0000001f);

    // write32(DDR_BASE + 0x00000204, 0x00080808);

    // write32(DDR_BASE + 0x0000020c, 0x00000000);

    // write32(DDR_BASE + 0x00000214, 0x070f0707);
    // if ...
    // write32(DDR_BASE + 0x00000218, 0x0f0f0f07); // 2GB
    // write32(DDR_BASE + 0x00000218, 0x0f0f0707); // 4GB

    // write32(DDR_BASE + 0x00000224, 0x07070707);
    // write32(DDR_BASE + 0x00000228, 0x07070707);
    // write32(DDR_BASE + 0x0000022c, 0x00000007);

    write32(DDR_BASE + 0x00000204, 0x00070707);
    write32(DDR_BASE + 0x00000208, 0x00000000);
    write32(DDR_BASE + 0x0000020c, 0x1f000000);
    write32(DDR_BASE + 0x00000210, 0x00001f1f);
    write32(DDR_BASE + 0x00000214, 0x060f0606);
    write32(DDR_BASE + 0x00000218, 0x0f0f0606); // 2GB
    write32(DDR_BASE + 0x0000021c, 0x00000f0f);
    write32(DDR_BASE + 0x00000224, 0x06060606);
    write32(DDR_BASE + 0x00000228, 0x06060606);
    write32(DDR_BASE + 0x0000022c, 0x00000006);

    write32(DDR_BASE + 0x00000240, 0x06070944);
    write32(DDR_BASE + 0x00000244, 0x00000000);
    write32(DDR_BASE + 0x00000250, 0x804b1f18);
    write32(DDR_BASE + 0x00000254, 0x00002000);
    write32(DDR_BASE + 0x0000025c, 0x0f000001);
    write32(DDR_BASE + 0x00000264, 0x0f00007f);
    write32(DDR_BASE + 0x0000026c, 0x0f00007f);
    write32(DDR_BASE + 0x00000270, 0x04040208);
    write32(DDR_BASE + 0x00000274, 0x08400810);
    write32(DDR_BASE + 0x00000300, 0x00000000);
    write32(DDR_BASE + 0x00000304, 0x00000000);
    write32(DDR_BASE + 0x0000030c, 0x00000000);
    write32(DDR_BASE + 0x00000320, 0x00000001);
    write32(DDR_BASE + 0x00000328, 0x00000000);
    write32(DDR_BASE + 0x0000036c, 0x00000011);
    write32(DDR_BASE + 0x00000490, 0x00000001);
    write32(DDR_BASE + 0x00000540, 0x00000001);
    write32(DDR_BASE + 0x000005f0, 0x00000001);
    write32(DDR_BASE + 0x000006a0, 0x00000001);
    write32(DDR_BASE + 0x00000750, 0x00000001);
    write32(DDR_BASE + 0x00002020, 0x00000202);
    write32(DDR_BASE + 0x00002024, 0xf491fa51);
    write32(DDR_BASE + 0x00002034, 0x00408a04);
    write32(DDR_BASE + 0x00002050, 0xf0210000);
    write32(DDR_BASE + 0x00002064, 0x00518057);
    write32(DDR_BASE + 0x00002068, 0x00280000);
    write32(DDR_BASE + 0x000020dc, 0x00440024);
    write32(DDR_BASE + 0x000020e0, 0x00310008);
    write32(DDR_BASE + 0x000020e8, 0x0000004d);
    write32(DDR_BASE + 0x000020ec, 0x0000004d);
    write32(DDR_BASE + 0x000020f4, 0x0000032f);
    write32(DDR_BASE + 0x000020f8, 0x00000004);
    write32(DDR_BASE + 0x00002100, 0x171b161c);
    write32(DDR_BASE + 0x00002104, 0x00050528);
    write32(DDR_BASE + 0x00002108, 0x060c0e12);
    write32(DDR_BASE + 0x0000210c, 0x00a0a006);
    write32(DDR_BASE + 0x00002110, 0x0c04070c);
    write32(DDR_BASE + 0x00002114, 0x02040a0a);
    write32(DDR_BASE + 0x00002118, 0x01010006);
    write32(DDR_BASE + 0x0000211c, 0x00000402);
    write32(DDR_BASE + 0x00002120, 0x00000101);
    write32(DDR_BASE + 0x00002130, 0x00020000);
    write32(DDR_BASE + 0x00002134, 0x0b100002);
    write32(DDR_BASE + 0x00002138, 0x0000005c);
    write32(DDR_BASE + 0x0000213c, 0x80000000);
    write32(DDR_BASE + 0x00002144, 0x00860043);
    write32(DDR_BASE + 0x00002180, 0xc29b0014);
    write32(DDR_BASE + 0x00002190, 0x03938208);
    write32(DDR_BASE + 0x00002194, 0x00090202);
    write32(DDR_BASE + 0x000021b4, 0x00001308);
    write32(DDR_BASE + 0x00002240, 0x06070944);
    write32(DDR_BASE + 0x00003020, 0x00000202);
    write32(DDR_BASE + 0x00003024, 0xf491fa51);
    write32(DDR_BASE + 0x00003034, 0x00408a04);
    write32(DDR_BASE + 0x00003050, 0x48210000);
    write32(DDR_BASE + 0x00003064, 0x00518057);
    write32(DDR_BASE + 0x00003068, 0x00280000);
    write32(DDR_BASE + 0x000030dc, 0x00440024);
    write32(DDR_BASE + 0x000030e0, 0x00310008);
    write32(DDR_BASE + 0x000030e8, 0x0000004d);
    write32(DDR_BASE + 0x000030ec, 0x0000004d);
    write32(DDR_BASE + 0x000030f4, 0x0000032f);
    write32(DDR_BASE + 0x000030f8, 0x00000004);
    write32(DDR_BASE + 0x00003100, 0x171b161c);
    write32(DDR_BASE + 0x00003104, 0x00050528);
    write32(DDR_BASE + 0x00003108, 0x060c0e12);
    write32(DDR_BASE + 0x0000310c, 0x00a0a006);
    write32(DDR_BASE + 0x00003110, 0x0c04070c);
    write32(DDR_BASE + 0x00003114, 0x02040a0a);
    write32(DDR_BASE + 0x00003118, 0x01010006);
    write32(DDR_BASE + 0x0000311c, 0x00000402);
    write32(DDR_BASE + 0x00003120, 0x00000101);
    write32(DDR_BASE + 0x00003130, 0x00020000);
    write32(DDR_BASE + 0x00003134, 0x0b100002);
    write32(DDR_BASE + 0x00003138, 0x0000005c);
    write32(DDR_BASE + 0x0000313c, 0x80000000);
    write32(DDR_BASE + 0x00003144, 0x00860043);
    write32(DDR_BASE + 0x00003180, 0xc29b0014);
    write32(DDR_BASE + 0x00003190, 0x03938208);
    write32(DDR_BASE + 0x00003194, 0x00090202);
    write32(DDR_BASE + 0x000031b4, 0x00001308);
    write32(DDR_BASE + 0x00003240, 0x06070944);
    write32(DDR_BASE + 0x00004020, 0x00000202);
    write32(DDR_BASE + 0x00004024, 0xf491fa51);
    write32(DDR_BASE + 0x00004034, 0x00408a04);
    write32(DDR_BASE + 0x00004050, 0x70210000);
    write32(DDR_BASE + 0x00004064, 0x00510057);
    write32(DDR_BASE + 0x00004068, 0x00280000);
    write32(DDR_BASE + 0x000040dc, 0x00440024);
    write32(DDR_BASE + 0x000040e0, 0x00310008);
    write32(DDR_BASE + 0x000040e8, 0x0000004d);
    write32(DDR_BASE + 0x000040ec, 0x0000004d);
    write32(DDR_BASE + 0x000040f4, 0x0000032f);
    write32(DDR_BASE + 0x000040f8, 0x00000004);
    write32(DDR_BASE + 0x00004100, 0x171b161c);
    write32(DDR_BASE + 0x00004104, 0x00050528);
    write32(DDR_BASE + 0x00004108, 0x060c0e12);
    write32(DDR_BASE + 0x0000410c, 0x00a0a006);
    write32(DDR_BASE + 0x00004110, 0x0c04070c);
    write32(DDR_BASE + 0x00004114, 0x02040a0a);
    write32(DDR_BASE + 0x00004118, 0x01010006);
    write32(DDR_BASE + 0x0000411c, 0x00000402);
    write32(DDR_BASE + 0x00004120, 0x00000101);
    write32(DDR_BASE + 0x00004130, 0x00020000);
    write32(DDR_BASE + 0x00004134, 0x0b100002);
    write32(DDR_BASE + 0x00004138, 0x0000005c);
    write32(DDR_BASE + 0x0000413c, 0x80000000);
    write32(DDR_BASE + 0x00004144, 0x00860043);
    write32(DDR_BASE + 0x00004180, 0xc29b0014);
    write32(DDR_BASE + 0x00004190, 0x03938208);
    write32(DDR_BASE + 0x00004194, 0x00090202);
    write32(DDR_BASE + 0x000041b4, 0x00001308);
    write32(DDR_BASE + 0x00004240, 0x06070944);
    read32(DDR_BASE + 0x00000060);
    write32(DDR_BASE + 0x00000400, 0x00000000);
    write32(DDR_BASE + 0x00000404, 0x0000400f);
    write32(DDR_BASE + 0x000004b4, 0x0000400f);
    write32(DDR_BASE + 0x00000564, 0x0000400f);
    write32(DDR_BASE + 0x00000614, 0x0000400f);
    write32(DDR_BASE + 0x000006c4, 0x0000400f);
    write32(DDR_BASE + 0x00000404, 0x0000500f);
    write32(DDR_BASE + 0x000004b4, 0x0000500f);
    write32(DDR_BASE + 0x00000564, 0x0000500f);
    write32(DDR_BASE + 0x00000614, 0x0000500f);
    write32(DDR_BASE + 0x000006c4, 0x0000500f);
    write32(DDR_BASE + 0x00000404, 0x0000500f);
    write32(DDR_BASE + 0x000004b4, 0x0000500f);
    write32(DDR_BASE + 0x00000564, 0x0000500f);
    write32(DDR_BASE + 0x00000614, 0x0000500f);
    write32(DDR_BASE + 0x000006c4, 0x0000500f);
    write32(DDR_BASE + 0x00000404, 0x0000100f);
    write32(DDR_BASE + 0x000004b4, 0x0000100f);
    write32(DDR_BASE + 0x00000564, 0x0000100f);
    write32(DDR_BASE + 0x00000614, 0x0000100f);
    write32(DDR_BASE + 0x000006c4, 0x0000100f);
    write32(DDR_BASE + 0x00000408, 0x0000400f);
    write32(DDR_BASE + 0x000004b8, 0x0000400f);
    write32(DDR_BASE + 0x00000568, 0x0000400f);
    write32(DDR_BASE + 0x00000618, 0x0000400f);
    write32(DDR_BASE + 0x000006c8, 0x0000400f);
    write32(DDR_BASE + 0x00000408, 0x0000500f);
    write32(DDR_BASE + 0x000004b8, 0x0000500f);
    write32(DDR_BASE + 0x00000568, 0x0000500f);
    write32(DDR_BASE + 0x00000618, 0x0000500f);
    write32(DDR_BASE + 0x000006c8, 0x0000500f);
    write32(DDR_BASE + 0x00000408, 0x0000500f);
    write32(DDR_BASE + 0x000004b8, 0x0000500f);
    write32(DDR_BASE + 0x00000568, 0x0000500f);
    write32(DDR_BASE + 0x00000618, 0x0000500f);
    write32(DDR_BASE + 0x000006c8, 0x0000500f);
    write32(DDR_BASE + 0x00000408, 0x0000100f);
    write32(DDR_BASE + 0x000004b8, 0x0000100f);
    write32(DDR_BASE + 0x00000568, 0x0000100f);
    write32(DDR_BASE + 0x00000618, 0x0000100f);
    write32(DDR_BASE + 0x000006c8, 0x0000100f);
    read32(DDR_BASE + 0x00000030);
    write32(DDR_BASE + 0x00000030, 0x00000020);

    let v = read32(MEM_CTL_POWER_LPI_CTL);
    // bit 17: DDR controller init done
    write32(MEM_CTL_POWER_LPI_CTL, v | 0x0002_0000);

    write32(DDR_BASE + 0x00000304, 0x00000000);
    read32(DDR_BASE + 0x00000030);
    write32(DDR_BASE + 0x00000030, 0x00000020);
    read32(DDR_BASE + 0x00000030);
    write32(DDR_BASE + 0x00000030, 0x00000020);
    read32(DDR_BASE + 0x000001c4);
    write32(DDR_BASE + 0x000001c4, 0xd1000000);
    write32(DDR_BASE + 0x00000320, 0x00000000);
    write32(DDR_BASE + 0x000001b0, 0x00000014);
    write32(DDR_BASE + 0x000001b0, 0x00000014);
    write32(DDR_BASE + 0x00000304, 0x00000002);
    read32(DDR_BASE + 0x000000d0);
    read32(DDR_BASE + 0x000001c0);
    read32(DDR_BASE + 0x00000000);
    read32(DDR_BASE + 0x000000dc);
    read32(DDR_BASE + 0x000000dc);
    read32(DDR_BASE + 0x000000e0);
    read32(DDR_BASE + 0x000000e8);
    read32(DDR_BASE + 0x000000e8);
    read32(DDR_BASE + 0x000000e0);
    read32(DDR_BASE + 0x000000ec);
    read32(DDR_BASE + 0x000000ec);
    read32(DDR_BASE + 0x000000d0);
    read32(DDR_BASE + 0x000001c0);
    read32(DDR_BASE + 0x00000000);
    read32(DDR_BASE + 0x000000dc);
    read32(DDR_BASE + 0x000000dc);
    read32(DDR_BASE + 0x000000e0);
    read32(DDR_BASE + 0x000000e8);
    read32(DDR_BASE + 0x000000e8);
    read32(DDR_BASE + 0x000000e0);
    read32(DDR_BASE + 0x000000ec);
    read32(DDR_BASE + 0x000000ec);
    read32(DDR_BASE + 0x000000d0);

    init_phy();

    write32(DDR_BASE + 0x000001b0, 0x00000034);
    while read32(DDR_BASE + 0x000001bc) & 0x1 != 0x1 {}

    write32(DDR_BASE + 0x000001b0, 0x00000014);
    write32(DDR_BASE + 0x000001b0, 0x00000015);
    write32(DDR_BASE + 0x00000030, 0x00000000);
    write32(DDR_BASE + 0x00000030, 0x00000000);
    write32(DDR_BASE + 0x00000320, 0x00000001);

    while read32(DDR_BASE + 0x0000_0324) & 0x1 != 0x1 {}

    while read32(DDR_BASE + 0x0000_0004) & 0x1 != 0x1 {}

    write32(DDR_BASE + 0x000001c4, 0xd1000000);
    write32(DDR_BASE + 0x00000320, 0x00000000);
    write32(DDR_BASE + 0x000000d0, 0x00020002);
    write32(DDR_BASE + 0x00000320, 0x00000001);

    while read32(DDR_BASE + 0x0000_0324) & 0x1 != 0x1 {}

    write32(DDR_BASE + 0x00000304, 0x00000000);
    write32(DDR_BASE + 0x00000030, 0x00000000);
    write32(DDR_BASE + 0x00000030, 0x00000000);
    write32(DDR_BASE + 0x00000490, 0x00000001);
    write32(DDR_BASE + 0x00000540, 0x00000001);
    write32(DDR_BASE + 0x000005f0, 0x00000001);
    write32(DDR_BASE + 0x000006a0, 0x00000001);
    write32(DDR_BASE + 0x00000750, 0x00000001);

    write32(DDR_BASE + 0x00000060, 0x00000000);
    write32(DDR_BASE + 0x00000050, 0x98210000);

    if DEBUG {
        debug_ca_dq_dbytes();
    }
}

fn debug_ca_dq_dbytes() {
    let addr = DDR_PHY + 4 * 0x5401b;
    println!("MR12-CA {:08x} @ {addr:08x}", read32(addr));
    let addr = DDR_PHY + 4 * 0x5401c;
    println!("MR14-DQ {:08x} @ {addr:08x}", read32(addr));

    for b in 0..4 {
        for i in 0..8 {
            let addr = DDR_PHY + 4 * (0x0001_0040 + b * 0x1000 + i * 0x100);
            println!("dbyte{b} {:08x} @ {addr:08x}", read32(addr));
        }
    }
}

fn train_message(train_data: u32) {
    let msg = match train_data {
        0x00 => "End of initialization",
        0x01 => "End of fine write leveling",
        0x02 => "End of read enable training",
        0x03 => "End of read delay center optimization",
        0x04 => "End of write delay center optimization",
        0x05 => "End of 2D read delay/voltage center optimization",
        0x06 => "End of 2D write delay /voltage center optimization",
        0x07 => "Firmware run has completed 2667",
        0x08 => "Enter streaming message mode",
        0x09 => "End of max read latency training",
        0x0a => "End of read dq deskew training",
        0x0b => "End of LCDL offset calibration",
        0x0c => "End of LRDIMM Specific training (DWL, MREP, MRD and MWD)",
        0x0d => "End of CA training",
        0xfd => "End of MPR read delay center optimization",
        0xfe => "End of Write leveling coarse delay",
        0xff => "FATAL ERROR 2667.",
        _ => "Un-recognized message... !",
    };
    println!("{train_data:08x}: PMU Major Msg: {msg}");
}
