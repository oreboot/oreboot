#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(non_upper_case_globals)]

/*
 * This file is part of the coreboot project.
 *
 * Copyright 2021 the oreboot authors
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; version 2 of the License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 */

// from:
/* SPDX-License-Identifier: GPL-2.0-or-later */
/**
 ******************************************************************************
 * @file  clkgen_ctrl_macro.h
 * @author  StarFive Technology
 * @version  V1.0
 * @date  06/25/2020
 * @brief
 ******************************************************************************
 * @copy
 *
 * THE PRESENT SOFTWARE WHICH IS FOR GUIDANCE ONLY AIMS AT PROVIDING CUSTOMERS
 * WITH CODING INFORMATION REGARDING THEIR PRODUCTS IN ORDER FOR THEM TO SAVE
 * TIME. AS A RESULT, STARFIVE SHALL NOT BE HELD LIABLE FOR ANY
 * DIRECT, INDIRECT OR CONSEQUENTIAL DAMAGES WITH RESPECT TO ANY CLAIMS ARISING
 * FROM THE CONTENT OF SUCH SOFTWARE AND/OR THE USE MADE BY CUSTOMERS OF THE
 * CODING INFORMATION CONTAINED HEREIN IN CONNECTION WITH THEIR PRODUCTS.
 *
 * COPYRIGHT 2020 Shanghai StarFive Technology Co., Ltd.
 */
use clock::ClockNode;
//use core::ops;
use model::*;

use crate::is_qemu;
//use crate::reg;
use core::ptr;
//use register::mmio::ReadWrite;
//use register::register_bitfields;

// No register block. I don't want to vet that all these
// things are nice and contiguous.
pub const CLKGEN_BASE_ADDR: u32 = 0x1180_0000;
pub const DENALI_CTL_00_DATA: u32 = 0x00000a00;
pub const clk_cpundbus_root_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x0;
pub const clk_dla_root_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x4;
pub const clk_dsp_root_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x8;
pub const clk_gmacusb_root_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0xC;
pub const clk_perh0_root_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x10;
pub const clk_perh1_root_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x14;
pub const clk_vin_root_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x18;
pub const clk_vout_root_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x1C;
pub const clk_audio_root_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x20;
pub const clk_cdechifi4_root_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x24;
pub const clk_cdec_root_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x28;
pub const clk_voutbus_root_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x2C;
pub const clk_cpunbus_root_div_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x30;
pub const clk_dsp_root_div_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x34;
pub const clk_perh0_src_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x38;
pub const clk_perh1_src_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x3C;
pub const clk_pll0_testout_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x40;
pub const clk_pll1_testout_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x44;
pub const clk_pll2_testout_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x48;
pub const clk_pll2_refclk_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x4C;
pub const clk_cpu_core_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x50;
pub const clk_cpu_axi_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x54;
pub const clk_ahb_bus_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x58;
pub const clk_apb1_bus_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x5C;
pub const clk_apb2_bus_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x60;
pub const clk_dom3ahb_bus_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x64;
pub const clk_dom7ahb_bus_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x68;
pub const clk_u74_core0_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x6C;
pub const clk_u74_core1_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x70;
pub const clk_u74_axi_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x74;
pub const clk_u74rtc_toggle_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x78;
pub const clk_sgdma2p_axi_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x7C;
pub const clk_dma2pnoc_axi_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x80;
pub const clk_sgdma2p_ahb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x84;
pub const clk_dla_bus_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x88;
pub const clk_dla_axi_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x8C;
pub const clk_dlanoc_axi_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x90;
pub const clk_dla_apb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x94;
pub const clk_vp6_core_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x98;
pub const clk_vp6bus_src_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x9C;
pub const clk_vp6_axi_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0xA0;
pub const clk_vcdecbus_src_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0xA4;
pub const clk_vdec_bus_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0xA8;
pub const clk_vdec_axi_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0xAC;
pub const clk_vdecbrg_mainclk_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0xB0;
pub const clk_vdec_bclk_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0xB4;
pub const clk_vdec_cclk_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0xB8;
pub const clk_vdec_apb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0xBC;
pub const clk_jpeg_axi_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0xC0;
pub const clk_jpeg_cclk_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0xC4;
pub const clk_jpeg_apb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0xC8;
pub const clk_gc300_2x_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0xCC;
pub const clk_gc300_ahb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0xD0;
pub const clk_jpcgc300_axibus_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0xD4;
pub const clk_gc300_axi_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0xD8;
pub const clk_jpcgc300_mainclk_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0xDC;
pub const clk_venc_bus_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0xE0;
pub const clk_venc_axi_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0xE4;
pub const clk_vencbrg_mainclk_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0xE8;
pub const clk_venc_bclk_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0xEC;
pub const clk_venc_cclk_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0xF0;
pub const clk_venc_apb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0xF4;
pub const clk_ddrpll_div2_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0xF8;
pub const clk_ddrpll_div4_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0xFC;
pub const clk_ddrpll_div8_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x100;
pub const clk_ddrosc_div2_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x104;
pub const clk_ddrc0_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x108;
pub const clk_ddrc1_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x10C;
pub const clk_ddrphy_apb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x110;
pub const clk_noc_rob_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x114;
pub const clk_noc_cog_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x118;
pub const clk_nne_ahb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x11C;
pub const clk_nnebus_src1_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x120;
pub const clk_nne_bus_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x124;
pub const clk_nne_axi_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x128;
pub const clk_nnenoc_axi_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x12C;
pub const clk_dlaslv_axi_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x130;
pub const clk_dspx2c_axi_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x134;
pub const clk_hifi4_src_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x138;
pub const clk_hifi4_corefree_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x13C;
pub const clk_hifi4_core_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x140;
pub const clk_hifi4_bus_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x144;
pub const clk_hifi4_axi_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x148;
pub const clk_hifi4noc_axi_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x14C;
pub const clk_sgdma1p_bus_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x150;
pub const clk_sgdma1p_axi_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x154;
pub const clk_dma1p_axi_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x158;
pub const clk_x2c_axi_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x15C;
pub const clk_usb_bus_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x160;
pub const clk_usb_axi_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x164;
pub const clk_usbnoc_axi_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x168;
pub const clk_usbphy_rootdiv_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x16C;
pub const clk_usbphy_125m_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x170;
pub const clk_usbphy_plldiv25m_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x174;
pub const clk_usbphy_25m_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x178;
pub const clk_audio_div_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x17C;
pub const clk_audio_src_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x180;
pub const clk_audio_12288_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x184;
pub const clk_vin_src_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x188;
pub const clk_isp0_bus_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x18C;
pub const clk_isp0_axi_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x190;
pub const clk_isp0noc_axi_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x194;
pub const clk_ispslv_axi_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x198;
pub const clk_isp1_bus_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x19C;
pub const clk_isp1_axi_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x1A0;
pub const clk_isp1noc_axi_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x1A4;
pub const clk_vin_bus_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x1A8;
pub const clk_vin_axi_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x1AC;
pub const clk_vinnoc_axi_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x1B0;
pub const clk_vout_src_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x1B4;
pub const clk_dispbus_src_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x1B8;
pub const clk_disp_bus_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x1BC;
pub const clk_disp_axi_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x1C0;
pub const clk_dispnoc_axi_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x1C4;
pub const clk_sdio0_ahb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x1C8;
pub const clk_sdio0_cclkint_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x1CC;
pub const clk_sdio0_cclkint_inv_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x1D0;
pub const clk_sdio1_ahb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x1D4;
pub const clk_sdio1_cclkint_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x1D8;
pub const clk_sdio1_cclkint_inv_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x1DC;
pub const clk_gmac_ahb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x1E0;
pub const clk_gmac_root_div_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x1E4;
pub const clk_gmac_ptp_refclk_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x1E8;
pub const clk_gmac_gtxclk_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x1EC;
pub const clk_gmac_rmii_txclk_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x1F0;
pub const clk_gmac_rmii_rxclk_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x1F4;
pub const clk_gmac_tx_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x1F8;
pub const clk_gmac_tx_inv_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x1FC;
pub const clk_gmac_rx_pre_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x200;
pub const clk_gmac_rx_inv_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x204;
pub const clk_gmac_rmii_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x208;
pub const clk_gmac_tophyref_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x20C;
pub const clk_spi2ahb_ahb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x210;
pub const clk_spi2ahb_core_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x214;
pub const clk_ezmaster_ahb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x218;
pub const clk_e24_ahb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x21C;
pub const clk_e24rtc_toggle_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x220;
pub const clk_qspi_ahb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x224;
pub const clk_qspi_apb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x228;
pub const clk_qspi_refclk_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x22C;
pub const clk_sec_ahb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x230;
pub const clk_aes_clk_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x234;
pub const clk_sha_clk_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x238;
pub const clk_pka_clk_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x23C;
pub const clk_trng_apb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x240;
pub const clk_otp_apb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x244;
pub const clk_uart0_apb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x248;
pub const clk_uart0_core_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x24C;
pub const clk_uart1_apb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x250;
pub const clk_uart1_core_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x254;
pub const clk_spi0_apb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x258;
pub const clk_spi0_core_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x25C;
pub const clk_spi1_apb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x260;
pub const clk_spi1_core_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x264;
pub const clk_i2c0_apb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x268;
pub const clk_i2c0_core_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x26C;
pub const clk_i2c1_apb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x270;
pub const clk_i2c1_core_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x274;
pub const clk_gpio_apb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x278;
pub const clk_uart2_apb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x27C;
pub const clk_uart2_core_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x280;
pub const clk_uart3_apb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x284;
pub const clk_uart3_core_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x288;
pub const clk_spi2_apb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x28C;
pub const clk_spi2_core_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x290;
pub const clk_spi3_apb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x294;
pub const clk_spi3_core_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x298;
pub const clk_i2c2_apb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x29C;
pub const clk_i2c2_core_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x2A0;
pub const clk_i2c3_apb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x2A4;
pub const clk_i2c3_core_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x2A8;
pub const clk_wdtimer_apb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x2AC;
pub const clk_wdt_coreclk_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x2B0;
pub const clk_timer0_coreclk_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x2B4;
pub const clk_timer1_coreclk_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x2B8;
pub const clk_timer2_coreclk_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x2BC;
pub const clk_timer3_coreclk_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x2C0;
pub const clk_timer4_coreclk_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x2C4;
pub const clk_timer5_coreclk_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x2C8;
pub const clk_timer6_coreclk_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x2CC;
pub const clk_vp6intc_apb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x2D0;
pub const clk_pwm_apb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x2D4;
pub const clk_msi_apb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x2D8;
pub const clk_temp_apb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x2DC;
pub const clk_temp_sense_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x2E0;
pub const clk_syserr_apb_ctrl_REG_ADDR: u32 = CLKGEN_BASE_ADDR + 0x2E4;

fn poke32(a: u32, v: u32) {
    let y = a as *mut u32;
    unsafe {
        ptr::write_volatile(y, v);
    }
}

fn peek32(a: u32) -> u32 {
    let y = a as *const u32;
    unsafe { ptr::read_volatile(y) }
}

pub fn _ENABLE_CLOCK_clk_cpundbus_root_() {}

