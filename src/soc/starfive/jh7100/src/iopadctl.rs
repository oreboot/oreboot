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
 * @file  syscon_iopad_ctrl_macro.h
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
use core::ptr;

pub const SYSCON_IOPAD_CTRL_BASE_ADDR: u32 = 0x00_1185_8000;
pub const syscon_iopad_ctrl_register0_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x0;
pub const syscon_iopad_ctrl_register1_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x4;
pub const syscon_iopad_ctrl_register2_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x8;
pub const syscon_iopad_ctrl_register3_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0xC;
pub const syscon_iopad_ctrl_register4_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x10;
pub const syscon_iopad_ctrl_register5_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x14;
pub const syscon_iopad_ctrl_register6_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x18;
pub const syscon_iopad_ctrl_register7_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x1C;
pub const syscon_iopad_ctrl_register8_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x20;
pub const syscon_iopad_ctrl_register9_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x24;
pub const syscon_iopad_ctrl_register10_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x28;
pub const syscon_iopad_ctrl_register11_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x2C;
pub const syscon_iopad_ctrl_register12_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x30;
pub const syscon_iopad_ctrl_register13_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x34;
pub const syscon_iopad_ctrl_register14_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x38;
pub const syscon_iopad_ctrl_register15_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x3C;
pub const syscon_iopad_ctrl_register16_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x40;
pub const syscon_iopad_ctrl_register17_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x44;
pub const syscon_iopad_ctrl_register18_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x48;
pub const syscon_iopad_ctrl_register19_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x4C;
pub const syscon_iopad_ctrl_register20_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x50;
pub const syscon_iopad_ctrl_register21_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x54;
pub const syscon_iopad_ctrl_register22_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x58;
pub const syscon_iopad_ctrl_register23_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x5C;
pub const syscon_iopad_ctrl_register24_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x60;
pub const syscon_iopad_ctrl_register25_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x64;
pub const syscon_iopad_ctrl_register26_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x68;
pub const syscon_iopad_ctrl_register27_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x6C;
pub const syscon_iopad_ctrl_register28_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x70;
pub const syscon_iopad_ctrl_register29_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x74;
pub const syscon_iopad_ctrl_register30_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x78;
pub const syscon_iopad_ctrl_register31_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x7C;
pub const syscon_iopad_ctrl_register32_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x80;
pub const syscon_iopad_ctrl_register33_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x84;
pub const syscon_iopad_ctrl_register34_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x88;
pub const syscon_iopad_ctrl_register35_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x8C;
pub const syscon_iopad_ctrl_register36_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x90;
pub const syscon_iopad_ctrl_register37_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x94;
pub const syscon_iopad_ctrl_register38_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x98;
pub const syscon_iopad_ctrl_register39_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x9C;
pub const syscon_iopad_ctrl_register40_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0xA0;
pub const syscon_iopad_ctrl_register41_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0xA4;
pub const syscon_iopad_ctrl_register42_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0xA8;
pub const syscon_iopad_ctrl_register43_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0xAC;
pub const syscon_iopad_ctrl_register44_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0xB0;
pub const syscon_iopad_ctrl_register45_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0xB4;
pub const syscon_iopad_ctrl_register46_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0xB8;
pub const syscon_iopad_ctrl_register47_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0xBC;
pub const syscon_iopad_ctrl_register48_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0xC0;
pub const syscon_iopad_ctrl_register49_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0xC4;
pub const syscon_iopad_ctrl_register50_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0xC8;
pub const syscon_iopad_ctrl_register51_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0xCC;
pub const syscon_iopad_ctrl_register52_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0xD0;
pub const syscon_iopad_ctrl_register53_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0xD4;
pub const syscon_iopad_ctrl_register54_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0xD8;
pub const syscon_iopad_ctrl_register55_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0xDC;
pub const syscon_iopad_ctrl_register56_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0xE0;
pub const syscon_iopad_ctrl_register57_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0xE4;
pub const syscon_iopad_ctrl_register58_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0xE8;
pub const syscon_iopad_ctrl_register59_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0xEC;
pub const syscon_iopad_ctrl_register60_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0xF0;
pub const syscon_iopad_ctrl_register61_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0xF4;
pub const syscon_iopad_ctrl_register62_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0xF8;
pub const syscon_iopad_ctrl_register63_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0xFC;
pub const syscon_iopad_ctrl_register64_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x100;
pub const syscon_iopad_ctrl_register65_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x104;
pub const syscon_iopad_ctrl_register66_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x108;
pub const syscon_iopad_ctrl_register67_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x10C;
pub const syscon_iopad_ctrl_register68_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x110;
pub const syscon_iopad_ctrl_register69_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x114;
pub const syscon_iopad_ctrl_register70_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x118;
pub const syscon_iopad_ctrl_register71_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x11C;
pub const syscon_iopad_ctrl_register72_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x120;
pub const syscon_iopad_ctrl_register73_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x124;
pub const syscon_iopad_ctrl_register74_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x128;
pub const syscon_iopad_ctrl_register75_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x12C;
pub const syscon_iopad_ctrl_register76_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x130;
pub const syscon_iopad_ctrl_register77_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x134;
pub const syscon_iopad_ctrl_register78_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x138;
pub const syscon_iopad_ctrl_register79_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x13C;
pub const syscon_iopad_ctrl_register80_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x140;
pub const syscon_iopad_ctrl_register81_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x144;
pub const syscon_iopad_ctrl_register82_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x148;
pub const syscon_iopad_ctrl_register83_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x14C;
pub const syscon_iopad_ctrl_register84_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x150;
pub const syscon_iopad_ctrl_register85_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x154;
pub const syscon_iopad_ctrl_register86_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x158;
pub const syscon_iopad_ctrl_register87_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x15C;
pub const syscon_iopad_ctrl_register88_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x160;
pub const syscon_iopad_ctrl_register89_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x164;
pub const syscon_iopad_ctrl_register90_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x168;
pub const syscon_iopad_ctrl_register91_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x16C;
pub const syscon_iopad_ctrl_register92_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x170;
pub const syscon_iopad_ctrl_register93_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x174;
pub const syscon_iopad_ctrl_register94_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x178;
pub const syscon_iopad_ctrl_register95_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x17C;
pub const syscon_iopad_ctrl_register96_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x180;
pub const syscon_iopad_ctrl_register97_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x184;
pub const syscon_iopad_ctrl_register98_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x188;
pub const syscon_iopad_ctrl_register99_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x18C;
pub const syscon_iopad_ctrl_register100_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x190;
pub const syscon_iopad_ctrl_register101_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x194;
pub const syscon_iopad_ctrl_register102_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x198;
pub const syscon_iopad_ctrl_register103_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x19C;
pub const syscon_iopad_ctrl_register104_REG_ADDR: u32 = SYSCON_IOPAD_CTRL_BASE_ADDR + 0x1A0;

