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
 * @file  rstgen_ctrl_macro.h
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
use model::*;
//use crate::reg;
use crate::clock::_ENABLE_CLOCK_clk_msi_apb_;
use crate::clock::_ENABLE_CLOCK_clk_x2c_axi_;
use core::ptr;
//pub mod clock;

pub const RSTGEN_BASE_ADDR: u32 = 0x1184_0000;
pub const rstgen_Software_RESET_assert0_REG_ADDR: u32 = RSTGEN_BASE_ADDR + 0x0;
pub const rstgen_Software_RESET_assert1_REG_ADDR: u32 = RSTGEN_BASE_ADDR + 0x4;
pub const rstgen_Software_RESET_assert2_REG_ADDR: u32 = RSTGEN_BASE_ADDR + 0x8;
pub const rstgen_Software_RESET_assert3_REG_ADDR: u32 = RSTGEN_BASE_ADDR + 0xC;

pub const rstgen_Software_RESET_status0_REG_ADDR: u32 = RSTGEN_BASE_ADDR + 0x10;
pub const rstgen_Software_RESET_status1_REG_ADDR: u32 = RSTGEN_BASE_ADDR + 0x14;
pub const rstgen_Software_RESET_status2_REG_ADDR: u32 = RSTGEN_BASE_ADDR + 0x18;
pub const rstgen_Software_RESET_status3_REG_ADDR: u32 = RSTGEN_BASE_ADDR + 0x1C;