pub fn _SWITCH_CLOCK_clk_cpundbus_root_SOURCE_clk_osc_sys_() {
    let mut v = peek32(clk_cpundbus_root_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x0 & 0x3) << 24;
    poke32(clk_cpundbus_root_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_cpundbus_root_SOURCE_clk_pll0_out_() {
    let mut v = peek32(clk_cpundbus_root_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x1 & 0x3) << 24;
    poke32(clk_cpundbus_root_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_cpundbus_root_SOURCE_clk_pll1_out_() {
    let mut v = peek32(clk_cpundbus_root_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x2 & 0x3) << 24;
    poke32(clk_cpundbus_root_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_cpundbus_root_SOURCE_clk_pll2_out_() {
    let mut v = peek32(clk_cpundbus_root_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x3 & 0x3) << 24;
    poke32(clk_cpundbus_root_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_SOURCE_STATUS_clk_cpundbus_root_() -> u32 {
    let v = peek32(clk_cpundbus_root_ctrl_REG_ADDR) >> 24;
    v & 0x3
}

pub fn _ENABLE_CLOCK_clk_dla_root_() {}

pub fn _SWITCH_CLOCK_clk_dla_root_SOURCE_clk_osc_sys_() {
    let mut v = peek32(clk_dla_root_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x0 & 0x3) << 24;
    poke32(clk_dla_root_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_dla_root_SOURCE_clk_pll1_out_() {
    let mut v = peek32(clk_dla_root_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x1 & 0x3) << 24;
    poke32(clk_dla_root_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_dla_root_SOURCE_clk_pll2_out_() {
    let mut v = peek32(clk_dla_root_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x2 & 0x3) << 24;
    poke32(clk_dla_root_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_SOURCE_STATUS_clk_dla_root_() -> u32 {
    let v = peek32(clk_dla_root_ctrl_REG_ADDR) >> 24;
    v & 0x3
}

pub fn _ENABLE_CLOCK_clk_dsp_root_() {}

pub fn _SWITCH_CLOCK_clk_dsp_root_SOURCE_clk_osc_sys_() {
    let mut v = peek32(clk_dsp_root_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x0 & 0x3) << 24;
    poke32(clk_dsp_root_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_dsp_root_SOURCE_clk_pll0_out_() {
    let mut v = peek32(clk_dsp_root_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x1 & 0x3) << 24;
    poke32(clk_dsp_root_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_dsp_root_SOURCE_clk_pll1_out_() {
    let mut v = peek32(clk_dsp_root_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x2 & 0x3) << 24;
    poke32(clk_dsp_root_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_dsp_root_SOURCE_clk_pll2_out_() {
    let mut v = peek32(clk_dsp_root_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x3 & 0x3) << 24;
    poke32(clk_dsp_root_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_SOURCE_STATUS_clk_dsp_root_() -> u32 {
    let v = peek32(clk_dsp_root_ctrl_REG_ADDR) >> 24;
    v & 0x3
}

pub fn _ENABLE_CLOCK_clk_gmacusb_root_() {}

pub fn _SWITCH_CLOCK_clk_gmacusb_root_SOURCE_clk_osc_sys_() {
    let mut v = peek32(clk_gmacusb_root_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x0 & 0x3) << 24;
    poke32(clk_gmacusb_root_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_gmacusb_root_SOURCE_clk_pll0_out_() {
    let mut v = peek32(clk_gmacusb_root_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x1 & 0x3) << 24;
    poke32(clk_gmacusb_root_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_gmacusb_root_SOURCE_clk_pll2_out_() {
    let mut v = peek32(clk_gmacusb_root_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x2 & 0x3) << 24;
    poke32(clk_gmacusb_root_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_SOURCE_STATUS_clk_gmacusb_root_() -> u32 {
    let v = peek32(clk_gmacusb_root_ctrl_REG_ADDR) >> 24;
    v & 0x3
}

pub fn _ENABLE_CLOCK_clk_perh0_root_() {}

pub fn _SWITCH_CLOCK_clk_perh0_root_SOURCE_clk_osc_sys_() {
    let mut v = peek32(clk_perh0_root_ctrl_REG_ADDR);
    v &= !(0x1 << 24);
    v |= (0x0 & 0x1) << 24;
    poke32(clk_perh0_root_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_perh0_root_SOURCE_clk_pll0_out_() {
    let mut v = peek32(clk_perh0_root_ctrl_REG_ADDR);
    v &= !(0x1 << 24);
    v |= (0x1 & 0x1) << 24;
    poke32(clk_perh0_root_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_SOURCE_STATUS_clk_perh0_root_() -> u32 {
    let v = peek32(clk_perh0_root_ctrl_REG_ADDR) >> 24;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_perh1_root_() {}

pub fn _SWITCH_CLOCK_clk_perh1_root_SOURCE_clk_osc_sys_() {
    let mut v = peek32(clk_perh1_root_ctrl_REG_ADDR);
    v &= !(0x1 << 24);
    v |= (0x0 & 0x1) << 24;
    poke32(clk_perh1_root_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_perh1_root_SOURCE_clk_pll2_out_() {
    let mut v = peek32(clk_perh1_root_ctrl_REG_ADDR);
    v &= !(0x1 << 24);
    v |= (0x1 & 0x1) << 24;
    poke32(clk_perh1_root_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_SOURCE_STATUS_clk_perh1_root_() -> u32 {
    let v = peek32(clk_perh1_root_ctrl_REG_ADDR) >> 24;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_vin_root_() {}

pub fn _SWITCH_CLOCK_clk_vin_root_SOURCE_clk_osc_sys_() {
    let mut v = peek32(clk_vin_root_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x0 & 0x3) << 24;
    poke32(clk_vin_root_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_vin_root_SOURCE_clk_pll1_out_() {
    let mut v = peek32(clk_vin_root_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x1 & 0x3) << 24;
    poke32(clk_vin_root_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_vin_root_SOURCE_clk_pll2_out_() {
    let mut v = peek32(clk_vin_root_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x2 & 0x3) << 24;
    poke32(clk_vin_root_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_SOURCE_STATUS_clk_vin_root_() -> u32 {
    let v = peek32(clk_vin_root_ctrl_REG_ADDR) >> 24;
    v & 0x3
}

pub fn _ENABLE_CLOCK_clk_vout_root_() {}

pub fn _SWITCH_CLOCK_clk_vout_root_SOURCE_clk_osc_aud_() {
    let mut v = peek32(clk_vout_root_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x0 & 0x3) << 24;
    poke32(clk_vout_root_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_vout_root_SOURCE_clk_pll0_out_() {
    let mut v = peek32(clk_vout_root_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x1 & 0x3) << 24;
    poke32(clk_vout_root_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_vout_root_SOURCE_clk_pll2_out_() {
    let mut v = peek32(clk_vout_root_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x2 & 0x3) << 24;
    poke32(clk_vout_root_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_SOURCE_STATUS_clk_vout_root_() -> u32 {
    let v = peek32(clk_vout_root_ctrl_REG_ADDR) >> 24;
    v & 0x3
}

pub fn _ENABLE_CLOCK_clk_audio_root_() {
    let mut v = peek32(clk_audio_root_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_audio_root_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_audio_root_() {
    let mut v = peek32(clk_audio_root_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_audio_root_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_audio_root_() -> u32 {
    let v = peek32(clk_audio_root_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_audio_root_(div: u32) {
    let mut v = peek32(clk_audio_root_ctrl_REG_ADDR);
    v &= !(0xF);
    v |= (div & 0xF);
    poke32(clk_audio_root_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_audio_root_() -> u32 {
    let v = peek32(clk_audio_root_ctrl_REG_ADDR);
    v & 0xf
}

pub fn _ENABLE_CLOCK_clk_cdechifi4_root_() {}

pub fn _SWITCH_CLOCK_clk_cdechifi4_root_SOURCE_clk_osc_sys_() {
    let mut v = peek32(clk_cdechifi4_root_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x0 & 0x3) << 24;
    poke32(clk_cdechifi4_root_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_cdechifi4_root_SOURCE_clk_pll1_out_() {
    let mut v = peek32(clk_cdechifi4_root_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x1 & 0x3) << 24;
    poke32(clk_cdechifi4_root_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_cdechifi4_root_SOURCE_clk_pll2_out_() {
    let mut v = peek32(clk_cdechifi4_root_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x2 & 0x3) << 24;
    poke32(clk_cdechifi4_root_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_SOURCE_STATUS_clk_cdechifi4_root_() -> u32 {
    let v = peek32(clk_cdechifi4_root_ctrl_REG_ADDR) >> 24;
    v & 0x3
}

pub fn _ENABLE_CLOCK_clk_cdec_root_() {}

pub fn _SWITCH_CLOCK_clk_cdec_root_SOURCE_clk_osc_sys_() {
    let mut v = peek32(clk_cdec_root_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x0 & 0x3) << 24;
    poke32(clk_cdec_root_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_cdec_root_SOURCE_clk_pll0_out_() {
    let mut v = peek32(clk_cdec_root_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x1 & 0x3) << 24;
    poke32(clk_cdec_root_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_cdec_root_SOURCE_clk_pll1_out_() {
    let mut v = peek32(clk_cdec_root_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x2 & 0x3) << 24;
    poke32(clk_cdec_root_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_SOURCE_STATUS_clk_cdec_root_() -> u32 {
    let v = peek32(clk_cdec_root_ctrl_REG_ADDR) >> 24;
    v & 0x3
}

pub fn _ENABLE_CLOCK_clk_voutbus_root_() {}

pub fn _SWITCH_CLOCK_clk_voutbus_root_SOURCE_clk_osc_aud_() {
    let mut v = peek32(clk_voutbus_root_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x0 & 0x3) << 24;
    poke32(clk_voutbus_root_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_voutbus_root_SOURCE_clk_pll0_out_() {
    let mut v = peek32(clk_voutbus_root_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x1 & 0x3) << 24;
    poke32(clk_voutbus_root_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_voutbus_root_SOURCE_clk_pll2_out_() {
    let mut v = peek32(clk_voutbus_root_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x2 & 0x3) << 24;
    poke32(clk_voutbus_root_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_SOURCE_STATUS_clk_voutbus_root_() -> u32 {
    let v = peek32(clk_voutbus_root_ctrl_REG_ADDR) >> 24;
    v & 0x3
}

pub fn _ENABLE_CLOCK_clk_cpunbus_root_div_() {}

pub fn _DIVIDE_CLOCK_clk_cpunbus_root_div_(div: u32) {
    let mut v = peek32(clk_cpunbus_root_div_ctrl_REG_ADDR);
    v &= !(0x3);
    v |= (div & 0x3);
    poke32(clk_cpunbus_root_div_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_cpunbus_root_div_() -> u32 {
    let v = peek32(clk_cpunbus_root_div_ctrl_REG_ADDR);
    v & 0x3
}

pub fn _ENABLE_CLOCK_clk_dsp_root_div_() {}

pub fn _DIVIDE_CLOCK_clk_dsp_root_div_(div: u32) {
    let mut v = peek32(clk_dsp_root_div_ctrl_REG_ADDR);
    v &= !(0x7);
    v |= (div & 0x7);
    poke32(clk_dsp_root_div_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_dsp_root_div_() -> u32 {
    let v = peek32(clk_dsp_root_div_ctrl_REG_ADDR);
    v & 0x7
}

pub fn _ENABLE_CLOCK_clk_perh0_src_() {}

pub fn _DIVIDE_CLOCK_clk_perh0_src_(div: u32) {
    let mut v = peek32(clk_perh0_src_ctrl_REG_ADDR);
    v &= !(0x7);
    v |= (div & 0x7);
    poke32(clk_perh0_src_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_perh0_src_() -> u32 {
    let v = peek32(clk_perh0_src_ctrl_REG_ADDR);
    v & 0x7
}

pub fn _ENABLE_CLOCK_clk_perh1_src_() {}

pub fn _DIVIDE_CLOCK_clk_perh1_src_(div: u32) {
    let mut v = peek32(clk_perh1_src_ctrl_REG_ADDR);
    v &= !(0x7);
    v |= (div & 0x7);
    poke32(clk_perh1_src_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_perh1_src_() -> u32 {
    let v = peek32(clk_perh1_src_ctrl_REG_ADDR);
    v & 0x7
}

pub fn _ENABLE_CLOCK_clk_pll0_testout_() {
    let mut v = peek32(clk_pll0_testout_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_pll0_testout_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_pll0_testout_() {
    let mut v = peek32(clk_pll0_testout_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_pll0_testout_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_pll0_testout_() -> u32 {
    let v = peek32(clk_pll0_testout_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_pll0_testout_(div: u32) {
    let mut v = peek32(clk_pll0_testout_ctrl_REG_ADDR);
    v &= !(0x1F);
    v |= (div & 0x1F);
    poke32(clk_pll0_testout_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_pll0_testout_() -> u32 {
    let v = peek32(clk_pll0_testout_ctrl_REG_ADDR);
    v & 0x1f
}

pub fn _ENABLE_CLOCK_clk_pll1_testout_() {
    let mut v = peek32(clk_pll1_testout_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_pll1_testout_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_pll1_testout_() {
    let mut v = peek32(clk_pll1_testout_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_pll1_testout_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_pll1_testout_() -> u32 {
    let v = peek32(clk_pll1_testout_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_pll1_testout_(div: u32) {
    let mut v = peek32(clk_pll1_testout_ctrl_REG_ADDR);
    v &= !(0x1F);
    v |= (div & 0x1F);
    poke32(clk_pll1_testout_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_pll1_testout_() -> u32 {
    let v = peek32(clk_pll1_testout_ctrl_REG_ADDR);
    v & 0x1f
}

pub fn _ENABLE_CLOCK_clk_pll2_testout_() {
    let mut v = peek32(clk_pll2_testout_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_pll2_testout_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_pll2_testout_() {
    let mut v = peek32(clk_pll2_testout_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_pll2_testout_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_pll2_testout_() -> u32 {
    let v = peek32(clk_pll2_testout_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_pll2_testout_(div: u32) {
    let mut v = peek32(clk_pll2_testout_ctrl_REG_ADDR);
    v &= !(0x1F);
    v |= (div & 0x1F);
    poke32(clk_pll2_testout_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_pll2_testout_() -> u32 {
    let v = peek32(clk_pll2_testout_ctrl_REG_ADDR);
    v & 0x1f
}

pub fn _ENABLE_CLOCK_clk_pll2_refclk_() {}

pub fn _SWITCH_CLOCK_clk_pll2_refclk_SOURCE_clk_osc_sys_() {
    let mut v = peek32(clk_pll2_refclk_ctrl_REG_ADDR);
    v &= !(0x1 << 24);
    v |= (0x0 & 0x1) << 24;
    poke32(clk_pll2_refclk_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_pll2_refclk_SOURCE_clk_osc_aud_() {
    let mut v = peek32(clk_pll2_refclk_ctrl_REG_ADDR);
    v &= !(0x1 << 24);
    v |= (0x1 & 0x1) << 24;
    poke32(clk_pll2_refclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_SOURCE_STATUS_clk_pll2_refclk_() -> u32 {
    let v = peek32(clk_pll2_refclk_ctrl_REG_ADDR) >> 24;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_cpu_core_() {}

pub fn _DIVIDE_CLOCK_clk_cpu_core_(div: u32) {
    let mut v = peek32(clk_cpu_core_ctrl_REG_ADDR);
    v &= !(0xF);
    v |= (div & 0xF);
    poke32(clk_cpu_core_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_cpu_core_() -> u32 {
    let v = peek32(clk_cpu_core_ctrl_REG_ADDR);
    v & 0xf
}

pub fn _ENABLE_CLOCK_clk_cpu_axi_() {}

pub fn _DIVIDE_CLOCK_clk_cpu_axi_(div: u32) {
    let mut v = peek32(clk_cpu_axi_ctrl_REG_ADDR);
    v &= !(0xF);
    v |= (div & 0xF);
    poke32(clk_cpu_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_cpu_axi_() -> u32 {
    let v = peek32(clk_cpu_axi_ctrl_REG_ADDR);
    v & 0xf
}

pub fn _ENABLE_CLOCK_clk_ahb_bus_() {}

pub fn _DIVIDE_CLOCK_clk_ahb_bus_(div: u32) {
    let mut v = peek32(clk_ahb_bus_ctrl_REG_ADDR);
    v &= !(0xF);
    v |= (div & 0xF);
    poke32(clk_ahb_bus_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_ahb_bus_() -> u32 {
    let v = peek32(clk_ahb_bus_ctrl_REG_ADDR);
    v & 0xf
}

pub fn _ENABLE_CLOCK_clk_apb1_bus_() {}

pub fn _DIVIDE_CLOCK_clk_apb1_bus_(div: u32) {
    let mut v = peek32(clk_apb1_bus_ctrl_REG_ADDR);
    v &= !(0xF);
    v |= (div & 0xF);
    poke32(clk_apb1_bus_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_apb1_bus_() -> u32 {
    let v = peek32(clk_apb1_bus_ctrl_REG_ADDR);
    v & 0xf
}

pub fn _ENABLE_CLOCK_clk_apb2_bus_() {}

pub fn _DIVIDE_CLOCK_clk_apb2_bus_(div: u32) {
    let mut v = peek32(clk_apb2_bus_ctrl_REG_ADDR);
    v &= !(0xF);
    v |= (div & 0xF);
    poke32(clk_apb2_bus_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_apb2_bus_() -> u32 {
    let v = peek32(clk_apb2_bus_ctrl_REG_ADDR);
    v & 0xf
}

pub fn _ENABLE_CLOCK_clk_dom3ahb_bus_() {
    let mut v = peek32(clk_dom3ahb_bus_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_dom3ahb_bus_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_dom3ahb_bus_() {
    let mut v = peek32(clk_dom3ahb_bus_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_dom3ahb_bus_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_dom3ahb_bus_() -> u32 {
    let v = peek32(clk_dom3ahb_bus_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_dom7ahb_bus_() {
    let mut v = peek32(clk_dom7ahb_bus_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_dom7ahb_bus_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_dom7ahb_bus_() {
    let mut v = peek32(clk_dom7ahb_bus_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_dom7ahb_bus_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_dom7ahb_bus_() -> u32 {
    let v = peek32(clk_dom7ahb_bus_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_u74_core0_() {
    let mut v = peek32(clk_u74_core0_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_u74_core0_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_u74_core0_() {
    let mut v = peek32(clk_u74_core0_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_u74_core0_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_u74_core0_() -> u32 {
    let v = peek32(clk_u74_core0_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_u74_core1_() {
    let mut v = peek32(clk_u74_core1_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_u74_core1_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_u74_core1_() {
    let mut v = peek32(clk_u74_core1_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_u74_core1_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_u74_core1_() -> u32 {
    let v = peek32(clk_u74_core1_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_u74_core1_(div: u32) {
    let mut v = peek32(clk_u74_core1_ctrl_REG_ADDR);
    v &= !(0xF);
    v |= (div & 0xF);
    poke32(clk_u74_core1_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_u74_core1_() -> u32 {
    let v = peek32(clk_u74_core1_ctrl_REG_ADDR);
    v & 0xf
}

pub fn _ENABLE_CLOCK_clk_u74_axi_() {
    let mut v = peek32(clk_u74_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_u74_axi_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_u74_axi_() {
    let mut v = peek32(clk_u74_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_u74_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_u74_axi_() -> u32 {
    let v = peek32(clk_u74_axi_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_u74rtc_toggle_() {
    let mut v = peek32(clk_u74rtc_toggle_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_u74rtc_toggle_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_u74rtc_toggle_() {
    let mut v = peek32(clk_u74rtc_toggle_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_u74rtc_toggle_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_u74rtc_toggle_() -> u32 {
    let v = peek32(clk_u74rtc_toggle_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_sgdma2p_axi_() {
    let mut v = peek32(clk_sgdma2p_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_sgdma2p_axi_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_sgdma2p_axi_() {
    let mut v = peek32(clk_sgdma2p_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_sgdma2p_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_sgdma2p_axi_() -> u32 {
    let v = peek32(clk_sgdma2p_axi_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_dma2pnoc_axi_() {
    let mut v = peek32(clk_dma2pnoc_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_dma2pnoc_axi_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_dma2pnoc_axi_() {
    let mut v = peek32(clk_dma2pnoc_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_dma2pnoc_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_dma2pnoc_axi_() -> u32 {
    let v = peek32(clk_dma2pnoc_axi_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_sgdma2p_ahb_() {
    let mut v = peek32(clk_sgdma2p_ahb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_sgdma2p_ahb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_sgdma2p_ahb_() {
    let mut v = peek32(clk_sgdma2p_ahb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_sgdma2p_ahb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_sgdma2p_ahb_() -> u32 {
    let v = peek32(clk_sgdma2p_ahb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_dla_bus_() {}

pub fn _DIVIDE_CLOCK_clk_dla_bus_(div: u32) {
    let mut v = peek32(clk_dla_bus_ctrl_REG_ADDR);
    v &= !(0x7);
    v |= (div & 0x7);
    poke32(clk_dla_bus_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_dla_bus_() -> u32 {
    let v = peek32(clk_dla_bus_ctrl_REG_ADDR);
    v & 0x7
}

pub fn _ENABLE_CLOCK_clk_dla_axi_() {
    let mut v = peek32(clk_dla_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_dla_axi_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_dla_axi_() {
    let mut v = peek32(clk_dla_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_dla_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_dla_axi_() -> u32 {
    let v = peek32(clk_dla_axi_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_dlanoc_axi_() {
    let mut v = peek32(clk_dlanoc_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_dlanoc_axi_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_dlanoc_axi_() {
    let mut v = peek32(clk_dlanoc_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_dlanoc_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_dlanoc_axi_() -> u32 {
    let v = peek32(clk_dlanoc_axi_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_dla_apb_() {
    let mut v = peek32(clk_dla_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_dla_apb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_dla_apb_() {
    let mut v = peek32(clk_dla_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_dla_apb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_dla_apb_() -> u32 {
    let v = peek32(clk_dla_apb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_vp6_core_() {
    let mut v = peek32(clk_vp6_core_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_vp6_core_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_vp6_core_() {
    let mut v = peek32(clk_vp6_core_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_vp6_core_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_vp6_core_() -> u32 {
    let v = peek32(clk_vp6_core_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_vp6_core_(div: u32) {
    let mut v = peek32(clk_vp6_core_ctrl_REG_ADDR);
    v &= !(0x7);
    v |= (div & 0x7);
    poke32(clk_vp6_core_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_vp6_core_() -> u32 {
    let v = peek32(clk_vp6_core_ctrl_REG_ADDR);
    v & 0x7
}

pub fn _ENABLE_CLOCK_clk_vp6bus_src_() {}

pub fn _DIVIDE_CLOCK_clk_vp6bus_src_(div: u32) {
    let mut v = peek32(clk_vp6bus_src_ctrl_REG_ADDR);
    v &= !(0x7);
    v |= (div & 0x7);
    poke32(clk_vp6bus_src_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_vp6bus_src_() -> u32 {
    let v = peek32(clk_vp6bus_src_ctrl_REG_ADDR);
    v & 0x7
}

pub fn _ENABLE_CLOCK_clk_vp6_axi_() {
    let mut v = peek32(clk_vp6_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_vp6_axi_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_vp6_axi_() {
    let mut v = peek32(clk_vp6_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_vp6_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_vp6_axi_() -> u32 {
    let v = peek32(clk_vp6_axi_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_vp6_axi_(div: u32) {
    let mut v = peek32(clk_vp6_axi_ctrl_REG_ADDR);
    v &= !(0x7);
    v |= (div & 0x7);
    poke32(clk_vp6_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_vp6_axi_() -> u32 {
    let v = peek32(clk_vp6_axi_ctrl_REG_ADDR);
    v & 0x7
}

pub fn _ENABLE_CLOCK_clk_vcdecbus_src_() {}

pub fn _DIVIDE_CLOCK_clk_vcdecbus_src_(div: u32) {
    let mut v = peek32(clk_vcdecbus_src_ctrl_REG_ADDR);
    v &= !(0x7);
    v |= (div & 0x7);
    poke32(clk_vcdecbus_src_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_vcdecbus_src_() -> u32 {
    let v = peek32(clk_vcdecbus_src_ctrl_REG_ADDR);
    v & 0x7
}

pub fn _ENABLE_CLOCK_clk_vdec_bus_() {}

pub fn _DIVIDE_CLOCK_clk_vdec_bus_(div: u32) {
    let mut v = peek32(clk_vdec_bus_ctrl_REG_ADDR);
    v &= !(0xF);
    v |= (div & 0xF);
    poke32(clk_vdec_bus_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_vdec_bus_() -> u32 {
    let v = peek32(clk_vdec_bus_ctrl_REG_ADDR);
    v & 0xf
}

pub fn _ENABLE_CLOCK_clk_vdec_axi_() {
    let mut v = peek32(clk_vdec_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_vdec_axi_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_vdec_axi_() {
    let mut v = peek32(clk_vdec_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_vdec_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_vdec_axi_() -> u32 {
    let v = peek32(clk_vdec_axi_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_vdecbrg_mainclk_() {
    let mut v = peek32(clk_vdecbrg_mainclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_vdecbrg_mainclk_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_vdecbrg_mainclk_() {
    let mut v = peek32(clk_vdecbrg_mainclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_vdecbrg_mainclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_vdecbrg_mainclk_() -> u32 {
    let v = peek32(clk_vdecbrg_mainclk_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_vdec_bclk_() {
    let mut v = peek32(clk_vdec_bclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_vdec_bclk_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_vdec_bclk_() {
    let mut v = peek32(clk_vdec_bclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_vdec_bclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_vdec_bclk_() -> u32 {
    let v = peek32(clk_vdec_bclk_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_vdec_bclk_(div: u32) {
    let mut v = peek32(clk_vdec_bclk_ctrl_REG_ADDR);
    v &= !(0xF);
    v |= (div & 0xF);
    poke32(clk_vdec_bclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_vdec_bclk_() -> u32 {
    let v = peek32(clk_vdec_bclk_ctrl_REG_ADDR);
    v & 0xf
}

pub fn _ENABLE_CLOCK_clk_vdec_cclk_() {
    let mut v = peek32(clk_vdec_cclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_vdec_cclk_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_vdec_cclk_() {
    let mut v = peek32(clk_vdec_cclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_vdec_cclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_vdec_cclk_() -> u32 {
    let v = peek32(clk_vdec_cclk_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_vdec_cclk_(div: u32) {
    let mut v = peek32(clk_vdec_cclk_ctrl_REG_ADDR);
    v &= !(0xF);
    v |= (div & 0xF);
    poke32(clk_vdec_cclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_vdec_cclk_() -> u32 {
    let v = peek32(clk_vdec_cclk_ctrl_REG_ADDR);
    v & 0xf
}

pub fn _ENABLE_CLOCK_clk_vdec_apb_() {
    let mut v = peek32(clk_vdec_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_vdec_apb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_vdec_apb_() {
    let mut v = peek32(clk_vdec_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_vdec_apb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_vdec_apb_() -> u32 {
    let v = peek32(clk_vdec_apb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_jpeg_axi_() {
    let mut v = peek32(clk_jpeg_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_jpeg_axi_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_jpeg_axi_() {
    let mut v = peek32(clk_jpeg_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_jpeg_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_jpeg_axi_() -> u32 {
    let v = peek32(clk_jpeg_axi_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_jpeg_axi_(div: u32) {
    let mut v = peek32(clk_jpeg_axi_ctrl_REG_ADDR);
    v &= !(0xF);
    v |= (div & 0xF);
    poke32(clk_jpeg_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_jpeg_axi_() -> u32 {
    let v = peek32(clk_jpeg_axi_ctrl_REG_ADDR);
    v & 0xf
}

pub fn _ENABLE_CLOCK_clk_jpeg_cclk_() {
    let mut v = peek32(clk_jpeg_cclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_jpeg_cclk_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_jpeg_cclk_() {
    let mut v = peek32(clk_jpeg_cclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_jpeg_cclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_jpeg_cclk_() -> u32 {
    let v = peek32(clk_jpeg_cclk_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_jpeg_cclk_(div: u32) {
    let mut v = peek32(clk_jpeg_cclk_ctrl_REG_ADDR);
    v &= !(0xF);
    v |= (div & 0xF);
    poke32(clk_jpeg_cclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_jpeg_cclk_() -> u32 {
    let v = peek32(clk_jpeg_cclk_ctrl_REG_ADDR);
    v & 0xf
}

pub fn _ENABLE_CLOCK_clk_jpeg_apb_() {
    let mut v = peek32(clk_jpeg_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_jpeg_apb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_jpeg_apb_() {
    let mut v = peek32(clk_jpeg_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_jpeg_apb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_jpeg_apb_() -> u32 {
    let v = peek32(clk_jpeg_apb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_gc300_2x_() {
    let mut v = peek32(clk_gc300_2x_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_gc300_2x_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_gc300_2x_() {
    let mut v = peek32(clk_gc300_2x_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_gc300_2x_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_gc300_2x_() -> u32 {
    let v = peek32(clk_gc300_2x_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_gc300_2x_(div: u32) {
    let mut v = peek32(clk_gc300_2x_ctrl_REG_ADDR);
    v &= !(0xF);
    v |= (div & 0xF);
    poke32(clk_gc300_2x_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_gc300_2x_() -> u32 {
    let v = peek32(clk_gc300_2x_ctrl_REG_ADDR);
    v & 0xf
}

pub fn _ENABLE_CLOCK_clk_gc300_ahb_() {
    let mut v = peek32(clk_gc300_ahb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_gc300_ahb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_gc300_ahb_() {
    let mut v = peek32(clk_gc300_ahb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_gc300_ahb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_gc300_ahb_() -> u32 {
    let v = peek32(clk_gc300_ahb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_jpcgc300_axibus_() {}

pub fn _DIVIDE_CLOCK_clk_jpcgc300_axibus_(div: u32) {
    let mut v = peek32(clk_jpcgc300_axibus_ctrl_REG_ADDR);
    v &= !(0xF);
    v |= (div & 0xF);
    poke32(clk_jpcgc300_axibus_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_jpcgc300_axibus_() -> u32 {
    let v = peek32(clk_jpcgc300_axibus_ctrl_REG_ADDR);
    v & 0xf
}

pub fn _ENABLE_CLOCK_clk_gc300_axi_() {
    let mut v = peek32(clk_gc300_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_gc300_axi_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_gc300_axi_() {
    let mut v = peek32(clk_gc300_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_gc300_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_gc300_axi_() -> u32 {
    let v = peek32(clk_gc300_axi_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_jpcgc300_mainclk_() {
    let mut v = peek32(clk_jpcgc300_mainclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_jpcgc300_mainclk_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_jpcgc300_mainclk_() {
    let mut v = peek32(clk_jpcgc300_mainclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_jpcgc300_mainclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_jpcgc300_mainclk_() -> u32 {
    let v = peek32(clk_jpcgc300_mainclk_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_venc_bus_() {}

pub fn _DIVIDE_CLOCK_clk_venc_bus_(div: u32) {
    let mut v = peek32(clk_venc_bus_ctrl_REG_ADDR);
    v &= !(0xF);
    v |= (div & 0xF);
    poke32(clk_venc_bus_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_venc_bus_() -> u32 {
    let v = peek32(clk_venc_bus_ctrl_REG_ADDR);
    v & 0xf
}

pub fn _ENABLE_CLOCK_clk_venc_axi_() {
    let mut v = peek32(clk_venc_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_venc_axi_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_venc_axi_() {
    let mut v = peek32(clk_venc_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_venc_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_venc_axi_() -> u32 {
    let v = peek32(clk_venc_axi_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_vencbrg_mainclk_() {
    let mut v = peek32(clk_vencbrg_mainclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_vencbrg_mainclk_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_vencbrg_mainclk_() {
    let mut v = peek32(clk_vencbrg_mainclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_vencbrg_mainclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_vencbrg_mainclk_() -> u32 {
    let v = peek32(clk_vencbrg_mainclk_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_venc_bclk_() {
    let mut v = peek32(clk_venc_bclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_venc_bclk_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_venc_bclk_() {
    let mut v = peek32(clk_venc_bclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_venc_bclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_venc_bclk_() -> u32 {
    let v = peek32(clk_venc_bclk_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_venc_bclk_(div: u32) {
    let mut v = peek32(clk_venc_bclk_ctrl_REG_ADDR);
    v &= !(0xF);
    v |= (div & 0xF);
    poke32(clk_venc_bclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_venc_bclk_() -> u32 {
    let v = peek32(clk_venc_bclk_ctrl_REG_ADDR);
    v & 0xf
}

pub fn _ENABLE_CLOCK_clk_venc_cclk_() {
    let mut v = peek32(clk_venc_cclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_venc_cclk_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_venc_cclk_() {
    let mut v = peek32(clk_venc_cclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_venc_cclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_venc_cclk_() -> u32 {
    let v = peek32(clk_venc_cclk_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_venc_cclk_(div: u32) {
    let mut v = peek32(clk_venc_cclk_ctrl_REG_ADDR);
    v &= !(0xF);
    v |= (div & 0xF);
    poke32(clk_venc_cclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_venc_cclk_() -> u32 {
    let v = peek32(clk_venc_cclk_ctrl_REG_ADDR);
    v & 0xf
}

pub fn _ENABLE_CLOCK_clk_venc_apb_() {
    let mut v = peek32(clk_venc_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_venc_apb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_venc_apb_() {
    let mut v = peek32(clk_venc_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_venc_apb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_venc_apb_() -> u32 {
    let v = peek32(clk_venc_apb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_ddrpll_div2_() {
    let mut v = peek32(clk_ddrpll_div2_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_ddrpll_div2_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_ddrpll_div2_() {
    let mut v = peek32(clk_ddrpll_div2_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_ddrpll_div2_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_ddrpll_div2_() -> u32 {
    let v = peek32(clk_ddrpll_div2_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_ddrpll_div2_(div: u32) {
    let mut v = peek32(clk_ddrpll_div2_ctrl_REG_ADDR);
    v &= !(0x3);
    v |= (div & 0x3);
    poke32(clk_ddrpll_div2_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_ddrpll_div2_() -> u32 {
    let v = peek32(clk_ddrpll_div2_ctrl_REG_ADDR);
    v & 0x3
}

pub fn _ENABLE_CLOCK_clk_ddrpll_div4_() {
    let mut v = peek32(clk_ddrpll_div4_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_ddrpll_div4_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_ddrpll_div4_() {
    let mut v = peek32(clk_ddrpll_div4_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_ddrpll_div4_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_ddrpll_div4_() -> u32 {
    let v = peek32(clk_ddrpll_div4_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_ddrpll_div4_(div: u32) {
    let mut v = peek32(clk_ddrpll_div4_ctrl_REG_ADDR);
    v &= !(0x3);
    v |= (div & 0x3);
    poke32(clk_ddrpll_div4_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_ddrpll_div4_() -> u32 {
    let v = peek32(clk_ddrpll_div4_ctrl_REG_ADDR);
    v & 0x3
}

pub fn _ENABLE_CLOCK_clk_ddrpll_div8_() {
    let mut v = peek32(clk_ddrpll_div8_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_ddrpll_div8_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_ddrpll_div8_() {
    let mut v = peek32(clk_ddrpll_div8_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_ddrpll_div8_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_ddrpll_div8_() -> u32 {
    let v = peek32(clk_ddrpll_div8_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_ddrpll_div8_(div: u32) {
    let mut v = peek32(clk_ddrpll_div8_ctrl_REG_ADDR);
    v &= !(0x3);
    v |= (div & 0x3);
    poke32(clk_ddrpll_div8_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_ddrpll_div8_() -> u32 {
    let v = peek32(clk_ddrpll_div8_ctrl_REG_ADDR);
    v & 0x3
}

pub fn _ENABLE_CLOCK_clk_ddrosc_div2_() {
    let mut v = peek32(clk_ddrosc_div2_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_ddrosc_div2_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_ddrosc_div2_() {
    let mut v = peek32(clk_ddrosc_div2_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_ddrosc_div2_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_ddrosc_div2_() -> u32 {
    let v = peek32(clk_ddrosc_div2_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_ddrosc_div2_(div: u32) {
    let mut v = peek32(clk_ddrosc_div2_ctrl_REG_ADDR);
    v &= !(0x3);
    v |= (div & 0x3);
    poke32(clk_ddrosc_div2_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_ddrosc_div2_() -> u32 {
    let v = peek32(clk_ddrosc_div2_ctrl_REG_ADDR);
    v & 0x3
}

pub fn _ENABLE_CLOCK_clk_ddrc0_() {
    let mut v = peek32(clk_ddrc0_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_ddrc0_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_ddrc0_() {
    let mut v = peek32(clk_ddrc0_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_ddrc0_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_ddrc0_() -> u32 {
    let v = peek32(clk_ddrc0_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _SWITCH_CLOCK_clk_ddrc0_SOURCE_clk_ddrosc_div2_() {
    let mut v = peek32(clk_ddrc0_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x0 & 0x3) << 24;
    poke32(clk_ddrc0_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_ddrc0_SOURCE_clk_ddrpll_div2_() {
    let mut v = peek32(clk_ddrc0_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x1 & 0x3) << 24;
    poke32(clk_ddrc0_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_ddrc0_SOURCE_clk_ddrpll_div4_() {
    let mut v = peek32(clk_ddrc0_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x2 & 0x3) << 24;
    poke32(clk_ddrc0_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_ddrc0_SOURCE_clk_ddrpll_div8_() {
    let mut v = peek32(clk_ddrc0_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x3 & 0x3) << 24;
    poke32(clk_ddrc0_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_SOURCE_STATUS_clk_ddrc0_() -> u32 {
    let v = peek32(clk_ddrc0_ctrl_REG_ADDR) >> 24;
    v & 0x3
}

pub fn _ENABLE_CLOCK_clk_ddrc1_() {
    let mut v = peek32(clk_ddrc1_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_ddrc1_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_ddrc1_() {
    let mut v = peek32(clk_ddrc1_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_ddrc1_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_ddrc1_() -> u32 {
    let v = peek32(clk_ddrc1_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _SWITCH_CLOCK_clk_ddrc1_SOURCE_clk_ddrosc_div2_() {
    let mut v = peek32(clk_ddrc1_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x0 & 0x3) << 24;
    poke32(clk_ddrc1_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_ddrc1_SOURCE_clk_ddrpll_div2_() {
    let mut v = peek32(clk_ddrc1_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x1 & 0x3) << 24;
    poke32(clk_ddrc1_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_ddrc1_SOURCE_clk_ddrpll_div4_() {
    let mut v = peek32(clk_ddrc1_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x2 & 0x3) << 24;
    poke32(clk_ddrc1_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_ddrc1_SOURCE_clk_ddrpll_div8_() {
    let mut v = peek32(clk_ddrc1_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x3 & 0x3) << 24;
    poke32(clk_ddrc1_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_SOURCE_STATUS_clk_ddrc1_() -> u32 {
    let v = peek32(clk_ddrc1_ctrl_REG_ADDR) >> 24;
    v & 0x3
}

pub fn _ENABLE_CLOCK_clk_ddrphy_apb_() {
    let mut v = peek32(clk_ddrphy_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_ddrphy_apb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_ddrphy_apb_() {
    let mut v = peek32(clk_ddrphy_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_ddrphy_apb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_ddrphy_apb_() -> u32 {
    let v = peek32(clk_ddrphy_apb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_noc_rob_() {}

pub fn _DIVIDE_CLOCK_clk_noc_rob_(div: u32) {
    let mut v = peek32(clk_noc_rob_ctrl_REG_ADDR);
    v &= !(0xF);
    v |= (div & 0xF);
    poke32(clk_noc_rob_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_noc_rob_() -> u32 {
    let v = peek32(clk_noc_rob_ctrl_REG_ADDR);
    v & 0xf
}

pub fn _ENABLE_CLOCK_clk_noc_cog_() {}

pub fn _DIVIDE_CLOCK_clk_noc_cog_(div: u32) {
    let mut v = peek32(clk_noc_cog_ctrl_REG_ADDR);
    v &= !(0xF);
    v |= (div & 0xF);
    poke32(clk_noc_cog_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_noc_cog_() -> u32 {
    let v = peek32(clk_noc_cog_ctrl_REG_ADDR);
    v & 0xf
}

pub fn _ENABLE_CLOCK_clk_nne_ahb_() {
    let mut v = peek32(clk_nne_ahb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_nne_ahb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_nne_ahb_() {
    let mut v = peek32(clk_nne_ahb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_nne_ahb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_nne_ahb_() -> u32 {
    let v = peek32(clk_nne_ahb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_nnebus_src1_() {}

pub fn _DIVIDE_CLOCK_clk_nnebus_src1_(div: u32) {
    let mut v = peek32(clk_nnebus_src1_ctrl_REG_ADDR);
    v &= !(0x7);
    v |= (div & 0x7);
    poke32(clk_nnebus_src1_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_nnebus_src1_() -> u32 {
    let v = peek32(clk_nnebus_src1_ctrl_REG_ADDR);
    v & 0x7
}

pub fn _ENABLE_CLOCK_clk_nne_bus_() {}

pub fn _SWITCH_CLOCK_clk_nne_bus_SOURCE_clk_cpu_axi_() {
    let mut v = peek32(clk_nne_bus_ctrl_REG_ADDR);
    v &= !(0x1 << 24);
    v |= (0x0 & 0x1) << 24;
    poke32(clk_nne_bus_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_nne_bus_SOURCE_clk_nnebus_src1_() {
    let mut v = peek32(clk_nne_bus_ctrl_REG_ADDR);
    v &= !(0x1 << 24);
    v |= (0x1 & 0x1) << 24;
    poke32(clk_nne_bus_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_SOURCE_STATUS_clk_nne_bus_() -> u32 {
    let v = peek32(clk_nne_bus_ctrl_REG_ADDR) >> 24;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_nne_axi_() {
    let mut v = peek32(clk_nne_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_nne_axi_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_nne_axi_() {
    let mut v = peek32(clk_nne_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_nne_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_nne_axi_() -> u32 {
    let v = peek32(clk_nne_axi_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_nnenoc_axi_() {
    let mut v = peek32(clk_nnenoc_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_nnenoc_axi_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_nnenoc_axi_() {
    let mut v = peek32(clk_nnenoc_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_nnenoc_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_nnenoc_axi_() -> u32 {
    let v = peek32(clk_nnenoc_axi_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_dlaslv_axi_() {
    let mut v = peek32(clk_dlaslv_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_dlaslv_axi_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_dlaslv_axi_() {
    let mut v = peek32(clk_dlaslv_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_dlaslv_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_dlaslv_axi_() -> u32 {
    let v = peek32(clk_dlaslv_axi_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_dspx2c_axi_() {
    let mut v = peek32(clk_dspx2c_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_dspx2c_axi_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_dspx2c_axi_() {
    let mut v = peek32(clk_dspx2c_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_dspx2c_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_dspx2c_axi_() -> u32 {
    let v = peek32(clk_dspx2c_axi_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_hifi4_src_() {}

pub fn _DIVIDE_CLOCK_clk_hifi4_src_(div: u32) {
    let mut v = peek32(clk_hifi4_src_ctrl_REG_ADDR);
    v &= !(0x7);
    v |= (div & 0x7);
    poke32(clk_hifi4_src_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_hifi4_src_() -> u32 {
    let v = peek32(clk_hifi4_src_ctrl_REG_ADDR);
    v & 0x7
}

pub fn _ENABLE_CLOCK_clk_hifi4_corefree_() {}

pub fn _DIVIDE_CLOCK_clk_hifi4_corefree_(div: u32) {
    let mut v = peek32(clk_hifi4_corefree_ctrl_REG_ADDR);
    v &= !(0xF);
    v |= (div & 0xF);
    poke32(clk_hifi4_corefree_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_hifi4_corefree_() -> u32 {
    let v = peek32(clk_hifi4_corefree_ctrl_REG_ADDR);
    v & 0xf
}

pub fn _ENABLE_CLOCK_clk_hifi4_core_() {
    let mut v = peek32(clk_hifi4_core_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_hifi4_core_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_hifi4_core_() {
    let mut v = peek32(clk_hifi4_core_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_hifi4_core_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_hifi4_core_() -> u32 {
    let v = peek32(clk_hifi4_core_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_hifi4_bus_() {}

pub fn _DIVIDE_CLOCK_clk_hifi4_bus_(div: u32) {
    let mut v = peek32(clk_hifi4_bus_ctrl_REG_ADDR);
    v &= !(0xF);
    v |= (div & 0xF);
    poke32(clk_hifi4_bus_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_hifi4_bus_() -> u32 {
    let v = peek32(clk_hifi4_bus_ctrl_REG_ADDR);
    v & 0xf
}

pub fn _ENABLE_CLOCK_clk_hifi4_axi_() {
    let mut v = peek32(clk_hifi4_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_hifi4_axi_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_hifi4_axi_() {
    let mut v = peek32(clk_hifi4_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_hifi4_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_hifi4_axi_() -> u32 {
    let v = peek32(clk_hifi4_axi_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_hifi4noc_axi_() {
    let mut v = peek32(clk_hifi4noc_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_hifi4noc_axi_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_hifi4noc_axi_() {
    let mut v = peek32(clk_hifi4noc_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_hifi4noc_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_hifi4noc_axi_() -> u32 {
    let v = peek32(clk_hifi4noc_axi_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_sgdma1p_bus_() {}

pub fn _DIVIDE_CLOCK_clk_sgdma1p_bus_(div: u32) {
    let mut v = peek32(clk_sgdma1p_bus_ctrl_REG_ADDR);
    v &= !(0xF);
    v |= (div & 0xF);
    poke32(clk_sgdma1p_bus_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_sgdma1p_bus_() -> u32 {
    let v = peek32(clk_sgdma1p_bus_ctrl_REG_ADDR);
    v & 0xf
}

pub fn _ENABLE_CLOCK_clk_sgdma1p_axi_() {
    let mut v = peek32(clk_sgdma1p_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_sgdma1p_axi_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_sgdma1p_axi_() {
    let mut v = peek32(clk_sgdma1p_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_sgdma1p_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_sgdma1p_axi_() -> u32 {
    let v = peek32(clk_sgdma1p_axi_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_dma1p_axi_() {
    let mut v = peek32(clk_dma1p_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_dma1p_axi_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_dma1p_axi_() {
    let mut v = peek32(clk_dma1p_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_dma1p_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_dma1p_axi_() -> u32 {
    let v = peek32(clk_dma1p_axi_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_x2c_axi_() {
    let mut v = peek32(clk_x2c_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_x2c_axi_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_x2c_axi_() {
    let mut v = peek32(clk_x2c_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_x2c_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_x2c_axi_() -> u32 {
    let v = peek32(clk_x2c_axi_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_x2c_axi_(div: u32) {
    let mut v = peek32(clk_x2c_axi_ctrl_REG_ADDR);
    v &= !(0xF);
    v |= (div & 0xF);
    poke32(clk_x2c_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_x2c_axi_() -> u32 {
    let v = peek32(clk_x2c_axi_ctrl_REG_ADDR);
    v & 0xf
}

pub fn _ENABLE_CLOCK_clk_usb_bus_() {}

pub fn _DIVIDE_CLOCK_clk_usb_bus_(div: u32) {
    let mut v = peek32(clk_usb_bus_ctrl_REG_ADDR);
    v &= !(0xF);
    v |= (div & 0xF);
    poke32(clk_usb_bus_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_usb_bus_() -> u32 {
    let v = peek32(clk_usb_bus_ctrl_REG_ADDR);
    v & 0xf
}

pub fn _ENABLE_CLOCK_clk_usb_axi_() {
    let mut v = peek32(clk_usb_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_usb_axi_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_usb_axi_() {
    let mut v = peek32(clk_usb_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_usb_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_usb_axi_() -> u32 {
    let v = peek32(clk_usb_axi_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_usbnoc_axi_() {
    let mut v = peek32(clk_usbnoc_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_usbnoc_axi_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_usbnoc_axi_() {
    let mut v = peek32(clk_usbnoc_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_usbnoc_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_usbnoc_axi_() -> u32 {
    let v = peek32(clk_usbnoc_axi_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_usbphy_rootdiv_() {}

pub fn _DIVIDE_CLOCK_clk_usbphy_rootdiv_(div: u32) {
    let mut v = peek32(clk_usbphy_rootdiv_ctrl_REG_ADDR);
    v &= !(0x7);
    v |= (div & 0x7);
    poke32(clk_usbphy_rootdiv_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_usbphy_rootdiv_() -> u32 {
    let v = peek32(clk_usbphy_rootdiv_ctrl_REG_ADDR);
    v & 0x7
}

pub fn _ENABLE_CLOCK_clk_usbphy_125m_() {
    let mut v = peek32(clk_usbphy_125m_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_usbphy_125m_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_usbphy_125m_() {
    let mut v = peek32(clk_usbphy_125m_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_usbphy_125m_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_usbphy_125m_() -> u32 {
    let v = peek32(clk_usbphy_125m_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_usbphy_125m_(div: u32) {
    let mut v = peek32(clk_usbphy_125m_ctrl_REG_ADDR);
    v &= !(0xF);
    v |= (div & 0xF);
    poke32(clk_usbphy_125m_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_usbphy_125m_() -> u32 {
    let v = peek32(clk_usbphy_125m_ctrl_REG_ADDR);
    v & 0xf
}

pub fn _ENABLE_CLOCK_clk_usbphy_plldiv25m_() {
    let mut v = peek32(clk_usbphy_plldiv25m_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_usbphy_plldiv25m_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_usbphy_plldiv25m_() {
    let mut v = peek32(clk_usbphy_plldiv25m_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_usbphy_plldiv25m_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_usbphy_plldiv25m_() -> u32 {
    let v = peek32(clk_usbphy_plldiv25m_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_usbphy_plldiv25m_(div: u32) {
    let mut v = peek32(clk_usbphy_plldiv25m_ctrl_REG_ADDR);
    v &= !(0x3F);
    v |= (div & 0x3F);
    poke32(clk_usbphy_plldiv25m_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_usbphy_plldiv25m_() -> u32 {
    let v = peek32(clk_usbphy_plldiv25m_ctrl_REG_ADDR);
    v & 0x3f
}

pub fn _ENABLE_CLOCK_clk_usbphy_25m_() {}

pub fn _SWITCH_CLOCK_clk_usbphy_25m_SOURCE_clk_osc_sys_() {
    let mut v = peek32(clk_usbphy_25m_ctrl_REG_ADDR);
    v &= !(0x1 << 24);
    v |= (0x0 & 0x1) << 24;
    poke32(clk_usbphy_25m_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_usbphy_25m_SOURCE_clk_usbphy_plldiv25m_() {
    let mut v = peek32(clk_usbphy_25m_ctrl_REG_ADDR);
    v &= !(0x1 << 24);
    v |= (0x1 & 0x1) << 24;
    poke32(clk_usbphy_25m_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_SOURCE_STATUS_clk_usbphy_25m_() -> u32 {
    let v = peek32(clk_usbphy_25m_ctrl_REG_ADDR) >> 24;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_audio_div_() {}

pub fn _DIVIDE_CLOCK_clk_audio_div_(div: u32) {
    let mut v = peek32(clk_audio_div_ctrl_REG_ADDR);
    v &= !(0x3FFFF);
    v |= (div & 0x3FFFF);
    poke32(clk_audio_div_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_audio_div_() -> u32 {
    let v = peek32(clk_audio_div_ctrl_REG_ADDR);
    v & 0x3ffff
}

pub fn _ENABLE_CLOCK_clk_audio_src_() {
    let mut v = peek32(clk_audio_src_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_audio_src_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_audio_src_() {
    let mut v = peek32(clk_audio_src_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_audio_src_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_audio_src_() -> u32 {
    let v = peek32(clk_audio_src_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_audio_12288_() {
    let mut v = peek32(clk_audio_12288_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_audio_12288_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_audio_12288_() {
    let mut v = peek32(clk_audio_12288_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_audio_12288_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_audio_12288_() -> u32 {
    let v = peek32(clk_audio_12288_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_vin_src_() {
    let mut v = peek32(clk_vin_src_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_vin_src_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_vin_src_() {
    let mut v = peek32(clk_vin_src_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_vin_src_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_vin_src_() -> u32 {
    let v = peek32(clk_vin_src_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_vin_src_(div: u32) {
    let mut v = peek32(clk_vin_src_ctrl_REG_ADDR);
    v &= !(0x7);
    v |= (div & 0x7);
    poke32(clk_vin_src_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_vin_src_() -> u32 {
    let v = peek32(clk_vin_src_ctrl_REG_ADDR);
    v & 0x7
}

pub fn _ENABLE_CLOCK_clk_isp0_bus_() {}

pub fn _DIVIDE_CLOCK_clk_isp0_bus_(div: u32) {
    let mut v = peek32(clk_isp0_bus_ctrl_REG_ADDR);
    v &= !(0xF);
    v |= (div & 0xF);
    poke32(clk_isp0_bus_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_isp0_bus_() -> u32 {
    let v = peek32(clk_isp0_bus_ctrl_REG_ADDR);
    v & 0xf
}

pub fn _ENABLE_CLOCK_clk_isp0_axi_() {
    let mut v = peek32(clk_isp0_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_isp0_axi_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_isp0_axi_() {
    let mut v = peek32(clk_isp0_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_isp0_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_isp0_axi_() -> u32 {
    let v = peek32(clk_isp0_axi_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_isp0noc_axi_() {
    let mut v = peek32(clk_isp0noc_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_isp0noc_axi_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_isp0noc_axi_() {
    let mut v = peek32(clk_isp0noc_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_isp0noc_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_isp0noc_axi_() -> u32 {
    let v = peek32(clk_isp0noc_axi_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_ispslv_axi_() {
    let mut v = peek32(clk_ispslv_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_ispslv_axi_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_ispslv_axi_() {
    let mut v = peek32(clk_ispslv_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_ispslv_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_ispslv_axi_() -> u32 {
    let v = peek32(clk_ispslv_axi_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_isp1_bus_() {}

pub fn _DIVIDE_CLOCK_clk_isp1_bus_(div: u32) {
    let mut v = peek32(clk_isp1_bus_ctrl_REG_ADDR);
    v &= !(0xF);
    v |= (div & 0xF);
    poke32(clk_isp1_bus_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_isp1_bus_() -> u32 {
    let v = peek32(clk_isp1_bus_ctrl_REG_ADDR);
    v & 0xf
}

pub fn _ENABLE_CLOCK_clk_isp1_axi_() {
    let mut v = peek32(clk_isp1_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_isp1_axi_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_isp1_axi_() {
    let mut v = peek32(clk_isp1_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_isp1_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_isp1_axi_() -> u32 {
    let v = peek32(clk_isp1_axi_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_isp1noc_axi_() {
    let mut v = peek32(clk_isp1noc_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_isp1noc_axi_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_isp1noc_axi_() {
    let mut v = peek32(clk_isp1noc_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_isp1noc_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_isp1noc_axi_() -> u32 {
    let v = peek32(clk_isp1noc_axi_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_vin_bus_() {}

pub fn _DIVIDE_CLOCK_clk_vin_bus_(div: u32) {
    let mut v = peek32(clk_vin_bus_ctrl_REG_ADDR);
    v &= !(0xF);
    v |= (div & 0xF);
    poke32(clk_vin_bus_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_vin_bus_() -> u32 {
    let v = peek32(clk_vin_bus_ctrl_REG_ADDR);
    v & 0xf
}

pub fn _ENABLE_CLOCK_clk_vin_axi_() {
    let mut v = peek32(clk_vin_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_vin_axi_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_vin_axi_() {
    let mut v = peek32(clk_vin_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_vin_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_vin_axi_() -> u32 {
    let v = peek32(clk_vin_axi_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_vinnoc_axi_() {
    let mut v = peek32(clk_vinnoc_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_vinnoc_axi_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_vinnoc_axi_() {
    let mut v = peek32(clk_vinnoc_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_vinnoc_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_vinnoc_axi_() -> u32 {
    let v = peek32(clk_vinnoc_axi_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_vout_src_() {
    let mut v = peek32(clk_vout_src_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_vout_src_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_vout_src_() {
    let mut v = peek32(clk_vout_src_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_vout_src_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_vout_src_() -> u32 {
    let v = peek32(clk_vout_src_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_vout_src_(div: u32) {
    let mut v = peek32(clk_vout_src_ctrl_REG_ADDR);
    v &= !(0x7);
    v |= (div & 0x7);
    poke32(clk_vout_src_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_vout_src_() -> u32 {
    let v = peek32(clk_vout_src_ctrl_REG_ADDR);
    v & 0x7
}

pub fn _ENABLE_CLOCK_clk_dispbus_src_() {}

pub fn _DIVIDE_CLOCK_clk_dispbus_src_(div: u32) {
    let mut v = peek32(clk_dispbus_src_ctrl_REG_ADDR);
    v &= !(0x7);
    v |= (div & 0x7);
    poke32(clk_dispbus_src_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_dispbus_src_() -> u32 {
    let v = peek32(clk_dispbus_src_ctrl_REG_ADDR);
    v & 0x7
}

pub fn _ENABLE_CLOCK_clk_disp_bus_() {}

pub fn _DIVIDE_CLOCK_clk_disp_bus_(div: u32) {
    let mut v = peek32(clk_disp_bus_ctrl_REG_ADDR);
    v &= !(0x7);
    v |= (div & 0x7);
    poke32(clk_disp_bus_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_disp_bus_() -> u32 {
    let v = peek32(clk_disp_bus_ctrl_REG_ADDR);
    v & 0x7
}

pub fn _ENABLE_CLOCK_clk_disp_axi_() {
    let mut v = peek32(clk_disp_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_disp_axi_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_disp_axi_() {
    let mut v = peek32(clk_disp_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_disp_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_disp_axi_() -> u32 {
    let v = peek32(clk_disp_axi_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_dispnoc_axi_() {
    let mut v = peek32(clk_dispnoc_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_dispnoc_axi_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_dispnoc_axi_() {
    let mut v = peek32(clk_dispnoc_axi_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_dispnoc_axi_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_dispnoc_axi_() -> u32 {
    let v = peek32(clk_dispnoc_axi_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_sdio0_ahb_() {
    let mut v = peek32(clk_sdio0_ahb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_sdio0_ahb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_sdio0_ahb_() {
    let mut v = peek32(clk_sdio0_ahb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_sdio0_ahb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_sdio0_ahb_() -> u32 {
    let v = peek32(clk_sdio0_ahb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_sdio0_cclkint_() {
    let mut v = peek32(clk_sdio0_cclkint_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_sdio0_cclkint_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_sdio0_cclkint_() {
    let mut v = peek32(clk_sdio0_cclkint_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_sdio0_cclkint_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_sdio0_cclkint_() -> u32 {
    let v = peek32(clk_sdio0_cclkint_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_sdio0_cclkint_(div: u32) {
    let mut v = peek32(clk_sdio0_cclkint_ctrl_REG_ADDR);
    v &= !(0x1F);
    v |= (div & 0x1F);
    poke32(clk_sdio0_cclkint_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_sdio0_cclkint_() -> u32 {
    let v = peek32(clk_sdio0_cclkint_ctrl_REG_ADDR);
    v & 0x1f
}

pub fn _ENABLE_CLOCK_clk_sdio0_cclkint_inv_() {}

pub fn _SET_CLOCK_clk_sdio0_cclkint_inv_POLARITY_() {
    let mut v = peek32(clk_sdio0_cclkint_inv_ctrl_REG_ADDR);
    v &= !(0x1 << 30);
    v |= (0x1 & 0x1) << 30;
    poke32(clk_sdio0_cclkint_inv_ctrl_REG_ADDR, v);
}

pub fn _UNSET_CLOCK_clk_sdio0_cclkint_inv_POLARITY_() {
    let mut v = peek32(clk_sdio0_cclkint_inv_ctrl_REG_ADDR);
    v &= !(0x1 << 30);
    v |= (0x0 & 0x1) << 30;
    poke32(clk_sdio0_cclkint_inv_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_POLARITY_STATUS_clk_sdio0_cclkint_inv_() -> u32 {
    let v = peek32(clk_sdio0_cclkint_inv_ctrl_REG_ADDR) >> 30;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_sdio1_ahb_() {
    let mut v = peek32(clk_sdio1_ahb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_sdio1_ahb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_sdio1_ahb_() {
    let mut v = peek32(clk_sdio1_ahb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_sdio1_ahb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_sdio1_ahb_() -> u32 {
    let v = peek32(clk_sdio1_ahb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_sdio1_cclkint_() {
    let mut v = peek32(clk_sdio1_cclkint_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_sdio1_cclkint_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_sdio1_cclkint_() {
    let mut v = peek32(clk_sdio1_cclkint_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_sdio1_cclkint_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_sdio1_cclkint_() -> u32 {
    let v = peek32(clk_sdio1_cclkint_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_sdio1_cclkint_(div: u32) {
    let mut v = peek32(clk_sdio1_cclkint_ctrl_REG_ADDR);
    v &= !(0x1F);
    v |= (div & 0x1F);
    poke32(clk_sdio1_cclkint_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_sdio1_cclkint_() -> u32 {
    let v = peek32(clk_sdio1_cclkint_ctrl_REG_ADDR);
    v & 0x1f
}

pub fn _ENABLE_CLOCK_clk_sdio1_cclkint_inv_() {}

pub fn _SET_CLOCK_clk_sdio1_cclkint_inv_POLARITY_() {
    let mut v = peek32(clk_sdio1_cclkint_inv_ctrl_REG_ADDR);
    v &= !(0x1 << 30);
    v |= (0x1 & 0x1) << 30;
    poke32(clk_sdio1_cclkint_inv_ctrl_REG_ADDR, v);
}

pub fn _UNSET_CLOCK_clk_sdio1_cclkint_inv_POLARITY_() {
    let mut v = peek32(clk_sdio1_cclkint_inv_ctrl_REG_ADDR);
    v &= !(0x1 << 30);
    v |= (0x0 & 0x1) << 30;
    poke32(clk_sdio1_cclkint_inv_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_POLARITY_STATUS_clk_sdio1_cclkint_inv_() -> u32 {
    let v = peek32(clk_sdio1_cclkint_inv_ctrl_REG_ADDR) >> 30;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_gmac_ahb_() {
    let mut v = peek32(clk_gmac_ahb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_gmac_ahb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_gmac_ahb_() {
    let mut v = peek32(clk_gmac_ahb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_gmac_ahb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_gmac_ahb_() -> u32 {
    let v = peek32(clk_gmac_ahb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_gmac_root_div_() {}

pub fn _DIVIDE_CLOCK_clk_gmac_root_div_(div: u32) {
    let mut v = peek32(clk_gmac_root_div_ctrl_REG_ADDR);
    v &= !(0xF);
    v |= (div & 0xF);
    poke32(clk_gmac_root_div_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_gmac_root_div_() -> u32 {
    let v = peek32(clk_gmac_root_div_ctrl_REG_ADDR);
    v & 0xf
}

pub fn _ENABLE_CLOCK_clk_gmac_ptp_refclk_() {
    let mut v = peek32(clk_gmac_ptp_refclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_gmac_ptp_refclk_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_gmac_ptp_refclk_() {
    let mut v = peek32(clk_gmac_ptp_refclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_gmac_ptp_refclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_gmac_ptp_refclk_() -> u32 {
    let v = peek32(clk_gmac_ptp_refclk_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_gmac_ptp_refclk_(div: u32) {
    let mut v = peek32(clk_gmac_ptp_refclk_ctrl_REG_ADDR);
    v &= !(0x1F);
    v |= (div & 0x1F);
    poke32(clk_gmac_ptp_refclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_gmac_ptp_refclk_() -> u32 {
    let v = peek32(clk_gmac_ptp_refclk_ctrl_REG_ADDR);
    v & 0x1f
}

pub fn _ENABLE_CLOCK_clk_gmac_gtxclk_() {
    let mut v = peek32(clk_gmac_gtxclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_gmac_gtxclk_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_gmac_gtxclk_() {
    let mut v = peek32(clk_gmac_gtxclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_gmac_gtxclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_gmac_gtxclk_() -> u32 {
    let v = peek32(clk_gmac_gtxclk_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_gmac_gtxclk_(div: u32) {
    let mut v = peek32(clk_gmac_gtxclk_ctrl_REG_ADDR);
    v &= !(0xFF);
    v |= (div & 0xFF);
    poke32(clk_gmac_gtxclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_gmac_gtxclk_() -> u32 {
    let v = peek32(clk_gmac_gtxclk_ctrl_REG_ADDR);
    v & 0xff
}

pub fn _ENABLE_CLOCK_clk_gmac_rmii_txclk_() {
    let mut v = peek32(clk_gmac_rmii_txclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_gmac_rmii_txclk_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_gmac_rmii_txclk_() {
    let mut v = peek32(clk_gmac_rmii_txclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_gmac_rmii_txclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_gmac_rmii_txclk_() -> u32 {
    let v = peek32(clk_gmac_rmii_txclk_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_gmac_rmii_txclk_(div: u32) {
    let mut v = peek32(clk_gmac_rmii_txclk_ctrl_REG_ADDR);
    v &= !(0xF);
    v |= (div & 0xF);
    poke32(clk_gmac_rmii_txclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_gmac_rmii_txclk_() -> u32 {
    let v = peek32(clk_gmac_rmii_txclk_ctrl_REG_ADDR);
    v & 0xf
}

pub fn _ENABLE_CLOCK_clk_gmac_rmii_rxclk_() {
    let mut v = peek32(clk_gmac_rmii_rxclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_gmac_rmii_rxclk_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_gmac_rmii_rxclk_() {
    let mut v = peek32(clk_gmac_rmii_rxclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_gmac_rmii_rxclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_gmac_rmii_rxclk_() -> u32 {
    let v = peek32(clk_gmac_rmii_rxclk_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_gmac_rmii_rxclk_(div: u32) {
    let mut v = peek32(clk_gmac_rmii_rxclk_ctrl_REG_ADDR);
    v &= !(0xF);
    v |= (div & 0xF);
    poke32(clk_gmac_rmii_rxclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_gmac_rmii_rxclk_() -> u32 {
    let v = peek32(clk_gmac_rmii_rxclk_ctrl_REG_ADDR);
    v & 0xf
}

pub fn _ENABLE_CLOCK_clk_gmac_tx_() {}

pub fn _SWITCH_CLOCK_clk_gmac_tx_SOURCE_clk_gmac_gtxclk_() {
    let mut v = peek32(clk_gmac_tx_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x0 & 0x3) << 24;
    poke32(clk_gmac_tx_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_gmac_tx_SOURCE_clk_gmac_mii_txclk_() {
    let mut v = peek32(clk_gmac_tx_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x1 & 0x3) << 24;
    poke32(clk_gmac_tx_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_gmac_tx_SOURCE_clk_gmac_rmii_txclk_() {
    let mut v = peek32(clk_gmac_tx_ctrl_REG_ADDR);
    v &= !(0x3 << 24);
    v |= (0x2 & 0x3) << 24;
    poke32(clk_gmac_tx_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_SOURCE_STATUS_clk_gmac_tx_() -> u32 {
    let v = peek32(clk_gmac_tx_ctrl_REG_ADDR) >> 24;
    v & 0x3
}

pub fn _ENABLE_CLOCK_clk_gmac_tx_inv_() {}

pub fn _SET_CLOCK_clk_gmac_tx_inv_POLARITY_() {
    let mut v = peek32(clk_gmac_tx_inv_ctrl_REG_ADDR);
    v &= !(0x1 << 30);
    v |= (0x1 & 0x1) << 30;
    poke32(clk_gmac_tx_inv_ctrl_REG_ADDR, v);
}

pub fn _UNSET_CLOCK_clk_gmac_tx_inv_POLARITY_() {
    let mut v = peek32(clk_gmac_tx_inv_ctrl_REG_ADDR);
    v &= !(0x1 << 30);
    v |= (0x0 & 0x1) << 30;
    poke32(clk_gmac_tx_inv_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_POLARITY_STATUS_clk_gmac_tx_inv_() -> u32 {
    let v = peek32(clk_gmac_tx_inv_ctrl_REG_ADDR) >> 30;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_gmac_rx_pre_() {}

pub fn _SWITCH_CLOCK_clk_gmac_rx_pre_SOURCE_clk_gmac_gr_mii_rxclk_() {
    let mut v = peek32(clk_gmac_rx_pre_ctrl_REG_ADDR);
    v &= !(0x1 << 24);
    v |= (0x0 & 0x1) << 24;
    poke32(clk_gmac_rx_pre_ctrl_REG_ADDR, v);
}

pub fn _SWITCH_CLOCK_clk_gmac_rx_pre_SOURCE_clk_gmac_rmii_rxclk_() {
    let mut v = peek32(clk_gmac_rx_pre_ctrl_REG_ADDR);
    v &= !(0x1 << 24);
    v |= (0x1 & 0x1) << 24;
    poke32(clk_gmac_rx_pre_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_SOURCE_STATUS_clk_gmac_rx_pre_() -> u32 {
    let v = peek32(clk_gmac_rx_pre_ctrl_REG_ADDR) >> 24;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_gmac_rx_inv_() {}

pub fn _SET_CLOCK_clk_gmac_rx_inv_POLARITY_() {
    let mut v = peek32(clk_gmac_rx_inv_ctrl_REG_ADDR);
    v &= !(0x1 << 30);
    v |= (0x1 & 0x1) << 30;
    poke32(clk_gmac_rx_inv_ctrl_REG_ADDR, v);
}

pub fn _UNSET_CLOCK_clk_gmac_rx_inv_POLARITY_() {
    let mut v = peek32(clk_gmac_rx_inv_ctrl_REG_ADDR);
    v &= !(0x1 << 30);
    v |= (0x0 & 0x1) << 30;
    poke32(clk_gmac_rx_inv_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_POLARITY_STATUS_clk_gmac_rx_inv_() -> u32 {
    let v = peek32(clk_gmac_rx_inv_ctrl_REG_ADDR) >> 30;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_gmac_rmii_() {
    let mut v = peek32(clk_gmac_rmii_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_gmac_rmii_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_gmac_rmii_() {
    let mut v = peek32(clk_gmac_rmii_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_gmac_rmii_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_gmac_rmii_() -> u32 {
    let v = peek32(clk_gmac_rmii_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_gmac_tophyref_() {
    let mut v = peek32(clk_gmac_tophyref_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_gmac_tophyref_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_gmac_tophyref_() {
    let mut v = peek32(clk_gmac_tophyref_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_gmac_tophyref_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_gmac_tophyref_() -> u32 {
    let v = peek32(clk_gmac_tophyref_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_gmac_tophyref_(div: u32) {
    let mut v = peek32(clk_gmac_tophyref_ctrl_REG_ADDR);
    v &= !(0x7F);
    v |= (div & 0x7F);
    poke32(clk_gmac_tophyref_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_gmac_tophyref_() -> u32 {
    let v = peek32(clk_gmac_tophyref_ctrl_REG_ADDR);
    v & 0x7f
}

pub fn _ENABLE_CLOCK_clk_spi2ahb_ahb_() {
    let mut v = peek32(clk_spi2ahb_ahb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_spi2ahb_ahb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_spi2ahb_ahb_() {
    let mut v = peek32(clk_spi2ahb_ahb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_spi2ahb_ahb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_spi2ahb_ahb_() -> u32 {
    let v = peek32(clk_spi2ahb_ahb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_spi2ahb_core_() {
    let mut v = peek32(clk_spi2ahb_core_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_spi2ahb_core_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_spi2ahb_core_() {
    let mut v = peek32(clk_spi2ahb_core_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_spi2ahb_core_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_spi2ahb_core_() -> u32 {
    let v = peek32(clk_spi2ahb_core_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_spi2ahb_core_(div: u32) {
    let mut v = peek32(clk_spi2ahb_core_ctrl_REG_ADDR);
    v &= !(0x1F);
    v |= (div & 0x1F);
    poke32(clk_spi2ahb_core_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_spi2ahb_core_() -> u32 {
    let v = peek32(clk_spi2ahb_core_ctrl_REG_ADDR);
    v & 0x1f
}

pub fn _ENABLE_CLOCK_clk_ezmaster_ahb_() {
    let mut v = peek32(clk_ezmaster_ahb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_ezmaster_ahb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_ezmaster_ahb_() {
    let mut v = peek32(clk_ezmaster_ahb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_ezmaster_ahb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_ezmaster_ahb_() -> u32 {
    let v = peek32(clk_ezmaster_ahb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_e24_ahb_() {
    let mut v = peek32(clk_e24_ahb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_e24_ahb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_e24_ahb_() {
    let mut v = peek32(clk_e24_ahb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_e24_ahb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_e24_ahb_() -> u32 {
    let v = peek32(clk_e24_ahb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_e24rtc_toggle_() {
    let mut v = peek32(clk_e24rtc_toggle_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_e24rtc_toggle_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_e24rtc_toggle_() {
    let mut v = peek32(clk_e24rtc_toggle_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_e24rtc_toggle_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_e24rtc_toggle_() -> u32 {
    let v = peek32(clk_e24rtc_toggle_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_qspi_ahb_() {
    let mut v = peek32(clk_qspi_ahb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_qspi_ahb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_qspi_ahb_() {
    let mut v = peek32(clk_qspi_ahb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_qspi_ahb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_qspi_ahb_() -> u32 {
    let v = peek32(clk_qspi_ahb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_qspi_apb_() {
    let mut v = peek32(clk_qspi_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_qspi_apb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_qspi_apb_() {
    let mut v = peek32(clk_qspi_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_qspi_apb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_qspi_apb_() -> u32 {
    let v = peek32(clk_qspi_apb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_qspi_refclk_() {
    let mut v = peek32(clk_qspi_refclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_qspi_refclk_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_qspi_refclk_() {
    let mut v = peek32(clk_qspi_refclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_qspi_refclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_qspi_refclk_() -> u32 {
    let v = peek32(clk_qspi_refclk_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_qspi_refclk_(div: u32) {
    let mut v = peek32(clk_qspi_refclk_ctrl_REG_ADDR);
    v &= !(0x1F);
    v |= (div & 0x1F);
    poke32(clk_qspi_refclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_qspi_refclk_() -> u32 {
    let v = peek32(clk_qspi_refclk_ctrl_REG_ADDR);
    v & 0x1f
}

pub fn _ENABLE_CLOCK_clk_sec_ahb_() {
    let mut v = peek32(clk_sec_ahb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_sec_ahb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_sec_ahb_() {
    let mut v = peek32(clk_sec_ahb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_sec_ahb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_sec_ahb_() -> u32 {
    let v = peek32(clk_sec_ahb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_aes_clk_() {
    let mut v = peek32(clk_aes_clk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_aes_clk_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_aes_clk_() {
    let mut v = peek32(clk_aes_clk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_aes_clk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_aes_clk_() -> u32 {
    let v = peek32(clk_aes_clk_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_sha_clk_() {
    let mut v = peek32(clk_sha_clk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_sha_clk_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_sha_clk_() {
    let mut v = peek32(clk_sha_clk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_sha_clk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_sha_clk_() -> u32 {
    let v = peek32(clk_sha_clk_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_pka_clk_() {
    let mut v = peek32(clk_pka_clk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_pka_clk_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_pka_clk_() {
    let mut v = peek32(clk_pka_clk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_pka_clk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_pka_clk_() -> u32 {
    let v = peek32(clk_pka_clk_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_trng_apb_() {
    let mut v = peek32(clk_trng_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_trng_apb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_trng_apb_() {
    let mut v = peek32(clk_trng_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_trng_apb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_trng_apb_() -> u32 {
    let v = peek32(clk_trng_apb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_otp_apb_() {
    let mut v = peek32(clk_otp_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_otp_apb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_otp_apb_() {
    let mut v = peek32(clk_otp_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_otp_apb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_otp_apb_() -> u32 {
    let v = peek32(clk_otp_apb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_uart0_apb_() {
    let mut v = peek32(clk_uart0_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_uart0_apb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_uart0_apb_() {
    let mut v = peek32(clk_uart0_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_uart0_apb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_uart0_apb_() -> u32 {
    let v = peek32(clk_uart0_apb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_uart0_core_() {
    let mut v = peek32(clk_uart0_core_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_uart0_core_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_uart0_core_() {
    let mut v = peek32(clk_uart0_core_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_uart0_core_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_uart0_core_() -> u32 {
    let v = peek32(clk_uart0_core_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_uart0_core_(div: u32) {
    let mut v = peek32(clk_uart0_core_ctrl_REG_ADDR);
    v &= !(0x3F);
    v |= (div & 0x3F);
    poke32(clk_uart0_core_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_uart0_core_() -> u32 {
    let v = peek32(clk_uart0_core_ctrl_REG_ADDR);
    v & 0x3f
}

pub fn _ENABLE_CLOCK_clk_uart1_apb_() {
    let mut v = peek32(clk_uart1_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_uart1_apb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_uart1_apb_() {
    let mut v = peek32(clk_uart1_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_uart1_apb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_uart1_apb_() -> u32 {
    let v = peek32(clk_uart1_apb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_uart1_core_() {
    let mut v = peek32(clk_uart1_core_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_uart1_core_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_uart1_core_() {
    let mut v = peek32(clk_uart1_core_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_uart1_core_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_uart1_core_() -> u32 {
    let v = peek32(clk_uart1_core_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_uart1_core_(div: u32) {
    let mut v = peek32(clk_uart1_core_ctrl_REG_ADDR);
    v &= !(0x3F);
    v |= (div & 0x3F);
    poke32(clk_uart1_core_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_uart1_core_() -> u32 {
    let v = peek32(clk_uart1_core_ctrl_REG_ADDR);
    v & 0x3f
}

pub fn _ENABLE_CLOCK_clk_spi0_apb_() {
    let mut v = peek32(clk_spi0_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_spi0_apb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_spi0_apb_() {
    let mut v = peek32(clk_spi0_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_spi0_apb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_spi0_apb_() -> u32 {
    let v = peek32(clk_spi0_apb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_spi0_core_() {
    let mut v = peek32(clk_spi0_core_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_spi0_core_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_spi0_core_() {
    let mut v = peek32(clk_spi0_core_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_spi0_core_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_spi0_core_() -> u32 {
    let v = peek32(clk_spi0_core_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_spi0_core_(div: u32) {
    let mut v = peek32(clk_spi0_core_ctrl_REG_ADDR);
    v &= !(0x3F);
    v |= (div & 0x3F);
    poke32(clk_spi0_core_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_spi0_core_() -> u32 {
    let v = peek32(clk_spi0_core_ctrl_REG_ADDR);
    v & 0x3f
}

pub fn _ENABLE_CLOCK_clk_spi1_apb_() {
    let mut v = peek32(clk_spi1_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_spi1_apb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_spi1_apb_() {
    let mut v = peek32(clk_spi1_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_spi1_apb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_spi1_apb_() -> u32 {
    let v = peek32(clk_spi1_apb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_spi1_core_() {
    let mut v = peek32(clk_spi1_core_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_spi1_core_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_spi1_core_() {
    let mut v = peek32(clk_spi1_core_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_spi1_core_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_spi1_core_() -> u32 {
    let v = peek32(clk_spi1_core_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_spi1_core_(div: u32) {
    let mut v = peek32(clk_spi1_core_ctrl_REG_ADDR);
    v &= !(0x3F);
    v |= (div & 0x3F);
    poke32(clk_spi1_core_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_spi1_core_() -> u32 {
    let v = peek32(clk_spi1_core_ctrl_REG_ADDR);
    v & 0x3f
}

pub fn _ENABLE_CLOCK_clk_i2c0_apb_() {
    let mut v = peek32(clk_i2c0_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_i2c0_apb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_i2c0_apb_() {
    let mut v = peek32(clk_i2c0_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_i2c0_apb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_i2c0_apb_() -> u32 {
    let v = peek32(clk_i2c0_apb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_i2c0_core_() {
    let mut v = peek32(clk_i2c0_core_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_i2c0_core_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_i2c0_core_() {
    let mut v = peek32(clk_i2c0_core_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_i2c0_core_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_i2c0_core_() -> u32 {
    let v = peek32(clk_i2c0_core_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_i2c0_core_(div: u32) {
    let mut v = peek32(clk_i2c0_core_ctrl_REG_ADDR);
    v &= !(0x3F);
    v |= (div & 0x3F);
    poke32(clk_i2c0_core_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_i2c0_core_() -> u32 {
    let v = peek32(clk_i2c0_core_ctrl_REG_ADDR);
    v & 0x3f
}

pub fn _ENABLE_CLOCK_clk_i2c1_apb_() {
    let mut v = peek32(clk_i2c1_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_i2c1_apb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_i2c1_apb_() {
    let mut v = peek32(clk_i2c1_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_i2c1_apb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_i2c1_apb_() -> u32 {
    let v = peek32(clk_i2c1_apb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_i2c1_core_() {
    let mut v = peek32(clk_i2c1_core_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_i2c1_core_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_i2c1_core_() {
    let mut v = peek32(clk_i2c1_core_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_i2c1_core_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_i2c1_core_() -> u32 {
    let v = peek32(clk_i2c1_core_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_i2c1_core_(div: u32) {
    let mut v = peek32(clk_i2c1_core_ctrl_REG_ADDR);
    v &= !(0x3F);
    v |= (div & 0x3F);
    poke32(clk_i2c1_core_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_i2c1_core_() -> u32 {
    let v = peek32(clk_i2c1_core_ctrl_REG_ADDR);
    v & 0x3f
}

pub fn _ENABLE_CLOCK_clk_gpio_apb_() {
    let mut v = peek32(clk_gpio_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_gpio_apb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_gpio_apb_() {
    let mut v = peek32(clk_gpio_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_gpio_apb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_gpio_apb_() -> u32 {
    let v = peek32(clk_gpio_apb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_uart2_apb_() {
    let mut v = peek32(clk_uart2_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_uart2_apb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_uart2_apb_() {
    let mut v = peek32(clk_uart2_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_uart2_apb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_uart2_apb_() -> u32 {
    let v = peek32(clk_uart2_apb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_uart2_core_() {
    let mut v = peek32(clk_uart2_core_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_uart2_core_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_uart2_core_() {
    let mut v = peek32(clk_uart2_core_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_uart2_core_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_uart2_core_() -> u32 {
    let v = peek32(clk_uart2_core_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_uart2_core_(div: u32) {
    let mut v = peek32(clk_uart2_core_ctrl_REG_ADDR);
    v &= !(0x3F);
    v |= (div & 0x3F);
    poke32(clk_uart2_core_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_uart2_core_() -> u32 {
    let v = peek32(clk_uart2_core_ctrl_REG_ADDR);
    v & 0x3f
}

pub fn _ENABLE_CLOCK_clk_uart3_apb_() {
    let mut v = peek32(clk_uart3_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_uart3_apb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_uart3_apb_() {
    let mut v = peek32(clk_uart3_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_uart3_apb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_uart3_apb_() -> u32 {
    let v = peek32(clk_uart3_apb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_uart3_core_() {
    let mut v = peek32(clk_uart3_core_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_uart3_core_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_uart3_core_() {
    let mut v = peek32(clk_uart3_core_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_uart3_core_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_uart3_core_() -> u32 {
    let v = peek32(clk_uart3_core_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_uart3_core_(div: u32) {
    let mut v = peek32(clk_uart3_core_ctrl_REG_ADDR);
    v &= !(0x3F);
    v |= (div & 0x3F);
    poke32(clk_uart3_core_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_uart3_core_() -> u32 {
    let v = peek32(clk_uart3_core_ctrl_REG_ADDR);
    v & 0x3f
}

pub fn _ENABLE_CLOCK_clk_spi2_apb_() {
    let mut v = peek32(clk_spi2_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_spi2_apb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_spi2_apb_() {
    let mut v = peek32(clk_spi2_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_spi2_apb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_spi2_apb_() -> u32 {
    let v = peek32(clk_spi2_apb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_spi2_core_() {
    let mut v = peek32(clk_spi2_core_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_spi2_core_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_spi2_core_() {
    let mut v = peek32(clk_spi2_core_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_spi2_core_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_spi2_core_() -> u32 {
    let v = peek32(clk_spi2_core_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_spi2_core_(div: u32) {
    let mut v = peek32(clk_spi2_core_ctrl_REG_ADDR);
    v &= !(0x3F);
    v |= (div & 0x3F);
    poke32(clk_spi2_core_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_spi2_core_() -> u32 {
    let v = peek32(clk_spi2_core_ctrl_REG_ADDR);
    v & 0x3f
}

pub fn _ENABLE_CLOCK_clk_spi3_apb_() {
    let mut v = peek32(clk_spi3_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_spi3_apb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_spi3_apb_() {
    let mut v = peek32(clk_spi3_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_spi3_apb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_spi3_apb_() -> u32 {
    let v = peek32(clk_spi3_apb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_spi3_core_() {
    let mut v = peek32(clk_spi3_core_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_spi3_core_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_spi3_core_() {
    let mut v = peek32(clk_spi3_core_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_spi3_core_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_spi3_core_() -> u32 {
    let v = peek32(clk_spi3_core_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_spi3_core_(div: u32) {
    let mut v = peek32(clk_spi3_core_ctrl_REG_ADDR);
    v &= !(0x3F);
    v |= (div & 0x3F);
    poke32(clk_spi3_core_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_spi3_core_() -> u32 {
    let v = peek32(clk_spi3_core_ctrl_REG_ADDR);
    v & 0x3f
}

pub fn _ENABLE_CLOCK_clk_i2c2_apb_() {
    let mut v = peek32(clk_i2c2_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_i2c2_apb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_i2c2_apb_() {
    let mut v = peek32(clk_i2c2_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_i2c2_apb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_i2c2_apb_() -> u32 {
    let v = peek32(clk_i2c2_apb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_i2c2_core_() {
    let mut v = peek32(clk_i2c2_core_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_i2c2_core_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_i2c2_core_() {
    let mut v = peek32(clk_i2c2_core_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_i2c2_core_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_i2c2_core_() -> u32 {
    let v = peek32(clk_i2c2_core_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_i2c2_core_(div: u32) {
    let mut v = peek32(clk_i2c2_core_ctrl_REG_ADDR);
    v &= !(0x3F);
    v |= (div & 0x3F);
    poke32(clk_i2c2_core_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_i2c2_core_() -> u32 {
    let v = peek32(clk_i2c2_core_ctrl_REG_ADDR);
    v & 0x3f
}

pub fn _ENABLE_CLOCK_clk_i2c3_apb_() {
    let mut v = peek32(clk_i2c3_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_i2c3_apb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_i2c3_apb_() {
    let mut v = peek32(clk_i2c3_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_i2c3_apb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_i2c3_apb_() -> u32 {
    let v = peek32(clk_i2c3_apb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_i2c3_core_() {
    let mut v = peek32(clk_i2c3_core_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_i2c3_core_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_i2c3_core_() {
    let mut v = peek32(clk_i2c3_core_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_i2c3_core_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_i2c3_core_() -> u32 {
    let v = peek32(clk_i2c3_core_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_i2c3_core_(div: u32) {
    let mut v = peek32(clk_i2c3_core_ctrl_REG_ADDR);
    v &= !(0x3F);
    v |= (div & 0x3F);
    poke32(clk_i2c3_core_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_i2c3_core_() -> u32 {
    let v = peek32(clk_i2c3_core_ctrl_REG_ADDR);
    v & 0x3f
}

pub fn _ENABLE_CLOCK_clk_wdtimer_apb_() {
    let mut v = peek32(clk_wdtimer_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_wdtimer_apb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_wdtimer_apb_() {
    let mut v = peek32(clk_wdtimer_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_wdtimer_apb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_wdtimer_apb_() -> u32 {
    let v = peek32(clk_wdtimer_apb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_wdt_coreclk_() {
    let mut v = peek32(clk_wdt_coreclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_wdt_coreclk_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_wdt_coreclk_() {
    let mut v = peek32(clk_wdt_coreclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_wdt_coreclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_wdt_coreclk_() -> u32 {
    let v = peek32(clk_wdt_coreclk_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_wdt_coreclk_(div: u32) {
    let mut v = peek32(clk_wdt_coreclk_ctrl_REG_ADDR);
    v &= !(0x3F);
    v |= (div & 0x3F);
    poke32(clk_wdt_coreclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_wdt_coreclk_() -> u32 {
    let v = peek32(clk_wdt_coreclk_ctrl_REG_ADDR);
    v & 0x3f
}

pub fn _ENABLE_CLOCK_clk_timer0_coreclk_() {
    let mut v = peek32(clk_timer0_coreclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_timer0_coreclk_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_timer0_coreclk_() {
    let mut v = peek32(clk_timer0_coreclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_timer0_coreclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_timer0_coreclk_() -> u32 {
    let v = peek32(clk_timer0_coreclk_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_timer0_coreclk_(div: u32) {
    let mut v = peek32(clk_timer0_coreclk_ctrl_REG_ADDR);
    v &= !(0x3F);
    v |= (div & 0x3F);
    poke32(clk_timer0_coreclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_timer0_coreclk_() -> u32 {
    let v = peek32(clk_timer0_coreclk_ctrl_REG_ADDR);
    v & 0x3f
}

pub fn _ENABLE_CLOCK_clk_timer1_coreclk_() {
    let mut v = peek32(clk_timer1_coreclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_timer1_coreclk_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_timer1_coreclk_() {
    let mut v = peek32(clk_timer1_coreclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_timer1_coreclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_timer1_coreclk_() -> u32 {
    let v = peek32(clk_timer1_coreclk_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_timer1_coreclk_(div: u32) {
    let mut v = peek32(clk_timer1_coreclk_ctrl_REG_ADDR);
    v &= !(0x3F);
    v |= (div & 0x3F);
    poke32(clk_timer1_coreclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_timer1_coreclk_() -> u32 {
    let v = peek32(clk_timer1_coreclk_ctrl_REG_ADDR);
    v & 0x3f
}

pub fn _ENABLE_CLOCK_clk_timer2_coreclk_() {
    let mut v = peek32(clk_timer2_coreclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_timer2_coreclk_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_timer2_coreclk_() {
    let mut v = peek32(clk_timer2_coreclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_timer2_coreclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_timer2_coreclk_() -> u32 {
    let v = peek32(clk_timer2_coreclk_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_timer2_coreclk_(div: u32) {
    let mut v = peek32(clk_timer2_coreclk_ctrl_REG_ADDR);
    v &= !(0x3F);
    v |= (div & 0x3F);
    poke32(clk_timer2_coreclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_timer2_coreclk_() -> u32 {
    let v = peek32(clk_timer2_coreclk_ctrl_REG_ADDR);
    v & 0x3f
}

pub fn _ENABLE_CLOCK_clk_timer3_coreclk_() {
    let mut v = peek32(clk_timer3_coreclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_timer3_coreclk_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_timer3_coreclk_() {
    let mut v = peek32(clk_timer3_coreclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_timer3_coreclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_timer3_coreclk_() -> u32 {
    let v = peek32(clk_timer3_coreclk_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_timer3_coreclk_(div: u32) {
    let mut v = peek32(clk_timer3_coreclk_ctrl_REG_ADDR);
    v &= !(0x3F);
    v |= (div & 0x3F);
    poke32(clk_timer3_coreclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_timer3_coreclk_() -> u32 {
    let v = peek32(clk_timer3_coreclk_ctrl_REG_ADDR);
    v & 0x3f
}

pub fn _ENABLE_CLOCK_clk_timer4_coreclk_() {
    let mut v = peek32(clk_timer4_coreclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_timer4_coreclk_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_timer4_coreclk_() {
    let mut v = peek32(clk_timer4_coreclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_timer4_coreclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_timer4_coreclk_() -> u32 {
    let v = peek32(clk_timer4_coreclk_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_timer4_coreclk_(div: u32) {
    let mut v = peek32(clk_timer4_coreclk_ctrl_REG_ADDR);
    v &= !(0x3F);
    v |= (div & 0x3F);
    poke32(clk_timer4_coreclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_timer4_coreclk_() -> u32 {
    let v = peek32(clk_timer4_coreclk_ctrl_REG_ADDR);
    v & 0x3f
}

pub fn _ENABLE_CLOCK_clk_timer5_coreclk_() {
    let mut v = peek32(clk_timer5_coreclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_timer5_coreclk_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_timer5_coreclk_() {
    let mut v = peek32(clk_timer5_coreclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_timer5_coreclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_timer5_coreclk_() -> u32 {
    let v = peek32(clk_timer5_coreclk_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_timer5_coreclk_(div: u32) {
    let mut v = peek32(clk_timer5_coreclk_ctrl_REG_ADDR);
    v &= !(0x3F);
    v |= (div & 0x3F);
    poke32(clk_timer5_coreclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_timer5_coreclk_() -> u32 {
    let v = peek32(clk_timer5_coreclk_ctrl_REG_ADDR);
    v & 0x3f
}

pub fn _ENABLE_CLOCK_clk_timer6_coreclk_() {
    let mut v = peek32(clk_timer6_coreclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_timer6_coreclk_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_timer6_coreclk_() {
    let mut v = peek32(clk_timer6_coreclk_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_timer6_coreclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_timer6_coreclk_() -> u32 {
    let v = peek32(clk_timer6_coreclk_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_timer6_coreclk_(div: u32) {
    let mut v = peek32(clk_timer6_coreclk_ctrl_REG_ADDR);
    v &= !(0x3F);
    v |= (div & 0x3F);
    poke32(clk_timer6_coreclk_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_timer6_coreclk_() -> u32 {
    let v = peek32(clk_timer6_coreclk_ctrl_REG_ADDR);
    v & 0x3f
}

pub fn _ENABLE_CLOCK_clk_vp6intc_apb_() {
    let mut v = peek32(clk_vp6intc_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_vp6intc_apb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_vp6intc_apb_() {
    let mut v = peek32(clk_vp6intc_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_vp6intc_apb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_vp6intc_apb_() -> u32 {
    let v = peek32(clk_vp6intc_apb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_pwm_apb_() {
    let mut v = peek32(clk_pwm_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_pwm_apb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_pwm_apb_() {
    let mut v = peek32(clk_pwm_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_pwm_apb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_pwm_apb_() -> u32 {
    let v = peek32(clk_pwm_apb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_msi_apb_() {
    let mut v = peek32(clk_msi_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_msi_apb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_msi_apb_() {
    let mut v = peek32(clk_msi_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_msi_apb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_msi_apb_() -> u32 {
    let v = peek32(clk_msi_apb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_temp_apb_() {
    let mut v = peek32(clk_temp_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_temp_apb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_temp_apb_() {
    let mut v = peek32(clk_temp_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_temp_apb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_temp_apb_() -> u32 {
    let v = peek32(clk_temp_apb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ENABLE_CLOCK_clk_temp_sense_() {
    let mut v = peek32(clk_temp_sense_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_temp_sense_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_temp_sense_() {
    let mut v = peek32(clk_temp_sense_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_temp_sense_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_temp_sense_() -> u32 {
    let v = peek32(clk_temp_sense_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _DIVIDE_CLOCK_clk_temp_sense_(div: u32) {
    let mut v = peek32(clk_temp_sense_ctrl_REG_ADDR);
    v &= !(0x1F);
    v |= (div & 0x1F);
    poke32(clk_temp_sense_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_DIVIDE_STATUS_clk_temp_sense_() -> u32 {
    let v = peek32(clk_temp_sense_ctrl_REG_ADDR);
    v & 0x1f
}

pub fn _ENABLE_CLOCK_clk_syserr_apb_() {
    let mut v = peek32(clk_syserr_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(clk_syserr_apb_ctrl_REG_ADDR, v);
}

pub fn _DISABLE_CLOCK_clk_syserr_apb_() {
    let mut v = peek32(clk_syserr_apb_ctrl_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(clk_syserr_apb_ctrl_REG_ADDR, v);
}

pub fn _GET_CLOCK_ENABLE_STATUS_clk_syserr_apb_() -> u32 {
    let v = peek32(clk_syserr_apb_ctrl_REG_ADDR) >> 31;
    v & 0x1
}

pub struct Clock<'a> {
    base: usize,
    clks: &'a mut [&'a mut dyn ClockNode],
}
/*
impl<'a> ops::Deref for Clock<'a> {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}
*/
impl<'a> Driver for Clock<'a> {
    fn init(&mut self) -> Result<()> {
        /* nothing to do. */
        Ok(())
    }

    fn pread(&self, _data: &mut [u8], _offset: usize) -> Result<usize> {
        NOT_IMPLEMENTED
    }

    fn pwrite(&mut self, data: &[u8], _offset: usize) -> Result<usize> {
        match data {
            b"on" => {
                self.clock_init();
                Ok(1)
            }
            _ => Ok(0),
        }
    }

    fn shutdown(&mut self) {}
}

/* Clock initialization should only be done in romstage. */

impl<'a> Clock<'a> {
    pub fn new(clks: &'a mut [&'a mut dyn ClockNode]) -> Clock<'a> {
        Clock::<'a> { base: 0, clks }
    }

    /// Returns a pointer to the register block
    pub fn ptr(&self) -> usize {
        self.base
    }

    fn init_coreclk(&self) {
        // TODO: make base a parameter.
        _SWITCH_CLOCK_clk_cpundbus_root_SOURCE_clk_pll0_out_();
        _SWITCH_CLOCK_clk_dla_root_SOURCE_clk_pll1_out_();
        _SWITCH_CLOCK_clk_dsp_root_SOURCE_clk_pll2_out_();
        _SWITCH_CLOCK_clk_perh0_root_SOURCE_clk_pll0_out_();

        // not enabled in original.
        // slow down nne bus can fix nne50 & vp6 ram scan issue,
        // as well as vin_subsys reg scan issue.
        //	_SWITCH_CLOCK_clk_nne_bus_SOURCE_clk_cpu_axi_;
    }

    //    fn init_pll_ddr(&self) {}

    fn init_pll_ge(&self) {}

    fn clock_init(&mut self) {
        if is_qemu() {
            return;
        }

        // Update the peripheral clock dividers of UART, SPI and I2C to safe
        // values as we can't put them in reset before changing frequency.
        let hfclk = 1_000_000_000; // 1GHz
        for clk in self.clks.iter_mut() {
            if false {
                clk.set_clock_rate(hfclk);
            }
        }

        self.init_coreclk();

        // These take like 16 cycles to actually propagate. We can't go sending
        // stuff before they come out of reset. So wait.
        // TODO: Add a register to read the current reset states, or DDR Control
        // device?
        for _ in 0..=255 {
            arch::nop();
        }
        self.init_pll_ge();
        //        self.dev_reset
        //            .set(reset_mask(false, false, false, false, false));

        arch::fence();
    }
}