pub fn _SET_SYSCON_REG_register0_SCFG_gpio_pad_ctrl_0(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register0_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register0_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register0_SCFG_gpio_pad_ctrl_0() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register0_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register1_SCFG_gpio_pad_ctrl_1(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register1_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register1_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register1_SCFG_gpio_pad_ctrl_1() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register1_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register2_SCFG_gpio_pad_ctrl_2(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register2_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register2_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register2_SCFG_gpio_pad_ctrl_2() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register2_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register3_SCFG_gpio_pad_ctrl_3(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register3_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register3_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register3_SCFG_gpio_pad_ctrl_3() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register3_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register4_SCFG_gpio_pad_ctrl_4(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register4_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register4_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register4_SCFG_gpio_pad_ctrl_4() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register4_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register5_SCFG_gpio_pad_ctrl_5(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register5_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register5_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register5_SCFG_gpio_pad_ctrl_5() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register5_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register6_SCFG_gpio_pad_ctrl_6(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register6_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register6_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register6_SCFG_gpio_pad_ctrl_6() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register6_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register7_SCFG_gpio_pad_ctrl_7(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register7_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register7_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register7_SCFG_gpio_pad_ctrl_7() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register7_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register8_SCFG_gpio_pad_ctrl_8(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register8_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register8_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register8_SCFG_gpio_pad_ctrl_8() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register8_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register9_SCFG_gpio_pad_ctrl_9(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register9_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register9_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register9_SCFG_gpio_pad_ctrl_9() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register9_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register10_SCFG_gpio_pad_ctrl_10(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register10_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register10_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register10_SCFG_gpio_pad_ctrl_10() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register10_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register11_SCFG_gpio_pad_ctrl_11(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register11_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register11_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register11_SCFG_gpio_pad_ctrl_11() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register11_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register12_SCFG_gpio_pad_ctrl_12(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register12_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register12_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register12_SCFG_gpio_pad_ctrl_12() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register12_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register13_SCFG_gpio_pad_ctrl_13(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register13_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register13_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register13_SCFG_gpio_pad_ctrl_13() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register13_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register14_SCFG_gpio_pad_ctrl_14(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register14_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register14_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register14_SCFG_gpio_pad_ctrl_14() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register14_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register15_SCFG_gpio_pad_ctrl_15(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register15_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register15_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register15_SCFG_gpio_pad_ctrl_15() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register15_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register16_SCFG_gpio_pad_ctrl_16(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register16_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register16_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register16_SCFG_gpio_pad_ctrl_16() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register16_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register17_SCFG_gpio_pad_ctrl_17(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register17_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register17_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register17_SCFG_gpio_pad_ctrl_17() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register17_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register18_SCFG_gpio_pad_ctrl_18(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register18_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register18_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register18_SCFG_gpio_pad_ctrl_18() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register18_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register19_SCFG_gpio_pad_ctrl_19(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register19_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register19_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register19_SCFG_gpio_pad_ctrl_19() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register19_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register20_SCFG_gpio_pad_ctrl_20(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register20_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register20_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register20_SCFG_gpio_pad_ctrl_20() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register20_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register21_SCFG_gpio_pad_ctrl_21(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register21_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register21_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register21_SCFG_gpio_pad_ctrl_21() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register21_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register22_SCFG_gpio_pad_ctrl_22(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register22_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register22_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register22_SCFG_gpio_pad_ctrl_22() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register22_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register23_SCFG_gpio_pad_ctrl_23(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register23_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register23_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register23_SCFG_gpio_pad_ctrl_23() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register23_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register24_SCFG_gpio_pad_ctrl_24(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register24_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register24_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register24_SCFG_gpio_pad_ctrl_24() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register24_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register25_SCFG_gpio_pad_ctrl_25(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register25_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register25_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register25_SCFG_gpio_pad_ctrl_25() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register25_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register26_SCFG_gpio_pad_ctrl_26(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register26_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register26_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register26_SCFG_gpio_pad_ctrl_26() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register26_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register27_SCFG_gpio_pad_ctrl_27(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register27_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register27_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register27_SCFG_gpio_pad_ctrl_27() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register27_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register28_SCFG_gpio_pad_ctrl_28(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register28_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register28_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register28_SCFG_gpio_pad_ctrl_28() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register28_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register29_SCFG_gpio_pad_ctrl_29(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register29_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register29_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register29_SCFG_gpio_pad_ctrl_29() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register29_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register30_SCFG_gpio_pad_ctrl_30(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register30_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register30_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register30_SCFG_gpio_pad_ctrl_30() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register30_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register31_SCFG_gpio_pad_ctrl_31(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register31_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register31_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register31_SCFG_gpio_pad_ctrl_31() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register31_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register32_SCFG_funcshare_pad_ctrl_0(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register32_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register32_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register32_SCFG_funcshare_pad_ctrl_0() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register32_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register33_SCFG_funcshare_pad_ctrl_1(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register33_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register33_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register33_SCFG_funcshare_pad_ctrl_1() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register33_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register34_SCFG_funcshare_pad_ctrl_2(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register34_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register34_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register34_SCFG_funcshare_pad_ctrl_2() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register34_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register35_SCFG_funcshare_pad_ctrl_3(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register35_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register35_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register35_SCFG_funcshare_pad_ctrl_3() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register35_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register36_SCFG_funcshare_pad_ctrl_4(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register36_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register36_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register36_SCFG_funcshare_pad_ctrl_4() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register36_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register37_SCFG_funcshare_pad_ctrl_5(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register37_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register37_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register37_SCFG_funcshare_pad_ctrl_5() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register37_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register38_SCFG_funcshare_pad_ctrl_6(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register38_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register38_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register38_SCFG_funcshare_pad_ctrl_6() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register38_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register39_SCFG_funcshare_pad_ctrl_7(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register39_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register39_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register39_SCFG_funcshare_pad_ctrl_7() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register39_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register40_SCFG_funcshare_pad_ctrl_8(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register40_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register40_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register40_SCFG_funcshare_pad_ctrl_8() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register40_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register41_SCFG_funcshare_pad_ctrl_9(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register41_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register41_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register41_SCFG_funcshare_pad_ctrl_9() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register41_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register42_SCFG_funcshare_pad_ctrl_10(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register42_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register42_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register42_SCFG_funcshare_pad_ctrl_10() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register42_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register43_SCFG_funcshare_pad_ctrl_11(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register43_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register43_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register43_SCFG_funcshare_pad_ctrl_11() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register43_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register44_SCFG_funcshare_pad_ctrl_12(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register44_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register44_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register44_SCFG_funcshare_pad_ctrl_12() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register44_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register45_SCFG_funcshare_pad_ctrl_13(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register45_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register45_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register45_SCFG_funcshare_pad_ctrl_13() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register45_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register46_SCFG_funcshare_pad_ctrl_14(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register46_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register46_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register46_SCFG_funcshare_pad_ctrl_14() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register46_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register47_SCFG_funcshare_pad_ctrl_15(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register47_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register47_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register47_SCFG_funcshare_pad_ctrl_15() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register47_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register48_SCFG_funcshare_pad_ctrl_16(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register48_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register48_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register48_SCFG_funcshare_pad_ctrl_16() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register48_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register49_SCFG_funcshare_pad_ctrl_17(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register49_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register49_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register49_SCFG_funcshare_pad_ctrl_17() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register49_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register50_SCFG_funcshare_pad_ctrl_18(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register50_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register50_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register50_SCFG_funcshare_pad_ctrl_18() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register50_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register51_SCFG_funcshare_pad_ctrl_19(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register51_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register51_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register51_SCFG_funcshare_pad_ctrl_19() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register51_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register52_SCFG_funcshare_pad_ctrl_20(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register52_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register52_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register52_SCFG_funcshare_pad_ctrl_20() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register52_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register53_SCFG_funcshare_pad_ctrl_21(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register53_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register53_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register53_SCFG_funcshare_pad_ctrl_21() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register53_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register54_SCFG_funcshare_pad_ctrl_22(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register54_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register54_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register54_SCFG_funcshare_pad_ctrl_22() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register54_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register55_SCFG_funcshare_pad_ctrl_23(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register55_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register55_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register55_SCFG_funcshare_pad_ctrl_23() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register55_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register56_SCFG_funcshare_pad_ctrl_24(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register56_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register56_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register56_SCFG_funcshare_pad_ctrl_24() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register56_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register57_SCFG_funcshare_pad_ctrl_25(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register57_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register57_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register57_SCFG_funcshare_pad_ctrl_25() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register57_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register58_SCFG_funcshare_pad_ctrl_26(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register58_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register58_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register58_SCFG_funcshare_pad_ctrl_26() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register58_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register59_SCFG_funcshare_pad_ctrl_27(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register59_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register59_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register59_SCFG_funcshare_pad_ctrl_27() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register59_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register60_SCFG_funcshare_pad_ctrl_28(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register60_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register60_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register60_SCFG_funcshare_pad_ctrl_28() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register60_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register61_SCFG_funcshare_pad_ctrl_29(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register61_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register61_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register61_SCFG_funcshare_pad_ctrl_29() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register61_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register62_SCFG_funcshare_pad_ctrl_30(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register62_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register62_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register62_SCFG_funcshare_pad_ctrl_30() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register62_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register63_SCFG_funcshare_pad_ctrl_31(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register63_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register63_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register63_SCFG_funcshare_pad_ctrl_31() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register63_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register64_SCFG_funcshare_pad_ctrl_32(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register64_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register64_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register64_SCFG_funcshare_pad_ctrl_32() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register64_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register65_SCFG_funcshare_pad_ctrl_33(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register65_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register65_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register65_SCFG_funcshare_pad_ctrl_33() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register65_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register66_SCFG_funcshare_pad_ctrl_34(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register66_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register66_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register66_SCFG_funcshare_pad_ctrl_34() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register66_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register67_SCFG_funcshare_pad_ctrl_35(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register67_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register67_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register67_SCFG_funcshare_pad_ctrl_35() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register67_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register68_SCFG_funcshare_pad_ctrl_36(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register68_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register68_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register68_SCFG_funcshare_pad_ctrl_36() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register68_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register69_SCFG_funcshare_pad_ctrl_37(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register69_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register69_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register69_SCFG_funcshare_pad_ctrl_37() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register69_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register70_SCFG_funcshare_pad_ctrl_38(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register70_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register70_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register70_SCFG_funcshare_pad_ctrl_38() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register70_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register71_SCFG_funcshare_pad_ctrl_39(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register71_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register71_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register71_SCFG_funcshare_pad_ctrl_39() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register71_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register72_SCFG_funcshare_pad_ctrl_40(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register72_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register72_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register72_SCFG_funcshare_pad_ctrl_40() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register72_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register73_SCFG_funcshare_pad_ctrl_41(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register73_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register73_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register73_SCFG_funcshare_pad_ctrl_41() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register73_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register74_SCFG_funcshare_pad_ctrl_42(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register74_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register74_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register74_SCFG_funcshare_pad_ctrl_42() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register74_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register75_SCFG_funcshare_pad_ctrl_43(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register75_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register75_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register75_SCFG_funcshare_pad_ctrl_43() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register75_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register76_SCFG_funcshare_pad_ctrl_44(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register76_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register76_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register76_SCFG_funcshare_pad_ctrl_44() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register76_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register77_SCFG_funcshare_pad_ctrl_45(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register77_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register77_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register77_SCFG_funcshare_pad_ctrl_45() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register77_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register78_SCFG_funcshare_pad_ctrl_46(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register78_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register78_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register78_SCFG_funcshare_pad_ctrl_46() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register78_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register79_SCFG_funcshare_pad_ctrl_47(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register79_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register79_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register79_SCFG_funcshare_pad_ctrl_47() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register79_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register80_SCFG_funcshare_pad_ctrl_48(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register80_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register80_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register80_SCFG_funcshare_pad_ctrl_48() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register80_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register81_SCFG_funcshare_pad_ctrl_49(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register81_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register81_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register81_SCFG_funcshare_pad_ctrl_49() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register81_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register82_SCFG_funcshare_pad_ctrl_50(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register82_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register82_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register82_SCFG_funcshare_pad_ctrl_50() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register82_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register83_SCFG_funcshare_pad_ctrl_51(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register83_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register83_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register83_SCFG_funcshare_pad_ctrl_51() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register83_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register84_SCFG_funcshare_pad_ctrl_52(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register84_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register84_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register84_SCFG_funcshare_pad_ctrl_52() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register84_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register85_SCFG_funcshare_pad_ctrl_53(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register85_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register85_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register85_SCFG_funcshare_pad_ctrl_53() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register85_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register86_SCFG_funcshare_pad_ctrl_54(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register86_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register86_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register86_SCFG_funcshare_pad_ctrl_54() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register86_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register87_SCFG_funcshare_pad_ctrl_55(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register87_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register87_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register87_SCFG_funcshare_pad_ctrl_55() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register87_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register88_SCFG_funcshare_pad_ctrl_56(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register88_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register88_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register88_SCFG_funcshare_pad_ctrl_56() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register88_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register89_SCFG_funcshare_pad_ctrl_57(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register89_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register89_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register89_SCFG_funcshare_pad_ctrl_57() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register89_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register90_SCFG_funcshare_pad_ctrl_58(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register90_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register90_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register90_SCFG_funcshare_pad_ctrl_58() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register90_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register91_SCFG_funcshare_pad_ctrl_59(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register91_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register91_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register91_SCFG_funcshare_pad_ctrl_59() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register91_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register92_SCFG_funcshare_pad_ctrl_60(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register92_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register92_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register92_SCFG_funcshare_pad_ctrl_60() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register92_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register93_SCFG_funcshare_pad_ctrl_61(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register93_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register93_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register93_SCFG_funcshare_pad_ctrl_61() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register93_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register94_SCFG_funcshare_pad_ctrl_62(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register94_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register94_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register94_SCFG_funcshare_pad_ctrl_62() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register94_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register95_SCFG_funcshare_pad_ctrl_63(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register95_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register95_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register95_SCFG_funcshare_pad_ctrl_63() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register95_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register96_SCFG_funcshare_pad_ctrl_64(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register96_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register96_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register96_SCFG_funcshare_pad_ctrl_64() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register96_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register97_SCFG_funcshare_pad_ctrl_65(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register97_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register97_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register97_SCFG_funcshare_pad_ctrl_65() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register97_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register98_SCFG_funcshare_pad_ctrl_66(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register98_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register98_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register98_SCFG_funcshare_pad_ctrl_66() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register98_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register99_SCFG_funcshare_pad_ctrl_67(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register99_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register99_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register99_SCFG_funcshare_pad_ctrl_67() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register99_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register100_SCFG_funcshare_pad_ctrl_68(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register100_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register100_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register100_SCFG_funcshare_pad_ctrl_68() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register100_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register101_SCFG_funcshare_pad_ctrl_69(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register101_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register101_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register101_SCFG_funcshare_pad_ctrl_69() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register101_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register102_SCFG_funcshare_pad_ctrl_70(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register102_REG_ADDR);
    nv &= !(0xFFFFFFFF);
    nv |= (v & 0xFFFFFFFF);
    poke32(syscon_iopad_ctrl_register102_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register102_SCFG_funcshare_pad_ctrl_70() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register102_REG_ADDR);
    v & 0xFFFFFFFF
}

pub fn _SET_SYSCON_REG_register103_SCFG_qspi_ioctrl(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register103_REG_ADDR);
    nv &= !(0x7F);
    nv |= (v & 0x7F);
    poke32(syscon_iopad_ctrl_register103_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register103_SCFG_qspi_ioctrl() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register103_REG_ADDR);
    v & 0x7f
}

pub fn _SET_SYSCON_REG_register104_SCFG_io_padshare_sel(v: u32) -> () {
    let mut nv = peek32(syscon_iopad_ctrl_register104_REG_ADDR);
    nv &= !(0x7);
    nv |= (v & 0x7);
    poke32(syscon_iopad_ctrl_register104_REG_ADDR, nv);
}

pub fn _GET_SYSCON_REG_register104_SCFG_io_padshare_sel() -> u32 {
    let v = peek32(syscon_iopad_ctrl_register104_REG_ADDR);
    v & 0x7
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

pub struct IOpadctl {
    base: usize,
}
/*
impl ops::Deref for IOpadctl {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}
*/
impl Driver for IOpadctl {
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
                self.iopadctl_init();
                Ok(1)
            }
            b"early" => {
                self.iopadctl_early();
                Ok(1)
            }
            _ => Ok(0),
        }
    }

    fn shutdown(&mut self) {}
}

/* IOpadctl initialization should only be done in romstage. */

impl IOpadctl {
    pub fn new() -> IOpadctl {
        IOpadctl { base: 0 }
    }

    /// Returns a pointer to the register block
    fn ptr(&self) -> usize {
        self.base
    }

    fn iopadctl_early(&mut self) {
        //for illegal instruction exception
        _SET_SYSCON_REG_register50_SCFG_funcshare_pad_ctrl_18(0x00c000c0);
    }
    fn iopadctl_init(&mut self) {
        _SET_SYSCON_REG_register104_SCFG_io_padshare_sel(6);
        _SET_SYSCON_REG_register32_SCFG_funcshare_pad_ctrl_0(0x00c00000);
        _SET_SYSCON_REG_register33_SCFG_funcshare_pad_ctrl_1(0x00c000c0);
        _SET_SYSCON_REG_register34_SCFG_funcshare_pad_ctrl_2(0x00c000c0);
        _SET_SYSCON_REG_register35_SCFG_funcshare_pad_ctrl_3(0x00c000c0);
        _SET_SYSCON_REG_register39_SCFG_funcshare_pad_ctrl_7(0x00c300c3);
        _SET_SYSCON_REG_register38_SCFG_funcshare_pad_ctrl_6(0x00c00000);
        arch::fence();
    }
}
