// for now.
#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(non_upper_case_globals)]

/*
 * This file is part of the coreboot project.
 *
 * Copyright 2021 (C) The oreboot authors
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

/* SPDX-License-Identifier: GPL-2.0-or-later */
/**
 ******************************************************************************
 * @file  syscon_sysmain_ctrl_macro.h
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
//use core::ops;
use model::*;

//use crate::reg;
use core::ptr;
//use register::mmio::ReadWrite;
//use register::register_bitfields;

// No register block. I don't want to vet that all these
// things are nice and contiguous.
// :g/pub const (.*\): u32 = \(.*\)/s//pub const \1: u32 = \2;;
pub const SYSCON_SYSMAIN_CTRL_BASE_ADDR: u32 = 0x11850000;
pub const syscon_sysmain_ctrl_SCFG_pll0_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x0;
pub const syscon_sysmain_ctrl_SCFG_pll1_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x4;
pub const syscon_sysmain_ctrl_SCFG_pll2_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x8;
pub const syscon_sysmain_ctrl_SCFG_plls_stat_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0xC;
pub const syscon_sysmain_ctrl_register4_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x10;
pub const syscon_sysmain_ctrl_SCFG_u74_boot_vect0_low_REG_ADDR: u32 =
    SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x14;
pub const syscon_sysmain_ctrl_SCFG_u74_boot_vect0_hi_REG_ADDR: u32 =
    SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x18;
pub const syscon_sysmain_ctrl_SCFG_u74_boot_vect1_low_REG_ADDR: u32 =
    SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x1C;
pub const syscon_sysmain_ctrl_SCFG_u74_boot_vect1_hi_REG_ADDR: u32 =
    SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x20;
pub const syscon_sysmain_ctrl_SCFG_u74_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x24;
pub const syscon_sysmain_ctrl_register10_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x28;
pub const syscon_sysmain_ctrl_register11_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x2C;
pub const syscon_sysmain_ctrl_register12_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x30;
pub const syscon_sysmain_ctrl_register13_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x34;
pub const syscon_sysmain_ctrl_register14_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x38;
pub const syscon_sysmain_ctrl_register15_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x3C;
pub const syscon_sysmain_ctrl_register16_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x40;
pub const syscon_sysmain_ctrl_register17_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x44;
pub const syscon_sysmain_ctrl_register18_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x48;
pub const syscon_sysmain_ctrl_register19_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x4C;
pub const syscon_sysmain_ctrl_register20_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x50;
pub const syscon_sysmain_ctrl_register21_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x54;
pub const syscon_sysmain_ctrl_register22_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x58;
pub const syscon_sysmain_ctrl_register23_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x5C;
pub const syscon_sysmain_ctrl_qspi_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x60;
pub const syscon_sysmain_ctrl_intmem_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x64;
pub const syscon_sysmain_ctrl_register26_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x68;
pub const syscon_sysmain_ctrl_register27_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x6C;
pub const syscon_sysmain_ctrl_register28_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x70;
pub const syscon_sysmain_ctrl_register29_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x74;
pub const syscon_sysmain_ctrl_SCFG_gmac_timestamp0_REG_ADDR: u32 =
    SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x78;
pub const syscon_sysmain_ctrl_SCFG_gmac_timestamp1_REG_ADDR: u32 =
    SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x7C;
pub const syscon_sysmain_ctrl_register32_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x80;
pub const syscon_sysmain_ctrl_register33_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x84;
pub const syscon_sysmain_ctrl_register34_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x88;
pub const syscon_sysmain_ctrl_register35_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x8C;
pub const syscon_sysmain_ctrl_register36_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x90;
pub const syscon_sysmain_ctrl_register37_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x94;
pub const syscon_sysmain_ctrl_register38_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x98;
pub const syscon_sysmain_ctrl_register39_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x9C;
pub const syscon_sysmain_ctrl_register40_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0xA0;
pub const syscon_sysmain_ctrl_SCFG_intC1_7to0_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0xA4;
pub const syscon_sysmain_ctrl_SCFG_intC0_src15to8_REG_ADDR: u32 =
    SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0xA8;
pub const syscon_sysmain_ctrl_SCFG_intC0_src23to16_REG_ADDR: u32 =
    SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0xAC;
pub const syscon_sysmain_ctrl_SCFG_intC0_src31to24_REG_ADDR: u32 =
    SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0xB0;
pub const syscon_sysmain_ctrl_register47_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0xBC;
pub const syscon_sysmain_ctrl_register48_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0xC0;
pub const syscon_sysmain_ctrl_register52_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0xC4;
pub const syscon_sysmain_ctrl_register49_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0xC8;
pub const syscon_sysmain_ctrl_register50_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0xCC;
pub const syscon_sysmain_ctrl_register51_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0xD0;
pub const syscon_sysmain_ctrl_register66_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0xD8;
pub const syscon_sysmain_ctrl_register53_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0xDC;
pub const syscon_sysmain_ctrl_register54_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0xE0;
pub const syscon_sysmain_ctrl_register55_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0xE4;
pub const syscon_sysmain_ctrl_register56_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0xE8;
pub const syscon_sysmain_ctrl_register57_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0xEC;
pub const syscon_sysmain_ctrl_register58_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0xF0;
pub const syscon_sysmain_ctrl_register59_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0xF4;
pub const syscon_sysmain_ctrl_register60_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0xF8;
pub const syscon_sysmain_ctrl_register61_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0xFC;
pub const syscon_sysmain_ctrl_register62_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x100;
pub const syscon_sysmain_ctrl_register63_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x104;
pub const syscon_sysmain_ctrl_register64_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x108;
pub const syscon_sysmain_ctrl_register65_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x10C;
pub const syscon_sysmain_ctrl_register68_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x110;
pub const syscon_sysmain_ctrl_register67_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x114;
pub const syscon_sysmain_ctrl_register69_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x118;
pub const syscon_sysmain_ctrl_register70_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x11C;
pub const syscon_sysmain_ctrl_register71_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x120;
pub const syscon_sysmain_ctrl_register72_REG_ADDR: u32 = SYSCON_SYSMAIN_CTRL_BASE_ADDR + 0x124;

// :g/uint32_t .*=MA_INTW/let mut nv = peek32/
// :g/poke32/s/_ezchip.*value_/v/
// :g/poke32/s//poke32/

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

pub fn _SET_SYSCON_REG_SCFG_pll0_reset(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_pll0_REG_ADDR);
    nv &= !(0x1);
    nv |= (v & 0x1);
    poke32(syscon_sysmain_ctrl_SCFG_pll0_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_pll0_reset() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_pll0_REG_ADDR);
    v & 0x1
}

pub fn _SET_SYSCON_REG_SCFG_pll0_pwrdn(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_pll0_REG_ADDR);
    nv &= !(0x1 << 1);
    nv |= (v & 0x1) << 1;
    poke32(syscon_sysmain_ctrl_SCFG_pll0_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_pll0_pwrdn() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_pll0_REG_ADDR) >> 1;
    v & 0x1
}

pub fn _SET_SYSCON_REG_SCFG_pll0_intfb(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_pll0_REG_ADDR);
    nv &= !(0x1 << 2);
    nv |= (v & 0x1) << 2;
    poke32(syscon_sysmain_ctrl_SCFG_pll0_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_pll0_intfb() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_pll0_REG_ADDR) >> 2;
    v & 0x1
}

pub fn _SET_SYSCON_REG_SCFG_pll0_bypass(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_pll0_REG_ADDR);
    nv &= !(0x1 << 3);
    nv |= (v & 0x1) << 3;
    poke32(syscon_sysmain_ctrl_SCFG_pll0_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_pll0_bypass() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_pll0_REG_ADDR) >> 3;
    v & 0x1
}

pub fn _SET_SYSCON_REG_SCFG_pll0_clk_refdiv(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_pll0_REG_ADDR);
    nv &= !(0xF << 4);
    nv |= (v & 0xF) << 4;
    poke32(syscon_sysmain_ctrl_SCFG_pll0_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_pll0_clk_refdiv() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_pll0_REG_ADDR) >> 4;
    v & 0xf
}

pub fn _SET_SYSCON_REG_SCFG_pll0_clk_fbkdiv(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_pll0_REG_ADDR);
    nv &= !(0x3F << 8);
    nv |= (v & 0x3F) << 8;
    poke32(syscon_sysmain_ctrl_SCFG_pll0_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_pll0_clk_fbkdiv() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_pll0_REG_ADDR) >> 8;
    v & 0x3f
}

pub fn _SET_SYSCON_REG_SCFG_pll0_bw_adj(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_pll0_REG_ADDR);
    nv &= !(0x3F << 16);
    nv |= (v & 0x3F) << 16;
    poke32(syscon_sysmain_ctrl_SCFG_pll0_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_pll0_bw_adj() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_pll0_REG_ADDR) >> 16;
    v & 0x3f
}

pub fn _SET_SYSCON_REG_SCFG_pll0_clk_outdiv(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_pll0_REG_ADDR);
    nv &= !(0xF << 24);
    nv |= (v & 0xF) << 24;
    poke32(syscon_sysmain_ctrl_SCFG_pll0_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_pll0_clk_outdiv() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_pll0_REG_ADDR) >> 24;
    v & 0xf
}

pub fn _SET_SYSCON_REG_SCFG_pll1_reset(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_pll1_REG_ADDR);
    nv &= !(0x1);
    nv |= (v & 0x1);
    poke32(syscon_sysmain_ctrl_SCFG_pll1_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_pll1_reset() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_pll1_REG_ADDR);
    v & 0x1
}

pub fn _SET_SYSCON_REG_SCFG_pll1_pwrdn(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_pll1_REG_ADDR);
    nv &= !(0x1 << 1);
    nv |= (v & 0x1) << 1;
    poke32(syscon_sysmain_ctrl_SCFG_pll1_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_pll1_pwrdn() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_pll1_REG_ADDR) >> 1;
    v & 0x1
}

pub fn _SET_SYSCON_REG_SCFG_pll1_intfb(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_pll1_REG_ADDR);
    nv &= !(0x1 << 2);
    nv |= (v & 0x1) << 2;
    poke32(syscon_sysmain_ctrl_SCFG_pll1_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_pll1_intfb() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_pll1_REG_ADDR) >> 2;
    v & 0x1
}

pub fn _SET_SYSCON_REG_SCFG_pll1_bypass(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_pll1_REG_ADDR);
    nv &= !(0x1 << 3);
    nv |= (v & 0x1) << 3;
    poke32(syscon_sysmain_ctrl_SCFG_pll1_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_pll1_bypass() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_pll1_REG_ADDR) >> 3;
    v & 0x1
}

pub fn _SET_SYSCON_REG_SCFG_pll1_clk_refdiv(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_pll1_REG_ADDR);
    nv &= !(0xF << 4);
    nv |= (v & 0xF) << 4;
    poke32(syscon_sysmain_ctrl_SCFG_pll1_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_pll1_clk_refdiv() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_pll1_REG_ADDR) >> 4;
    v & 0xf
}

pub fn _SET_SYSCON_REG_SCFG_pll1_clk_fbkdiv(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_pll1_REG_ADDR);
    nv &= !(0x3F << 8);
    nv |= (v & 0x3F) << 8;
    poke32(syscon_sysmain_ctrl_SCFG_pll1_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_pll1_clk_fbkdiv() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_pll1_REG_ADDR) >> 8;
    v & 0x3f
}

pub fn _SET_SYSCON_REG_SCFG_pll1_bw_adj(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_pll1_REG_ADDR);
    nv &= !(0x3F << 16);
    nv |= (v & 0x3F) << 16;
    poke32(syscon_sysmain_ctrl_SCFG_pll1_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_pll1_bw_adj() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_pll1_REG_ADDR) >> 16;
    v & 0x3f
}

pub fn _SET_SYSCON_REG_SCFG_pll1_clk_outdiv(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_pll1_REG_ADDR);
    nv &= !(0xF << 24);
    nv |= (v & 0xF) << 24;
    poke32(syscon_sysmain_ctrl_SCFG_pll1_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_pll1_clk_outdiv() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_pll1_REG_ADDR) >> 24;
    v & 0xf
}

pub fn _SET_SYSCON_REG_SCFG_pll2_reset(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_pll2_REG_ADDR);
    nv &= !(0x1);
    nv |= (v & 0x1);
    poke32(syscon_sysmain_ctrl_SCFG_pll2_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_pll2_reset() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_pll2_REG_ADDR);
    v & 0x1
}

pub fn _SET_SYSCON_REG_SCFG_pll2_pwrdn(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_pll2_REG_ADDR);
    nv &= !(0x1 << 1);
    nv |= (v & 0x1) << 1;
    poke32(syscon_sysmain_ctrl_SCFG_pll2_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_pll2_pwrdn() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_pll2_REG_ADDR) >> 1;
    v & 0x1
}

pub fn _SET_SYSCON_REG_SCFG_pll2_intfb(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_pll2_REG_ADDR);
    nv &= !(0x1 << 2);
    nv |= (v & 0x1) << 2;
    poke32(syscon_sysmain_ctrl_SCFG_pll2_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_pll2_intfb() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_pll2_REG_ADDR) >> 2;
    v & 0x1
}

pub fn _SET_SYSCON_REG_SCFG_pll2_bypass(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_pll2_REG_ADDR);
    nv &= !(0x1 << 3);
    nv |= (v & 0x1) << 3;
    poke32(syscon_sysmain_ctrl_SCFG_pll2_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_pll2_bypass() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_pll2_REG_ADDR) >> 3;
    v & 0x1
}

pub fn _SET_SYSCON_REG_SCFG_pll2_clk_refdiv(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_pll2_REG_ADDR);
    nv &= !(0xF << 4);
    nv |= (v & 0xF) << 4;
    poke32(syscon_sysmain_ctrl_SCFG_pll2_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_pll2_clk_refdiv() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_pll2_REG_ADDR) >> 4;
    v & 0xf
}

pub fn _SET_SYSCON_REG_SCFG_pll2_clk_fbkdiv(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_pll2_REG_ADDR);
    nv &= !(0x3F << 8);
    nv |= (v & 0x3F) << 8;
    poke32(syscon_sysmain_ctrl_SCFG_pll2_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_pll2_clk_fbkdiv() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_pll2_REG_ADDR) >> 8;
    v & 0x3f
}

pub fn _SET_SYSCON_REG_SCFG_pll2_bw_adj(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_pll2_REG_ADDR);
    nv &= !(0x3F << 16);
    nv |= (v & 0x3F) << 16;
    poke32(syscon_sysmain_ctrl_SCFG_pll2_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_pll2_bw_adj() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_pll2_REG_ADDR) >> 16;
    v & 0x3f
}

pub fn _SET_SYSCON_REG_SCFG_pll2_clk_outdiv(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_pll2_REG_ADDR);
    nv &= !(0xF << 24);
    nv |= (v & 0xF) << 24;
    poke32(syscon_sysmain_ctrl_SCFG_pll2_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_pll2_clk_outdiv() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_pll2_REG_ADDR) >> 24;
    v & 0xf
}

pub fn _SET_SYSCON_REG_SCFG_plls_stat_pll0_test(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_plls_stat_REG_ADDR);
    nv &= !(0x1);
    nv |= (v & 0x1);
    poke32(syscon_sysmain_ctrl_SCFG_plls_stat_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_plls_stat_pll0_test() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_plls_stat_REG_ADDR);
    v & 0x1
}

pub fn _SET_SYSCON_REG_SCFG_plls_stat_pll1_test(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_plls_stat_REG_ADDR);
    nv &= !(0x1 << 1);
    nv |= (v & 0x1) << 1;
    poke32(syscon_sysmain_ctrl_SCFG_plls_stat_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_plls_stat_pll1_test() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_plls_stat_REG_ADDR) >> 1;
    v & 0x1
}

pub fn _SET_SYSCON_REG_SCFG_plls_stat_pll2_test(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_plls_stat_REG_ADDR);
    nv &= !(0x1 << 2);
    nv |= (v & 0x1) << 2;
    poke32(syscon_sysmain_ctrl_SCFG_plls_stat_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_plls_stat_pll2_test() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_plls_stat_REG_ADDR) >> 2;
    v & 0x1
}

pub fn _GET_SYSCON_REG_SCFG_plls_stat_pll0_lock() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_plls_stat_REG_ADDR) >> 4;
    v & 0x1
}

pub fn _GET_SYSCON_REG_SCFG_plls_stat_pll0_ref_slip() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_plls_stat_REG_ADDR) >> 5;
    v & 0x1
}

pub fn _GET_SYSCON_REG_SCFG_plls_stat_pll0_fdbk_slip() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_plls_stat_REG_ADDR) >> 6;
    v & 0x1
}

pub fn _GET_SYSCON_REG_SCFG_plls_stat_pll1_lock() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_plls_stat_REG_ADDR) >> 8;
    v & 0x1
}

pub fn _GET_SYSCON_REG_SCFG_plls_stat_pll1_ref_slip() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_plls_stat_REG_ADDR) >> 9;
    v & 0x1
}

pub fn _GET_SYSCON_REG_SCFG_plls_stat_pll1_fdbk_slip() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_plls_stat_REG_ADDR) >> 10;
    v & 0x1
}

pub fn _GET_SYSCON_REG_SCFG_plls_stat_pll2_lock() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_plls_stat_REG_ADDR) >> 12;
    v & 0x1
}

pub fn _GET_SYSCON_REG_SCFG_plls_stat_pll2_ref_slip() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_plls_stat_REG_ADDR) >> 13;
    v & 0x1
}

pub fn _GET_SYSCON_REG_SCFG_plls_stat_pll2_fdbk_slip() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_plls_stat_REG_ADDR) >> 14;
    v & 0x1
}

pub fn _GET_SYSCON_REG_register4_SCFG_u74_halt_from_tile0() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register4_REG_ADDR);
    v & 0x1
}

pub fn _GET_SYSCON_REG_register4_SCFG_u74_halt_from_tile1() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register4_REG_ADDR) >> 1;
    v & 0x1
}

pub fn _GET_SYSCON_REG_register4_SCFG_u74_debug_ndreset() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register4_REG_ADDR) >> 2;
    v & 0x1
}

pub fn _GET_SYSCON_REG_register4_SCFG_u74_debug_dmactive() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register4_REG_ADDR) >> 3;
    v & 0x1
}

pub fn _SET_SYSCON_REG_SCFG_u74_boot_vect0_low_b32(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_u74_boot_vect0_low_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_sysmain_ctrl_SCFG_u74_boot_vect0_low_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_u74_boot_vect0_low_b32() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_u74_boot_vect0_low_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_SCFG_u74_boot_vect0_hi_b6(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_u74_boot_vect0_hi_REG_ADDR);
    nv &= !(0x3F);
    nv |= (v & 0x3F);
    poke32(syscon_sysmain_ctrl_SCFG_u74_boot_vect0_hi_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_u74_boot_vect0_hi_b6() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_u74_boot_vect0_hi_REG_ADDR);
    v & 0x3f
}

pub fn _SET_SYSCON_REG_SCFG_u74_boot_vect1_low_b32(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_u74_boot_vect1_low_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_sysmain_ctrl_SCFG_u74_boot_vect1_low_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_u74_boot_vect1_low_b32() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_u74_boot_vect1_low_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_SCFG_u74_boot_vect1_hi_b6(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_u74_boot_vect1_hi_REG_ADDR);
    nv &= !(0x3F);
    nv |= (v & 0x3F);
    poke32(syscon_sysmain_ctrl_SCFG_u74_boot_vect1_hi_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_u74_boot_vect1_hi_b6() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_u74_boot_vect1_hi_REG_ADDR);
    v & 0x3f
}

pub fn _SET_SYSCON_REG_SCFG_u74_PRID(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_u74_REG_ADDR);
    nv &= !(0x7FF);
    nv |= (v & 0x7FF);
    poke32(syscon_sysmain_ctrl_SCFG_u74_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_u74_PRID() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_u74_REG_ADDR);
    v & 0x7ff
}

pub fn _GET_SYSCON_REG_register10_e24_halt() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register10_REG_ADDR);
    v & 0x1
}

pub fn _GET_SYSCON_REG_register10_e24_dbg_reset() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register10_REG_ADDR) >> 1;
    v & 0x1
}

pub fn _GET_SYSCON_REG_register10_e24_dbg_active() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register10_REG_ADDR) >> 2;
    v & 0x1
}

pub fn _SET_SYSCON_REG_register11_SCFG_nbdla_pwrbus_ram_a_pd(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register11_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_sysmain_ctrl_register11_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register11_SCFG_nbdla_pwrbus_ram_a_pd() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register11_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register12_SCFG_nbdla_pwrbus_ram_c_pd(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register12_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_sysmain_ctrl_register12_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register12_SCFG_nbdla_pwrbus_ram_c_pd() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register12_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register13_SCFG_nbdla_pwrbus_ram_o_pd(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register13_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_sysmain_ctrl_register13_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register13_SCFG_nbdla_pwrbus_ram_o_pd() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register13_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register14_SCFG_nbdla_pwrbus_ram_p_pd(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register14_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_sysmain_ctrl_register14_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register14_SCFG_nbdla_pwrbus_ram_p_pd() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register14_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register15_SCFG_nbdla_pwrbus_ram_x_pd(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register15_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_sysmain_ctrl_register15_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register15_SCFG_nbdla_pwrbus_ram_x_pd() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register15_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register16_SCFG_nbdla_globclk_ovr_on(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register16_REG_ADDR);
    nv &= !(0x1);
    nv |= (v & 0x1);
    poke32(syscon_sysmain_ctrl_register16_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register16_SCFG_nbdla_globclk_ovr_on() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register16_REG_ADDR);
    v & 0x1
}

pub fn _SET_SYSCON_REG_register16_SCFG_nbdla_disable_clock_gating(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register16_REG_ADDR);
    nv &= !(0x1 << 1);
    nv |= (v & 0x1) << 1;
    poke32(syscon_sysmain_ctrl_register16_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register16_SCFG_nbdla_disable_clock_gating() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register16_REG_ADDR) >> 1;
    v & 0x1
}

pub fn _SET_SYSCON_REG_register16_SCFG_nbdla_direct_reset(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register16_REG_ADDR);
    nv &= !(0x1 << 2);
    nv |= (v & 0x1) << 2;
    poke32(syscon_sysmain_ctrl_register16_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register16_SCFG_nbdla_direct_reset() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register16_REG_ADDR) >> 2;
    v & 0x1
}

pub fn _SET_SYSCON_REG_register16_SCFG_nbdla_clkgating_en(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register16_REG_ADDR);
    nv &= !(0x1 << 3);
    nv |= (v & 0x1) << 3;
    poke32(syscon_sysmain_ctrl_register16_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register16_SCFG_nbdla_clkgating_en() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register16_REG_ADDR) >> 3;
    v & 0x1
}

pub fn _GET_SYSCON_REG_register17_SCFG_jpegc_cur_inst_a() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register17_REG_ADDR);
    v & 0x3
}

pub fn _GET_SYSCON_REG_register18_SCFG_wave511_vpu_idle() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register18_REG_ADDR);
    v & 0x1
}

pub fn _GET_SYSCON_REG_register19_SCFG_wave521_vpu_idle() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register19_REG_ADDR);
    v & 0x1
}

pub fn _SET_SYSCON_REG_register20_u0_syscon_162_SCFG_gc300_csys_req(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register20_REG_ADDR);
    nv &= !(0x1);
    nv |= (v & 0x1);
    poke32(syscon_sysmain_ctrl_register20_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register20_u0_syscon_162_SCFG_gc300_csys_req() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register20_REG_ADDR);
    v & 0x1
}

pub fn _GET_SYSCON_REG_register21_u0_syscon_162_SCFG_gc300_cactive() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register21_REG_ADDR);
    v & 0x1
}

pub fn _GET_SYSCON_REG_register21_u0_syscon_162_SCFG_gc300_csys_ack() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register21_REG_ADDR) >> 1;
    v & 0x1
}

pub fn _GET_SYSCON_REG_register22_u0_syscon_162_SCFG_gc300_debug_out() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register22_REG_ADDR);
    v & 0xff
}

pub fn _SET_SYSCON_REG_register23_SCFG_cmsensor_rst0(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register23_REG_ADDR);
    nv &= !(0x1);
    nv |= (v & 0x1);
    poke32(syscon_sysmain_ctrl_register23_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register23_SCFG_cmsensor_rst0() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register23_REG_ADDR);
    v & 0x1
}

pub fn _SET_SYSCON_REG_register23_SCFG_cmsensor_rst1(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register23_REG_ADDR);
    nv &= !(0x1 << 1);
    nv |= (v & 0x1) << 1;
    poke32(syscon_sysmain_ctrl_register23_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register23_SCFG_cmsensor_rst1() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register23_REG_ADDR) >> 1;
    v & 0x1
}

pub fn _SET_SYSCON_REG_qspi_SCFG_sram_config(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_qspi_REG_ADDR);
    nv &= !(0xFF);
    nv |= (v & 0xFF);
    poke32(syscon_sysmain_ctrl_qspi_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_qspi_SCFG_sram_config() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_qspi_REG_ADDR);
    v & 0xff
}

pub fn _SET_SYSCON_REG_intmem_SCFG_sram_config(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_intmem_REG_ADDR);
    nv &= !(0xFF);
    nv |= (v & 0xFF);
    poke32(syscon_sysmain_ctrl_intmem_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_intmem_SCFG_sram_config() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_intmem_REG_ADDR);
    v & 0xff
}

pub fn _SET_SYSCON_REG_intmem_SCFG_sram_config_rom(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_intmem_REG_ADDR);
    nv &= !(0xFF << 8);
    nv |= (v & 0xFF) << 8;
    poke32(syscon_sysmain_ctrl_intmem_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_intmem_SCFG_sram_config_rom() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_intmem_REG_ADDR) >> 8;
    v & 0xff
}

pub fn _SET_SYSCON_REG_register26_SCFG_dma1p2p_sel(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register26_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_sysmain_ctrl_register26_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register26_SCFG_dma1p2p_sel() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register26_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register27_SCFG_dmaezMst_sel(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register27_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_sysmain_ctrl_register27_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register27_SCFG_dmaezMst_sel() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register27_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register28_SCFG_gmac_phy_intf_sel(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register28_REG_ADDR);
    nv &= !(0x7);
    nv |= (v & 0x7);
    poke32(syscon_sysmain_ctrl_register28_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register28_SCFG_gmac_phy_intf_sel() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register28_REG_ADDR);
    v & 0x7
}

pub fn _SET_SYSCON_REG_register28_gmac_SCFG_sram_cfg(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register28_REG_ADDR);
    nv &= !(0xFF << 4);
    nv |= (v & 0xFF) << 4;
    poke32(syscon_sysmain_ctrl_register28_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register28_gmac_SCFG_sram_cfg() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register28_REG_ADDR) >> 4;
    v & 0xff
}

pub fn _GET_SYSCON_REG_register29_gmac_speed() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register29_REG_ADDR);
    v & 0x3
}

pub fn _GET_SYSCON_REG_register29_gmac_ptp_pps() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register29_REG_ADDR) >> 2;
    v & 0x1
}

pub fn _GET_SYSCON_REG_register29_gmac_tx_ckg_ctrl() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register29_REG_ADDR) >> 3;
    v & 0x1
}

pub fn _GET_SYSCON_REG_SCFG_gmac_timestamp0_ptp() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_gmac_timestamp0_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _GET_SYSCON_REG_SCFG_gmac_timestamp1_ptp() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_gmac_timestamp1_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register32_SCFG_gmac_phy_rstn(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register32_REG_ADDR);
    nv &= !(0x1);
    nv |= (v & 0x1);
    poke32(syscon_sysmain_ctrl_register32_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register32_SCFG_gmac_phy_rstn() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register32_REG_ADDR);
    v & 0x1
}

pub fn _SET_SYSCON_REG_register33_SCFG_sdio0_hbig_endian(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register33_REG_ADDR);
    nv &= !(0x1);
    nv |= (v & 0x1);
    poke32(syscon_sysmain_ctrl_register33_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register33_SCFG_sdio0_hbig_endian() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register33_REG_ADDR);
    v & 0x1
}

pub fn _SET_SYSCON_REG_register33_SCFG_sdio0_m_hbig_endian(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register33_REG_ADDR);
    nv &= !(0x1 << 1);
    nv |= (v & 0x1) << 1;
    poke32(syscon_sysmain_ctrl_register33_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register33_SCFG_sdio0_m_hbig_endian() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register33_REG_ADDR) >> 1;
    v & 0x1
}

pub fn _SET_SYSCON_REG_register33_sdio0_SCFG_sram_config(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register33_REG_ADDR);
    nv &= !(0xFF << 2);
    nv |= (v & 0xFF) << 2;
    poke32(syscon_sysmain_ctrl_register33_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register33_sdio0_SCFG_sram_config() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register33_REG_ADDR) >> 2;
    v & 0xff
}

pub fn _SET_SYSCON_REG_register34_SCFG_sdio1_hbig_endian(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register34_REG_ADDR);
    nv &= !(0x1);
    nv |= (v & 0x1);
    poke32(syscon_sysmain_ctrl_register34_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register34_SCFG_sdio1_hbig_endian() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register34_REG_ADDR);
    v & 0x1
}

pub fn _SET_SYSCON_REG_register34_SCFG_sdio1_m_hbig_endian(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register34_REG_ADDR);
    nv &= !(0x1 << 1);
    nv |= (v & 0x1) << 1;
    poke32(syscon_sysmain_ctrl_register34_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register34_SCFG_sdio1_m_hbig_endian() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register34_REG_ADDR) >> 1;
    v & 0x1
}

pub fn _SET_SYSCON_REG_register34_sdio1_SCFG_sram_config(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register34_REG_ADDR);
    nv &= !(0xFF << 2);
    nv |= (v & 0xFF) << 2;
    poke32(syscon_sysmain_ctrl_register34_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register34_sdio1_SCFG_sram_config() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register34_REG_ADDR) >> 2;
    v & 0xff
}

pub fn _SET_SYSCON_REG_register35_SCFG_spi2ahb_mode(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register35_REG_ADDR);
    nv &= !(0x3);
    nv |= (v & 0x3);
    poke32(syscon_sysmain_ctrl_register35_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register35_SCFG_spi2ahb_mode() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register35_REG_ADDR);
    v & 0x3
}

pub fn _GET_SYSCON_REG_register36_spi2ahb_sleep() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register36_REG_ADDR);
    v & 0x1
}

pub fn _SET_SYSCON_REG_register37_ezmst_SCFG_sram_config(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register37_REG_ADDR);
    nv &= !(0xFF);
    nv |= (v & 0xFF);
    poke32(syscon_sysmain_ctrl_register37_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register37_ezmst_SCFG_sram_config() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register37_REG_ADDR);
    v & 0xff
}

pub fn _SET_SYSCON_REG_register38_sec_SCFG_sram_cfg(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register38_REG_ADDR);
    nv &= !(0xFF);
    nv |= (v & 0xFF);
    poke32(syscon_sysmain_ctrl_register38_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register38_sec_SCFG_sram_cfg() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register38_REG_ADDR);
    v & 0xff
}

pub fn _SET_SYSCON_REG_register39_uart0_SCFG_sram_config(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register39_REG_ADDR);
    nv &= !(0xFF);
    nv |= (v & 0xFF);
    poke32(syscon_sysmain_ctrl_register39_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register39_uart0_SCFG_sram_config() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register39_REG_ADDR);
    v & 0xff
}

pub fn _SET_SYSCON_REG_register39_uart1_SCFG_sram_config(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register39_REG_ADDR);
    nv &= !(0xFF << 8);
    nv |= (v & 0xFF) << 8;
    poke32(syscon_sysmain_ctrl_register39_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register39_uart1_SCFG_sram_config() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register39_REG_ADDR) >> 8;
    v & 0xff
}

pub fn _GET_SYSCON_REG_register40_trng_secure_mode() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register40_REG_ADDR);
    v & 0x1
}

pub fn _GET_SYSCON_REG_register40_trng_nonce_mode() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register40_REG_ADDR) >> 1;
    v & 0x1
}

pub fn _SET_SYSCON_REG_SCFG_intC1_7to0_int_src1(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_intC1_7to0_REG_ADDR);
    nv &= !(0xFF);
    nv |= (v & 0xFF);
    poke32(syscon_sysmain_ctrl_SCFG_intC1_7to0_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_intC1_7to0_int_src1() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_intC1_7to0_REG_ADDR);
    v & 0xff
}

pub fn _SET_SYSCON_REG_SCFG_intC0_src15to8_int_src1(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_intC0_src15to8_REG_ADDR);
    nv &= !(0xFF);
    nv |= (v & 0xFF);
    poke32(syscon_sysmain_ctrl_SCFG_intC0_src15to8_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_intC0_src15to8_int_src1() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_intC0_src15to8_REG_ADDR);
    v & 0xff
}

pub fn _SET_SYSCON_REG_SCFG_intC0_src23to16_int_src1(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_intC0_src23to16_REG_ADDR);
    nv &= !(0xFF);
    nv |= (v & 0xFF);
    poke32(syscon_sysmain_ctrl_SCFG_intC0_src23to16_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_intC0_src23to16_int_src1() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_intC0_src23to16_REG_ADDR);
    v & 0xff
}

pub fn _SET_SYSCON_REG_SCFG_intC0_src31to24_int_src1(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_SCFG_intC0_src31to24_REG_ADDR);
    nv &= !(0xFF);
    nv |= (v & 0xFF);
    poke32(syscon_sysmain_ctrl_SCFG_intC0_src31to24_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_SCFG_intC0_src31to24_int_src1() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_SCFG_intC0_src31to24_REG_ADDR);
    v & 0xff
}

pub fn _SET_SYSCON_REG_register47_e24_reset_vector(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register47_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_sysmain_ctrl_register47_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register47_e24_reset_vector() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register47_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register48_SCFG_qspi_sclk_dlychain_sel(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register48_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_sysmain_ctrl_register48_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register48_SCFG_qspi_sclk_dlychain_sel() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register48_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register52_SCFG_gmac_rxclk_dlychain_sel(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register52_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_sysmain_ctrl_register52_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register52_SCFG_gmac_rxclk_dlychain_sel() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register52_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register49_SCFG_gmac_gtxclk_dlychain_sel(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register49_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_sysmain_ctrl_register49_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register49_SCFG_gmac_gtxclk_dlychain_sel() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register49_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register50_SCFG_sdio0_cclk_dlychain_sel(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register50_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_sysmain_ctrl_register50_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register50_SCFG_sdio0_cclk_dlychain_sel() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register50_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register51_SCFG_sdio1_cclk_dlychain_sel(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register51_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_sysmain_ctrl_register51_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register51_SCFG_sdio1_cclk_dlychain_sel() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register51_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register66_SCFG_axi_cache_sel(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register66_REG_ADDR);
    nv &= !(0xFF);
    nv |= (v & 0xFF);
    poke32(syscon_sysmain_ctrl_register66_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register66_SCFG_axi_cache_sel() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register66_REG_ADDR);
    v & 0xff
}

pub fn _SET_SYSCON_REG_register66_SCFG_default_arcache(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register66_REG_ADDR);
    nv &= !(0xF << 8);
    nv |= (v & 0xF) << 8;
    poke32(syscon_sysmain_ctrl_register66_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register66_SCFG_default_arcache() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register66_REG_ADDR) >> 8;
    v & 0xf
}

pub fn _SET_SYSCON_REG_register66_SCFG_default_awcache(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register66_REG_ADDR);
    nv &= !(0xF << 12);
    nv |= (v & 0xF) << 12;
    poke32(syscon_sysmain_ctrl_register66_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register66_SCFG_default_awcache() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register66_REG_ADDR) >> 12;
    v & 0xf
}

pub fn _SET_SYSCON_REG_register53_SCFG_axi_cache_sel(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register53_REG_ADDR);
    nv &= !(0xFF);
    nv |= (v & 0xFF);
    poke32(syscon_sysmain_ctrl_register53_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register53_SCFG_axi_cache_sel() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register53_REG_ADDR);
    v & 0xff
}

pub fn _SET_SYSCON_REG_register53_SCFG_default_arcache(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register53_REG_ADDR);
    nv &= !(0xF << 8);
    nv |= (v & 0xF) << 8;
    poke32(syscon_sysmain_ctrl_register53_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register53_SCFG_default_arcache() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register53_REG_ADDR) >> 8;
    v & 0xf
}

pub fn _SET_SYSCON_REG_register53_SCFG_default_awcache(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register53_REG_ADDR);
    nv &= !(0xF << 12);
    nv |= (v & 0xF) << 12;
    poke32(syscon_sysmain_ctrl_register53_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register53_SCFG_default_awcache() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register53_REG_ADDR) >> 12;
    v & 0xf
}

pub fn _SET_SYSCON_REG_register54_SCFG_axi_cache_sel(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register54_REG_ADDR);
    nv &= !(0xFF);
    nv |= (v & 0xFF);
    poke32(syscon_sysmain_ctrl_register54_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register54_SCFG_axi_cache_sel() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register54_REG_ADDR);
    v & 0xff
}

pub fn _SET_SYSCON_REG_register54_SCFG_default_arcache(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register54_REG_ADDR);
    nv &= !(0xF << 8);
    nv |= (v & 0xF) << 8;
    poke32(syscon_sysmain_ctrl_register54_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register54_SCFG_default_arcache() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register54_REG_ADDR) >> 8;
    v & 0xf
}

pub fn _SET_SYSCON_REG_register54_SCFG_default_awcache(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register54_REG_ADDR);
    nv &= !(0xF << 12);
    nv |= (v & 0xF) << 12;
    poke32(syscon_sysmain_ctrl_register54_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register54_SCFG_default_awcache() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register54_REG_ADDR) >> 12;
    v & 0xf
}

pub fn _SET_SYSCON_REG_register55_SCFG_axi_cache_sel(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register55_REG_ADDR);
    nv &= !(0xFF);
    nv |= (v & 0xFF);
    poke32(syscon_sysmain_ctrl_register55_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register55_SCFG_axi_cache_sel() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register55_REG_ADDR);
    v & 0xff
}

pub fn _SET_SYSCON_REG_register55_SCFG_default_arcache(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register55_REG_ADDR);
    nv &= !(0xF << 8);
    nv |= (v & 0xF) << 8;
    poke32(syscon_sysmain_ctrl_register55_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register55_SCFG_default_arcache() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register55_REG_ADDR) >> 8;
    v & 0xf
}

pub fn _SET_SYSCON_REG_register55_SCFG_default_awcache(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register55_REG_ADDR);
    nv &= !(0xF << 12);
    nv |= (v & 0xF) << 12;
    poke32(syscon_sysmain_ctrl_register55_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register55_SCFG_default_awcache() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register55_REG_ADDR) >> 12;
    v & 0xf
}

pub fn _SET_SYSCON_REG_register56_SCFG_axi_cache_sel(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register56_REG_ADDR);
    nv &= !(0xFF);
    nv |= (v & 0xFF);
    poke32(syscon_sysmain_ctrl_register56_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register56_SCFG_axi_cache_sel() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register56_REG_ADDR);
    v & 0xff
}

pub fn _SET_SYSCON_REG_register56_SCFG_default_arcache(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register56_REG_ADDR);
    nv &= !(0xF << 8);
    nv |= (v & 0xF) << 8;
    poke32(syscon_sysmain_ctrl_register56_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register56_SCFG_default_arcache() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register56_REG_ADDR) >> 8;
    v & 0xf
}

pub fn _SET_SYSCON_REG_register56_SCFG_default_awcache(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register56_REG_ADDR);
    nv &= !(0xF << 12);
    nv |= (v & 0xF) << 12;
    poke32(syscon_sysmain_ctrl_register56_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register56_SCFG_default_awcache() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register56_REG_ADDR) >> 12;
    v & 0xf
}

pub fn _SET_SYSCON_REG_register57_SCFG_axi_cache_sel(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register57_REG_ADDR);
    nv &= !(0xFF);
    nv |= (v & 0xFF);
    poke32(syscon_sysmain_ctrl_register57_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register57_SCFG_axi_cache_sel() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register57_REG_ADDR);
    v & 0xff
}

pub fn _SET_SYSCON_REG_register57_SCFG_default_arcache(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register57_REG_ADDR);
    nv &= !(0xF << 8);
    nv |= (v & 0xF) << 8;
    poke32(syscon_sysmain_ctrl_register57_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register57_SCFG_default_arcache() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register57_REG_ADDR) >> 8;
    v & 0xf
}

pub fn _SET_SYSCON_REG_register57_SCFG_default_awcache(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register57_REG_ADDR);
    nv &= !(0xF << 12);
    nv |= (v & 0xF) << 12;
    poke32(syscon_sysmain_ctrl_register57_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register57_SCFG_default_awcache() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register57_REG_ADDR) >> 12;
    v & 0xf
}

pub fn _SET_SYSCON_REG_register58_SCFG_axi_cache_sel(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register58_REG_ADDR);
    nv &= !(0xFF);
    nv |= (v & 0xFF);
    poke32(syscon_sysmain_ctrl_register58_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register58_SCFG_axi_cache_sel() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register58_REG_ADDR);
    v & 0xff
}

pub fn _SET_SYSCON_REG_register58_SCFG_default_arcache(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register58_REG_ADDR);
    nv &= !(0xF << 8);
    nv |= (v & 0xF) << 8;
    poke32(syscon_sysmain_ctrl_register58_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register58_SCFG_default_arcache() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register58_REG_ADDR) >> 8;
    v & 0xf
}

pub fn _SET_SYSCON_REG_register58_SCFG_default_awcache(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register58_REG_ADDR);
    nv &= !(0xF << 12);
    nv |= (v & 0xF) << 12;
    poke32(syscon_sysmain_ctrl_register58_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register58_SCFG_default_awcache() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register58_REG_ADDR) >> 12;
    v & 0xf
}

pub fn _SET_SYSCON_REG_register59_SCFG_axi_cache_sel(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register59_REG_ADDR);
    nv &= !(0xFF);
    nv |= (v & 0xFF);
    poke32(syscon_sysmain_ctrl_register59_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register59_SCFG_axi_cache_sel() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register59_REG_ADDR);
    v & 0xff
}

pub fn _SET_SYSCON_REG_register59_SCFG_default_arcache(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register59_REG_ADDR);
    nv &= !(0xF << 8);
    nv |= (v & 0xF) << 8;
    poke32(syscon_sysmain_ctrl_register59_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register59_SCFG_default_arcache() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register59_REG_ADDR) >> 8;
    v & 0xf
}

pub fn _SET_SYSCON_REG_register59_SCFG_default_awcache(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register59_REG_ADDR);
    nv &= !(0xF << 12);
    nv |= (v & 0xF) << 12;
    poke32(syscon_sysmain_ctrl_register59_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register59_SCFG_default_awcache() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register59_REG_ADDR) >> 12;
    v & 0xf
}

pub fn _SET_SYSCON_REG_register60_SCFG_axi_cache_sel(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register60_REG_ADDR);
    nv &= !(0xFF);
    nv |= (v & 0xFF);
    poke32(syscon_sysmain_ctrl_register60_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register60_SCFG_axi_cache_sel() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register60_REG_ADDR);
    v & 0xff
}

pub fn _SET_SYSCON_REG_register60_SCFG_default_arcache(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register60_REG_ADDR);
    nv &= !(0xF << 8);
    nv |= (v & 0xF) << 8;
    poke32(syscon_sysmain_ctrl_register60_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register60_SCFG_default_arcache() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register60_REG_ADDR) >> 8;
    v & 0xf
}

pub fn _SET_SYSCON_REG_register60_SCFG_default_awcache(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register60_REG_ADDR);
    nv &= !(0xF << 12);
    nv |= (v & 0xF) << 12;
    poke32(syscon_sysmain_ctrl_register60_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register60_SCFG_default_awcache() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register60_REG_ADDR) >> 12;
    v & 0xf
}

pub fn _SET_SYSCON_REG_register61_SCFG_axi_cache_sel(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register61_REG_ADDR);
    nv &= !(0xFF);
    nv |= (v & 0xFF);
    poke32(syscon_sysmain_ctrl_register61_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register61_SCFG_axi_cache_sel() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register61_REG_ADDR);
    v & 0xff
}

pub fn _SET_SYSCON_REG_register61_SCFG_default_arcache(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register61_REG_ADDR);
    nv &= !(0xF << 8);
    nv |= (v & 0xF) << 8;
    poke32(syscon_sysmain_ctrl_register61_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register61_SCFG_default_arcache() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register61_REG_ADDR) >> 8;
    v & 0xf
}

pub fn _SET_SYSCON_REG_register61_SCFG_default_awcache(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register61_REG_ADDR);
    nv &= !(0xF << 12);
    nv |= (v & 0xF) << 12;
    poke32(syscon_sysmain_ctrl_register61_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register61_SCFG_default_awcache() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register61_REG_ADDR) >> 12;
    v & 0xf
}

pub fn _SET_SYSCON_REG_register62_SCFG_axi_cache_sel(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register62_REG_ADDR);
    nv &= !(0xFF);
    nv |= (v & 0xFF);
    poke32(syscon_sysmain_ctrl_register62_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register62_SCFG_axi_cache_sel() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register62_REG_ADDR);
    v & 0xff
}

pub fn _SET_SYSCON_REG_register62_SCFG_default_arcache(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register62_REG_ADDR);
    nv &= !(0xF << 8);
    nv |= (v & 0xF) << 8;
    poke32(syscon_sysmain_ctrl_register62_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register62_SCFG_default_arcache() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register62_REG_ADDR) >> 8;
    v & 0xf
}

pub fn _SET_SYSCON_REG_register62_SCFG_default_awcache(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register62_REG_ADDR);
    nv &= !(0xF << 12);
    nv |= (v & 0xF) << 12;
    poke32(syscon_sysmain_ctrl_register62_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register62_SCFG_default_awcache() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register62_REG_ADDR) >> 12;
    v & 0xf
}

pub fn _SET_SYSCON_REG_register63_SCFG_axi_cache_sel(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register63_REG_ADDR);
    nv &= !(0xFF);
    nv |= (v & 0xFF);
    poke32(syscon_sysmain_ctrl_register63_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register63_SCFG_axi_cache_sel() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register63_REG_ADDR);
    v & 0xff
}

pub fn _SET_SYSCON_REG_register63_SCFG_default_arcache(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register63_REG_ADDR);
    nv &= !(0xF << 8);
    nv |= (v & 0xF) << 8;
    poke32(syscon_sysmain_ctrl_register63_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register63_SCFG_default_arcache() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register63_REG_ADDR) >> 8;
    v & 0xf
}

pub fn _SET_SYSCON_REG_register63_SCFG_default_awcache(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register63_REG_ADDR);
    nv &= !(0xF << 12);
    nv |= (v & 0xF) << 12;
    poke32(syscon_sysmain_ctrl_register63_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register63_SCFG_default_awcache() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register63_REG_ADDR) >> 12;
    v & 0xf
}

pub fn _SET_SYSCON_REG_register64_SCFG_axi_cache_sel(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register64_REG_ADDR);
    nv &= !(0xFF);
    nv |= (v & 0xFF);
    poke32(syscon_sysmain_ctrl_register64_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register64_SCFG_axi_cache_sel() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register64_REG_ADDR);
    v & 0xff
}

pub fn _SET_SYSCON_REG_register64_SCFG_default_arcache(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register64_REG_ADDR);
    nv &= !(0xF << 8);
    nv |= (v & 0xF) << 8;
    poke32(syscon_sysmain_ctrl_register64_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register64_SCFG_default_arcache() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register64_REG_ADDR) >> 8;
    v & 0xf
}

pub fn _SET_SYSCON_REG_register64_SCFG_default_awcache(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register64_REG_ADDR);
    nv &= !(0xF << 12);
    nv |= (v & 0xF) << 12;
    poke32(syscon_sysmain_ctrl_register64_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register64_SCFG_default_awcache() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register64_REG_ADDR) >> 12;
    v & 0xf
}

pub fn _SET_SYSCON_REG_register65_SCFG_axi_cache_sel(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register65_REG_ADDR);
    nv &= !(0xFF);
    nv |= (v & 0xFF);
    poke32(syscon_sysmain_ctrl_register65_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register65_SCFG_axi_cache_sel() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register65_REG_ADDR);
    v & 0xff
}

pub fn _SET_SYSCON_REG_register65_SCFG_default_arcache(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register65_REG_ADDR);
    nv &= !(0xF << 8);
    nv |= (v & 0xF) << 8;
    poke32(syscon_sysmain_ctrl_register65_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register65_SCFG_default_arcache() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register65_REG_ADDR) >> 8;
    v & 0xf
}

pub fn _SET_SYSCON_REG_register65_SCFG_default_awcache(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register65_REG_ADDR);
    nv &= !(0xF << 12);
    nv |= (v & 0xF) << 12;
    poke32(syscon_sysmain_ctrl_register65_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register65_SCFG_default_awcache() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register65_REG_ADDR) >> 12;
    v & 0xf
}

pub fn _SET_SYSCON_REG_register68_SCFG_disable_u74_memaxi_remap(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register68_REG_ADDR);
    nv &= !(0x1);
    nv |= (v & 0x1);
    poke32(syscon_sysmain_ctrl_register68_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register68_SCFG_disable_u74_memaxi_remap() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register68_REG_ADDR);
    v & 0x1
}

pub fn _SET_SYSCON_REG_register67_SCFG_axi_cache_sel(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register67_REG_ADDR);
    nv &= !(0xFF);
    nv |= (v & 0xFF);
    poke32(syscon_sysmain_ctrl_register67_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register67_SCFG_axi_cache_sel() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register67_REG_ADDR);
    v & 0xff
}

pub fn _SET_SYSCON_REG_register67_SCFG_default_arcache(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register67_REG_ADDR);
    nv &= !(0xF << 8);
    nv |= (v & 0xF) << 8;
    poke32(syscon_sysmain_ctrl_register67_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register67_SCFG_default_arcache() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register67_REG_ADDR) >> 8;
    v & 0xf
}

pub fn _SET_SYSCON_REG_register67_SCFG_default_awcache(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register67_REG_ADDR);
    nv &= !(0xF << 12);
    nv |= (v & 0xF) << 12;
    poke32(syscon_sysmain_ctrl_register67_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register67_SCFG_default_awcache() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register67_REG_ADDR) >> 12;
    v & 0xf
}

pub fn _SET_SYSCON_REG_register69_core1_en(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register69_REG_ADDR);
    nv &= !(0x1);
    nv |= (v & 0x1);
    poke32(syscon_sysmain_ctrl_register69_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register69_core1_en() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register69_REG_ADDR);
    v & 0x1
}

pub fn _SET_SYSCON_REG_register70_SCFG_boot_mode(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register70_REG_ADDR);
    nv &= !(0x1);
    nv |= (v & 0x1);
    poke32(syscon_sysmain_ctrl_register70_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register70_SCFG_boot_mode() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register70_REG_ADDR);
    v & 0x1
}

pub fn _GET_SYSCON_REG_register70_SCFG_u74_IOPAD_bootmode() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register70_REG_ADDR) >> 1;
    v & 0x1
}

pub fn _SET_SYSCON_REG_register71_SCFG_u74_reset_vector(v: u32) -> () {
    let mut nv = peek32(syscon_sysmain_ctrl_register71_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_sysmain_ctrl_register71_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register71_SCFG_u74_reset_vector() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register71_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _GET_SYSCON_REG_register72_u74_boot_device_sel() -> u32 {
    let v = peek32(syscon_sysmain_ctrl_register72_REG_ADDR);
    v & 0x7
}

pub struct Syscon {
    base: usize,
}
/*
impl ops::Deref for Syscon {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}
*/
impl Driver for Syscon {
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
                self.syscon_init();
                Ok(1)
            }
            _ => Ok(0),
        }
    }

    fn shutdown(&mut self) {}
}

/* Syscon initialization should only be done in romstage. */

impl Syscon {
    pub fn new() -> Syscon {
        Syscon { base: 0 }
    }

    /// Returns a pointer to the register block
    fn ptr(&self) -> usize {
        self.base
    }

    fn init_pll_ddr(&self) {}

    fn init_pll_ge(&self) {}

    pub fn finish(&mut self) {
        _SET_SYSCON_REG_register69_core1_en(1);
        arch::fence();
    }

    fn syscon_init(&mut self) {}
}