pub fn _READ_RESET_STATUS_rstgen_rstn_dom3ahb_bus_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR);
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_dom3ahb_bus_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1);
    v |= (0x1 & 0x1);
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR);
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_dom3ahb_bus_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1);
    v |= (0x0 & 0x1);
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR);
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_dom7ahb_bus_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 1;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_dom7ahb_bus_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 1);
    v |= (0x1 & 0x1) << 1;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 1;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_dom7ahb_bus_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 1);
    v |= (0x0 & 0x1) << 1;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 1;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rst_u74_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 2;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rst_u74_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 2);
    v |= (0x1 & 0x1) << 2;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 2;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rst_u74_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 2);
    v |= (0x0 & 0x1) << 2;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 2;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_u74_axi_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 3;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_u74_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 3);
    v |= (0x1 & 0x1) << 3;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 3;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_u74_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 3);
    v |= (0x0 & 0x1) << 3;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 3;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_sgdma2p_ahb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 4;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_sgdma2p_ahb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 4);
    v |= (0x1 & 0x1) << 4;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 4;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_sgdma2p_ahb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 4);
    v |= (0x0 & 0x1) << 4;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 4;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_sgdma2p_axi_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 5;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_sgdma2p_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 5);
    v |= (0x1 & 0x1) << 5;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 5;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_sgdma2p_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 5);
    v |= (0x0 & 0x1) << 5;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 5;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_dma2pnoc_aix_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 6;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_dma2pnoc_aix_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 6);
    v |= (0x1 & 0x1) << 6;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 6;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_dma2pnoc_aix_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 6);
    v |= (0x0 & 0x1) << 6;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 6;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_dla_axi_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 7;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_dla_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 7);
    v |= (0x1 & 0x1) << 7;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 7;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_dla_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 7);
    v |= (0x0 & 0x1) << 7;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 7;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_dlanoc_axi_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 8;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_dlanoc_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 8);
    v |= (0x1 & 0x1) << 8;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 8;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_dlanoc_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 8);
    v |= (0x0 & 0x1) << 8;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 8;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_dla_apb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 9;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_dla_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 9);
    v |= (0x1 & 0x1) << 9;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 9;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_dla_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 9);
    v |= (0x0 & 0x1) << 9;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 9;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rst_vp6_DReset_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 10;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rst_vp6_DReset_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 10);
    v |= (0x1 & 0x1) << 10;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 10;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rst_vp6_DReset_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 10);
    v |= (0x0 & 0x1) << 10;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 10;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rst_vp6_Breset_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 11;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rst_vp6_Breset_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 11);
    v |= (0x1 & 0x1) << 11;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 11;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rst_vp6_Breset_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 11);
    v |= (0x0 & 0x1) << 11;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 11;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_vp6_axi_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 12;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_vp6_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 12);
    v |= (0x1 & 0x1) << 12;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 12;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_vp6_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 12);
    v |= (0x0 & 0x1) << 12;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 12;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_vdecbrg_main_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 13;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_vdecbrg_main_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 13);
    v |= (0x1 & 0x1) << 13;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 13;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_vdecbrg_main_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 13);
    v |= (0x0 & 0x1) << 13;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 13;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_vdec_axi_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 14;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_vdec_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 14);
    v |= (0x1 & 0x1) << 14;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 14;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_vdec_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 14);
    v |= (0x0 & 0x1) << 14;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 14;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_vdec_bclk_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 15;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_vdec_bclk_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 15);
    v |= (0x1 & 0x1) << 15;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 15;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_vdec_bclk_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 15);
    v |= (0x0 & 0x1) << 15;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 15;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_vdec_cclk_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 16;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_vdec_cclk_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 16);
    v |= (0x1 & 0x1) << 16;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 16;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_vdec_cclk_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 16);
    v |= (0x0 & 0x1) << 16;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 16;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_vdec_apb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 17;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_vdec_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 17);
    v |= (0x1 & 0x1) << 17;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 17;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_vdec_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 17);
    v |= (0x0 & 0x1) << 17;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 17;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_jpeg_axi_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 18;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_jpeg_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 18);
    v |= (0x1 & 0x1) << 18;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 18;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_jpeg_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 18);
    v |= (0x0 & 0x1) << 18;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 18;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_jpeg_cclk_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 19;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_jpeg_cclk_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 19);
    v |= (0x1 & 0x1) << 19;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 19;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_jpeg_cclk_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 19);
    v |= (0x0 & 0x1) << 19;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 19;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_jpeg_apb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 20;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_jpeg_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 20);
    v |= (0x1 & 0x1) << 20;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 20;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_jpeg_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 20);
    v |= (0x0 & 0x1) << 20;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 20;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_jpcgc300_main_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 21;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_jpcgc300_main_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 21);
    v |= (0x1 & 0x1) << 21;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 21;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_jpcgc300_main_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 21);
    v |= (0x0 & 0x1) << 21;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 21;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_gc300_2x_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 22;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_gc300_2x_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 22);
    v |= (0x1 & 0x1) << 22;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 22;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_gc300_2x_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 22);
    v |= (0x0 & 0x1) << 22;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 22;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_gc300_axi_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 23;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_gc300_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 23);
    v |= (0x1 & 0x1) << 23;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 23;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_gc300_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 23);
    v |= (0x0 & 0x1) << 23;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 23;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_gc300_ahb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 24;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_gc300_ahb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 24);
    v |= (0x1 & 0x1) << 24;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 24;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_gc300_ahb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 24);
    v |= (0x0 & 0x1) << 24;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 24;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_venc_axi_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 25;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_venc_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 25);
    v |= (0x1 & 0x1) << 25;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 25;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_venc_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 25);
    v |= (0x0 & 0x1) << 25;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 25;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_vencbrg_main_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 26;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_vencbrg_main_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 26);
    v |= (0x1 & 0x1) << 26;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 26;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_vencbrg_main_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 26);
    v |= (0x0 & 0x1) << 26;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 26;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_venc_bclk_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 27;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_venc_bclk_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 27);
    v |= (0x1 & 0x1) << 27;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 27;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_venc_bclk_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 27);
    v |= (0x0 & 0x1) << 27;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 27;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_venc_cclk_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 28;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_venc_cclk_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 28);
    v |= (0x1 & 0x1) << 28;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 28;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_venc_cclk_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 28);
    v |= (0x0 & 0x1) << 28;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 28;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_venc_apb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 29;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_venc_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 29);
    v |= (0x1 & 0x1) << 29;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 29;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_venc_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 29);
    v |= (0x0 & 0x1) << 29;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 29;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_ddrphy_apb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 30;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_ddrphy_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 30);
    v |= (0x1 & 0x1) << 30;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 30;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_ddrphy_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 30);
    v |= (0x0 & 0x1) << 30;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 30;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_noc_rob_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_noc_rob_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 31;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_noc_rob_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert0_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(rstgen_Software_RESET_assert0_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status0_REG_ADDR) >> 31;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_noc_cog_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR);
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_noc_cog_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1);
    v |= (0x1 & 0x1);
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR);
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_noc_cog_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1);
    v |= (0x0 & 0x1);
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR);
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_hifi4_axi_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 1;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_hifi4_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 1);
    v |= (0x1 & 0x1) << 1;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 1;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_hifi4_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 1);
    v |= (0x0 & 0x1) << 1;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 1;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_hifi4noc_axi_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 2;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_hifi4noc_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 2);
    v |= (0x1 & 0x1) << 2;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 2;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_hifi4noc_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 2);
    v |= (0x0 & 0x1) << 2;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 2;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rst_hifi4_DReset_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 3;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rst_hifi4_DReset_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 3);
    v |= (0x1 & 0x1) << 3;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 3;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rst_hifi4_DReset_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 3);
    v |= (0x0 & 0x1) << 3;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 3;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rst_hifi4_Breset_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 4;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rst_hifi4_Breset_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 4);
    v |= (0x1 & 0x1) << 4;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 4;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rst_hifi4_Breset_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 4);
    v |= (0x0 & 0x1) << 4;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 4;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_usb_axi_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 5;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_usb_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 5);
    v |= (0x1 & 0x1) << 5;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 5;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_usb_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 5);
    v |= (0x0 & 0x1) << 5;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 5;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_usbnoc_axi_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 6;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_usbnoc_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 6);
    v |= (0x1 & 0x1) << 6;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 6;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_usbnoc_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 6);
    v |= (0x0 & 0x1) << 6;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 6;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_sgdma1p_axi_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 7;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_sgdma1p_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 7);
    v |= (0x1 & 0x1) << 7;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 7;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_sgdma1p_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 7);
    v |= (0x0 & 0x1) << 7;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 7;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_dma1p_axi_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 8;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_dma1p_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 8);
    v |= (0x1 & 0x1) << 8;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 8;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_dma1p_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 8);
    v |= (0x0 & 0x1) << 8;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 8;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_x2c_axi_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 9;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_x2c_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 9);
    v |= (0x1 & 0x1) << 9;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 9;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_x2c_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 9);
    v |= (0x0 & 0x1) << 9;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 9;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_nne_ahb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 10;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_nne_ahb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 10);
    v |= (0x1 & 0x1) << 10;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 10;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_nne_ahb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 10);
    v |= (0x0 & 0x1) << 10;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 10;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_nne_axi_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 11;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_nne_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 11);
    v |= (0x1 & 0x1) << 11;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 11;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_nne_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 11);
    v |= (0x0 & 0x1) << 11;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 11;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_nnenoc_axi_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 12;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_nnenoc_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 12);
    v |= (0x1 & 0x1) << 12;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 12;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_nnenoc_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 12);
    v |= (0x0 & 0x1) << 12;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 12;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_dlaslv_axi_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 13;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_dlaslv_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 13);
    v |= (0x1 & 0x1) << 13;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 13;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_dlaslv_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 13);
    v |= (0x0 & 0x1) << 13;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 13;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_dspx2c_axi_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 14;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_dspx2c_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 14);
    v |= (0x1 & 0x1) << 14;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 14;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_dspx2c_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 14);
    v |= (0x0 & 0x1) << 14;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 14;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_vin_src_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 15;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_vin_src_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 15);
    v |= (0x1 & 0x1) << 15;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 15;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_vin_src_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 15);
    v |= (0x0 & 0x1) << 15;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 15;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_ispslv_axi_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 16;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_ispslv_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 16);
    v |= (0x1 & 0x1) << 16;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 16;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_ispslv_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 16);
    v |= (0x0 & 0x1) << 16;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 16;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_vin_axi_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 17;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_vin_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 17);
    v |= (0x1 & 0x1) << 17;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 17;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_vin_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 17);
    v |= (0x0 & 0x1) << 17;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 17;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_vinnoc_axi_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 18;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_vinnoc_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 18);
    v |= (0x1 & 0x1) << 18;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 18;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_vinnoc_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 18);
    v |= (0x0 & 0x1) << 18;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 18;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_isp0_axi_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 19;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_isp0_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 19);
    v |= (0x1 & 0x1) << 19;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 19;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_isp0_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 19);
    v |= (0x0 & 0x1) << 19;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 19;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_isp0noc_axi_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 20;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_isp0noc_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 20);
    v |= (0x1 & 0x1) << 20;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 20;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_isp0noc_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 20);
    v |= (0x0 & 0x1) << 20;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 20;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_isp1_axi_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 21;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_isp1_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 21);
    v |= (0x1 & 0x1) << 21;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 21;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_isp1_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 21);
    v |= (0x0 & 0x1) << 21;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 21;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_isp1noc_axi_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 22;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_isp1noc_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 22);
    v |= (0x1 & 0x1) << 22;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 22;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_isp1noc_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 22);
    v |= (0x0 & 0x1) << 22;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 22;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_vout_src_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 23;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_vout_src_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 23);
    v |= (0x1 & 0x1) << 23;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 23;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_vout_src_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 23);
    v |= (0x0 & 0x1) << 23;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 23;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_disp_axi_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 24;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_disp_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 24);
    v |= (0x1 & 0x1) << 24;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 24;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_disp_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 24);
    v |= (0x0 & 0x1) << 24;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 24;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_dispnoc_axi_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 25;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_dispnoc_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 25);
    v |= (0x1 & 0x1) << 25;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 25;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_dispnoc_axi_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 25);
    v |= (0x0 & 0x1) << 25;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 25;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_sdio0_ahb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 26;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_sdio0_ahb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 26);
    v |= (0x1 & 0x1) << 26;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 26;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_sdio0_ahb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 26);
    v |= (0x0 & 0x1) << 26;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 26;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_sdio1_ahb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 27;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_sdio1_ahb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 27);
    v |= (0x1 & 0x1) << 27;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 27;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_sdio1_ahb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 27);
    v |= (0x0 & 0x1) << 27;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 27;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_gmac_ahb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 28;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_gmac_ahb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 28);
    v |= (0x1 & 0x1) << 28;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 28;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_gmac_ahb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 28);
    v |= (0x0 & 0x1) << 28;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 28;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_spi2ahb_ahb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 29;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_spi2ahb_ahb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 29);
    v |= (0x1 & 0x1) << 29;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 29;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_spi2ahb_ahb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 29);
    v |= (0x0 & 0x1) << 29;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 29;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_spi2ahb_core_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 30;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_spi2ahb_core_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 30);
    v |= (0x1 & 0x1) << 30;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 30;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_spi2ahb_core_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 30);
    v |= (0x0 & 0x1) << 30;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 30;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_ezmaster_ahb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_ezmaster_ahb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 31;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_ezmaster_ahb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert1_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(rstgen_Software_RESET_assert1_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status1_REG_ADDR) >> 31;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rst_e24_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR);
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rst_e24_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1);
    v |= (0x1 & 0x1);
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR);
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rst_e24_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1);
    v |= (0x0 & 0x1);
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR);
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_qspi_ahb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 1;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_qspi_ahb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 1);
    v |= (0x1 & 0x1) << 1;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 1;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_qspi_ahb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 1);
    v |= (0x0 & 0x1) << 1;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 1;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_qspi_core_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 2;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_qspi_core_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 2);
    v |= (0x1 & 0x1) << 2;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 2;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_qspi_core_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 2);
    v |= (0x0 & 0x1) << 2;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 2;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_qspi_apb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 3;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_qspi_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 3);
    v |= (0x1 & 0x1) << 3;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 3;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_qspi_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 3);
    v |= (0x0 & 0x1) << 3;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 3;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_sec_ahb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 4;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_sec_ahb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 4);
    v |= (0x1 & 0x1) << 4;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 4;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_sec_ahb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 4);
    v |= (0x0 & 0x1) << 4;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 4;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_aes_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 5;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_aes_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 5);
    v |= (0x1 & 0x1) << 5;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 5;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_aes_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 5);
    v |= (0x0 & 0x1) << 5;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 5;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_pka_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 6;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_pka_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 6);
    v |= (0x1 & 0x1) << 6;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 6;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_pka_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 6);
    v |= (0x0 & 0x1) << 6;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 6;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_sha_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 7;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_sha_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 7);
    v |= (0x1 & 0x1) << 7;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 7;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_sha_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 7);
    v |= (0x0 & 0x1) << 7;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 7;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_trng_apb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 8;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_trng_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 8);
    v |= (0x1 & 0x1) << 8;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 8;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_trng_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 8);
    v |= (0x0 & 0x1) << 8;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 8;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_otp_apb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 9;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_otp_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 9);
    v |= (0x1 & 0x1) << 9;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 9;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_otp_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 9);
    v |= (0x0 & 0x1) << 9;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 9;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_uart0_apb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 10;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_uart0_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 10);
    v |= (0x1 & 0x1) << 10;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 10;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_uart0_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 10);
    v |= (0x0 & 0x1) << 10;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 10;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_uart0_core_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 11;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_uart0_core_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 11);
    v |= (0x1 & 0x1) << 11;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 11;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_uart0_core_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 11);
    v |= (0x0 & 0x1) << 11;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 11;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_uart1_apb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 12;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_uart1_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 12);
    v |= (0x1 & 0x1) << 12;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 12;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_uart1_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 12);
    v |= (0x0 & 0x1) << 12;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 12;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_uart1_core_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 13;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_uart1_core_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 13);
    v |= (0x1 & 0x1) << 13;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 13;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_uart1_core_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 13);
    v |= (0x0 & 0x1) << 13;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 13;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_spi0_apb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 14;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_spi0_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 14);
    v |= (0x1 & 0x1) << 14;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 14;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_spi0_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 14);
    v |= (0x0 & 0x1) << 14;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 14;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_spi0_core_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 15;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_spi0_core_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 15);
    v |= (0x1 & 0x1) << 15;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 15;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_spi0_core_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 15);
    v |= (0x0 & 0x1) << 15;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 15;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_spi1_apb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 16;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_spi1_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 16);
    v |= (0x1 & 0x1) << 16;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 16;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_spi1_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 16);
    v |= (0x0 & 0x1) << 16;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 16;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_spi1_core_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 17;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_spi1_core_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 17);
    v |= (0x1 & 0x1) << 17;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 17;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_spi1_core_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 17);
    v |= (0x0 & 0x1) << 17;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 17;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_i2c0_apb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 18;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_i2c0_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 18);
    v |= (0x1 & 0x1) << 18;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 18;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_i2c0_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 18);
    v |= (0x0 & 0x1) << 18;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 18;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_i2c0_core_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 19;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_i2c0_core_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 19);
    v |= (0x1 & 0x1) << 19;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 19;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_i2c0_core_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 19);
    v |= (0x0 & 0x1) << 19;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 19;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_i2c1_apb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 20;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_i2c1_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 20);
    v |= (0x1 & 0x1) << 20;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 20;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_i2c1_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 20);
    v |= (0x0 & 0x1) << 20;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 20;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_i2c1_core_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 21;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_i2c1_core_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 21);
    v |= (0x1 & 0x1) << 21;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 21;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_i2c1_core_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 21);
    v |= (0x0 & 0x1) << 21;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 21;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_gpio_apb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 22;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_gpio_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 22);
    v |= (0x1 & 0x1) << 22;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 22;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_gpio_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 22);
    v |= (0x0 & 0x1) << 22;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 22;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_uart2_apb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 23;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_uart2_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 23);
    v |= (0x1 & 0x1) << 23;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 23;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_uart2_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 23);
    v |= (0x0 & 0x1) << 23;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 23;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_uart2_core_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 24;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_uart2_core_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 24);
    v |= (0x1 & 0x1) << 24;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 24;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_uart2_core_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 24);
    v |= (0x0 & 0x1) << 24;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 24;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_uart3_apb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 25;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_uart3_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 25);
    v |= (0x1 & 0x1) << 25;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 25;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_uart3_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 25);
    v |= (0x0 & 0x1) << 25;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 25;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_uart3_core_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 26;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_uart3_core_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 26);
    v |= (0x1 & 0x1) << 26;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 26;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_uart3_core_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 26);
    v |= (0x0 & 0x1) << 26;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 26;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_spi2_apb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 27;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_spi2_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 27);
    v |= (0x1 & 0x1) << 27;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 27;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_spi2_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 27);
    v |= (0x0 & 0x1) << 27;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 27;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_spi2_core_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 28;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_spi2_core_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 28);
    v |= (0x1 & 0x1) << 28;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 28;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_spi2_core_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 28);
    v |= (0x0 & 0x1) << 28;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 28;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_spi3_apb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 29;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_spi3_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 29);
    v |= (0x1 & 0x1) << 29;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 29;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_spi3_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 29);
    v |= (0x0 & 0x1) << 29;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 29;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_spi3_core_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 30;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_spi3_core_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 30);
    v |= (0x1 & 0x1) << 30;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 30;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_spi3_core_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 30);
    v |= (0x0 & 0x1) << 30;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 30;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_i2c2_apb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 31;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_i2c2_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x1 & 0x1) << 31;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 31;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_i2c2_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert2_REG_ADDR);
    v &= !(0x1 << 31);
    v |= (0x0 & 0x1) << 31;
    poke32(rstgen_Software_RESET_assert2_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status2_REG_ADDR) >> 31;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_i2c2_core_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status3_REG_ADDR);
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_i2c2_core_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1);
    v |= (0x1 & 0x1);
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR);
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_i2c2_core_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1);
    v |= (0x0 & 0x1);
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR);
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_i2c3_apb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 1;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_i2c3_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 1);
    v |= (0x1 & 0x1) << 1;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 1;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_i2c3_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 1);
    v |= (0x0 & 0x1) << 1;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 1;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_i2c3_core_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 2;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_i2c3_core_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 2);
    v |= (0x1 & 0x1) << 2;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 2;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_i2c3_core_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 2);
    v |= (0x0 & 0x1) << 2;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 2;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_wdtimer_apb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 3;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_wdtimer_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 3);
    v |= (0x1 & 0x1) << 3;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 3;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_wdtimer_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 3);
    v |= (0x0 & 0x1) << 3;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 3;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_wdt_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 4;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_wdt_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 4);
    v |= (0x1 & 0x1) << 4;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 4;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_wdt_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 4);
    v |= (0x0 & 0x1) << 4;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 4;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_timer0_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 5;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_timer0_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 5);
    v |= (0x1 & 0x1) << 5;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 5;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_timer0_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 5);
    v |= (0x0 & 0x1) << 5;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 5;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_timer1_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 6;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_timer1_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 6);
    v |= (0x1 & 0x1) << 6;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 6;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_timer1_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 6);
    v |= (0x0 & 0x1) << 6;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 6;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_timer2_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 7;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_timer2_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 7);
    v |= (0x1 & 0x1) << 7;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 7;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_timer2_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 7);
    v |= (0x0 & 0x1) << 7;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 7;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_timer3_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 8;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_timer3_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 8);
    v |= (0x1 & 0x1) << 8;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 8;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_timer3_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 8);
    v |= (0x0 & 0x1) << 8;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 8;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_timer4_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 9;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_timer4_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 9);
    v |= (0x1 & 0x1) << 9;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 9;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_timer4_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 9);
    v |= (0x0 & 0x1) << 9;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 9;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_timer5_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 10;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_timer5_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 10);
    v |= (0x1 & 0x1) << 10;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 10;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_timer5_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 10);
    v |= (0x0 & 0x1) << 10;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 10;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_timer6_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 11;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_timer6_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 11);
    v |= (0x1 & 0x1) << 11;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 11;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_timer6_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 11);
    v |= (0x0 & 0x1) << 11;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 11;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_vp6intc_apb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 12;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_vp6intc_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 12);
    v |= (0x1 & 0x1) << 12;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 12;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_vp6intc_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 12);
    v |= (0x0 & 0x1) << 12;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 12;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_pwm_apb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 13;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_pwm_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 13);
    v |= (0x1 & 0x1) << 13;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 13;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_pwm_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 13);
    v |= (0x0 & 0x1) << 13;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 13;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_msi_apb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 14;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_msi_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 14);
    v |= (0x1 & 0x1) << 14;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 14;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_msi_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 14);
    v |= (0x0 & 0x1) << 14;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 14;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_temp_apb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 15;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_temp_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 15);
    v |= (0x1 & 0x1) << 15;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 15;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_temp_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 15);
    v |= (0x0 & 0x1) << 15;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 15;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_temp_sense_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 16;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_temp_sense_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 16);
    v |= (0x1 & 0x1) << 16;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 16;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_temp_sense_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 16);
    v |= (0x0 & 0x1) << 16;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 16;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

