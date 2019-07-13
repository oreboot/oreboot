#![no_std]
#![allow(non_snake_case)]
#![allow(unused_attributes)]
#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(non_camel_case_types)]

pub mod ramtable;
#[macro_use]
pub mod ram;
use crate::print;
use core::fmt;
use core::ptr;
use soc::aspeed::ast2500;

const UART5DR: u32 = 0x1E78_4000;

// u-bmc modified
// Setting lifted from ast-g5-phy.h from OpenBMC u-boot
const CONFIG_DRAM_ECC_SIZE: u32 = 0x10000000;

/******************************************************************************
 r4 : return program counter
 r5 : DDR speed timing table base address
 Free registers:
 r0, r1, r2, r3, r6, r7, r8, r9, r10, r11
******************************************************************************/
const ASTMMC_INIT_VER: u32 = 0x12; //        @ 8bit verison
const ASTMMC_INIT_DATE: u32 = 0x20171027; //     @ Release

/******************************************************************************
 BMC side DDR IO driving manual mode fine-tuning, used to improve CK/CKN Vix violation.
 Default disabled, the driver setting is hardware auto tuned.

 ASTMMC_DDR4_MANUAL_RPU | ASTMMC_DDR4_MANUAL_RPD
 -----------------------+-----------------------
           No           |           x          : manual mode disabled
           Yes          |          No          : enable Rpu     manual setting
           Yes          |          Yes         : enable Rpu/Rpd manual setting
******************************************************************************/
//const ASTMMC_DDR4_MANUAL_RPU 0x0             @ 0x0-0xF, larger value means weaker: u32 = driving;
//const ASTMMC_DDR4_MANUAL_RPD 0x0             @ 0x0-0xF, larger value means stronger: u32 = driving;

/******************************************************************************
 Select initial reset mode as WDT_Full
 WDT_Full is a more complete reset mode than WDT_SOC.
 But if FW has other initial code executed before platform.S, then it should use WDT_SOC mode.
 Use WDT_Full may clear the initial result of prior initial code.
******************************************************************************/
//#define ASTMMC_INIT_RESET_MODE_FULL

/******************************************************************************
 There is a compatibility issue for Hynix DDR4 SDRAM.
 Hynix DDR4 SDRAM is more weak on noise margin compared to Micron and Samsung DDR4.
 To well support Hynix DDR4, it requlres to slow down the DDR4 operating frequency
 from 1600Mbps to 1536/1488/1440 Mbps. The target frequency that can be used depends
 on the MB layout. Customer can find the appropriate frequency for their products.
 Below are the new defined parameters for the Hynix DDR4 supporting.
******************************************************************************/
//const CONFIG_DDR4_SUPPORT_HYNIX              @ Enable this when Hynix DDR4 included in the: u32 = BOM;
//#define CONFIG_DDR4_HYNIX_SET_1536
//#define CONFIG_DDR4_HYNIX_SET_1488
const CONFIG_DDR4_HYNIX_SET_1440: u32 = 1;

const ASTMMC_REGIDX_010: u32 = 0x00 / 4;
const ASTMMC_REGIDX_014: u32 = 0x04 / 4;
const ASTMMC_REGIDX_018: u32 = 0x08 / 4;
const ASTMMC_REGIDX_020: u32 = 0x0C / 4;
const ASTMMC_REGIDX_024: u32 = 0x10 / 4;
const ASTMMC_REGIDX_02C: u32 = 0x14 / 4;
const ASTMMC_REGIDX_030: u32 = 0x18 / 4;
const ASTMMC_REGIDX_214: u32 = 0x1C / 4;
const ASTMMC_REGIDX_2E0: u32 = 0x20 / 4;
const ASTMMC_REGIDX_2E4: u32 = 0x24 / 4;
const ASTMMC_REGIDX_2E8: u32 = 0x28 / 4;
const ASTMMC_REGIDX_2EC: u32 = 0x2C / 4;
const ASTMMC_REGIDX_2F0: u32 = 0x30 / 4;
const ASTMMC_REGIDX_2F4: u32 = 0x34 / 4;
const ASTMMC_REGIDX_2F8: u32 = 0x38 / 4;
const ASTMMC_REGIDX_RFC: u32 = 0x3C / 4;
const ASTMMC_REGIDX_PLL: u32 = 0x40 / 4;
const ASTMMC_INIT_RESET_MODE_FULL: u32 = 0x0;

// From bluecmd:
const CONFIG_DRAM_1333: u32 = 0;
//Do not activate. I don't see any board that ever used that, and 1600 seems to be the default.

const CONFIG_DRAM_ECC: u32 = 0;

const CONFIG_DDR4_SUPPORT_HYNIX: u32 = 0;
//Do not set, seems to be for Hynix only memory type. Possibly remove.

const CONFIG_DDR4_HYNIX_SET_1536: u32 = 0;
//Do not set, seems to be for Hynix only memory type. Possibly remove.

const CONFIG_DDR4_HYNIX_SET_1488: u32 = 0;
//Do not set, seems to be for Hynix only memory type. Possibly remove.

const CONFIG_DRAM_UART_TO_UART1: u32 = 0;
//Do not set. Set motherboard pin J118 and J119 to 2-3 to route to UART5.

const CONFIG_DRAM_UART_38400: u32 = 0;
const CONFIG_DRAM_UART_57600: u32 = 0;
const CONFIG_DRAM_UART_115200: u32 = 1;
//I recommend 57600 (or better yet: create a 115200 because it's 2019).
//The reason 57600 was chosen was that the OCP Leopards have it, happy to move as much as possible to 115200.

const CONFIG_DDR3_8GSTACK: u32 = 0;
//I guess irrelevant as we're using DDR4.

const CONFIG_DRAM_EXT_TEMP: u32 = 0;
//Leave unset, don't see any other boards setting it and seems to be for higher temperature ranges.

const CONFIG_DDR4_4GX8: u32 = 0;
//Unsure, but I guess disable as I don't see any other users of this. This is the one I'm least sure about though.

const ASTMMC_DDR4_MANUAL_RPD: u32 = 0;
const ASTMMC_DDR4_MANUAL_RPU: u32 = 0;
//Disable, which will leave it on auto it seems.

// That will result in DDR4 @ 1600 MHz. However, if that does work you can try CONFIG_DDR4_SUPPORT_HYNIX = 1, that will configure the DDR4 to be Hynix and 1440 MHz.
// The EVB also has Hynix DDR4 on it but it works fine with the defaults so not sure the Hynix overrides are that needed.

// And finally, I would not set ECC - guessing it is not used as I cannot find any reference to it either on the memory chip or other users.
// Some users set the ECC_SIZE but not activate the use of ECC.