pub fn _READ_RESET_STATUS_rstgen_rstn_syserr_apb_() -> u32 {
    let v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 17;
    v & 0x1
}

pub fn _ASSERT_RESET_rstgen_rstn_syserr_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 17);
    v |= (0x1 & 0x1) << 17;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 17;
        v &= 0x1;
        if !v != 0x0 {
            break;
        }
    }
}

pub fn _CLEAR_RESET_rstgen_rstn_syserr_apb_() -> () {
    let mut v = peek32(rstgen_Software_RESET_assert3_REG_ADDR);
    v &= !(0x1 << 17);
    v |= (0x0 & 0x1) << 17;
    poke32(rstgen_Software_RESET_assert3_REG_ADDR, v);
    loop {
        let mut v = peek32(rstgen_Software_RESET_status3_REG_ADDR) >> 17;
        v &= 0x1;
        if !v != 0x1 {
            break;
        }
    }
}

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

pub struct RSTgen {
    base: usize,
}
/*
impl ops::Deref for RSTgen {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}
*/
impl Driver for RSTgen {
    fn init(&mut self) -> Result<()> {
        /* nothing to do. */
        Ok(())
    }

    fn pread(&self, _data: &mut [u8], _offset: usize) -> Result<usize> {
        NOT_IMPLEMENTED
    }