fn poke(v: u32, a: u32) -> () {
    let y = a as *mut u32;
    unsafe {
        ptr::write_volatile(y, v);
    }
}
fn peek(a: u32) -> u32 {
    let y = a as *const u32;
    unsafe { ptr::read_volatile(y) }
}
pub fn ram(w: &mut print::WriteTo) -> () {
    let mut tptr = ramtable::TIME_TABLE_DDR3_1333;
    let mut r0 = 0u32;
    let mut r1 = 0u32;
    let mut r2 = 0u32;
    let mut r3 = 0u32;
    let r4 = 0u32;
    let mut r5 = 0u32;
    let mut r6 = 0u32;
    let mut r7 = 0u32;
    let mut r8 = 0u32;
    let mut r9 = 0u32;
    let mut r10 = 0u32;
    let mut r11 = 0u32;
    let mut z = false;
    let mut gt;
    let mut lt;
    let mut s = State::PowerOn;
    fmt::write(w, format_args!("Start s is {:#?}\n", s)).expect("oh no");
    loop {
        fmt::write(w, format_args!("loop s is {:?}\n", s)).expect("oh no");
        s = match s {
            State::Exit => {
                fmt::write(w, format_args!("DRAM done\n")).expect("oh no");
                break;

                State::init_dram
            }
            // This will be duplicative of init_dram for now. All we're trying to do
            // first is get some kinda serial output on power on. Nothing more.
            State::PowerOn => State::uartSETUP,
            State::uartSETUP => {
                // Put only the bare minimum code here needed for uart5.
                // There shall be no magic numbers.
                //
                // let's see if it worked ...
                loop {
                    r0 = UART5DR; /*"    ldr   r0, =0x1e784000"*/
                    r1 = 'O' as u32;
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                }
                State::init_dram
            }
            State::init_dram => {
                /* save lr */

                /********************************************
                  Initial Reset Procedure : Begin
                *******************************************/
                /* Clear AHB bus lock condition */
                r0 = 0x1e600000 as u32; /*"    ldr   r0, =0x1e600000"*/
                r1 = 0xAEED1A03 as u32; /*"    ldr   r1, =0xAEED1A03"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e600084 as u32; /*"    ldr   r0, =0x1e600084"*/
                r1 = 0x00010000 as u32; /*"    ldr   r1, =0x00010000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = r0 + 0x4 as u32; /*"    add   r0, r0, #0x4"*/
                r1 = 0x0 as u32; /*"    mov   r1, #0x0"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e2000 as u32; /*"    ldr   r0, =0x1e6e2000"*/
                r1 = 0x1688a8a8 as u32; /*"    ldr   r1, =0x1688a8a8"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Reset again */
                r0 = 0x1e6e2070 as u32; /*"    ldr   r0, =0x1e6e2070                        @ check fast reset flag"*/
                r2 = 0x08000000 as u32; /*"    ldr   r2, =0x08000000"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == r2; /*"    tst   r1, r2"*/
                if z {
                    s = State::bypass_first_reset;
                    continue;
                } /*"    beq   bypass_first_reset"*/

                r0 = 0x1e785010 as u32; /*"    ldr   r0, =0x1e785010"*/
                r3 = peek(r0); /*"    ldr   r3, [r0]"*/
                z = r3 == 0x0 as u32;

                if z {
                    s = State::start_first_reset;
                    continue;
                } /*"    beq   start_first_reset"*/
                // The real question: what is this code? It's not first reset, not bypass first reset.
                r0 = r0 + 0x04 as u32; /*"    add   r0, r0, #0x04"*/
                r3 = 0x77 as u32; /*"    mov   r3, #0x77"*/
                poke(r3, r0); /*"    str   r3, [r0]"*/
                r0 = 0x1e720004 as u32; /*"    ldr   r0, =0x1e720004                        @ Copy initial strap register to 0x1e720004"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = r0 + 0x04 as u32; /*"    add   r0, r0, #0x04                          @ Copy initial strap register to 0x1e720008"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = r0 + 0x04 as u32; /*"    add   r0, r0, #0x04                          @ Copy initial strap register to 0x1e72000c"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e207c as u32; /*"    ldr   r0, =0x1e6e207c                        @ clear fast reset flag"*/
                poke(r2, r0); /*"    str   r2, [r0]"*/
                r0 = 0x1e6e203c as u32; /*"    ldr   r0, =0x1e6e203c                        @ clear watchdog reset flag"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 = r1 | 0x01 as u32; /*"    and   r1, r1, #0x01"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e78501c as u32; /*"    ldr   r0, =0x1e78501c                        @ restore normal mask setting"*/
                r1 = 0x023FFFF3 as u32; /*"    ldr   r1, =0x023FFFF3                        @ added 2016.09.06"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                s = State::bypass_first_reset;
                continue; /*"    b     bypass_first_reset"*/

                State::start_first_reset
            }
            State::start_first_reset => {
                if ASTMMC_INIT_RESET_MODE_FULL == 1 {
                    // #ifdef ASTMMC_INIT_RESET_MODE_FULL
                    r0 = 0x1e785004 as u32; /*"    ldr   r0, =0x1e785004"*/
                    r1 = 0x00000001 as u32; /*"    ldr   r1, =0x00000001"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e785008 as u32; /*"    ldr   r0, =0x1e785008"*/
                    r1 = 0x00004755 as u32; /*"    ldr   r1, =0x00004755"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e78500c as u32; /*"    ldr   r0, =0x1e78500c                        @ enable Full reset"*/
                    r1 = 0x00000033 as u32; /*"    ldr   r1, =0x00000033"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                } else {
                    // #else     /***** Clear LPC status : Begin *****/
                    r2 = 0 as u32; /*"    mov   r2, #0                                 @ set r2 = 0, freezed"*/
                    r0 = 0x1e787008 as u32; /*"    ldr   r0, =0x1e787008"*/
                    r1 = 0x7 as u32; /*"    mov   r1, #0x7"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e78700c as u32; /*"    ldr   r0, =0x1e78700c"*/
                    r1 = 0x3 as u32; /*"    mov   r1, #0x3"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e787020 as u32; /*"    ldr   r0, =0x1e787020"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e787034 as u32; /*"    ldr   r0, =0x1e787034"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e787004 as u32; /*"    ldr   r0, =0x1e787004"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e787010 as u32; /*"    ldr   r0, =0x1e787010"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e78701c as u32; /*"    ldr   r0, =0x1e78701c"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e787014 as u32; /*"    ldr   r0, =0x1e787014                        @ read clear"*/
                    /*r1 = */
                    peek(r0); /*"    ldr   r1, [r0]"*/
                    r0 = 0x1e787018 as u32; /*"    ldr   r0, =0x1e787018                        @ read clear"*/
                    /*r1 = */
                    peek(r0); /*"    ldr   r1, [r0]"*/
                    r0 = 0x1e787008 as u32; /*"    ldr   r0, =0x1e787008                        @ read clear"*/
                    /*r1 = */
                    peek(r0); /*"    ldr   r1, [r0]"*/
                    r0 = 0x1e78301c as u32; /*"    ldr   r0, =0x1e78301c                        @ read clear"*/
                    /*r1 = */
                    peek(r0); /*"    ldr   r1, [r0]"*/
                    r0 = 0x1e78d01c as u32; /*"    ldr   r0, =0x1e78d01c                        @ read clear"*/
                    /*r1 = */
                    peek(r0); /*"    ldr   r1, [r0]"*/
                    r0 = 0x1e78e01c as u32; /*"    ldr   r0, =0x1e78e01c                        @ read clear"*/
                    /*r1 = */
                    peek(r0); /*"    ldr   r1, [r0]"*/
                    r0 = 0x1e78f01c as u32; /*"    ldr   r0, =0x1e78f01c                        @ read clear"*/
                    /*r1 = */
                    peek(r0); /*"    ldr   r1, [r0]"*/
                    r0 = 0x1e788020 as u32; /*"    ldr   r0, =0x1e788020"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e788034 as u32; /*"    ldr   r0, =0x1e788034"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e78800c as u32; /*"    ldr   r0, =0x1e78800c"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e789008 as u32; /*"    ldr   r0, =0x1e789008"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e789010 as u32; /*"    ldr   r0, =0x1e789010"*/
                    r1 = 0x40 as u32; /*"    mov   r1, #0x40"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789024 as u32; /*"    ldr   r0, =0x1e789024                        @ read clear"*/
                    /*r1 = */
                    peek(r0); /*"    ldr   r1, [r0]"*/
                    r0 = 0x1e789028 as u32; /*"    ldr   r0, =0x1e789028                        @ read clear"*/
                    /*r1 = */
                    peek(r0); /*"    ldr   r1, [r0]"*/
                    r0 = 0x1e78902c as u32; /*"    ldr   r0, =0x1e78902c                        @ read clear"*/
                    /*r1 = */
                    peek(r0); /*"    ldr   r1, [r0]"*/
                    r0 = 0x1e789114 as u32; /*"    ldr   r0, =0x1e789114                        @ read clear"*/
                    /*r1 = */
                    peek(r0); /*"    ldr   r1, [r0]"*/
                    r0 = 0x1e789124 as u32; /*"    ldr   r0, =0x1e789124                        @ read clear"*/
                    /*r1 = */
                    peek(r0); /*"    ldr   r1, [r0]"*/
                    r0 = 0x1e78903c as u32; /*"    ldr   r0, =0x1e78903c"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e789040 as u32; /*"    ldr   r0, =0x1e789040"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e789044 as u32; /*"    ldr   r0, =0x1e789044"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e78911c as u32; /*"    ldr   r0, =0x1e78911c"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e78912c as u32; /*"    ldr   r0, =0x1e78912c"*/
                    r1 = 0x200 as u32; /*"    ldr   r1, =0x200"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789104 as u32; /*"    ldr   r0, =0x1e789104"*/
                    r1 = 0xcc00 as u32; /*"    ldr   r1, =0xcc00"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789108 as u32; /*"    ldr   r0, =0x1e789108"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e78910c as u32; /*"    ldr   r0, =0x1e78910c"*/
                    r1 = 0x1f0 as u32; /*"    ldr   r1, =0x1f0"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789170 as u32; /*"    ldr   r0, =0x1e789170"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e789174 as u32; /*"    ldr   r0, =0x1e789174"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e7890a0 as u32; /*"    ldr   r0, =0x1e7890a0"*/
                    r1 = 0xff00 as u32; /*"    ldr   r1, =0xff00"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e7890a4 as u32; /*"    ldr   r0, =0x1e7890a4"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e789080 as u32; /*"    ldr   r0, =0x1e789080"*/
                    r1 = 0x400 as u32; /*"    ldr   r1, =0x400"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789084 as u32; /*"    ldr   r0, =0x1e789084"*/
                    r1 = 0x0001000f as u32; /*"    ldr   r1, =0x0001000f"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789088 as u32; /*"    ldr   r0, =0x1e789088"*/
                    r1 = 0x3000fff8 as u32; /*"    ldr   r1, =0x3000fff8"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e78908c as u32; /*"    ldr   r0, =0x1e78908c"*/
                    r1 = 0xfff8f007 as u32; /*"    ldr   r1, =0xfff8f007"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789098 as u32; /*"    ldr   r0, =0x1e789098"*/
                    r1 = 0x00000a30 as u32; /*"    ldr   r1, =0x00000a30"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e78909c as u32; /*"    ldr   r0, =0x1e78909c"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e789100 as u32; /*"    ldr   r0, =0x1e789100"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e789130 as u32; /*"    ldr   r0, =0x1e789130"*/
                    r1 = 0x00000080 as u32; /*"    ldr   r1, =0x00000080"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789138 as u32; /*"    ldr   r0, =0x1e789138"*/
                    r1 = 0x00010198 as u32; /*"    ldr   r1, =0x00010198"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789140 as u32; /*"    ldr   r0, =0x1e789140"*/
                    r1 = 0x0000a000 as u32; /*"    ldr   r1, =0x0000a000"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789158 as u32; /*"    ldr   r0, =0x1e789158"*/
                    r1 = 0x00000080 as u32; /*"    ldr   r1, =0x00000080"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789180 as u32; /*"    ldr   r0, =0x1e789180"*/
                    r1 = 0xb6db1bff as u32; /*"    ldr   r1, =0xb6db1bff"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789184 as u32; /*"    ldr   r0, =0x1e789184"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e789188 as u32; /*"    ldr   r0, =0x1e789188"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e78918c as u32; /*"    ldr   r0, =0x1e78918c"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e789190 as u32; /*"    ldr   r0, =0x1e789190"*/
                    r1 = 0x05020100 as u32; /*"    ldr   r1, =0x05020100"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789194 as u32; /*"    ldr   r0, =0x1e789194"*/
                    r1 = 0x07000706 as u32; /*"    ldr   r1, =0x07000706"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789198 as u32; /*"    ldr   r0, =0x1e789198"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e78919c as u32; /*"    ldr   r0, =0x1e78919c"*/
                    r1 = 0x30 as u32; /*"    ldr   r1, =0x30"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e7891a0 as u32; /*"    ldr   r0, =0x1e7891a0"*/
                    r1 = 0x00008100 as u32; /*"    ldr   r1, =0x00008100"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e7891a4 as u32; /*"    ldr   r0, =0x1e7891a4"*/
                    r1 = 0x2000 as u32; /*"    ldr   r1, =0x2000"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e7891a8 as u32; /*"    ldr   r0, =0x1e7891a8"*/
                    r1 = 0x3ff as u32; /*"    ldr   r1, =0x3ff"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e7891ac as u32; /*"    ldr   r0, =0x1e7891ac"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e789240 as u32; /*"    ldr   r0, =0x1e789240"*/
                    r1 = 0xff as u32; /*"    mov   r1, #0xff"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789244 as u32; /*"    ldr   r0, =0x1e789244"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789248 as u32; /*"    ldr   r0, =0x1e789248"*/
                    r1 = 0x80 as u32; /*"    mov   r1, #0x80"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789250 as u32; /*"    ldr   r0, =0x1e789250"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e789254 as u32; /*"    ldr   r0, =0x1e789254"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    /***** Clear LPC status : End *****/

                    r0 = 0x1e62009c as u32; /*"    ldr   r0, =0x1e62009c                        @ clear software strap flag for doing again after reset"*/
                    r1 = 0xAEEDFC20 as u32; /*"    ldr   r1, =0xAEEDFC20"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e785004 as u32; /*"    ldr   r0, =0x1e785004"*/
                    r1 = 0x00000001 as u32; /*"    ldr   r1, =0x00000001"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e785008 as u32; /*"    ldr   r0, =0x1e785008"*/
                    r1 = 0x00004755 as u32; /*"    ldr   r1, =0x00004755"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e78501c as u32; /*"    ldr   r0, =0x1e78501c                        @ enable full mask of SOC reset"*/
                    r1 = 0x03FFFFFF as u32; /*"    ldr   r1, =0x03FFFFFF                        @ added 2016.09.06"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e78500c as u32; /*"    ldr   r0, =0x1e78500c                        @ enable SOC reset"*/
                    r1 = 0x00000013 as u32; /*"    ldr   r1, =0x00000013"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                } // #endif
                State::wait_first_reset
            }
            State::wait_first_reset => {
                // What are we doing here? Simply put, we've kicked off a reset from
                // above, and we loop here. At some point the reset comes in and we're back to
                // the beginning.
                s = State::wait_first_reset;
                continue; /*"    b     wait_first_reset"*/

                /********************************************
                  Initial Reset Procedure : End
                *******************************************/

                State::bypass_first_reset
            }
            State::bypass_first_reset => {
                /* Enable Timer separate clear mode */
                r0 = 0x1e782038 as u32; /*"    ldr   r0, =0x1e782038"*/
                r1 = 0xAE as u32; /*"    mov   r1, #0xAE"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Test - DRAM initial time */
                r0 = 0x1e78203c as u32; /*"    ldr   r0, =0x1e78203c"*/
                r1 = 0x0000F000 as u32; /*"    ldr   r1, =0x0000F000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e782044 as u32; /*"    ldr   r0, =0x1e782044"*/
                r1 = 0xFFFFFFFF as u32; /*"    ldr   r1, =0xFFFFFFFF"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e782030 as u32; /*"    ldr   r0, =0x1e782030"*/
                r2 = 3 as u32; /*"    mov   r2, #3"*/
                r1 = r2 << 12 as u32; /*"    mov   r1, r2, lsl #12"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Test - DRAM initial time */

                /*Set Scratch register Bit 7 before initialize*/
                r0 = 0x1e6e2000 as u32; /*"    ldr   r0, =0x1e6e2000"*/
                r1 = 0x1688a8a8 as u32; /*"    ldr   r1, =0x1688a8a8"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e2040 as u32; /*"    ldr   r0, =0x1e6e2040"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 = r1 | 0x80 as u32; /*"    orr   r1, r1, #0x80"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Change LPC reset source to PERST# when eSPI mode enabled */
                r0 = 0x1e6e2070 as u32; /*"    ldr   r0, =0x1e6e2070"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r0 = 0x1e6e207c as u32; /*"    ldr   r0, =0x1e6e207c"*/
                r2 = 0x02000000 as u32; /*"    ldr   r2, =0x02000000"*/
                r3 = 0x00004000 as u32; /*"    ldr   r3, =0x00004000"*/
                z = r1 == r2; /*"    tst   r1, r2"*/
                if !z {
                    poke(r3, r0);
                } /*"    strne r3, [r0]"*/

                /* Configure USB ports to the correct pin state */
                r0 = 0x1e6e200c as u32; /*"    ldr   r0, =0x1e6e200c                        @ enable portA clock"*/
                r2 = 0x00004000 as u32; /*"    ldr   r2, =0x00004000"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 = r1 | r2; /*"    orr   r1, r1, r2"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e2090 as u32; /*"    ldr   r0, =0x1e6e2090                        @ set portA as host mode"*/
                r1 = 0x2000A000 as u32; /*"    ldr   r1, =0x2000A000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e2094 as u32; /*"    ldr   r0, =0x1e6e2094                        @ set portB as host mode"*/
                r1 = 0x00004000 as u32; /*"    ldr   r1, =0x00004000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e2070 as u32; /*"    ldr   r0, =0x1e6e2070"*/
                r2 = 0x00800000 as u32; /*"    ldr   r2, =0x00800000"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == r2; /*"    tst   r1, r2"*/
                if z {
                    s = State::bypass_USB_init;
                    continue;
                } /*"    beq   bypass_USB_init"*/
                r0 = 0x1e6e207c as u32; /*"    ldr   r0, =0x1e6e207c"*/
                poke(r2, r0); /*"    str   r2, [r0]"*/

                /* Delay about 1ms */
                clear_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    clear_delay_timer"*/
                r2 = 0x000003E8 as u32; /*"    ldr   r2, =0x000003E8                        @ Set Timer3 Reload = 1 ms"*/
                init_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    init_delay_timer"*/
                State::wait_usb_init
            }
            State::wait_usb_init => {
                check_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    check_delay_timer"*/
                if !z {
                    s = State::wait_usb_init;
                    continue;
                } /*"    bne   wait_usb_init"*/
                clear_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    clear_delay_timer"*/
                /* end delay 1ms */

                r0 = 0x1e6e2070 as u32; /*"    ldr   r0, =0x1e6e2070"*/
                r1 = 0x00800000 as u32; /*"    ldr   r1, =0x00800000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                State::bypass_USB_init
            }
            State::bypass_USB_init => {
                /* Enable AXI_P */
                //r0 = 0x00000016 as u32; /*"    ldr   r0, =0x00000016"*/
                //mrc(p15, 0, r1, c15, c2, 4);/*"    mrc   p15, 0, r1, c15, c2, 4"*/
                //mcr(p15, 0, r0, c15, c2, 4);/*"    mcr   p15, 0, r0, c15, c2, 4"*/
                /******************************************************************************
                Disable WDT2 for 2nd boot function
                ******************************************************************************/
                /*
                    if CONFIG_FIRMWARE_2ND_BOOT == 0  { // #ifndef CONFIG_FIRMWARE_2ND_BOOT
                    r0 = 0x1e78502c as u32;/*"    ldr   r0, =0x1e78502c"*/
                r1 = 0 as u32;/*"    mov   r1, #0"*/
                poke(r1,r0);/*"    str   r1, [r0]"*/
                } // #endif
                */
                /******************************************************************************
                Disable WDT3 for SPI Address mode (3 or 4 bytes) detection function
                ******************************************************************************/
                r0 = 0x1e78504c as u32; /*"    ldr   r0, =0x1e78504c"*/
                r1 = 0 as u32; /*"    mov   r1, #0"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0000 as u32; /*"    ldr   r0, =0x1e6e0000"*/
                r1 = 0xFC600309 as u32; /*"    ldr   r1, =0xFC600309"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Check Scratch Register Bit 6 */
                r0 = 0x1e6e2040 as u32; /*"    ldr   r0, =0x1e6e2040"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 = r1 & !0xFFFFFFBF as u32; /*"    bic   r1, r1, #0xFFFFFFBF"*/
                r2 = r1 >> 6 as u32; /*"    mov   r2, r1, lsr #6"*/
                z = r2 == 0x01 as u32;

                if z {
                    s = State::platform_exit;
                    continue;
                } /*"    beq   platform_exit"*/

                /* Disable VGA display */
                r0 = 0x1e6e202c as u32; /*"    ldr   r0, =0x1e6e202c"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 = r1 | 0x40 as u32; /*"    orr   r1, r1, #0x40"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e2070 as u32; /*"    ldr   r0, =0x1e6e2070                        @ Load strap register"*/
                r3 = peek(r0); /*"    ldr   r3, [r0]"*/

                /* Set M-PLL */
                if CONFIG_DRAM_1333 == 1 {
                    // #if   defined (CONFIG_DRAM_1333)
                    r2 = 0xC48066C0 as u32; /*"    ldr   r2, =0xC48066C0                        @ load PLL parameter for 24Mhz CLKIN (330)"*/
                } else {
                    // #else 	r2 = 0x93002400 as u32;/*"    ldr   r2, =0x93002400                        @ load PLL parameter for 24Mhz CLKIN (396)"*/
                    if CONFIG_DDR4_SUPPORT_HYNIX == 1 {
                        // #if   defined (CONFIG_DDR4_SUPPORT_HYNIX)
                        r1 = r3 >> 24 as u32; /*"    mov   r1, r3, lsr #24                        @ Check DDR4"*/
                        z = r1 == 0x01 as u32; /*"    tst   r1, #0x01"*/
                        if z {
                            s = State::bypass_mpll_hynix_mode_1;
                            continue;
                        } /*"    beq   bypass_mpll_hynix_mode_1"*/
                        if CONFIG_DDR4_HYNIX_SET_1536 == 1 {
                            // #if   defined (CONFIG_DDR4_HYNIX_SET_1536)
                            r2 = 0x930023E0 as u32; /*"    ldr   r2, =0x930023E0                        @ load PLL parameter for 24Mhz CLKIN (384)"*/
                        } else if CONFIG_DDR4_HYNIX_SET_1488 == 1 {
                            // #elif defined (CONFIG_DDR4_HYNIX_SET_1488)
                            r2 = 0x930023C0 as u32; /*"    ldr   r2, =0x930023C0                        @ load PLL parameter for 24Mhz CLKIN (372)"*/
                        } else {
                            // #else 	r2 = 0x930023A0 as u32;/*"    ldr   r2, =0x930023A0                        @ load PLL parameter for 24Mhz CLKIN (360)"*/
                        } // #endif
                        s = State::bypass_mpll_hynix_mode_1;
                        continue; /*"\tb bypass_mpll_hynix_mode_1"*/
                    } // #endif
                } // #endif

                State::bypass_mpll_hynix_mode_1
            }
            State::bypass_mpll_hynix_mode_1 => {
                r1 = r3 >> 23 as u32; /*"    mov   r1, r3, lsr #23                        @ Check CLKIN = 25MHz"*/
                z = r1 == 0x01 as u32; /*"    tst   r1, #0x01"*/
                if z {
                    s = State::set_MPLL;
                    continue;
                } /*"    beq   set_MPLL"*/
                if CONFIG_DRAM_1333 == 1 {
                    // #if   defined (CONFIG_DRAM_1333)
                    r2 = 0xC4806680 as u32; /*"    ldr   r2, =0xC4806680                        @ load PLL parameter for 25Mhz CLKIN (331)"*/
                } else {
                    // #else 	r2 = 0x930023E0 as u32;/*"    ldr   r2, =0x930023E0                        @ load PLL parameter for 25Mhz CLKIN (400)"*/
                    if CONFIG_DDR4_SUPPORT_HYNIX == 1 {
                        // #if   defined (CONFIG_DDR4_SUPPORT_HYNIX)
                        r1 = r3 >> 24 as u32; /*"    mov   r1, r3, lsr #24                        @ Check DDR4"*/
                        z = r1 == 0x01 as u32; /*"    tst   r1, #0x01"*/
                        if z {
                            s = State::bypass_mpll_hynix_mode_2;
                            continue;
                        } /*"    beq   bypass_mpll_hynix_mode_2"*/
                        if CONFIG_DDR4_HYNIX_SET_1536 == 1 {
                            // #if   defined (CONFIG_DDR4_HYNIX_SET_1536)
                            r2 = 0x930023C0 as u32; /*"    ldr   r2, =0x930023C0                        @ load PLL parameter for 24Mhz CLKIN (387.5)"*/
                        } else if CONFIG_DDR4_HYNIX_SET_1488 == 1 {
                            // #elif defined (CONFIG_DDR4_HYNIX_SET_1488)
                            r2 = 0x930023A0 as u32; /*"    ldr   r2, =0x930023A0                        @ load PLL parameter for 24Mhz CLKIN (375)"*/
                        } else {
                            // #else 	r2 = 0x93002380 as u32;/*"    ldr   r2, =0x93002380                        @ load PLL parameter for 24Mhz CLKIN (362.5)"*/
                            s = State::bypass_mpll_hynix_mode_2;
                            continue; /*"    b   bypass_mpll_hynix_mode_2"*/
                        } // #endif
                    } // #endif
                } // #endif
                State::bypass_mpll_hynix_mode_2
            }
            State::bypass_mpll_hynix_mode_2 => {
                r0 = 0x1e6e2160 as u32; /*"    ldr   r0, =0x1e6e2160                        @ set 24M Jitter divider (HPLL=825MHz)"*/
                r1 = 0x00011320 as u32; /*"    ldr   r1, =0x00011320"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                State::set_MPLL
            }
            State::set_MPLL => {
                r0 = 0x1e6e2020 as u32; /*"    ldr   r0, =0x1e6e2020                        @ M-PLL (DDR SDRAM) Frequency"*/
                poke(r2, r0); /*"    str   r2, [r0]"*/

                clear_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    clear_delay_timer"*/

                /* Delay about 3ms */
                r2 = 0x00000BB8 as u32; /*"    ldr   r2, =0x00000BB8                        @ Set Timer3 Reload = 3 ms"*/
                init_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    init_delay_timer"*/
                State::wait_mpll_init
            }
            State::wait_mpll_init => {
                check_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    check_delay_timer"*/
                if !z {
                    s = State::wait_mpll_init;
                    continue;
                } /*"    bne   wait_mpll_init"*/
                clear_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    clear_delay_timer"*/
                /* end delay 3ms */

                /* Reset MMC */
                State::reset_mmc
            }
            State::reset_mmc => {
                r0 = 0x1e78505c as u32; /*"    ldr   r0, =0x1e78505c"*/
                r1 = 0x00000004 as u32; /*"    ldr   r1, =0x00000004"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e785044 as u32; /*"    ldr   r0, =0x1e785044"*/
                r1 = 0x00000001 as u32; /*"    ldr   r1, =0x00000001"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e785048 as u32; /*"    ldr   r0, =0x1e785048"*/
                r1 = 0x00004755 as u32; /*"    ldr   r1, =0x00004755"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e78504c as u32; /*"    ldr   r0, =0x1e78504c"*/
                r1 = 0x00000013 as u32; /*"    ldr   r1, =0x00000013"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                State::wait_mmc_reset
            }
            State::wait_mmc_reset => {
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == 0x02 as u32; /*"    tst   r1, #0x02"*/
                if !z {
                    s = State::wait_mmc_reset;
                    continue;
                } /*"    bne   wait_mmc_reset"*/

                r0 = 0x1e78505c as u32; /*"    ldr   r0, =0x1e78505c"*/
                r1 = 0x023FFFF3 as u32; /*"    ldr   r1, =0x023FFFF3"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e785044 as u32; /*"    ldr   r0, =0x1e785044"*/
                r1 = 0x000F4240 as u32; /*"    ldr   r1, =0x000F4240"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e785048 as u32; /*"    ldr   r0, =0x1e785048"*/
                r1 = 0x00004755 as u32; /*"    ldr   r1, =0x00004755"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e785054 as u32; /*"    ldr   r0, =0x1e785054"*/
                r1 = 0x00000077 as u32; /*"    ldr   r1, =0x00000077"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0000 as u32; /*"    ldr   r0, =0x1e6e0000"*/
                r1 = 0xFC600309 as u32; /*"    ldr   r1, =0xFC600309"*/
                State::wait_mmc_reset_done
            }
            State::wait_mmc_reset_done => {
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r2 = peek(r0); /*"    ldr   r2, [r0]"*/
                z = r2 == 0x1 as u32;

                if !z {
                    s = State::wait_mmc_reset_done;
                    continue;
                } /*"    bne   wait_mmc_reset_done"*/

                r0 = 0x1e6e0034 as u32; /*"    ldr   r0, =0x1e6e0034                        @ disable MMC request"*/
                r1 = 0x00020000 as u32; /*"    ldr   r1, =0x00020000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Delay about 10ms */
                r2 = 0x00002710 as u32; /*"    ldr   r2, =0x00002710                        @ Set Timer3 Reload = 10 ms"*/
                init_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    init_delay_timer"*/
                State::wait_ddr_reset
            }
            State::wait_ddr_reset => {
                check_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    check_delay_timer"*/
                if !z {
                    s = State::wait_ddr_reset;
                    continue;
                } /*"    bne   wait_ddr_reset"*/
                clear_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    clear_delay_timer"*/
                /* end delay 10ms */

                /* Debug - UART console message */
                if CONFIG_DRAM_UART_TO_UART1 == 1 {
                    // #ifdef CONFIG_DRAM_UART_TO_UART1
                    r0 = 0x1e78909c as u32; /*"    ldr   r0, =0x1e78909c                        @ route UART5 to UART Port1, 2016.08.29"*/
                    r1 = 0x10000004 as u32; /*"    ldr   r1, =0x10000004"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/

                    r0 = 0x1e6e2084 as u32; /*"    ldr   r0, =0x1e6e2084"*/
                    r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                    r2 = 0xC0 as u32; /*"    mov   r2, #0xC0                              @ Enable pinmux of TXD1/RXD1"*/
                    r1 = r1 | (r2 << 16 as u32); /*"    orr   r1, r1, r2, lsl #16"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                } // #endif

                r0 = 0x1e78400c as u32; /*"    ldr   r0, =0x1e78400c"*/
                r1 = 0x83 as u32; /*"    mov   r1, #0x83"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e202c as u32; /*"    ldr   r0, =0x1e6e202c"*/
                r2 = peek(r0); /*"    ldr   r2, [r0]"*/
                r2 = r2 >> 12 as u32; /*"    mov   r2, r2, lsr #12"*/
                z = r2 == 0x01 as u32; /*"    tst   r2, #0x01"*/
                r0 = 0x1e784000 as u32; /*"    ldr   r0, =0x1e784000"*/
                if z {
                    r1 = 0x0D as u32;
                } /*"    moveq r1, #0x0D                              @ Baudrate 115200"*/
                if !z {
                    r1 = 0x01 as u32;
                } /*"    movne r1, #0x01                              @ Baudrate 115200, div13"*/
                if CONFIG_DRAM_UART_38400 == 1 {
                    // #ifdef CONFIG_DRAM_UART_38400
                    if z {
                        r1 = 0x27 as u32;
                    } /*"    moveq r1, #0x27                              @ Baudrate 38400"*/
                    if !z {
                        r1 = 0x03 as u32;
                    } /*"    movne r1, #0x03                              @ Baudrate 38400 , div13"*/
                } // #endif
                if CONFIG_DRAM_UART_57600 == 1 {
                    // #ifdef CONFIG_DRAM_UART_57600
                    if z {
                        r1 = 0x1A as u32;
                    } /*"    moveq r1, #0x1A                              @ Baudrate 57600"*/
                    if !z {
                        r1 = 0x02 as u32;
                    } /*"    movne r1, #0x02                              @ Baudrate 57600 , div13"*/
                } // #endif
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e784004 as u32; /*"    ldr   r0, =0x1e784004"*/
                r1 = 0x00 as u32; /*"    mov   r1, #0x00"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e78400c as u32; /*"    ldr   r0, =0x1e78400c"*/
                r1 = 0x03 as u32; /*"    mov   r1, #0x03"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e784008 as u32; /*"    ldr   r0, =0x1e784008"*/
                r1 = 0x07 as u32; /*"    mov   r1, #0x07"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e784000 as u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x0D as u32; /*"    mov   r1, #0x0D                              @ '\\r'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x0A as u32; /*"    mov   r1, #0x0A                              @ '\\n'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x44 as u32; /*"    mov   r1, #0x44                              @ 'D'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x52 as u32; /*"    mov   r1, #0x52                              @ 'R'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x41 as u32; /*"    mov   r1, #0x41                              @ 'A'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x4D as u32; /*"    mov   r1, #0x4D                              @ 'M'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x20 as u32; /*"    mov   r1, #0x20                              @ ' '"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x49 as u32; /*"    mov   r1, #0x49                              @ 'I'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6E as u32; /*"    mov   r1, #0x6E                              @ 'n'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x69 as u32; /*"    mov   r1, #0x69                              @ 'i'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x74 as u32; /*"    mov   r1, #0x74                              @ 't'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x2D as u32; /*"    mov   r1, #0x2D                              @ '-'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x56 as u32; /*"    mov   r1, #0x56                              @ 'V'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = ASTMMC_INIT_VER as u32; /*"    mov   r1, #ASTMMC_INIT_VER"*/
                r1 = r1 >> 4 as u32; /*"    mov   r1, r1, lsr #4"*/
                print_hex_char!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    print_hex_char"*/
                r1 = ASTMMC_INIT_VER as u32; /*"    mov   r1, #ASTMMC_INIT_VER"*/
                print_hex_char!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    print_hex_char"*/
                r1 = 0x2D as u32; /*"    mov   r1, #0x2D                              @ '-'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e784014 as u32; /*"    ldr   r0, =0x1e784014"*/
                State::wait_print
            }
            State::wait_print => {
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == 0x40 as u32; /*"    tst   r1, #0x40"*/
                if z {
                    s = State::wait_print;
                    continue;
                } /*"    beq   wait_print"*/
                r0 = 0x1e784000 as u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x44 as u32; /*"    mov   r1, #0x44                              @ 'D'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x44 as u32; /*"    mov   r1, #0x44                              @ 'D'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x52 as u32; /*"    mov   r1, #0x52                              @ 'R'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Debug - UART console message */

                /******************************************************************************
                Init DRAM common registers
                ******************************************************************************/
                r0 = 0x1e6e0034 as u32; /*"    ldr   r0, =0x1e6e0034                        @ disable SDRAM reset"*/
                r1 = 0x00020080 as u32; /*"    ldr   r1, =0x00020080"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0008 as u32; /*"    ldr   r0, =0x1e6e0008"*/
                r1 = 0x2003000F as u32; /*"    ldr   r1, =0x2003000F                        /* VGA */"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0038 as u32; /*"    ldr   r0, =0x1e6e0038                        @ disable all DRAM requests except CPU during PHY init"*/
                r1 = 0xFFFFEBFF as u32; /*"    ldr   r1, =0xFFFFEBFF"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0040 as u32; /*"    ldr   r0, =0x1e6e0040"*/
                r1 = 0x88448844 as u32; /*"    ldr   r1, =0x88448844"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0044 as u32; /*"    ldr   r0, =0x1e6e0044"*/
                r1 = 0x24422288 as u32; /*"    ldr   r1, =0x24422288"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0048 as u32; /*"    ldr   r0, =0x1e6e0048"*/
                r1 = 0x22222222 as u32; /*"    ldr   r1, =0x22222222"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e004c as u32; /*"    ldr   r0, =0x1e6e004c"*/
                r1 = 0x22222222 as u32; /*"    ldr   r1, =0x22222222"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0050 as u32; /*"    ldr   r0, =0x1e6e0050"*/
                r1 = 0x80000000 as u32; /*"    ldr   r1, =0x80000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r1 = 0x00000000 as u32; /*"    ldr   r1, =0x00000000"*/
                r0 = 0x1e6e0208 as u32; /*"    ldr   r0, =0x1e6e0208                        @ PHY Setting"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e0218 as u32; /*"    ldr   r0, =0x1e6e0218"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e0220 as u32; /*"    ldr   r0, =0x1e6e0220"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e0228 as u32; /*"    ldr   r0, =0x1e6e0228"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e0230 as u32; /*"    ldr   r0, =0x1e6e0230"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e02a8 as u32; /*"    ldr   r0, =0x1e6e02a8"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e02b0 as u32; /*"    ldr   r0, =0x1e6e02b0"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0240 as u32; /*"    ldr   r0, =0x1e6e0240"*/
                r1 = 0x86000000 as u32; /*"    ldr   r1, =0x86000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0244 as u32; /*"    ldr   r0, =0x1e6e0244"*/
                r1 = 0x00008600 as u32; /*"    ldr   r1, =0x00008600"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0248 as u32; /*"    ldr   r0, =0x1e6e0248"*/
                r1 = 0x80000000 as u32; /*"    ldr   r1, =0x80000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e024c as u32; /*"    ldr   r0, =0x1e6e024c"*/
                r1 = 0x80808080 as u32; /*"    ldr   r1, =0x80808080"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Check DRAM Type by H/W Trapping */
                r0 = 0x1e6e2070 as u32; /*"    ldr   r0, =0x1e6e2070"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r2 = 0x01000000 as u32; /*"    ldr   r2, =0x01000000                        @ bit[24]=1 => DDR4"*/
                z = r1 == r2; /*"    tst   r1, r2"*/
                if !z {
                    s = State::ddr4_init;
                    continue;
                } /*"    bne   ddr4_init"*/
                s = State::ddr3_init;
                continue; /*"    b     ddr3_init"*/

                /******************************************************************************
                DDR3 Init
                ******************************************************************************/
                State::ddr3_init
            }
            State::ddr3_init => {
                /* Debug - UART console message */
                r0 = 0x1e784000 as u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x33 as u32; /*"    mov   r1, #0x33                              @ '3'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x0D as u32; /*"    mov   r1, #0x0D                              @ '\\r'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x0A as u32; /*"    mov   r1, #0x0A                              @ '\\n'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Debug - UART console message */

                if CONFIG_DRAM_1333 == 1 {
                    // #if   defined (CONFIG_DRAM_1333)
                    tptr = ramtable::TIME_TABLE_DDR3_1333 /*"    adrl  r5, TIME_TABLE_DDR3_1333               @ Init DRAM parameter table"*/
                } else {
                    // #else tptr = ramtable::TIME_TABLE_DDR3_1600/*"    adrl  r5, TIME_TABLE_DDR3_1600"*/
                } // #endif

                r0 = 0x1e6e0004 as u32; /*"    ldr   r0, =0x1e6e0004"*/
                if CONFIG_DDR3_8GSTACK == 1 {
                    // #ifdef CONFIG_DDR3_8GSTACK
                    r1 = 0x00000323 as u32; /*"    ldr   r1, =0x00000323                        @ Init to 8GB stack"*/
                } else {
                    // #else 	r1 = 0x00000303 as u32;/*"    ldr   r1, =0x00000303                        @ Init to 8GB"*/
                } // #endif
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0010 as u32; /*"    ldr   r0, =0x1e6e0010"*/
                r1 = tptr[(ASTMMC_REGIDX_010 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_010]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0014 as u32; /*"    ldr   r0, =0x1e6e0014"*/
                r1 = tptr[(ASTMMC_REGIDX_014 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_014]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0018 as u32; /*"    ldr   r0, =0x1e6e0018"*/
                r1 = tptr[(ASTMMC_REGIDX_018 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_018]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* DRAM Mode Register Setting */
                r0 = 0x1e6e0020 as u32; /*"    ldr   r0, =0x1e6e0020                        @ MRS_4/6"*/
                r1 = tptr[(ASTMMC_REGIDX_020 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_020]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0024 as u32; /*"    ldr   r0, =0x1e6e0024                        @ MRS_5"*/
                r1 = tptr[(ASTMMC_REGIDX_024 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_024]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e002c as u32; /*"    ldr   r0, =0x1e6e002c                        @ MRS_0/2"*/
                r1 = tptr[(ASTMMC_REGIDX_02C as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_02C]"*/
                r2 = 0x1 as u32; /*"    mov   r2, #0x1"*/
                r1 = r1 | (r2 << 8 as u32); /*"    orr   r1, r1, r2, lsl #8"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0030 as u32; /*"    ldr   r0, =0x1e6e0030                        @ MRS_1/3"*/
                r1 = tptr[(ASTMMC_REGIDX_030 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_030]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Start DDR PHY Setting */
                r0 = 0x1e6e0200 as u32; /*"    ldr   r0, =0x1e6e0200"*/
                r1 = 0x02492AAE as u32; /*"    ldr   r1, =0x02492AAE"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0204 as u32; /*"    ldr   r0, =0x1e6e0204"*/
                if CONFIG_DDR3_8GSTACK == 1 {
                    // #ifdef CONFIG_DDR3_8GSTACK
                    r1 = 0x10001001 as u32; /*"    ldr   r1, =0x10001001"*/
                } else {
                    // #else 	r1 = 0x00001001 as u32;/*"    ldr   r1, =0x00001001"*/
                } // #endif
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e020c as u32; /*"    ldr   r0, =0x1e6e020c"*/
                r1 = 0x55E00B0B as u32; /*"    ldr   r1, =0x55E00B0B"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0210 as u32; /*"    ldr   r0, =0x1e6e0210"*/
                r1 = 0x20000000 as u32; /*"    ldr   r1, =0x20000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0214 as u32; /*"    ldr   r0, =0x1e6e0214"*/
                r1 = tptr[(ASTMMC_REGIDX_214 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_214]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02e0 as u32; /*"    ldr   r0, =0x1e6e02e0"*/
                r1 = tptr[(ASTMMC_REGIDX_2E0 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_2E0]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02e4 as u32; /*"    ldr   r0, =0x1e6e02e4"*/
                r1 = tptr[(ASTMMC_REGIDX_2E4 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_2E4]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02e8 as u32; /*"    ldr   r0, =0x1e6e02e8"*/
                r1 = tptr[(ASTMMC_REGIDX_2E8 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_2E8]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02ec as u32; /*"    ldr   r0, =0x1e6e02ec"*/
                r1 = tptr[(ASTMMC_REGIDX_2EC as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_2EC]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02f0 as u32; /*"    ldr   r0, =0x1e6e02f0"*/
                r1 = tptr[(ASTMMC_REGIDX_2F0 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_2F0]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02f4 as u32; /*"    ldr   r0, =0x1e6e02f4"*/
                r1 = tptr[(ASTMMC_REGIDX_2F4 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_2F4]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02f8 as u32; /*"    ldr   r0, =0x1e6e02f8"*/
                r1 = tptr[(ASTMMC_REGIDX_2F8 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_2F8]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0290 as u32; /*"    ldr   r0, =0x1e6e0290"*/
                r1 = 0x00100008 as u32; /*"    ldr   r1, =0x00100008"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02c0 as u32; /*"    ldr   r0, =0x1e6e02c0"*/
                r1 = 0x00000006 as u32; /*"    ldr   r1, =0x00000006"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Controller Setting */
                r0 = 0x1e6e0060 as u32; /*"    ldr   r0, =0x1e6e0060                        @ Fire DDRPHY Init"*/
                r1 = 0x00000005 as u32; /*"    ldr   r1, =0x00000005"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0034 as u32; /*"    ldr   r0, =0x1e6e0034"*/
                r1 = 0x00020091 as u32; /*"    ldr   r1, =0x00020091"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Debug - UART console message */
                r0 = 0x1e784000 as u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x30 as u32; /*"    mov   r1, #0x30                              @ '0'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Debug - UART console message */

                r0 = 0x1e6e0120 as u32; /*"    ldr   r0, =0x1e6e0120"*/
                r1 = 0x00 as u32; /*"    mov   r1, #0x00"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                s = State::ddr_phy_init_process;
                continue; /*"    b     ddr_phy_init_process"*/

                State::ddr3_phyinit_done
            }
            State::ddr3_phyinit_done => {
                /********************************************
                 Check Read training margin
                ********************************************/
                r0 = 0x1e6e03a0 as u32; /*"    ldr   r0, =0x1e6e03a0                        @ check Gate Training Pass Window"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r2 = 0x150 as u32; /*"    ldr   r2, =0x150"*/
                r0 = r1 & !0xFF000000 as u32; /*"    bic   r0, r1, #0xFF000000"*/
                r0 = r0 & !0x00FF0000 as u32; /*"    bic   r0, r0, #0x00FF0000"*/
                z = r0 == r2;

                lt = r0 < r2; /*"    cmp   r0, r2"*/
                if lt {
                    s = State::ddr_test_fail;
                    continue;
                } /*"    blt   ddr_test_fail"*/
                r0 = r1 >> 16 as u32; /*"    mov   r0, r1, lsr #16"*/
                z = r0 == r2;

                lt = r0 < r2; /*"    cmp   r0, r2"*/
                if lt {
                    s = State::ddr_test_fail;
                    continue;
                } /*"    blt   ddr_test_fail"*/

                r0 = 0x1e6e03d0 as u32; /*"    ldr   r0, =0x1e6e03d0                        @ check Read Data Eye Training Pass Window"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r2 = 0x90 as u32; /*"    ldr   r2, =0x90"*/
                r0 = r1 & !0x0000FF00 as u32; /*"    bic   r0, r1, #0x0000FF00"*/
                z = r0 == r2;

                lt = r0 < r2; /*"    cmp   r0, r2"*/
                if lt {
                    s = State::ddr_test_fail;
                    continue;
                } /*"    blt   ddr_test_fail"*/
                r0 = r1 >> 8 as u32; /*"    mov   r0, r1, lsr #8"*/
                z = r0 == r2;

                lt = r0 < r2; /*"    cmp   r0, r2"*/
                if lt {
                    s = State::ddr_test_fail;
                    continue;
                } /*"    blt   ddr_test_fail"*/
                /*******************************************/

                /* Debug - UART console message */
                r0 = 0x1e784000 as u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x31 as u32; /*"    mov   r1, #0x31                              @ '1'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Debug - UART console message */

                r0 = 0x1e6e000c as u32; /*"    ldr   r0, =0x1e6e000c"*/
                r1 = 0x00000040 as u32; /*"    ldr   r1, =0x00000040"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                if CONFIG_DDR3_8GSTACK == 1 {
                    // #ifdef CONFIG_DDR3_8GSTACK
                    r0 = 0x1e6e0028 as u32; /*"    ldr   r0, =0x1e6e0028"*/
                    r1 = 0x00000025 as u32; /*"    ldr   r1, =0x00000025"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/

                    r0 = 0x1e6e0028 as u32; /*"    ldr   r0, =0x1e6e0028"*/
                    r1 = 0x00000027 as u32; /*"    ldr   r1, =0x00000027"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/

                    r0 = 0x1e6e0028 as u32; /*"    ldr   r0, =0x1e6e0028"*/
                    r1 = 0x00000023 as u32; /*"    ldr   r1, =0x00000023"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/

                    r0 = 0x1e6e0028 as u32; /*"    ldr   r0, =0x1e6e0028"*/
                    r1 = 0x00000021 as u32; /*"    ldr   r1, =0x00000021"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                } // #endif

                r0 = 0x1e6e0028 as u32; /*"    ldr   r0, =0x1e6e0028"*/
                r1 = 0x00000005 as u32; /*"    ldr   r1, =0x00000005"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0028 as u32; /*"    ldr   r0, =0x1e6e0028"*/
                r1 = 0x00000007 as u32; /*"    ldr   r1, =0x00000007"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0028 as u32; /*"    ldr   r0, =0x1e6e0028"*/
                r1 = 0x00000003 as u32; /*"    ldr   r1, =0x00000003"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0028 as u32; /*"    ldr   r0, =0x1e6e0028"*/
                r1 = 0x00000011 as u32; /*"    ldr   r1, =0x00000011"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e000c as u32; /*"    ldr   r0, =0x1e6e000c"*/
                r1 = 0x00005C41 as u32; /*"    ldr   r1, =0x00005C41"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0034 as u32; /*"    ldr   r0, =0x1e6e0034"*/
                r2 = 0x70000000 as u32; /*"    ldr   r2, =0x70000000"*/
                State::ddr3_check_dllrdy
            }
            State::ddr3_check_dllrdy => {
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == r2; /*"    tst   r1, r2"*/
                if !z {
                    s = State::ddr3_check_dllrdy;
                    continue;
                } /*"    bne   ddr3_check_dllrdy"*/

                r0 = 0x1e6e000c as u32; /*"    ldr   r0, =0x1e6e000c"*/
                if CONFIG_DRAM_EXT_TEMP == 1 {
                    // #ifdef CONFIG_DRAM_EXT_TEMP
                    r1 = 0x42AA2F81 as u32; /*"    ldr   r1, =0x42AA2F81"*/
                } else {
                    // #else 	r1 = 0x42AA5C81 as u32;/*"    ldr   r1, =0x42AA5C81"*/
                } // #endif
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0034 as u32; /*"    ldr   r0, =0x1e6e0034"*/
                r1 = 0x0001AF93 as u32; /*"    ldr   r1, =0x0001AF93"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0120 as u32; /*"    ldr   r0, =0x1e6e0120                        @ VGA Compatible Mode"*/
                r1 = tptr[(ASTMMC_REGIDX_PLL as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_PLL]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                s = State::calibration_End;
                continue; /*"    b     calibration_End"*/
                /******************************************************************************
                End DDR3 Init
                ******************************************************************************/
                /******************************************************************************
                DDR4 Init
                ******************************************************************************/
                State::ddr4_init
            }
            State::ddr4_init => {
                /* Debug - UART console message */
                r0 = 0x1e784000 as u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x34 as u32; /*"    mov   r1, #0x34                              @ '4'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x0D as u32; /*"    mov   r1, #0x0D                              @ '\\r'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x0A as u32; /*"    mov   r1, #0x0A                              @ '\\n'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Debug - UART console message */

                if CONFIG_DRAM_1333 == 1 {
                    // #if   defined (CONFIG_DRAM_1333)
                    tptr = ramtable::TIME_TABLE_DDR4_1333 /*"    adrl  r5, TIME_TABLE_DDR4_1333               @ Init DRAM parameter table"*/
                } else {
                    // #else tptr = ramtable::TIME_TABLE_DDR4_1600/*"    adrl  r5, TIME_TABLE_DDR4_1600"*/
                } // #endif

                r0 = 0x1e6e0004 as u32; /*"    ldr   r0, =0x1e6e0004"*/
                if CONFIG_DDR4_4GX8 == 1 {
                    // #ifdef CONFIG_DDR4_4GX8
                    r1 = 0x00002313 as u32; /*"    ldr   r1, =0x00002313                        @ Init to 8GB"*/
                } else {
                    // #else 	r1 = 0x00000313 as u32;/*"    ldr   r1, =0x00000313                        @ Init to 8GB"*/
                } // #endif
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0010 as u32; /*"    ldr   r0, =0x1e6e0010"*/
                r1 = tptr[(ASTMMC_REGIDX_010 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_010]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0014 as u32; /*"    ldr   r0, =0x1e6e0014"*/
                r1 = tptr[(ASTMMC_REGIDX_014 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_014]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0018 as u32; /*"    ldr   r0, =0x1e6e0018"*/
                r1 = tptr[(ASTMMC_REGIDX_018 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_018]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* DRAM Mode Register Setting */
                r0 = 0x1e6e0020 as u32; /*"    ldr   r0, =0x1e6e0020                        @ MRS_4/6"*/
                r1 = tptr[(ASTMMC_REGIDX_020 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_020]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0024 as u32; /*"    ldr   r0, =0x1e6e0024                        @ MRS_5"*/
                r1 = tptr[(ASTMMC_REGIDX_024 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_024]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e002c as u32; /*"    ldr   r0, =0x1e6e002c                        @ MRS_0/2"*/
                r1 = tptr[(ASTMMC_REGIDX_02C as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_02C]"*/
                r2 = 0x1 as u32; /*"    mov   r2, #0x1"*/
                r1 = r1 | (r2 << 8 as u32); /*"    orr   r1, r1, r2, lsl #8"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0030 as u32; /*"    ldr   r0, =0x1e6e0030                        @ MRS_1/3"*/
                r1 = tptr[(ASTMMC_REGIDX_030 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_030]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Start DDR PHY Setting */
                r0 = 0x1e6e0200 as u32; /*"    ldr   r0, =0x1e6e0200"*/
                r1 = 0x42492AAE as u32; /*"    ldr   r1, =0x42492AAE"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0204 as u32; /*"    ldr   r0, =0x1e6e0204"*/
                r1 = 0x09002800 as u32; /*"    ldr   r1, =0x09002800"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e020c as u32; /*"    ldr   r0, =0x1e6e020c"*/
                r1 = 0x55E00B0B as u32; /*"    ldr   r1, =0x55E00B0B"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0210 as u32; /*"    ldr   r0, =0x1e6e0210"*/
                r1 = 0x20000000 as u32; /*"    ldr   r1, =0x20000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0214 as u32; /*"    ldr   r0, =0x1e6e0214"*/
                r1 = tptr[(ASTMMC_REGIDX_214 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_214]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02e0 as u32; /*"    ldr   r0, =0x1e6e02e0"*/
                r1 = tptr[(ASTMMC_REGIDX_2E0 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_2E0]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02e4 as u32; /*"    ldr   r0, =0x1e6e02e4"*/
                r1 = tptr[(ASTMMC_REGIDX_2E4 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_2E4]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02e8 as u32; /*"    ldr   r0, =0x1e6e02e8"*/
                r1 = tptr[(ASTMMC_REGIDX_2E8 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_2E8]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02ec as u32; /*"    ldr   r0, =0x1e6e02ec"*/
                r1 = tptr[(ASTMMC_REGIDX_2EC as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_2EC]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02f0 as u32; /*"    ldr   r0, =0x1e6e02f0"*/
                r1 = tptr[(ASTMMC_REGIDX_2F0 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_2F0]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02f4 as u32; /*"    ldr   r0, =0x1e6e02f4"*/
                r1 = tptr[(ASTMMC_REGIDX_2F4 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_2F4]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02f8 as u32; /*"    ldr   r0, =0x1e6e02f8"*/
                r1 = tptr[(ASTMMC_REGIDX_2F8 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_2F8]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0290 as u32; /*"    ldr   r0, =0x1e6e0290"*/
                r1 = 0x00100008 as u32; /*"    ldr   r1, =0x00100008"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02c4 as u32; /*"    ldr   r0, =0x1e6e02c4"*/
                r1 = 0x3C183C3C as u32; /*"    ldr   r1, =0x3C183C3C"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02c8 as u32; /*"    ldr   r0, =0x1e6e02c8"*/
                r1 = 0x00631E0E as u32; /*"    ldr   r1, =0x00631E0E"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0034 as u32; /*"    ldr   r0, =0x1e6e0034"*/
                r1 = 0x0001A991 as u32; /*"    ldr   r1, =0x0001A991"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Debug - UART console message */
                r0 = 0x1e784000 as u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x30 as u32; /*"    mov   r1, #0x30                              @ '0'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Debug - UART console message */

                /********************************************
                Set Ron value to manual mode
                Target to fix DDR CK Vix issue
                Set Ron_pu = 0, Ron_pd = trained value
                *******************************************/
                if ASTMMC_DDR4_MANUAL_RPU == 1 {
                    // #ifdef ASTMMC_DDR4_MANUAL_RPU
                    r0 = 0x1e6e02c0 as u32; /*"    ldr   r0, =0x1e6e02c0"*/
                    r1 = 0x00001806 as u32; /*"    ldr   r1, =0x00001806"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e6e02cc as u32; /*"    ldr   r0, =0x1e6e02cc"*/
                    r1 = 0x00005050 as u32; /*"    ldr   r1, =0x00005050"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e6e0120 as u32; /*"    ldr   r0, =0x1e6e0120"*/
                    r1 = 0x04 as u32; /*"    mov   r1, #0x04"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e6e0060 as u32; /*"    ldr   r0, =0x1e6e0060                        @ Fire DDRPHY Init"*/
                    r1 = 0x05 as u32; /*"    mov   r1, #0x05"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    s = State::ddr_phy_init_process;
                    continue; /*"    b     ddr_phy_init_process"*/
                } // #endif // place here by ron

                State::ddr4_ron_phyinit_done
            }
            State::ddr4_ron_phyinit_done => {
                r0 = 0x1e6e0300 as u32; /*"    ldr   r0, =0x1e6e0300                        @ read calibrated Ron_pd"*/
                r3 = peek(r0); /*"    ldr   r3, [r0]"*/
                r3 = r3 & !0xFFFFFF0F as u32; /*"    bic   r3, r3, #0xFFFFFF0F"*/
                r0 = 0x1e6e0240 as u32; /*"    ldr   r0, =0x1e6e0240"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 = r1 & !0xFF000000 as u32; /*"    bic   r1, r1, #0xFF000000"*/
                r2 = ASTMMC_DDR4_MANUAL_RPU as u32; /*"    mov   r2, #ASTMMC_DDR4_MANUAL_RPU"*/
                r1 = r1 | (r2 << 24 as u32); /*"    orr   r1, r1, r2, lsl #24"*/
                if ASTMMC_DDR4_MANUAL_RPD == 1 {
                    // #ifdef ASTMMC_DDR4_MANUAL_RPD
                    r2 = ASTMMC_DDR4_MANUAL_RPD as u32; /*"    mov   r2, #ASTMMC_DDR4_MANUAL_RPD"*/
                    r1 = r1 | (r2 << 28 as u32); /*"    orr   r1, r1, r2, lsl #28"*/
                } else {
                    // #else r1 = r1 | (r3 << 24 as u32);/*"    orr   r1, r1, r3, lsl #24"*/
                } // #endif
                r1 = r1 | 0x02 as u32; /*"    orr   r1, r1, #0x02"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0060 as u32; /*"    ldr   r0, =0x1e6e0060                        @ Reset PHY"*/
                r1 = 0x00 as u32; /*"    mov   r1, #0x00"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                //#endif
                /********************************************
                     PHY Vref Scan
                //Can't find instruction for      r6 : recorded vref value/*"     r6 : recorded vref value"*/
                //Can't find instruction for      r7 : max read eye pass window/*"     r7 : max read eye pass window"*/
                //Can't find instruction for      r8 : passcnt/*"     r8 : passcnt"*/
                //Can't find instruction for      r9 : CBRtest result/*"     r9 : CBRtest result"*/
                //Can't find instruction for      r10: loopcnt/*"     r10: loopcnt"*/
                //Can't find instruction for      r11: free/*"     r11: free"*/
                    ********************************************/
                r0 = 0x1e720000 as u32; /*"    ldr   r0, =0x1e720000                        @ retry count"*/
                r1 = 0x5 as u32; /*"    mov   r1, #0x5"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                State::ddr4_vref_phy_cal_start
            }
            State::ddr4_vref_phy_cal_start => {
                r7 = 0x0 as u32; /*"    mov   r7, #0x0"*/
                r8 = 0x0 as u32; /*"    mov   r8, #0x0"*/
                r10 = 0x3F as u32; /*"    mov   r10, #0x3F"*/

                r0 = 0x1e720000 as u32; /*"    ldr   r0, =0x1e720000"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 = r1 - 0x01 as u32; /*"    subs  r1, r1, #0x01"*/
                if z {
                    s = State::ddr_test_fail;
                    continue;
                } /*"    beq   ddr_test_fail"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0120 as u32; /*"    ldr   r0, =0x1e6e0120"*/
                r1 = 0x00000001 as u32; /*"    ldr   r1, =0x00000001"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Debug - UART console message */
                r0 = 0x1e784000 as u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x61 as u32; /*"    mov   r1, #0x61                              @ 'a'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Debug - UART console message */

                r0 = 0x1e6e02c0 as u32; /*"    ldr   r0, =0x1e6e02c0"*/
                r1 = 0x00001C06 as u32; /*"    ldr   r1, =0x00001C06"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                State::ddr4_vref_phy_loop
            }
            State::ddr4_vref_phy_loop => {
                r0 = 0x1e6e0060 as u32; /*"    ldr   r0, =0x1e6e0060"*/
                r1 = 0x00000000 as u32; /*"    ldr   r1, =0x00000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r10 = r10 + 0x01 as u32; /*"    add   r10, r10, #0x01"*/
                z = r10 == 0x80 as u32;

                if z {
                    s = State::ddr4_vref_phy_test_fail;
                    continue;
                } /*"    beq   ddr4_vref_phy_test_fail                @ no valid margin and retry"*/

                r0 = 0x1e6e02cc as u32; /*"    ldr   r0, =0x1e6e02cc"*/
                r1 = r10 | (r10 << 8 as u32); /*"    orr   r1, r10, r10, lsl #8"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0060 as u32; /*"    ldr   r0, =0x1e6e0060"*/
                r1 = 0x00000005 as u32; /*"    ldr   r1, =0x00000005"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                s = State::ddr_phy_init_process;
                continue; /*"    b     ddr_phy_init_process"*/

                State::ddr4_vref_phy_phyinit_done
            }
            State::ddr4_vref_phy_phyinit_done => {
                s = State::cbr_test_start;
                continue; /*"    b     cbr_test_start"*/

                State::ddr4_vref_phy_cbrtest_done
            }
            State::ddr4_vref_phy_cbrtest_done => {
                r0 = 0x1e6e03d0 as u32; /*"    ldr   r0, =0x1e6e03d0                        @ read eye pass window"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r0 = 0x1e720000 as u32; /*"    ldr   r0, =0x1e720000"*/
                r0 = r0 + r10; /*"    add   r0, r0, r10, lsl #2"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                z = r9 == 0x01 as u32;

                if !z {
                    s = State::ddr4_vref_phy_test_fail;
                    continue;
                } /*"    bne   ddr4_vref_phy_test_fail"*/
                r8 = r8 + 0x01 as u32; /*"    add   r8, r8, #0x01"*/
                r0 = 0x1e6e03d0 as u32; /*"    ldr   r0, =0x1e6e03d0                        @ read eye pass window"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r2 = r1 >> 8 as u32; /*"    mov   r2, r1, lsr #8                         @ r2 = DQH"*/
                r1 = r1 | 0xFF as u32; /*"    and   r1, r1, #0xFF                          @ r1 = DQL"*/

                gt = r1 > r2;

                if gt {
                    r1 = r2;
                } /*"    movgt r1, r2                                 @ r1 = smaller one"*/
                z = r1 == r7;
                gt = r1 > r7;

                if gt {
                    r6 = r10;
                } /*"    movgt r6, r10"*/
                if gt {
                    r7 = r1;
                } /*"    movgt r7, r1"*/
                s = State::ddr4_vref_phy_loop;
                continue; /*"    b     ddr4_vref_phy_loop"*/

                State::ddr4_vref_phy_test_fail
            }
            State::ddr4_vref_phy_test_fail => {
                z = r8 == 0x0 as u32;

                if !z {
                    s = State::ddr4_vref_phy_loop_end;
                    continue;
                } /*"    bne   ddr4_vref_phy_loop_end"*/
                z = r10 == 0x80 as u32;

                if z {
                    s = State::ddr4_vref_phy_cal_start;
                    continue;
                } /*"    beq   ddr4_vref_phy_cal_start"*/
                s = State::ddr4_vref_phy_loop;
                continue; /*"    b     ddr4_vref_phy_loop"*/

                State::ddr4_vref_phy_loop_end
            }
            State::ddr4_vref_phy_loop_end => {
                z = r8 == 16 as u32;

                lt = r8 < 16 as u32; /*"    cmp   r8, #16                                @ check phyvref margin >= 16"*/
                if lt {
                    s = State::ddr_test_fail;
                    continue;
                } /*"    blt   ddr_test_fail"*/
                r0 = 0x1e6e02cc as u32; /*"    ldr   r0, =0x1e6e02cc"*/
                r1 = r6 | (r6 << 8 as u32); /*"    orr   r1, r6, r6, lsl #8"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e720010 as u32; /*"    ldr   r0, =0x1e720010"*/
                r1 = r6 | (r7 << 8 as u32); /*"    orr   r1, r6, r7, lsl #8"*/
                r1 = r1 | (r8 << 16 as u32); /*"    orr   r1, r1, r8, lsl #16"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /********************************************
                     DDR Vref Scan
                //Can't find instruction for      r6 : min/*"     r6 : min"*/
                //Can't find instruction for      r7 : max/*"     r7 : max"*/
                //Can't find instruction for      r8 : passcnt/*"     r8 : passcnt"*/
                //Can't find instruction for      r9 : CBRtest result/*"     r9 : CBRtest result"*/
                //Can't find instruction for      r10: loopcnt/*"     r10: loopcnt"*/
                //Can't find instruction for      r11: free/*"     r11: free"*/
                    ********************************************/
                r0 = 0x1e720000 as u32; /*"    ldr   r0, =0x1e720000                        @ retry count"*/
                r1 = 0x5 as u32; /*"    mov   r1, #0x5"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                State::ddr4_vref_ddr_cal_start
            }
            State::ddr4_vref_ddr_cal_start => {
                r6 = 0xFF as u32; /*"    mov   r6, #0xFF"*/
                r7 = 0x0 as u32; /*"    mov   r7, #0x0"*/
                r8 = 0x0 as u32; /*"    mov   r8, #0x0"*/
                r10 = 0x0 as u32; /*"    mov   r10, #0x0"*/

                r0 = 0x1e720000 as u32; /*"    ldr   r0, =0x1e720000"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 = r1 - 0x01 as u32; /*"    subs  r1, r1, #0x01"*/
                if z {
                    s = State::ddr_test_fail;
                    continue;
                } /*"    beq   ddr_test_fail"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0120 as u32; /*"    ldr   r0, =0x1e6e0120"*/
                r1 = 0x00000002 as u32; /*"    ldr   r1, =0x00000002"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Debug - UART console message */
                r0 = 0x1e784000 as u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x62 as u32; /*"    mov   r1, #0x62                              @ 'b'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Debug - UART console message */

                State::ddr4_vref_ddr_loop
            }
            State::ddr4_vref_ddr_loop => {
                r0 = 0x1e6e0060 as u32; /*"    ldr   r0, =0x1e6e0060"*/
                r1 = 0x00000000 as u32; /*"    ldr   r1, =0x00000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r10 = r10 + 0x01 as u32; /*"    add   r10, r10, #0x01"*/
                z = r10 == 0x40 as u32;

                if z {
                    s = State::ddr4_vref_ddr_test_fail;
                    continue;
                } /*"    beq   ddr4_vref_ddr_test_fail                @ no valid margin and retry"*/

                r0 = 0x1e6e02c0 as u32; /*"    ldr   r0, =0x1e6e02c0"*/
                r1 = 0x06 as u32; /*"    mov   r1, #0x06"*/
                r1 = r1 | (r10 << 8 as u32); /*"    orr   r1, r1, r10, lsl #8"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0060 as u32; /*"    ldr   r0, =0x1e6e0060"*/
                r1 = 0x00000005 as u32; /*"    ldr   r1, =0x00000005"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                s = State::ddr_phy_init_process;
                continue; /*"    b     ddr_phy_init_process"*/

                State::ddr4_vref_ddr_phyinit_done
            }
            State::ddr4_vref_ddr_phyinit_done => {
                s = State::cbr_test_start;
                continue; /*"    b     cbr_test_start"*/

                State::ddr4_vref_ddr_cbrtest_done
            }
            State::ddr4_vref_ddr_cbrtest_done => {
                z = r9 == 0x01 as u32;

                if !z {
                    s = State::ddr4_vref_ddr_test_fail;
                    continue;
                } /*"    bne   ddr4_vref_ddr_test_fail"*/
                r8 = r8 + 0x01 as u32; /*"    add   r8, r8, #0x01"*/

                gt = r6 > r10;

                if gt {
                    r6 = r10;
                } /*"    movgt r6, r10"*/
                z = r7 == r10;

                lt = r7 < r10; /*"    cmp   r7, r10"*/
                if lt {
                    r7 = r10;
                } /*"    movlt r7, r10"*/
                s = State::ddr4_vref_ddr_loop;
                continue; /*"    b     ddr4_vref_ddr_loop"*/

                State::ddr4_vref_ddr_test_fail
            }
            State::ddr4_vref_ddr_test_fail => {
                z = r8 == 0x0 as u32;

                if !z {
                    s = State::ddr4_vref_ddr_loop_end;
                    continue;
                } /*"    bne   ddr4_vref_ddr_loop_end"*/
                z = r10 == 0x40 as u32;

                if z {
                    s = State::ddr4_vref_ddr_cal_start;
                    continue;
                } /*"    beq   ddr4_vref_ddr_cal_start"*/
                s = State::ddr4_vref_ddr_loop;
                continue; /*"    b     ddr4_vref_ddr_loop"*/

                State::ddr4_vref_ddr_loop_end
            }
            State::ddr4_vref_ddr_loop_end => {
                r0 = 0x1e6e0060 as u32; /*"    ldr   r0, =0x1e6e0060"*/
                r1 = 0x00000000 as u32; /*"    ldr   r1, =0x00000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                z = r8 == 16 as u32;

                lt = r8 < 16 as u32; /*"    cmp   r8, #16                                @ check ddrvref margin >= 16"*/
                if lt {
                    s = State::ddr_test_fail;
                    continue;
                } /*"    blt   ddr_test_fail"*/
                r0 = 0x1e6e02c0 as u32; /*"    ldr   r0, =0x1e6e02c0"*/
                r1 = r6 + r7; /*"    add   r1, r6, r7"*/
                r1 = r1 + 0x01 as u32; /*"    add   r1, r1, #0x01"*/
                r2 = r1 >> 1 as u32; /*"    mov   r2, r1, lsr #1"*/
                r1 = r2 << 8 as u32; /*"    mov   r1, r2, lsl #8"*/
                r1 = r1 | 0x06 as u32; /*"    orr   r1, r1, #0x06"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e720014 as u32; /*"    ldr   r0, =0x1e720014"*/
                r1 = r6 | (r7 << 8 as u32); /*"    orr   r1, r6, r7, lsl #8"*/
                r1 = r1 | (r8 << 16 as u32); /*"    orr   r1, r1, r8, lsl #16"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Debug - UART console message */
                r0 = 0x1e784000 as u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x63 as u32; /*"    mov   r1, #0x63                              @ 'c'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Debug - UART console message */

                r0 = 0x1e6e0120 as u32; /*"    ldr   r0, =0x1e6e0120"*/
                r1 = 0x00000003 as u32; /*"    ldr   r1, =0x00000003"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0060 as u32; /*"    ldr   r0, =0x1e6e0060                        @ Fire DDRPHY Init"*/
                r1 = 0x00000005 as u32; /*"    ldr   r1, =0x00000005"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                s = State::ddr_phy_init_process;
                continue; /*"    b     ddr_phy_init_process"*/

                State::ddr4_phyinit_done
            }
            State::ddr4_phyinit_done => {
                /********************************************
                 Check Read training margin
                ********************************************/
                r0 = 0x1e6e03a0 as u32; /*"    ldr   r0, =0x1e6e03a0                        @ check Gate Training Pass Window"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r2 = 0x150 as u32; /*"    ldr   r2, =0x150"*/
                r0 = r1 & !0xFF000000 as u32; /*"    bic   r0, r1, #0xFF000000"*/
                r0 = r0 & !0x00FF0000 as u32; /*"    bic   r0, r0, #0x00FF0000"*/
                z = r0 == r2;

                lt = r0 < r2; /*"    cmp   r0, r2"*/
                if lt {
                    s = State::ddr_test_fail;
                    continue;
                } /*"    blt   ddr_test_fail"*/
                r0 = r1 >> 16 as u32; /*"    mov   r0, r1, lsr #16"*/
                z = r0 == r2;

                lt = r0 < r2; /*"    cmp   r0, r2"*/
                if lt {
                    s = State::ddr_test_fail;
                    continue;
                } /*"    blt   ddr_test_fail"*/

                r0 = 0x1e6e03d0 as u32; /*"    ldr   r0, =0x1e6e03d0                        @ check Read Data Eye Training Pass Window"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r2 = 0x90 as u32; /*"    ldr   r2, =0x90"*/
                r0 = r1 & !0x0000FF00 as u32; /*"    bic   r0, r1, #0x0000FF00"*/
                z = r0 == r2;

                lt = r0 < r2; /*"    cmp   r0, r2"*/
                if lt {
                    s = State::ddr_test_fail;
                    continue;
                } /*"    blt   ddr_test_fail"*/
                r0 = r1 >> 8 as u32; /*"    mov   r0, r1, lsr #8"*/
                z = r0 == r2;

                lt = r0 < r2; /*"    cmp   r0, r2"*/
                if lt {
                    s = State::ddr_test_fail;
                    continue;
                } /*"    blt   ddr_test_fail"*/
                /*******************************************/

                /*******************************************/
                /* Debug - UART console message */
                r0 = 0x1e784000 as u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x31 as u32; /*"    mov   r1, #0x31                              @ '1'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Debug - UART console message */

                r0 = 0x1e6e000c as u32; /*"    ldr   r0, =0x1e6e000c"*/
                if CONFIG_DRAM_EXT_TEMP == 1 {
                    // #ifdef CONFIG_DRAM_EXT_TEMP
                    r1 = 0x42AA2F81 as u32; /*"    ldr   r1, =0x42AA2F81"*/
                } else {
                    // #else 	r1 = 0x42AA5C81 as u32;/*"    ldr   r1, =0x42AA5C81"*/
                } // #endif
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0034 as u32; /*"    ldr   r0, =0x1e6e0034"*/
                r1 = 0x0001AF93 as u32; /*"    ldr   r1, =0x0001AF93"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0120 as u32; /*"    ldr   r0, =0x1e6e0120                        @ VGA Compatible Mode"*/
                r1 = tptr[(ASTMMC_REGIDX_PLL as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_PLL]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                s = State::calibration_End;
                continue; /*"    b     calibration_End"*/

                /******************************************************************************
                End DDR4 Init
                ******************************************************************************/
                /******************************************************************************
                Global Process
                ******************************************************************************/
                /********************************************
                 DDRPHY Init Process
                ********************************************/
                State::ddr_phy_init_process
            }
            State::ddr_phy_init_process => {
                clear_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    clear_delay_timer"*/
                /* Wait DDR PHY init done - timeout 300 ms */
                r2 = 0x000493E0 as u32; /*"    ldr   r2, =0x000493E0                        @ Set Timer3 Reload = 300 ms"*/
                init_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    init_delay_timer"*/
                r3 = 0x1e6e0060 as u32; /*"    ldr   r3, =0x1e6e0060"*/
                State::ddr_phy_init
            }
            State::ddr_phy_init => {
                check_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    check_delay_timer"*/
                if z {
                    s = State::ddr_phy_init_timeout;
                    continue;
                } /*"    beq   ddr_phy_init_timeout"*/
                r1 = peek(r3); /*"    ldr   r1, [r3]"*/
                z = r1 == 0x01 as u32; /*"    tst   r1, #0x01"*/
                if !z {
                    s = State::ddr_phy_init;
                    continue;
                } /*"    bne   ddr_phy_init"*/

                /* Check DDR PHY init status */
                r0 = 0x1e6e0300 as u32; /*"    ldr   r0, =0x1e6e0300"*/
                r2 = 0x000A0000 as u32; /*"    ldr   r2, =0x000A0000"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == r2; /*"    tst   r1, r2"*/
                if z {
                    s = State::ddr_phy_init_success;
                    continue;
                } /*"    beq   ddr_phy_init_success"*/

                State::ddr_phy_init_timeout
            }
            State::ddr_phy_init_timeout => {
                r0 = 0x1e6e0060 as u32; /*"    ldr   r0, =0x1e6e0060                        @ Reset PHY"*/
                r1 = 0x00 as u32; /*"    mov   r1, #0x00"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Debug - UART console message */
                r0 = 0x1e784000 as u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x2E as u32; /*"    mov   r1, #0x2E                              @ '.'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Debug - UART console message */

                clear_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    clear_delay_timer"*/
                /* Delay about 10us */
                r2 = 0x0000000A as u32; /*"    ldr   r2, =0x0000000A                        @ Set Timer3 Reload = 10 us"*/
                init_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    init_delay_timer"*/
                State::ddr_phy_init_delay_0
            }
            State::ddr_phy_init_delay_0 => {
                check_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    check_delay_timer"*/
                if !z {
                    s = State::ddr_phy_init_delay_0;
                    continue;
                } /*"    bne   ddr_phy_init_delay_0"*/
                clear_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    clear_delay_timer"*/
                /* end delay 10us */

                r0 = 0x1e6e0060 as u32; /*"    ldr   r0, =0x1e6e0060                        @ Fire PHY Init"*/
                r1 = 0x05 as u32; /*"    mov   r1, #0x05"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                s = State::ddr_phy_init_process;
                continue; /*"    b     ddr_phy_init_process"*/

                State::ddr_phy_init_success
            }
            State::ddr_phy_init_success => {
                clear_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    clear_delay_timer"*/
                r0 = 0x1e6e0060 as u32; /*"    ldr   r0, =0x1e6e0060"*/
                r1 = 0x06 as u32; /*"    mov   r1, #0x06"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0120 as u32; /*"    ldr   r0, =0x1e6e0120"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == 0 as u32;

                if z {
                    s = State::ddr3_phyinit_done;
                    continue;
                } /*"    beq   ddr3_phyinit_done"*/
                z = r1 == 1 as u32;

                if z {
                    s = State::ddr4_vref_phy_phyinit_done;
                    continue;
                } /*"    beq   ddr4_vref_phy_phyinit_done"*/
                z = r1 == 2 as u32;

                if z {
                    s = State::ddr4_vref_ddr_phyinit_done;
                    continue;
                } /*"    beq   ddr4_vref_ddr_phyinit_done"*/
                if ASTMMC_DDR4_MANUAL_RPU == 1 {
                    // #ifdef ASTMMC_DDR4_MANUAL_RPU
                    z = r1 == 4 as u32;

                    if z {
                        s = State::ddr4_ron_phyinit_done;
                        continue;
                    } /*"    beq   ddr4_ron_phyinit_done"*/
                } // #endif
                s = State::ddr4_phyinit_done;
                continue; /*"    b     ddr4_phyinit_done"*/

                /********************************************
                 CBRTest
                ********************************************/
                State::cbr_test_start
            }
            State::cbr_test_start => {
                r0 = 0x1e6e000c as u32; /*"    ldr   r0, =0x1e6e000c"*/
                r1 = 0x00005C01 as u32; /*"    ldr   r1, =0x00005C01"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e0074 as u32; /*"    ldr   r0, =0x1e6e0074"*/
                r1 = 0x0000FFFF as u32; /*"    ldr   r1, =0x0000FFFF                        @ test size = 64KB"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e007c as u32; /*"    ldr   r0, =0x1e6e007c"*/
                r1 = 0xFF00FF00 as u32; /*"    ldr   r1, =0xFF00FF00"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                State::cbr_test_single
            }
            State::cbr_test_single => {
                r0 = 0x1e6e0070 as u32; /*"    ldr   r0, =0x1e6e0070"*/
                r1 = 0x00000000 as u32; /*"    ldr   r1, =0x00000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x00000085 as u32; /*"    ldr   r1, =0x00000085"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r3 = 0x3000 as u32; /*"    ldr   r3, =0x3000"*/
                r11 = 0x50000 as u32; /*"    ldr   r11, =0x50000"*/
                State::cbr_wait_engine_idle_0
            }
            State::cbr_wait_engine_idle_0 => {
                r11 = r11 - 1 as u32; /*"    subs  r11, r11, #1"*/
                if z {
                    s = State::cbr_test_fail;
                    continue;
                } /*"    beq   cbr_test_fail"*/
                r2 = peek(r0); /*"    ldr   r2, [r0]"*/
                z = r2 == r3; /*"    tst   r2, r3                                 @ D[12] = idle bit"*/
                if z {
                    s = State::cbr_wait_engine_idle_0;
                    continue;
                } /*"    beq   cbr_wait_engine_idle_0"*/

                r0 = 0x1e6e0070 as u32; /*"    ldr   r0, =0x1e6e0070                        @ read fail bit status"*/
                r3 = 0x2000 as u32; /*"    ldr   r3, =0x2000"*/
                r2 = peek(r0); /*"    ldr   r2, [r0]"*/
                z = r2 == r3; /*"    tst   r2, r3                                 @ D[13] = fail bit"*/
                if !z {
                    s = State::cbr_test_fail;
                    continue;
                } /*"    bne   cbr_test_fail"*/

                State::cbr_test_burst
            }
            State::cbr_test_burst => {
                r1 = 0x00 as u32; /*"    mov   r1, #0x00                              @ initialize loop index, r1 is loop index"*/
                State::cbr_test_burst_loop
            }
            State::cbr_test_burst_loop => {
                r0 = 0x1e6e0070 as u32; /*"    ldr   r0, =0x1e6e0070"*/
                r2 = 0x00000000 as u32; /*"    ldr   r2, =0x00000000"*/
                poke(r2, r0); /*"    str   r2, [r0]"*/
                r2 = r1 << 3 as u32; /*"    mov   r2, r1, lsl #3"*/
                r2 = r2 | 0xC1 as u32; /*"    orr   r2, r2, #0xC1                          @ test command = 0xC1 | (datagen << 3)"*/
                poke(r2, r0); /*"    str   r2, [r0]"*/
                r3 = 0x3000 as u32; /*"    ldr   r3, =0x3000"*/
                r11 = 0x20000 as u32; /*"    ldr   r11, =0x20000"*/
                State::cbr_wait_engine_idle_1
            }
            State::cbr_wait_engine_idle_1 => {
                r11 = r11 - 1 as u32; /*"    subs  r11, r11, #1"*/
                if z {
                    s = State::cbr_test_fail;
                    continue;
                } /*"    beq   cbr_test_fail"*/
                r2 = peek(r0); /*"    ldr   r2, [r0]"*/
                z = r2 == r3; /*"    tst   r2, r3                                 @ D[12] = idle bit"*/
                if z {
                    s = State::cbr_wait_engine_idle_1;
                    continue;
                } /*"    beq   cbr_wait_engine_idle_1"*/

                r0 = 0x1e6e0070 as u32; /*"    ldr   r0, =0x1e6e0070                        @ read fail bit status"*/
                r3 = 0x2000 as u32; /*"    ldr   r3, =0x2000"*/
                r2 = peek(r0); /*"    ldr   r2, [r0]"*/
                z = r2 == r3; /*"    tst   r2, r3                                 @ D[13] = fail bit"*/
                if !z {
                    s = State::cbr_test_fail;
                    continue;
                } /*"    bne   cbr_test_fail"*/

                r1 = r1 + 1 as u32; /*"    add   r1, r1, #1                             @ increase the test mode index"*/
                z = r1 == 0x04 as u32;

                if !z {
                    s = State::cbr_test_burst_loop;
                    continue;
                } /*"    bne   cbr_test_burst_loop"*/

                r0 = 0x1e6e0070 as u32; /*"    ldr   r0, =0x1e6e0070"*/
                r1 = 0x00000000 as u32; /*"    ldr   r1, =0x00000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r9 = 0x1 as u32; /*"    mov   r9, #0x1"*/
                s = State::cbr_test_pattern_end;
                continue; /*"    b     cbr_test_pattern_end                   @ CBRTest() return(1)"*/

                State::cbr_test_fail
            }
            State::cbr_test_fail => {
                r0 = 0x1e6e0070 as u32; /*"    ldr   r0, =0x1e6e0070"*/
                r1 = 0x00000000 as u32; /*"    ldr   r1, =0x00000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r9 = 0x0 as u32; /*"    mov   r9, #0x0                               @ CBRTest() return(0)"*/

                State::cbr_test_pattern_end
            }
            State::cbr_test_pattern_end => {
                r0 = 0x1e6e000c as u32; /*"    ldr   r0, =0x1e6e000c"*/
                r1 = 0x00000000 as u32; /*"    ldr   r1, =0x00000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e0120 as u32; /*"    ldr   r0, =0x1e6e0120"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == 1 as u32;

                if z {
                    s = State::ddr4_vref_phy_cbrtest_done;
                    continue;
                } /*"    beq   ddr4_vref_phy_cbrtest_done"*/
                s = State::ddr4_vref_ddr_cbrtest_done;
                continue; /*"    b     ddr4_vref_ddr_cbrtest_done"*/

                /******************************************************************************
                Other features configuration
                *****************************************************************************/

                State::calibration_End
            }
            State::calibration_End => {
                /*******************************
                     Check DRAM Size
                //Can't find instruction for      1Gb : 0x80000000 ~ 0x87FFFFFF/*"     1Gb : 0x80000000 ~ 0x87FFFFFF"*/
                //Can't find instruction for      2Gb : 0x80000000 ~ 0x8FFFFFFF/*"     2Gb : 0x80000000 ~ 0x8FFFFFFF"*/
                //Can't find instruction for      4Gb : 0x80000000 ~ 0x9FFFFFFF/*"     4Gb : 0x80000000 ~ 0x9FFFFFFF"*/
                //Can't find instruction for      8Gb : 0x80000000 ~ 0xBFFFFFFF/*"     8Gb : 0x80000000 ~ 0xBFFFFFFF"*/
                    *******************************/
                r0 = 0x1e6e0004 as u32; /*"    ldr   r0, =0x1e6e0004"*/
                r6 = peek(r0); /*"    ldr   r6, [r0]"*/
                r6 = r6 & !0x00000003 as u32; /*"    bic   r6, r6, #0x00000003                    @ record MCR04"*/
                r7 = tptr[(ASTMMC_REGIDX_RFC as u32) as usize]; /*"    ldr   r7, [r5, #ASTMMC_REGIDX_RFC]"*/

                State::check_dram_size
            }
            State::check_dram_size => {
                r0 = 0xA0100000 as u32; /*"    ldr   r0, =0xA0100000"*/
                r1 = 0x41424344 as u32; /*"    ldr   r1, =0x41424344"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x90100000 as u32; /*"    ldr   r0, =0x90100000"*/
                r1 = 0x35363738 as u32; /*"    ldr   r1, =0x35363738"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x88100000 as u32; /*"    ldr   r0, =0x88100000"*/
                r1 = 0x292A2B2C as u32; /*"    ldr   r1, =0x292A2B2C"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x80100000 as u32; /*"    ldr   r0, =0x80100000"*/
                r1 = 0x1D1E1F10 as u32; /*"    ldr   r1, =0x1D1E1F10"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0xA0100000 as u32; /*"    ldr   r0, =0xA0100000"*/
                r1 = 0x41424344 as u32; /*"    ldr   r1, =0x41424344"*/
                r2 = peek(r0); /*"    ldr   r2, [r0]"*/
                z = r2 == r1;

                if z {
                    r6 = r6 | 0x03 as u32;
                } /*"    orreq r6, r6, #0x03"*/
                if z {
                    r7 = r7 >> 24 as u32;
                } /*"    moveq r7, r7, lsr #24"*/
                r3 = 0x38 as u32; /*"    mov   r3, #0x38                              @ '8'"*/
                if z {
                    s = State::check_dram_size_end;
                    continue;
                } /*"    beq   check_dram_size_end"*/
                r0 = 0x90100000 as u32; /*"    ldr   r0, =0x90100000"*/
                r1 = 0x35363738 as u32; /*"    ldr   r1, =0x35363738"*/
                r2 = peek(r0); /*"    ldr   r2, [r0]"*/
                z = r2 == r1;

                if z {
                    r6 = r6 | 0x02 as u32;
                } /*"    orreq r6, r6, #0x02"*/
                if z {
                    r7 = r7 >> 16 as u32;
                } /*"    moveq r7, r7, lsr #16"*/
                r3 = 0x34 as u32; /*"    mov   r3, #0x34                              @ '4'"*/
                if z {
                    s = State::check_dram_size_end;
                    continue;
                } /*"    beq   check_dram_size_end"*/
                r0 = 0x88100000 as u32; /*"    ldr   r0, =0x88100000"*/
                r1 = 0x292A2B2C as u32; /*"    ldr   r1, =0x292A2B2C"*/
                r2 = peek(r0); /*"    ldr   r2, [r0]"*/
                z = r2 == r1;

                if z {
                    r6 = r6 | 0x01 as u32;
                } /*"    orreq r6, r6, #0x01"*/
                if z {
                    r7 = r7 >> 8 as u32;
                } /*"    moveq r7, r7, lsr #8"*/
                r3 = 0x32 as u32; /*"    mov   r3, #0x32                              @ '2'"*/
                if z {
                    s = State::check_dram_size_end;
                    continue;
                } /*"    beq   check_dram_size_end"*/
                r3 = 0x31 as u32; /*"    mov   r3, #0x31                              @ '1'"*/

                State::check_dram_size_end
            }
            State::check_dram_size_end => {
                r0 = 0x1e6e0004 as u32; /*"    ldr   r0, =0x1e6e0004"*/
                poke(r6, r0); /*"    str   r6, [r0]"*/
                r0 = 0x1e6e0014 as u32; /*"    ldr   r0, =0x1e6e0014"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 = r1 & !0x000000FF as u32; /*"    bic   r1, r1, #0x000000FF"*/
                r7 = r7 | 0xFF as u32; /*"    and   r7, r7, #0xFF"*/
                r1 = r1 | r7; /*"    orr   r1, r1, r7"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Version Number */
                r0 = 0x1e6e0004 as u32; /*"    ldr   r0, =0x1e6e0004"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r2 = ASTMMC_INIT_VER as u32; /*"    mov   r2, #ASTMMC_INIT_VER"*/
                r1 = r1 | (r2 << 20 as u32); /*"    orr   r1, r1, r2, lsl #20"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0088 as u32; /*"    ldr   r0, =0x1e6e0088"*/
                r1 = ASTMMC_INIT_DATE as u32; /*"    ldr   r1, =ASTMMC_INIT_DATE"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Debug - UART console message */
                r0 = 0x1e784000 as u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x2D as u32; /*"    mov   r1, #0x2D                              @ '-'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                poke(r3, r0); /*"    str   r3, [r0]"*/
                r1 = 0x47 as u32; /*"    mov   r1, #0x47                              @ 'G'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x62 as u32; /*"    mov   r1, #0x62                              @ 'b'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x2D as u32; /*"    mov   r1, #0x2D                              @ '-'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Debug - UART console message */

                /* Enable DRAM Cache */
                r0 = 0x1e6e0004 as u32; /*"    ldr   r0, =0x1e6e0004"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r2 = 1 as u32; /*"    mov   r2, #1"*/
                r2 = r1 | (r2 << 12 as u32); /*"    orr   r2, r1, r2, lsl #12"*/
                poke(r2, r0); /*"    str   r2, [r0]"*/
                r3 = 0x00080000 as u32; /*"    ldr   r3, =0x00080000"*/
                State::dram_cache_init
            }
            State::dram_cache_init => {
                r2 = peek(r0); /*"    ldr   r2, [r0]"*/
                z = r2 == r3; /*"    tst   r2, r3"*/
                if z {
                    s = State::dram_cache_init;
                    continue;
                } /*"    beq   dram_cache_init"*/
                r2 = 1 as u32; /*"    mov   r2, #1"*/
                r1 = r1 | (r2 << 10 as u32); /*"    orr   r1, r1, r2, lsl #10"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Set DRAM requests threshold */
                r0 = 0x1e6e001c as u32; /*"    ldr   r0, =0x1e6e001c"*/
                r1 = 0x00000008 as u32; /*"    ldr   r1, =0x00000008"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e0038 as u32; /*"    ldr   r0, =0x1e6e0038"*/
                r1 = 0xFFFFFF00 as u32; /*"    ldr   r1, =0xFFFFFF00"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /********************************************
                 DDRTest
                ********************************************/
                State::ddr_test_start
            }
            State::ddr_test_start => {
                r0 = 0x1e6e0074 as u32; /*"    ldr   r0, =0x1e6e0074"*/
                r1 = 0x0000FFFF as u32; /*"    ldr   r1, =0x0000FFFF                        @ test size = 64KB"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e007c as u32; /*"    ldr   r0, =0x1e6e007c"*/
                r1 = 0xFF00FF00 as u32; /*"    ldr   r1, =0xFF00FF00"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                State::ddr_test_burst
            }
            State::ddr_test_burst => {
                r1 = 0x00 as u32; /*"    mov   r1, #0x00                              @ initialize loop index, r1 is loop index"*/
                State::ddr_test_burst_loop
            }
            State::ddr_test_burst_loop => {
                r0 = 0x1e6e0070 as u32; /*"    ldr   r0, =0x1e6e0070"*/
                r2 = 0x00000000 as u32; /*"    ldr   r2, =0x00000000"*/
                poke(r2, r0); /*"    str   r2, [r0]"*/
                r2 = r1 << 3 as u32; /*"    mov   r2, r1, lsl #3"*/
                r2 = r2 | 0xC1 as u32; /*"    orr   r2, r2, #0xC1                          @ test command = 0xC1 | (datagen << 3)"*/
                poke(r2, r0); /*"    str   r2, [r0]"*/
                r3 = 0x3000 as u32; /*"    ldr   r3, =0x3000"*/
                r11 = 0x20000 as u32; /*"    ldr   r11, =0x20000"*/
                State::ddr_wait_engine_idle_1
            }
            State::ddr_wait_engine_idle_1 => {
                r11 = r11 - 1 as u32; /*"    subs  r11, r11, #1"*/
                if z {
                    s = State::ddr_test_fail;
                    continue;
                } /*"    beq   ddr_test_fail"*/
                r2 = peek(r0); /*"    ldr   r2, [r0]"*/
                z = r2 == r3; /*"    tst   r2, r3                                 @ D[12] = idle bit"*/
                if z {
                    s = State::ddr_wait_engine_idle_1;
                    continue;
                } /*"    beq   ddr_wait_engine_idle_1"*/

                r0 = 0x1e6e0070 as u32; /*"    ldr   r0, =0x1e6e0070                        @ read fail bit status"*/
                r3 = 0x2000 as u32; /*"    ldr   r3, =0x2000"*/
                r2 = peek(r0); /*"    ldr   r2, [r0]"*/
                z = r2 == r3; /*"    tst   r2, r3                                 @ D[13] = fail bit"*/
                if !z {
                    s = State::ddr_test_fail;
                    continue;
                } /*"    bne   ddr_test_fail"*/

                r1 = r1 + 1 as u32; /*"    add   r1, r1, #1                             @ increase the test mode index"*/
                z = r1 == 0x01 as u32;

                if !z {
                    s = State::ddr_test_burst_loop;
                    continue;
                } /*"    bne   ddr_test_burst_loop"*/

                r0 = 0x1e6e0070 as u32; /*"    ldr   r0, =0x1e6e0070"*/
                r1 = 0x00000000 as u32; /*"    ldr   r1, =0x00000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                s = State::set_scratch;
                continue; /*"    b     set_scratch                            @ CBRTest() return(1)"*/

                State::ddr_test_fail
            }
            State::ddr_test_fail => {
                /* Debug - UART console message */
                r0 = 0x1e784000 as u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x46 as u32; /*"    mov   r1, #0x46                              @ 'F'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x61 as u32; /*"    mov   r1, #0x61                              @ 'a'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x69 as u32; /*"    mov   r1, #0x69                              @ 'i'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6C as u32; /*"    mov   r1, #0x6C                              @ 'l'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x0D as u32; /*"    mov   r1, #0x0D                              @ '\\r'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x0A as u32; /*"    mov   r1, #0x0A                              @ '\\n'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e784014 as u32; /*"    ldr   r0, =0x1e784014"*/
                State::wait_print_0
            }
            State::wait_print_0 => {
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == 0x40 as u32; /*"    tst   r1, #0x40"*/
                if z {
                    s = State::wait_print_0;
                    continue;
                } /*"    beq   wait_print_0"*/
                /* Debug - UART console message */
                s = State::reset_mmc;
                continue; /*"    b     reset_mmc"*/

                State::set_scratch
            }
            State::set_scratch => {
                /*Set Scratch register Bit 6 after ddr initial finished */
                r0 = 0x1e6e2040 as u32; /*"    ldr   r0, =0x1e6e2040"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 = r1 | 0x41 as u32; /*"    orr   r1, r1, #0x41"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Debug - UART console message */
                r0 = 0x1e784000 as u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x44 as u32; /*"    mov   r1, #0x44                              @ 'D'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6F as u32; /*"    mov   r1, #0x6F                              @ 'o'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6E as u32; /*"    mov   r1, #0x6E                              @ 'n'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x65 as u32; /*"    mov   r1, #0x65                              @ 'e'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x0D as u32; /*"    mov   r1, #0x0D                              @ '\\r'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x0A as u32; /*"    mov   r1, #0x0A                              @ '\\n'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Debug - UART console message */

                /* Enable VGA display */
                r0 = 0x1e6e202c as u32; /*"    ldr   r0, =0x1e6e202c"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 = r1 & !0x40 as u32; /*"    bic   r1, r1, #0x40"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Debug - UART console message */
                /* Print PHY timing information */
                r0 = 0x1e784014 as u32; /*"    ldr   r0, =0x1e784014"*/
                State::wait_print_1
            }
            State::wait_print_1 => {
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == 0x40 as u32; /*"    tst   r1, #0x40"*/
                if z {
                    s = State::wait_print_1;
                    continue;
                } /*"    beq   wait_print_1"*/

                r0 = 0x1e784000 as u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x52 as u32; /*"    mov   r1, #0x52                              @ 'R'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x65 as u32; /*"    mov   r1, #0x65                              @ 'e'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x61 as u32; /*"    mov   r1, #0x61                              @ 'a'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x64 as u32; /*"    mov   r1, #0x64                              @ 'd'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x20 as u32; /*"    mov   r1, #0x20                              @ ' '"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6D as u32; /*"    mov   r1, #0x6D                              @ 'm'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x61 as u32; /*"    mov   r1, #0x61                              @ 'a'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x72 as u32; /*"    mov   r1, #0x72                              @ 'r'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x67 as u32; /*"    mov   r1, #0x67                              @ 'g'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x69 as u32; /*"    mov   r1, #0x69                              @ 'i'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6E as u32; /*"    mov   r1, #0x6E                              @ 'n'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x2D as u32; /*"    mov   r1, #0x2D                              @ '-'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x44 as u32; /*"    mov   r1, #0x44                              @ 'D'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x4C as u32; /*"    mov   r1, #0x4C                              @ 'L'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x3A as u32; /*"    mov   r1, #0x3A                              @ ':'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e784014 as u32; /*"    ldr   r0, =0x1e784014"*/
                State::wait_print_2
            }
            State::wait_print_2 => {
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == 0x40 as u32; /*"    tst   r1, #0x40"*/
                if z {
                    s = State::wait_print_2;
                    continue;
                } /*"    beq   wait_print_2"*/

                r7 = 0x000001FE as u32; /*"    ldr   r7, =0x000001FE                        @ divide by 510"*/
                r8 = 10 as u32; /*"    mov   r8, #10                                @ multiply by 10"*/
                r9 = 0 as u32; /*"    mov   r9, #0                                 @ record violation"*/
                r0 = 0x1e6e0004 as u32; /*"    ldr   r0, =0x1e6e0004"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == 0x10 as u32; /*"    tst   r1, #0x10                              @ bit[4]=1 => DDR4"*/
                if !z {
                    r10 = 0x9A as u32;
                } /*"    movne r10, #0x9A                             @ DDR4 min = 0x99 (0.30)"*/
                if z {
                    r10 = 0xB3 as u32;
                } /*"    moveq r10, #0xB3                             @ DDR3 min = 0xB3 (0.35)"*/
                State::print_DQL_eye_margin
            }
            State::print_DQL_eye_margin => {
                r0 = 0x1e6e03d0 as u32; /*"    ldr   r0, =0x1e6e03d0"*/
                r2 = peek(r0); /*"    ldr   r2, [r0]"*/
                r2 = r2 | 0xFF as u32; /*"    and   r2, r2, #0xFF"*/
                z = r2 == r10;

                lt = r2 < r10; /*"    cmp   r2, r10                                @ check violation"*/
                if lt {
                    r9 = 1 as u32;
                } /*"    movlt r9, #1"*/
                r0 = 0x1e784000 as u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x30 as u32; /*"    mov   r1, #0x30                              @ '0'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x2E as u32; /*"    mov   r1, #0x2E                              @ '.'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r3 = 0x4 as u32; /*"    mov   r3, #0x4                               @ print 4 digits"*/
                State::print_DQL_div_loop
            }
            State::print_DQL_div_loop => {
                r2 = r8 * r2; /*"    mul   r2, r8, r2"*/
                z = r2 == r7;

                lt = r2 < r7; /*"    cmp   r2, r7"*/
                if lt {
                    s = State::print_DQL_div_0;
                    continue;
                } /*"    blt   print_DQL_div_0"*/
                r6 = 0x0 as u32; /*"    mov   r6, #0x0"*/
                State::print_DQL_div_digit
            }
            State::print_DQL_div_digit => {
                r2 = r2 - r7; /*"    sub   r2, r2, r7"*/
                r6 = r6 + 0x1 as u32; /*"    add   r6, r6, #0x1"*/
                z = r2 == r7;
                gt = r2 > r7;

                if gt || z {
                    s = State::print_DQL_div_digit;
                    continue;
                } /*"    bge   print_DQL_div_digit"*/
                s = State::print_DQL_div_n;
                continue; /*"    b     print_DQL_div_n"*/

                State::print_DQL_div_0
            }
            State::print_DQL_div_0 => {
                r1 = 0x30 as u32; /*"    mov   r1, #0x30                              @ '0'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                s = State::print_DQL_next;
                continue; /*"    b     print_DQL_next"*/
                State::print_DQL_div_n
            }
            State::print_DQL_div_n => {
                r1 = r6 + 0x30 as u32; /*"    add   r1, r6, #0x30                          @ print n"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                State::print_DQL_next
            }
            State::print_DQL_next => {
                r3 = r3 - 1 as u32; /*"    subs  r3, r3, #1"*/
                if z {
                    s = State::print_DQH_eye_margin;
                    continue;
                } /*"    beq   print_DQH_eye_margin"*/
                z = r2 == 0x0 as u32;

                if z {
                    s = State::print_DQH_eye_margin;
                    continue;
                } /*"    beq   print_DQH_eye_margin"*/
                s = State::print_DQL_div_loop;
                continue; /*"    b     print_DQL_div_loop"*/

                State::print_DQH_eye_margin
            }
            State::print_DQH_eye_margin => {
                r1 = 0x2F as u32; /*"    mov   r1, #0x2F                              @ '/'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x44 as u32; /*"    mov   r1, #0x44                              @ 'D'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x48 as u32; /*"    mov   r1, #0x48                              @ 'H'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x3A as u32; /*"    mov   r1, #0x3A                              @ ':'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e784014 as u32; /*"    ldr   r0, =0x1e784014"*/
                State::wait_print_3
            }
            State::wait_print_3 => {
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == 0x40 as u32; /*"    tst   r1, #0x40"*/
                if z {
                    s = State::wait_print_3;
                    continue;
                } /*"    beq   wait_print_3"*/

                r0 = 0x1e6e03d0 as u32; /*"    ldr   r0, =0x1e6e03d0"*/
                r2 = peek(r0); /*"    ldr   r2, [r0]"*/
                r2 = r2 >> 8 as u32; /*"    mov   r2, r2, lsr #8"*/
                r2 = r2 | 0xFF as u32; /*"    and   r2, r2, #0xFF"*/
                z = r2 == r10;

                lt = r2 < r10; /*"    cmp   r2, r10                                @ check violation"*/
                if lt {
                    r9 = 1 as u32;
                } /*"    movlt r9, #1"*/
                r0 = 0x1e784000 as u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x30 as u32; /*"    mov   r1, #0x30                              @ '0'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x2E as u32; /*"    mov   r1, #0x2E                              @ '.'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r3 = 0x4 as u32; /*"    mov   r3, #0x4                               @ print 4 digits"*/
                State::print_DQH_div_loop
            }
            State::print_DQH_div_loop => {
                r2 = r8 * r2; /*"    mul   r2, r8, r2"*/
                z = r2 == r7;

                lt = r2 < r7; /*"    cmp   r2, r7"*/
                if lt {
                    s = State::print_DQH_div_0;
                    continue;
                } /*"    blt   print_DQH_div_0"*/
                r6 = 0x0 as u32; /*"    mov   r6, #0x0"*/
                State::print_DQH_div_digit
            }
            State::print_DQH_div_digit => {
                r2 = r2 - r7; /*"    sub   r2, r2, r7"*/
                r6 = r6 + 0x1 as u32; /*"    add   r6, r6, #0x1"*/
                z = r2 == r7;
                gt = r2 > r7;

                if gt || z {
                    s = State::print_DQH_div_digit;
                    continue;
                } /*"    bge   print_DQH_div_digit"*/
                s = State::print_DQH_div_n;
                continue; /*"    b     print_DQH_div_n"*/

                State::print_DQH_div_0
            }
            State::print_DQH_div_0 => {
                r1 = 0x30 as u32; /*"    mov   r1, #0x30                              @ '0'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                s = State::print_DQH_next;
                continue; /*"    b     print_DQH_next"*/
                State::print_DQH_div_n
            }
            State::print_DQH_div_n => {
                r1 = r6 + 0x30 as u32; /*"    add   r1, r6, #0x30                          @ print n"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                State::print_DQH_next
            }
            State::print_DQH_next => {
                r3 = r3 - 1 as u32; /*"    subs  r3, r3, #1"*/
                if z {
                    s = State::print_DQ_eye_margin_last;
                    continue;
                } /*"    beq   print_DQ_eye_margin_last"*/
                z = r2 == 0x0 as u32;

                if z {
                    s = State::print_DQ_eye_margin_last;
                    continue;
                } /*"    beq   print_DQ_eye_margin_last"*/
                s = State::print_DQH_div_loop;
                continue; /*"    b     print_DQH_div_loop"*/

                State::print_DQ_eye_margin_last
            }
            State::print_DQ_eye_margin_last => {
                r1 = 0x20 as u32; /*"    mov   r1, #0x20                              @ ' '"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x43 as u32; /*"    mov   r1, #0x43                              @ 'C'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x4B as u32; /*"    mov   r1, #0x4B                              @ 'K'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0004 as u32; /*"    ldr   r0, =0x1e6e0004"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == 0x10 as u32; /*"    tst   r1, #0x10                              @ bit[4]=1 => DDR4"*/
                if !z {
                    r10 = 0x30 as u32;
                } /*"    movne r10, #0x30                             @ DDR4 min = 0.30"*/
                if z {
                    r10 = 0x35 as u32;
                } /*"    moveq r10, #0x35                             @ DDR4 min = 0.35"*/

                r0 = 0x1e784014 as u32; /*"    ldr   r0, =0x1e784014"*/
                State::wait_print_4
            }
            State::wait_print_4 => {
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == 0x40 as u32; /*"    tst   r1, #0x40"*/
                if z {
                    s = State::wait_print_4;
                    continue;
                } /*"    beq   wait_print_4"*/

                r0 = 0x1e784000 as u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x20 as u32; /*"    mov   r1, #0x20                              @ ' '"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x28 as u32; /*"    mov   r1, #0x28                              @ '('"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6D as u32; /*"    mov   r1, #0x6D                              @ 'm'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x69 as u32; /*"    mov   r1, #0x69                              @ 'i'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6E as u32; /*"    mov   r1, #0x6E                              @ 'n'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x3A as u32; /*"    mov   r1, #0x3A                              @ ':'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x30 as u32; /*"    mov   r1, #0x30                              @ '0'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x2E as u32; /*"    mov   r1, #0x2E                              @ '.'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x33 as u32; /*"    mov   r1, #0x33                              @ '3'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                poke(r10, r0); /*"    str   r10, [r0]"*/
                r1 = 0x29 as u32; /*"    mov   r1, #0x29                              @ ')'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                z = r9 == 0 as u32;

                if z {
                    s = State::print_DQ_margin_last;
                    continue;
                } /*"    beq   print_DQ_margin_last"*/
                r1 = 0x20 as u32; /*"    mov   r1, #0x20                              @ ' '"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e784014 as u32; /*"    ldr   r0, =0x1e784014"*/
                State::wait_print_5
            }
            State::wait_print_5 => {
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == 0x40 as u32; /*"    tst   r1, #0x40"*/
                if z {
                    s = State::wait_print_5;
                    continue;
                } /*"    beq   wait_print_5"*/

                r0 = 0x1e784000 as u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x57 as u32; /*"    mov   r1, #0x57                              @ 'W'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x61 as u32; /*"    mov   r1, #0x61                              @ 'a'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x72 as u32; /*"    mov   r1, #0x72                              @ 'r'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6E as u32; /*"    mov   r1, #0x6E                              @ 'n'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x69 as u32; /*"    mov   r1, #0x69                              @ 'i'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6E as u32; /*"    mov   r1, #0x6E                              @ 'n'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x67 as u32; /*"    mov   r1, #0x67                              @ 'g'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x3A as u32; /*"    mov   r1, #0x3A                              @ ':'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x20 as u32; /*"    mov   r1, #0x20                              @ ' '"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x4D as u32; /*"    mov   r1, #0x4D                              @ 'M'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x61 as u32; /*"    mov   r1, #0x61                              @ 'a'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x72 as u32; /*"    mov   r1, #0x72                              @ 'r'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x67 as u32; /*"    mov   r1, #0x67                              @ 'g'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x69 as u32; /*"    mov   r1, #0x69                              @ 'i'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6E as u32; /*"    mov   r1, #0x6E                              @ 'n'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e784014 as u32; /*"    ldr   r0, =0x1e784014"*/
                State::wait_print_6
            }
            State::wait_print_6 => {
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == 0x40 as u32; /*"    tst   r1, #0x40"*/
                if z {
                    s = State::wait_print_6;
                    continue;
                } /*"    beq   wait_print_6"*/
                r0 = 0x1e784000 as u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x20 as u32; /*"    mov   r1, #0x20                              @ ' '"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x74 as u32; /*"    mov   r1, #0x74                              @ 't'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6F as u32; /*"    mov   r1, #0x6F                              @ 'o'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6F as u32; /*"    mov   r1, #0x6F                              @ 'o'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x20 as u32; /*"    mov   r1, #0x20                              @ ' '"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x73 as u32; /*"    mov   r1, #0x73                              @ 's'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6D as u32; /*"    mov   r1, #0x6D                              @ 'm'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x61 as u32; /*"    mov   r1, #0x61                              @ 'a'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6C as u32; /*"    mov   r1, #0x6C                              @ 'l'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6C as u32; /*"    mov   r1, #0x6C                              @ 'l'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                State::print_DQ_margin_last
            }
            State::print_DQ_margin_last => {
                r1 = 0x0D as u32; /*"    mov   r1, #0x0D                              @ '\\r'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x0A as u32; /*"    mov   r1, #0x0A                              @ '\\n'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Debug - UART console message */

                State::platform_exit
            }
            State::platform_exit => {
                if CONFIG_DRAM_ECC == 1 {
                    // #ifdef CONFIG_DRAM_ECC
                    r0 = 0x1e6e0004 as u32; /*"    ldr   r0, =0x1e6e0004"*/
                    r2 = 0x00000880 as u32; /*"    ldr   r2, =0x00000880                        @ add cache range control, 2016.09.02"*/
                    r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                    r1 = r1 | r2; /*"    orr   r1, r1, r2"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/

                    r0 = 0x1e6e0054 as u32; /*"    ldr   r0, =0x1e6e0054"*/
                    r1 = CONFIG_DRAM_ECC_SIZE as u32; /*"    ldr   r1, =CONFIG_DRAM_ECC_SIZE              /* ECC protected memory size */"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/

                    r0 = 0x1e6e007C as u32; /*"    ldr   r0, =0x1e6e007C"*/
                    r1 = 0x00000000 as u32; /*"    ldr   r1, =0x00000000"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e6e0074 as u32; /*"    ldr   r0, =0x1e6e0074"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/

                    r0 = 0x1e6e0070 as u32; /*"    ldr   r0, =0x1e6e0070"*/
                    r1 = 0x00000221 as u32; /*"    ldr   r1, =0x00000221"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/

                    r2 = 0x00001000 as u32; /*"    ldr   r2, =0x00001000"*/
                } // #endif
                s = State::ecc_Init_Flag;
                continue; /*"b ecc_Init_Flag"*/

                State::ecc_Init_Flag
            }
            State::ecc_Init_Flag => {
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == r2; /*"    tst   r1, r2                                 @ D[12] = 1, Done"*/
                if z {
                    s = State::ecc_Init_Flag;
                    continue;
                } /*"    beq   ecc_Init_Flag"*/

                r1 = 0x00000000 as u32; /*"    ldr   r1, =0x00000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0050 as u32; /*"    ldr   r0, =0x1e6e0050"*/
                r1 = 0x80000000 as u32; /*"    ldr   r1, =0x80000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0050 as u32; /*"    ldr   r0, =0x1e6e0050"*/
                r1 = 0x00000000 as u32; /*"    ldr   r1, =0x00000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0070 as u32; /*"    ldr   r0, =0x1e6e0070"*/
                r1 = 0x00000400 as u32; /*"    ldr   r1, =0x00000400                        @ Enable ECC auto-scrubbing"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                //#endif

                /******************************************************************************
                SPI Timing Calibration
                ******************************************************************************/
                r2 = 0x0 as u32; /*"    mov   r2, #0x0"*/
                r6 = 0x0 as u32; /*"    mov   r6, #0x0"*/
                r7 = 0x0 as u32; /*"    mov   r7, #0x0"*/
                init_spi_checksum!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    init_spi_checksum"*/
                State::spi_checksum_wait_0
            }
            State::spi_checksum_wait_0 => {
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == r2; /*"    tst   r1, r2"*/
                if z {
                    s = State::spi_checksum_wait_0;
                    continue;
                } /*"    beq   spi_checksum_wait_0"*/
                r0 = 0x1e620090 as u32; /*"    ldr   r0, =0x1e620090"*/
                r5 = peek(r0); /*"    ldr   r5, [r0]                               @ record golden checksum"*/
                r0 = 0x1e620080 as u32; /*"    ldr   r0, =0x1e620080"*/
                r1 = 0x0 as u32; /*"    mov   r1, #0x0"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e620010 as u32; /*"    ldr   r0, =0x1e620010                        @ set to fast read mode"*/
                r1 = 0x000B0041 as u32; /*"    ldr   r1, =0x000B0041"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r6 = 0x00F7E6D0 as u32; /*"    ldr   r6, =0x00F7E6D0                        @ Init spiclk loop"*/
                r8 = 0x0 as u32; /*"    mov   r8, #0x0                               @ Init delay record"*/

                State::spi_cbr_next_clkrate
            }
            State::spi_cbr_next_clkrate => {
                r6 = r6 >> 0x4 as u32; /*"    mov   r6, r6, lsr #0x4"*/
                z = r6 == 0x0 as u32;

                if z {
                    s = State::spi_cbr_end;
                    continue;
                } /*"    beq   spi_cbr_end"*/

                r7 = 0x0 as u32; /*"    mov   r7, #0x0                               @ Init delay loop"*/
                r8 = r8 << 4 as u32; /*"    mov   r8, r8, lsl #4"*/

                State::spi_cbr_next_delay_s
            }
            State::spi_cbr_next_delay_s => {
                r2 = 0x8 as u32; /*"    mov   r2, #0x8"*/
                init_spi_checksum!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    init_spi_checksum"*/
                State::spi_checksum_wait_1
            }
            State::spi_checksum_wait_1 => {
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == r2; /*"    tst   r1, r2"*/
                if z {
                    s = State::spi_checksum_wait_1;
                    continue;
                } /*"    beq   spi_checksum_wait_1"*/
                r0 = 0x1e620090 as u32; /*"    ldr   r0, =0x1e620090"*/
                r2 = peek(r0); /*"    ldr   r2, [r0]                               @ read checksum"*/
                r0 = 0x1e620080 as u32; /*"    ldr   r0, =0x1e620080"*/
                r1 = 0x0 as u32; /*"    mov   r1, #0x0"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                z = r2 == r5;

                if !z {
                    s = State::spi_cbr_next_delay_e;
                    continue;
                } /*"    bne   spi_cbr_next_delay_e"*/

                r2 = 0x0 as u32; /*"    mov   r2, #0x0"*/
                init_spi_checksum!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    init_spi_checksum"*/
                State::spi_checksum_wait_2
            }
            State::spi_checksum_wait_2 => {
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == r2; /*"    tst   r1, r2"*/
                if z {
                    s = State::spi_checksum_wait_2;
                    continue;
                } /*"    beq   spi_checksum_wait_2"*/
                r0 = 0x1e620090 as u32; /*"    ldr   r0, =0x1e620090"*/
                r2 = peek(r0); /*"    ldr   r2, [r0]                               @ read checksum"*/
                r0 = 0x1e620080 as u32; /*"    ldr   r0, =0x1e620080"*/
                r1 = 0x0 as u32; /*"    mov   r1, #0x0"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                z = r2 == r5;

                if !z {
                    s = State::spi_cbr_next_delay_e;
                    continue;
                } /*"    bne   spi_cbr_next_delay_e"*/

                r8 = r8 | r7; /*"    orr   r8, r8, r7                             @ record passed delay"*/
                s = State::spi_cbr_next_clkrate;
                continue; /*"    b     spi_cbr_next_clkrate"*/

                State::spi_cbr_next_delay_e
            }
            State::spi_cbr_next_delay_e => {
                r7 = r7 + 0x1 as u32; /*"    add   r7, r7, #0x1"*/
                z = r7 == 0x6 as u32;

                lt = r7 < 0x6 as u32; /*"    cmp   r7, #0x6"*/
                if lt {
                    s = State::spi_cbr_next_delay_s;
                    continue;
                } /*"    blt   spi_cbr_next_delay_s"*/
                s = State::spi_cbr_next_clkrate;
                continue; /*"    b     spi_cbr_next_clkrate"*/

                State::spi_cbr_end
            }
            State::spi_cbr_end => {
                r0 = 0x1e620094 as u32; /*"    ldr   r0, =0x1e620094"*/
                poke(r8, r0); /*"    str   r8, [r0]"*/
                r0 = 0x1e620010 as u32; /*"    ldr   r0, =0x1e620010"*/
                r1 = 0x0 as u32; /*"    mov   r1, #0x0"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /******************************************************************************
                Miscellaneous Setting
                ******************************************************************************/
                /* Set UART DMA as AHB high priority master */
                r0 = 0x1e600000 as u32; /*"    ldr   r0, =0x1e600000"*/
                r1 = 0xAEED1A03 as u32; /*"    ldr   r1, =0xAEED1A03"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e600080 as u32; /*"    ldr   r0, =0x1e600080"*/
                r2 = 0x100 as u32; /*"    ldr   r2, =0x100"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 = r1 | r2; /*"    orr   r1, r1, r2"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Enable UART3/4 clock and disable LHCLK */
                r0 = 0x1e6e200c as u32; /*"    ldr   r0, =0x1e6e200c"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r2 = 0xF9FFFFFF as u32; /*"    ldr   r2, =0xF9FFFFFF"*/
                r1 = r1 | r2; /*"    and   r1, r1, r2"*/
                r2 = 0x10000000 as u32; /*"    ldr   r2, =0x10000000"*/
                r1 = r1 | r2; /*"    orr   r1, r1, r2"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e2008 as u32; /*"    ldr   r0, =0x1e6e2008                        @ Set Video ECLK phase"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r2 = 0x0ffffff3 as u32; /*"    ldr   r2, =0x0ffffff3"*/
                r1 = r1 | r2; /*"    and   r1, r1, r2"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e2004 as u32; /*"    ldr r0, =0x1e6e2004                          @ Enable JTAG Master, solve ARM stucked by JTAG issue"*/
                r1 = peek(r0); /*"    ldr r1, [r0]"*/
                r1 = r1 & !0x00400000 as u32; /*"    bic r1, r1, #0x00400000"*/
                poke(r1, r0); /*"    str r1, [r0]"*/

                /******************************************************************************
                Configure MAC timing
                ******************************************************************************/
                /* Enable D2PLL and set to 250MHz */
                r0 = 0x1e6e213c as u32; /*"    ldr   r0, =0x1e6e213c"*/
                r1 = 0x00000585 as u32; /*"    ldr   r1, =0x00000585                        @ Reset D2PLL"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e202c as u32; /*"    ldr   r0, =0x1e6e202c"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 = r1 & !0x10 as u32; /*"    bic   r1, r1, #0x10                          @ Enable D2PLL"*/
                r2 = 0x00200000 as u32; /*"    ldr   r2, =0x00200000                        @ Set CRT = 40MHz"*/
                r1 = r1 | r2; /*"    orr   r1, r1, r2"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r2 = 0x8E00A17C as u32; /*"    ldr   r2, =0x8E00A17C                        @ Set to 250MHz"*/

                r0 = 0x1e6e2070 as u32; /*"    ldr   r0, =0x1e6e2070                        @ Check CLKIN = 25MHz"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 = r1 >> 23 as u32; /*"    mov   r1, r1, lsr #23"*/
                z = r1 == 0x01 as u32; /*"    tst   r1, #0x01"*/
                if z {
                    s = State::set_D2PLL;
                    continue;
                } /*"    beq   set_D2PLL"*/
                r2 = 0x8E00A177 as u32; /*"    ldr   r2, =0x8E00A177"*/

                State::set_D2PLL
            }
            State::set_D2PLL => {
                r0 = 0x1e6e201c as u32; /*"    ldr   r0, =0x1e6e201c"*/
                poke(r2, r0); /*"    str   r2, [r0]"*/
                r0 = 0x1e6e213c as u32; /*"    ldr   r0, =0x1e6e213c                        @ Enable D2PLL"*/
                r1 = 0x00000580 as u32; /*"    ldr   r1, =0x00000580"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e204c as u32; /*"    ldr   r0, =0x1e6e204c"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 = r1 & !0xFF0000 as u32; /*"    bic   r1, r1, #0xFF0000"*/
                r2 = 0x00040000 as u32; /*"    ldr   r2, =0x00040000                        @ Set divider ratio"*/
                r1 = r1 | r2; /*"    orr   r1, r1, r2"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e2048 as u32; /*"    ldr   r0, =0x1e6e2048                        @ Set MAC interface delay timing = 1G"*/
                r1 = 0x80082208 as u32; /*"    ldr   r1, =0x80082208                        @ Select internal 125MHz"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e20b8 as u32; /*"    ldr   r0, =0x1e6e20b8                        @ Set MAC interface delay timing = 100M"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e20bc as u32; /*"    ldr   r0, =0x1e6e20bc                        @ Set MAC interface delay timing = 10M"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e2070 as u32; /*"    ldr   r0, =0x1e6e2070                        @ Set MAC AHB bus clock"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r2 = 0x04 as u32; /*"    mov   r2, #0x04                              @ Default RMII, set MHCLK = HPLL/10"*/
                z = r1 == 0xC0 as u32; /*"    tst   r1, #0xC0"*/
                if !z {
                    r2 = 0x02 as u32;
                } /*"    movne r2, #0x02                              @ if RGMII,     set MHCLK = HPLL/6"*/
                r0 = 0x1e6e2008 as u32; /*"    ldr   r0, =0x1e6e2008"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 = r1 & !0x00070000 as u32; /*"    bic   r1, r1, #0x00070000"*/
                r1 = r1 | (r2 << 16 as u32); /*"    orr   r1, r1, r2, lsl #16"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e21dc as u32; /*"    ldr   r0, =0x1e6e21dc                        @ Set MAC duty"*/
                r1 = 0x00666400 as u32; /*"    ldr   r1, =0x00666400"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e2090 as u32; /*"    ldr   r0, =0x1e6e2090                        @ Enable MAC interface pull low"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 = r1 & !0x0000F000 as u32; /*"    bic   r1, r1, #0x0000F000"*/
                r1 = r1 & !0x20000000 as u32; /*"    bic   r1, r1, #0x20000000                    @ Set USB portA as Device mode"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Test - DRAM initial time */
                r0 = 0x1e782040 as u32; /*"    ldr   r0, =0x1e782040"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r0 = 0xFFFFFFFF as u32; /*"    ldr   r0, =0xFFFFFFFF"*/
                r1 = r0 - r1; /*"    sub   r1, r0, r1"*/
                r0 = 0x1e6e008c as u32; /*"    ldr   r0, =0x1e6e008c"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e78203c as u32; /*"    ldr   r0, =0x1e78203c"*/
                r1 = 0x0000F000 as u32; /*"    ldr   r1, =0x0000F000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Test - DRAM initial time */

                r0 = 0x1e6e0000 as u32; /*"    ldr   r0, =0x1e6e0000                        @ disable MMC password"*/
                r1 = 0x0 as u32; /*"    mov   r1, #0x0"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Disable Timer separate mode */
                r0 = 0x1e782038 as u32; /*"    ldr   r0, =0x1e782038"*/
                r1 = 0xEA as u32; /*"    ldr   r1, =0xEA"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                s = State::Exit;
                continue; /*"\tb Exit"*/
                /* restore lr */

                /* back to arch calling code */

                // HERE works
            }
        }
    }
}

#[derive(Debug)]
enum State {
    init_dram = 0,
    start_first_reset = 1,
    wait_first_reset = 2,
    bypass_first_reset = 3,
    wait_usb_init = 4,
    bypass_USB_init = 5,
    bypass_mpll_hynix_mode_1 = 6,
    bypass_mpll_hynix_mode_2 = 7,
    set_MPLL = 8,
    wait_mpll_init = 9,
    reset_mmc = 10,
    wait_mmc_reset = 11,
    wait_mmc_reset_done = 12,
    wait_ddr_reset = 13,
    wait_print = 14,
    ddr3_init = 15,
    ddr3_phyinit_done = 16,
    ddr3_check_dllrdy = 17,
    ddr4_init = 18,
    ddr4_ron_phyinit_done = 19,
    ddr4_vref_phy_cal_start = 20,
    ddr4_vref_phy_loop = 21,
    ddr4_vref_phy_phyinit_done = 22,
    ddr4_vref_phy_cbrtest_done = 23,
    ddr4_vref_phy_test_fail = 24,
    ddr4_vref_phy_loop_end = 25,
    ddr4_vref_ddr_cal_start = 26,
    ddr4_vref_ddr_loop = 27,
    ddr4_vref_ddr_phyinit_done = 28,
    ddr4_vref_ddr_cbrtest_done = 29,
    ddr4_vref_ddr_test_fail = 30,
    ddr4_vref_ddr_loop_end = 31,
    ddr4_phyinit_done = 32,
    ddr_phy_init_process = 33,
    ddr_phy_init = 34,
    ddr_phy_init_timeout = 35,
    ddr_phy_init_delay_0 = 36,
    ddr_phy_init_success = 37,
    cbr_test_start = 38,
    cbr_test_single = 39,
    cbr_wait_engine_idle_0 = 40,
    cbr_test_burst = 41,
    cbr_test_burst_loop = 42,
    cbr_wait_engine_idle_1 = 43,
    cbr_test_fail = 44,
    cbr_test_pattern_end = 45,
    calibration_End = 46,
    check_dram_size = 47,
    check_dram_size_end = 48,
    dram_cache_init = 49,
    ddr_test_start = 50,
    ddr_test_burst = 51,
    ddr_test_burst_loop = 52,
    ddr_wait_engine_idle_1 = 53,
    ddr_test_fail = 54,
    wait_print_0 = 55,
    set_scratch = 56,
    wait_print_1 = 57,
    wait_print_2 = 58,
    print_DQL_eye_margin = 59,
    print_DQL_div_loop = 60,
    print_DQL_div_digit = 61,
    print_DQL_div_0 = 62,
    print_DQL_div_n = 63,
    print_DQL_next = 64,
    print_DQH_eye_margin = 65,
    wait_print_3 = 66,
    print_DQH_div_loop = 67,
    print_DQH_div_digit = 68,
    print_DQH_div_0 = 69,
    print_DQH_div_n = 70,
    print_DQH_next = 71,
    print_DQ_eye_margin_last = 72,
    wait_print_4 = 73,
    wait_print_5 = 74,
    wait_print_6 = 75,
    print_DQ_margin_last = 76,
    platform_exit = 77,
    ecc_Init_Flag = 78,
    spi_checksum_wait_0 = 79,
    spi_cbr_next_clkrate = 80,
    spi_cbr_next_delay_s = 81,
    spi_checksum_wait_1 = 82,
    spi_checksum_wait_2 = 83,
    spi_cbr_next_delay_e = 84,
    spi_cbr_end = 85,
    set_D2PLL = 86,
    uartSETUP,
    PowerOn,
    Exit,
}