    // so, yeah, this is maybe not the greatest idea ever.
    fn pwrite(&mut self, data: &[u8], _offset: usize) -> Result<usize> {
        match data {
            b"on" => {
                self.rstgen_init();
                Ok(1)
            }
            b"early" => {
                //                self.rstgen_early();
                Ok(0)
            }
            _ => Ok(0),
        }
    }

    fn shutdown(&mut self) {}
}

/* RSTgen initialization should only be done in romstage. */

impl RSTgen {
    pub fn new() -> RSTgen {
        RSTgen { base: 0 }
    }

    /// Returns a pointer to the register block
    fn ptr(&self) -> usize {
        self.base
    }
    fn rstgen_init(&mut self) {
        _CLEAR_RESET_rstgen_rstn_usbnoc_axi_();
        _CLEAR_RESET_rstgen_rstn_hifi4noc_axi_();

        _ENABLE_CLOCK_clk_x2c_axi_();
        _CLEAR_RESET_rstgen_rstn_x2c_axi_();

        _CLEAR_RESET_rstgen_rstn_dspx2c_axi_();
        _CLEAR_RESET_rstgen_rstn_dma1p_axi_();

        _ENABLE_CLOCK_clk_msi_apb_();
        _CLEAR_RESET_rstgen_rstn_msi_apb_();

        _ASSERT_RESET_rstgen_rstn_x2c_axi_();
        _CLEAR_RESET_rstgen_rstn_x2c_axi_();
        arch::fence();
    }
}
