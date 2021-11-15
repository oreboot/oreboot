pub mod ramtable;
#[macro_use]
pub mod ram;
use core::ptr;

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
//const CONFIG_DDR4_HYNIX_SET_1440: u32 = 1;

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
//const CONFIG_DRAM_UART_115200: u32 = 1;
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

fn poke(v: u32, a: u32) {
    let y = a as *mut u32;
    unsafe {
        ptr::write_volatile(y, v);
    }
}
fn peek(a: u32) -> u32 {
    let y = a as *const u32;
    unsafe { ptr::read_volatile(y) }
}
pub fn ram(w: &mut impl core::fmt::Write) {
    let mut tptr = ramtable::TIME_TABLE_DDR3_1333;
    let mut r0 = 0u32;
    let mut r1 = 0u32;
    let mut r2 = 0u32;
    let mut r3 = 0u32;
    let _r4 = 0u32;
    let mut r5 = 0u32;
    let mut r6 = 0u32;
    let mut r7 = 0u32;
    let mut r8 = 0u32;
    let mut r9 = 0u32;
    let mut r10 = 0u32;
    //let mut r11 = 0u32;
    let mut z = false;
    let mut gt;
    let mut lt;
    let mut s = State::PowerOn;
    write!(w, "Start s is {:#?}\n", s).expect("oh no");
    loop {
        write!(w, "loop s is {:?}\n", s).expect("oh no");
        s = match s {
            State::Exit => {
                write!(w, "DRAM done\n").expect("oh no");
                break;
            }
            // This will be duplicative of InitDram for now. All we're trying to do
            // first is get some kinda serial output on power on. Nothing more.
            State::PowerOn => State::UartSetup,
            State::UartSetup => {
                // Put only the bare minimum code here needed for uart5.
                // There shall be no magic numbers.
                //
                // let's see if it worked ...
                for _ in 0..32 {
                    r0 = UART5DR; /*"    ldr   r0, =0x1e784000"*/
                    r1 = 'O' as u32;
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                }
                State::InitDram
            }
            State::InitDram => {
                /* save lr */

                /********************************************
                  Initial Reset Procedure : Begin
                *******************************************/
                /* Clear AHB bus lock condition */
                r0 = 0x1e600000_u32; /*"    ldr   r0, =0x1e600000"*/
                r1 = 0xAEED1A03_u32; /*"    ldr   r1, =0xAEED1A03"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e600084_u32; /*"    ldr   r0, =0x1e600084"*/
                r1 = 0x00010000_u32; /*"    ldr   r1, =0x00010000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 += 0x4_u32; /*"    add   r0, r0, #0x4"*/
                r1 = 0x0_u32; /*"    mov   r1, #0x0"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e2000_u32; /*"    ldr   r0, =0x1e6e2000"*/
                r1 = 0x1688a8a8_u32; /*"    ldr   r1, =0x1688a8a8"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Reset again */
                r0 = 0x1e6e2070_u32; /*"    ldr   r0, =0x1e6e2070                        @ check fast reset flag"*/
                r2 = 0x08000000_u32; /*"    ldr   r2, =0x08000000"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == r2; /*"    tst   r1, r2"*/
                if z {
                    s = State::BypassFirstReset;
                    continue;
                } /*"    beq   BypassFirstReset"*/

                r0 = 0x1e785010_u32; /*"    ldr   r0, =0x1e785010"*/
                r3 = peek(r0); /*"    ldr   r3, [r0]"*/
                z = r3 == 0x0_u32;

                if z {
                    s = State::StartFirstReset;
                    continue;
                } /*"    beq   StartFirstReset"*/
                // The real question: what is this code? It's not first reset, not bypass first reset.
                r0 += 0x04_u32; /*"    add   r0, r0, #0x04"*/
                r3 = 0x77_u32; /*"    mov   r3, #0x77"*/
                poke(r3, r0); /*"    str   r3, [r0]"*/
                r0 = 0x1e720004_u32; /*"    ldr   r0, =0x1e720004                        @ Copy initial strap register to 0x1e720004"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 += 0x04_u32; /*"    add   r0, r0, #0x04                          @ Copy initial strap register to 0x1e720008"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 += 0x04_u32; /*"    add   r0, r0, #0x04                          @ Copy initial strap register to 0x1e72000c"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e207c_u32; /*"    ldr   r0, =0x1e6e207c                        @ clear fast reset flag"*/
                poke(r2, r0); /*"    str   r2, [r0]"*/
                r0 = 0x1e6e203c_u32; /*"    ldr   r0, =0x1e6e203c                        @ clear watchdog reset flag"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 |= 0x01_u32; /*"    and   r1, r1, #0x01"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e78501c_u32; /*"    ldr   r0, =0x1e78501c                        @ restore normal mask setting"*/
                r1 = 0x023FFFF3_u32; /*"    ldr   r1, =0x023FFFF3                        @ added 2016.09.06"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                s = State::BypassFirstReset;
                continue; /*"    b     BypassFirstReset"*/
            }
            State::StartFirstReset => {
                if ASTMMC_INIT_RESET_MODE_FULL == 1 {
                    // #ifdef ASTMMC_INIT_RESET_MODE_FULL
                    r0 = 0x1e785004_u32; /*"    ldr   r0, =0x1e785004"*/
                    r1 = 0x00000001_u32; /*"    ldr   r1, =0x00000001"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e785008_u32; /*"    ldr   r0, =0x1e785008"*/
                    r1 = 0x00004755_u32; /*"    ldr   r1, =0x00004755"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e78500c_u32; /*"    ldr   r0, =0x1e78500c                        @ enable Full reset"*/
                    r1 = 0x00000033_u32; /*"    ldr   r1, =0x00000033"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                } else {
                    // #else     /***** Clear LPC status : Begin *****/
                    r2 = 0_u32; /*"    mov   r2, #0                                 @ set r2 = 0, freezed"*/
                    r0 = 0x1e787008_u32; /*"    ldr   r0, =0x1e787008"*/
                    r1 = 0x7_u32; /*"    mov   r1, #0x7"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e78700c_u32; /*"    ldr   r0, =0x1e78700c"*/
                    r1 = 0x3_u32; /*"    mov   r1, #0x3"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e787020_u32; /*"    ldr   r0, =0x1e787020"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e787034_u32; /*"    ldr   r0, =0x1e787034"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e787004_u32; /*"    ldr   r0, =0x1e787004"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e787010_u32; /*"    ldr   r0, =0x1e787010"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e78701c_u32; /*"    ldr   r0, =0x1e78701c"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e787014_u32; /*"    ldr   r0, =0x1e787014                        @ read clear"*/
                    /*r1 = */
                    peek(r0); /*"    ldr   r1, [r0]"*/
                    r0 = 0x1e787018_u32; /*"    ldr   r0, =0x1e787018                        @ read clear"*/
                    /*r1 = */
                    peek(r0); /*"    ldr   r1, [r0]"*/
                    r0 = 0x1e787008_u32; /*"    ldr   r0, =0x1e787008                        @ read clear"*/
                    /*r1 = */
                    peek(r0); /*"    ldr   r1, [r0]"*/
                    r0 = 0x1e78301c_u32; /*"    ldr   r0, =0x1e78301c                        @ read clear"*/
                    /*r1 = */
                    peek(r0); /*"    ldr   r1, [r0]"*/
                    r0 = 0x1e78d01c_u32; /*"    ldr   r0, =0x1e78d01c                        @ read clear"*/
                    /*r1 = */
                    peek(r0); /*"    ldr   r1, [r0]"*/
                    r0 = 0x1e78e01c_u32; /*"    ldr   r0, =0x1e78e01c                        @ read clear"*/
                    /*r1 = */
                    peek(r0); /*"    ldr   r1, [r0]"*/
                    r0 = 0x1e78f01c_u32; /*"    ldr   r0, =0x1e78f01c                        @ read clear"*/
                    /*r1 = */
                    peek(r0); /*"    ldr   r1, [r0]"*/
                    r0 = 0x1e788020_u32; /*"    ldr   r0, =0x1e788020"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e788034_u32; /*"    ldr   r0, =0x1e788034"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e78800c_u32; /*"    ldr   r0, =0x1e78800c"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e789008_u32; /*"    ldr   r0, =0x1e789008"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e789010_u32; /*"    ldr   r0, =0x1e789010"*/
                    r1 = 0x40_u32; /*"    mov   r1, #0x40"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789024_u32; /*"    ldr   r0, =0x1e789024                        @ read clear"*/
                    /*r1 = */
                    peek(r0); /*"    ldr   r1, [r0]"*/
                    r0 = 0x1e789028_u32; /*"    ldr   r0, =0x1e789028                        @ read clear"*/
                    /*r1 = */
                    peek(r0); /*"    ldr   r1, [r0]"*/
                    r0 = 0x1e78902c_u32; /*"    ldr   r0, =0x1e78902c                        @ read clear"*/
                    /*r1 = */
                    peek(r0); /*"    ldr   r1, [r0]"*/
                    r0 = 0x1e789114_u32; /*"    ldr   r0, =0x1e789114                        @ read clear"*/
                    /*r1 = */
                    peek(r0); /*"    ldr   r1, [r0]"*/
                    r0 = 0x1e789124_u32; /*"    ldr   r0, =0x1e789124                        @ read clear"*/
                    /*r1 = */
                    peek(r0); /*"    ldr   r1, [r0]"*/
                    r0 = 0x1e78903c_u32; /*"    ldr   r0, =0x1e78903c"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e789040_u32; /*"    ldr   r0, =0x1e789040"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e789044_u32; /*"    ldr   r0, =0x1e789044"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e78911c_u32; /*"    ldr   r0, =0x1e78911c"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e78912c_u32; /*"    ldr   r0, =0x1e78912c"*/
                    r1 = 0x200_u32; /*"    ldr   r1, =0x200"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789104_u32; /*"    ldr   r0, =0x1e789104"*/
                    r1 = 0xcc00_u32; /*"    ldr   r1, =0xcc00"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789108_u32; /*"    ldr   r0, =0x1e789108"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e78910c_u32; /*"    ldr   r0, =0x1e78910c"*/
                    r1 = 0x1f0_u32; /*"    ldr   r1, =0x1f0"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789170_u32; /*"    ldr   r0, =0x1e789170"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e789174_u32; /*"    ldr   r0, =0x1e789174"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e7890a0_u32; /*"    ldr   r0, =0x1e7890a0"*/
                    r1 = 0xff00_u32; /*"    ldr   r1, =0xff00"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e7890a4_u32; /*"    ldr   r0, =0x1e7890a4"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e789080_u32; /*"    ldr   r0, =0x1e789080"*/
                    r1 = 0x400_u32; /*"    ldr   r1, =0x400"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789084_u32; /*"    ldr   r0, =0x1e789084"*/
                    r1 = 0x0001000f_u32; /*"    ldr   r1, =0x0001000f"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789088_u32; /*"    ldr   r0, =0x1e789088"*/
                    r1 = 0x3000fff8_u32; /*"    ldr   r1, =0x3000fff8"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e78908c_u32; /*"    ldr   r0, =0x1e78908c"*/
                    r1 = 0xfff8f007_u32; /*"    ldr   r1, =0xfff8f007"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789098_u32; /*"    ldr   r0, =0x1e789098"*/
                    r1 = 0x00000a30_u32; /*"    ldr   r1, =0x00000a30"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e78909c_u32; /*"    ldr   r0, =0x1e78909c"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e789100_u32; /*"    ldr   r0, =0x1e789100"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e789130_u32; /*"    ldr   r0, =0x1e789130"*/
                    r1 = 0x00000080_u32; /*"    ldr   r1, =0x00000080"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789138_u32; /*"    ldr   r0, =0x1e789138"*/
                    r1 = 0x00010198_u32; /*"    ldr   r1, =0x00010198"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789140_u32; /*"    ldr   r0, =0x1e789140"*/
                    r1 = 0x0000a000_u32; /*"    ldr   r1, =0x0000a000"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789158_u32; /*"    ldr   r0, =0x1e789158"*/
                    r1 = 0x00000080_u32; /*"    ldr   r1, =0x00000080"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789180_u32; /*"    ldr   r0, =0x1e789180"*/
                    r1 = 0xb6db1bff_u32; /*"    ldr   r1, =0xb6db1bff"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789184_u32; /*"    ldr   r0, =0x1e789184"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e789188_u32; /*"    ldr   r0, =0x1e789188"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e78918c_u32; /*"    ldr   r0, =0x1e78918c"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e789190_u32; /*"    ldr   r0, =0x1e789190"*/
                    r1 = 0x05020100_u32; /*"    ldr   r1, =0x05020100"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789194_u32; /*"    ldr   r0, =0x1e789194"*/
                    r1 = 0x07000706_u32; /*"    ldr   r1, =0x07000706"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789198_u32; /*"    ldr   r0, =0x1e789198"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e78919c_u32; /*"    ldr   r0, =0x1e78919c"*/
                    r1 = 0x30_u32; /*"    ldr   r1, =0x30"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e7891a0_u32; /*"    ldr   r0, =0x1e7891a0"*/
                    r1 = 0x00008100_u32; /*"    ldr   r1, =0x00008100"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e7891a4_u32; /*"    ldr   r0, =0x1e7891a4"*/
                    r1 = 0x2000_u32; /*"    ldr   r1, =0x2000"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e7891a8_u32; /*"    ldr   r0, =0x1e7891a8"*/
                    r1 = 0x3ff_u32; /*"    ldr   r1, =0x3ff"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e7891ac_u32; /*"    ldr   r0, =0x1e7891ac"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e789240_u32; /*"    ldr   r0, =0x1e789240"*/
                    r1 = 0xff_u32; /*"    mov   r1, #0xff"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789244_u32; /*"    ldr   r0, =0x1e789244"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789248_u32; /*"    ldr   r0, =0x1e789248"*/
                    r1 = 0x80_u32; /*"    mov   r1, #0x80"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e789250_u32; /*"    ldr   r0, =0x1e789250"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    r0 = 0x1e789254_u32; /*"    ldr   r0, =0x1e789254"*/
                    poke(r2, r0); /*"    str   r2, [r0]"*/
                    /***** Clear LPC status : End *****/

                    r0 = 0x1e62009c_u32; /*"    ldr   r0, =0x1e62009c                        @ clear software strap flag for doing again after reset"*/
                    r1 = 0xAEEDFC20_u32; /*"    ldr   r1, =0xAEEDFC20"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e785004_u32; /*"    ldr   r0, =0x1e785004"*/
                    r1 = 0x00000001_u32; /*"    ldr   r1, =0x00000001"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e785008_u32; /*"    ldr   r0, =0x1e785008"*/
                    r1 = 0x00004755_u32; /*"    ldr   r1, =0x00004755"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e78501c_u32; /*"    ldr   r0, =0x1e78501c                        @ enable full mask of SOC reset"*/
                    r1 = 0x03FFFFFF_u32; /*"    ldr   r1, =0x03FFFFFF                        @ added 2016.09.06"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e78500c_u32; /*"    ldr   r0, =0x1e78500c                        @ enable SOC reset"*/
                    r1 = 0x00000013_u32; /*"    ldr   r1, =0x00000013"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                } // #endif
                State::WaitFirstReset
            }
            State::WaitFirstReset => {
                // What are we doing here? Simply put, we've kicked off a reset from
                // above, and we loop here. At some point the reset comes in and we're back to
                // the beginning.
                s = State::WaitFirstReset;
                continue; /*"    b     WaitFirstReset"*/

                /********************************************
                  Initial Reset Procedure : End
                *******************************************/
            }
            State::BypassFirstReset => {
                /* Enable Timer separate clear mode */
                r0 = 0x1e782038_u32; /*"    ldr   r0, =0x1e782038"*/
                r1 = 0xAE_u32; /*"    mov   r1, #0xAE"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Test - DRAM initial time */
                r0 = 0x1e78203c_u32; /*"    ldr   r0, =0x1e78203c"*/
                r1 = 0x0000F000_u32; /*"    ldr   r1, =0x0000F000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e782044_u32; /*"    ldr   r0, =0x1e782044"*/
                r1 = 0xFFFFFFFF_u32; /*"    ldr   r1, =0xFFFFFFFF"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e782030_u32; /*"    ldr   r0, =0x1e782030"*/
                r2 = 3_u32; /*"    mov   r2, #3"*/
                r1 = r2 << 12_u32; /*"    mov   r1, r2, lsl #12"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Test - DRAM initial time */

                /*Set Scratch register Bit 7 before initialize*/
                r0 = 0x1e6e2000_u32; /*"    ldr   r0, =0x1e6e2000"*/
                r1 = 0x1688a8a8_u32; /*"    ldr   r1, =0x1688a8a8"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e2040_u32; /*"    ldr   r0, =0x1e6e2040"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 |= 0x80_u32; /*"    orr   r1, r1, #0x80"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Change LPC reset source to PERST# when eSPI mode enabled */
                r0 = 0x1e6e2070_u32; /*"    ldr   r0, =0x1e6e2070"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r0 = 0x1e6e207c_u32; /*"    ldr   r0, =0x1e6e207c"*/
                r2 = 0x02000000_u32; /*"    ldr   r2, =0x02000000"*/
                r3 = 0x00004000_u32; /*"    ldr   r3, =0x00004000"*/
                z = r1 == r2; /*"    tst   r1, r2"*/
                if !z {
                    poke(r3, r0);
                } /*"    strne r3, [r0]"*/

                /* Configure USB ports to the correct pin state */
                r0 = 0x1e6e200c_u32; /*"    ldr   r0, =0x1e6e200c                        @ enable portA clock"*/
                r2 = 0x00004000_u32; /*"    ldr   r2, =0x00004000"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 |= r2; /*"    orr   r1, r1, r2"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e2090_u32; /*"    ldr   r0, =0x1e6e2090                        @ set portA as host mode"*/
                r1 = 0x2000A000_u32; /*"    ldr   r1, =0x2000A000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e2094_u32; /*"    ldr   r0, =0x1e6e2094                        @ set portB as host mode"*/
                r1 = 0x00004000_u32; /*"    ldr   r1, =0x00004000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e2070_u32; /*"    ldr   r0, =0x1e6e2070"*/
                r2 = 0x00800000_u32; /*"    ldr   r2, =0x00800000"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == r2; /*"    tst   r1, r2"*/
                if z {
                    s = State::BypassUSBInit;
                    continue;
                } /*"    beq   BypassUSBInit"*/
                r0 = 0x1e6e207c_u32; /*"    ldr   r0, =0x1e6e207c"*/
                poke(r2, r0); /*"    str   r2, [r0]"*/

                /* Delay about 1ms */
                clear_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    clear_delay_timer"*/
                r2 = 0x000003E8_u32; /*"    ldr   r2, =0x000003E8                        @ Set Timer3 Reload = 1 ms"*/
                init_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    init_delay_timer"*/
                State::WaitUsbInit
            }
            State::WaitUsbInit => {
                check_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    check_delay_timer"*/
                if !z {
                    s = State::WaitUsbInit;
                    continue;
                } /*"    bne   WaitUsbInit"*/
                clear_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    clear_delay_timer"*/
                /* end delay 1ms */

                r0 = 0x1e6e2070_u32; /*"    ldr   r0, =0x1e6e2070"*/
                r1 = 0x00800000_u32; /*"    ldr   r1, =0x00800000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                State::BypassUSBInit
            }
            State::BypassUSBInit => {
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
                r0 = 0x1e78504c_u32; /*"    ldr   r0, =0x1e78504c"*/
                r1 = 0_u32; /*"    mov   r1, #0"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0000_u32; /*"    ldr   r0, =0x1e6e0000"*/
                r1 = 0xFC600309_u32; /*"    ldr   r1, =0xFC600309"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Check Scratch Register Bit 6 */
                r0 = 0x1e6e2040_u32; /*"    ldr   r0, =0x1e6e2040"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 &= !0xFFFFFFBF as u32; /*"    bic   r1, r1, #0xFFFFFFBF"*/
                r2 = r1 >> 6_u32; /*"    mov   r2, r1, lsr #6"*/
                z = r2 == 0x01_u32;

                if z {
                    s = State::PlatformExit;
                    continue;
                } /*"    beq   PlatformExit"*/

                /* Disable VGA display */
                r0 = 0x1e6e202c_u32; /*"    ldr   r0, =0x1e6e202c"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 |= 0x40_u32; /*"    orr   r1, r1, #0x40"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e2070_u32; /*"    ldr   r0, =0x1e6e2070                        @ Load strap register"*/
                r3 = peek(r0); /*"    ldr   r3, [r0]"*/

                /* Set M-PLL */
                if CONFIG_DRAM_1333 == 1 {
                    // #if   defined (CONFIG_DRAM_1333)
                    r2 = 0xC48066C0_u32; /*"    ldr   r2, =0xC48066C0                        @ load PLL parameter for 24Mhz CLKIN (330)"*/
                } else {
                    // #else 	r2 = 0x93002400 as u32;/*"    ldr   r2, =0x93002400                        @ load PLL parameter for 24Mhz CLKIN (396)"*/
                    if CONFIG_DDR4_SUPPORT_HYNIX == 1 {
                        // #if   defined (CONFIG_DDR4_SUPPORT_HYNIX)
                        r1 = r3 >> 24_u32; /*"    mov   r1, r3, lsr #24                        @ Check DDR4"*/
                        z = r1 == 0x01_u32; /*"    tst   r1, #0x01"*/
                        if z {
                            s = State::BypassMpllHynixMode1;
                            continue;
                        } /*"    beq   BypassMpllHynixMode1"*/
                        if CONFIG_DDR4_HYNIX_SET_1536 == 1 {
                            // #if   defined (CONFIG_DDR4_HYNIX_SET_1536)
                            r2 = 0x930023E0_u32; /*"    ldr   r2, =0x930023E0                        @ load PLL parameter for 24Mhz CLKIN (384)"*/
                        } else if CONFIG_DDR4_HYNIX_SET_1488 == 1 {
                            // #elif defined (CONFIG_DDR4_HYNIX_SET_1488)
                            r2 = 0x930023C0_u32; /*"    ldr   r2, =0x930023C0                        @ load PLL parameter for 24Mhz CLKIN (372)"*/
                        } else {
                            // #else 	r2 = 0x930023A0 as u32;/*"    ldr   r2, =0x930023A0                        @ load PLL parameter for 24Mhz CLKIN (360)"*/
                        } // #endif
                        s = State::BypassMpllHynixMode1;
                        continue; /*"\tb BypassMpllHynixMode1"*/
                    } // #endif
                } // #endif

                State::BypassMpllHynixMode1
            }
            State::BypassMpllHynixMode1 => {
                r1 = r3 >> 23_u32; /*"    mov   r1, r3, lsr #23                        @ Check CLKIN = 25MHz"*/
                z = r1 == 0x01_u32; /*"    tst   r1, #0x01"*/
                if z {
                    s = State::SetMPLL;
                    continue;
                } /*"    beq   SetMPLL"*/
                if CONFIG_DRAM_1333 == 1 {
                    // #if   defined (CONFIG_DRAM_1333)
                    r2 = 0xC4806680_u32; /*"    ldr   r2, =0xC4806680                        @ load PLL parameter for 25Mhz CLKIN (331)"*/
                } else {
                    // #else 	r2 = 0x930023E0 as u32;/*"    ldr   r2, =0x930023E0                        @ load PLL parameter for 25Mhz CLKIN (400)"*/
                    if CONFIG_DDR4_SUPPORT_HYNIX == 1 {
                        // #if   defined (CONFIG_DDR4_SUPPORT_HYNIX)
                        r1 = r3 >> 24_u32; /*"    mov   r1, r3, lsr #24                        @ Check DDR4"*/
                        z = r1 == 0x01_u32; /*"    tst   r1, #0x01"*/
                        if z {
                            s = State::BypassMpllHynixMode2;
                            continue;
                        } /*"    beq   BypassMpllHynixMode2"*/
                        if CONFIG_DDR4_HYNIX_SET_1536 == 1 {
                            // #if   defined (CONFIG_DDR4_HYNIX_SET_1536)
                            r2 = 0x930023C0_u32; /*"    ldr   r2, =0x930023C0                        @ load PLL parameter for 24Mhz CLKIN (387.5)"*/
                        } else if CONFIG_DDR4_HYNIX_SET_1488 == 1 {
                            // #elif defined (CONFIG_DDR4_HYNIX_SET_1488)
                            r2 = 0x930023A0_u32; /*"    ldr   r2, =0x930023A0                        @ load PLL parameter for 24Mhz CLKIN (375)"*/
                        } else {
                            // #else 	r2 = 0x93002380 as u32;/*"    ldr   r2, =0x93002380                        @ load PLL parameter for 24Mhz CLKIN (362.5)"*/
                            s = State::BypassMpllHynixMode2;
                            continue; /*"    b   BypassMpllHynixMode2"*/
                        } // #endif
                    } // #endif
                } // #endif
                State::BypassMpllHynixMode2
            }
            State::BypassMpllHynixMode2 => {
                r0 = 0x1e6e2160_u32; /*"    ldr   r0, =0x1e6e2160                        @ set 24M Jitter divider (HPLL=825MHz)"*/
                r1 = 0x00011320_u32; /*"    ldr   r1, =0x00011320"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                State::SetMPLL
            }
            State::SetMPLL => {
                r0 = 0x1e6e2020_u32; /*"    ldr   r0, =0x1e6e2020                        @ M-PLL (DDR SDRAM) Frequency"*/
                poke(r2, r0); /*"    str   r2, [r0]"*/

                clear_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    clear_delay_timer"*/

                /* Delay about 3ms */
                r2 = 0x00000BB8_u32; /*"    ldr   r2, =0x00000BB8                        @ Set Timer3 Reload = 3 ms"*/
                init_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    init_delay_timer"*/
                State::WaitMpllInit
            }
            State::WaitMpllInit => {
                check_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    check_delay_timer"*/
                if !z {
                    s = State::WaitMpllInit;
                    continue;
                } /*"    bne   WaitMpllInit"*/
                clear_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    clear_delay_timer"*/
                /* end delay 3ms */

                /* Reset MMC */
                State::ResetMmc
            }
            State::ResetMmc => {
                r0 = 0x1e78505c_u32; /*"    ldr   r0, =0x1e78505c"*/
                r1 = 0x00000004_u32; /*"    ldr   r1, =0x00000004"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e785044_u32; /*"    ldr   r0, =0x1e785044"*/
                r1 = 0x00000001_u32; /*"    ldr   r1, =0x00000001"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e785048_u32; /*"    ldr   r0, =0x1e785048"*/
                r1 = 0x00004755_u32; /*"    ldr   r1, =0x00004755"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e78504c_u32; /*"    ldr   r0, =0x1e78504c"*/
                r1 = 0x00000013_u32; /*"    ldr   r1, =0x00000013"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                State::WaitMmcReset
            }
            State::WaitMmcReset => {
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == 0x02_u32; /*"    tst   r1, #0x02"*/
                if !z {
                    s = State::WaitMmcReset;
                    continue;
                } /*"    bne   WaitMmcReset"*/

                r0 = 0x1e78505c_u32; /*"    ldr   r0, =0x1e78505c"*/
                r1 = 0x023FFFF3_u32; /*"    ldr   r1, =0x023FFFF3"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e785044_u32; /*"    ldr   r0, =0x1e785044"*/
                r1 = 0x000F4240_u32; /*"    ldr   r1, =0x000F4240"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e785048_u32; /*"    ldr   r0, =0x1e785048"*/
                r1 = 0x00004755_u32; /*"    ldr   r1, =0x00004755"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e785054_u32; /*"    ldr   r0, =0x1e785054"*/
                r1 = 0x00000077_u32; /*"    ldr   r1, =0x00000077"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0000_u32; /*"    ldr   r0, =0x1e6e0000"*/
                r1 = 0xFC600309_u32; /*"    ldr   r1, =0xFC600309"*/
                State::WaitMmcResetDone
            }
            State::WaitMmcResetDone => {
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r2 = peek(r0); /*"    ldr   r2, [r0]"*/
                z = r2 == 0x1_u32;

                if !z {
                    s = State::WaitMmcResetDone;
                    continue;
                } /*"    bne   WaitMmcResetDone"*/

                r0 = 0x1e6e0034_u32; /*"    ldr   r0, =0x1e6e0034                        @ disable MMC request"*/
                r1 = 0x00020000_u32; /*"    ldr   r1, =0x00020000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Delay about 10ms */
                r2 = 0x00002710_u32; /*"    ldr   r2, =0x00002710                        @ Set Timer3 Reload = 10 ms"*/
                init_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    init_delay_timer"*/
                State::WaitDdrReset
            }
            State::WaitDdrReset => {
                check_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    check_delay_timer"*/
                if !z {
                    s = State::WaitDdrReset;
                    continue;
                } /*"    bne   WaitDdrReset"*/
                clear_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    clear_delay_timer"*/
                /* end delay 10ms */

                /* Debug - UART console message */
                if CONFIG_DRAM_UART_TO_UART1 == 1 {
                    // #ifdef CONFIG_DRAM_UART_TO_UART1
                    r0 = 0x1e78909c_u32; /*"    ldr   r0, =0x1e78909c                        @ route UART5 to UART Port1, 2016.08.29"*/
                    r1 = 0x10000004_u32; /*"    ldr   r1, =0x10000004"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/

                    r0 = 0x1e6e2084_u32; /*"    ldr   r0, =0x1e6e2084"*/
                    r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                    r2 = 0xC0_u32; /*"    mov   r2, #0xC0                              @ Enable pinmux of TXD1/RXD1"*/
                    r1 |= r2 << 16_u32; /*"    orr   r1, r1, r2, lsl #16"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                } // #endif

                r0 = 0x1e78400c_u32; /*"    ldr   r0, =0x1e78400c"*/
                r1 = 0x83_u32; /*"    mov   r1, #0x83"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e202c_u32; /*"    ldr   r0, =0x1e6e202c"*/
                r2 = peek(r0); /*"    ldr   r2, [r0]"*/
                r2 >>= 12_u32; /*"    mov   r2, r2, lsr #12"*/
                z = r2 == 0x01_u32; /*"    tst   r2, #0x01"*/
                r0 = 0x1e784000_u32; /*"    ldr   r0, =0x1e784000"*/
                if z {
                    r1 = 0x0D_u32;
                } /*"    moveq r1, #0x0D                              @ Baudrate 115200"*/
                if !z {
                    r1 = 0x01_u32;
                } /*"    movne r1, #0x01                              @ Baudrate 115200, div13"*/
                if CONFIG_DRAM_UART_38400 == 1 {
                    // #ifdef CONFIG_DRAM_UART_38400
                    if z {
                        r1 = 0x27_u32;
                    } /*"    moveq r1, #0x27                              @ Baudrate 38400"*/
                    if !z {
                        r1 = 0x03_u32;
                    } /*"    movne r1, #0x03                              @ Baudrate 38400 , div13"*/
                } // #endif
                if CONFIG_DRAM_UART_57600 == 1 {
                    // #ifdef CONFIG_DRAM_UART_57600
                    if z {
                        r1 = 0x1A_u32;
                    } /*"    moveq r1, #0x1A                              @ Baudrate 57600"*/
                    if !z {
                        r1 = 0x02_u32;
                    } /*"    movne r1, #0x02                              @ Baudrate 57600 , div13"*/
                } // #endif
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e784004_u32; /*"    ldr   r0, =0x1e784004"*/
                r1 = 0x00_u32; /*"    mov   r1, #0x00"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e78400c_u32; /*"    ldr   r0, =0x1e78400c"*/
                r1 = 0x03_u32; /*"    mov   r1, #0x03"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e784008_u32; /*"    ldr   r0, =0x1e784008"*/
                r1 = 0x07_u32; /*"    mov   r1, #0x07"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e784000_u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x0D_u32; /*"    mov   r1, #0x0D                              @ '\\r'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x0A_u32; /*"    mov   r1, #0x0A                              @ '\\n'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x44_u32; /*"    mov   r1, #0x44                              @ 'D'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x52_u32; /*"    mov   r1, #0x52                              @ 'R'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x41_u32; /*"    mov   r1, #0x41                              @ 'A'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x4D_u32; /*"    mov   r1, #0x4D                              @ 'M'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x20_u32; /*"    mov   r1, #0x20                              @ ' '"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x49_u32; /*"    mov   r1, #0x49                              @ 'I'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6E_u32; /*"    mov   r1, #0x6E                              @ 'n'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x69_u32; /*"    mov   r1, #0x69                              @ 'i'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x74_u32; /*"    mov   r1, #0x74                              @ 't'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x2D_u32; /*"    mov   r1, #0x2D                              @ '-'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x56_u32; /*"    mov   r1, #0x56                              @ 'V'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = ASTMMC_INIT_VER as u32; /*"    mov   r1, #ASTMMC_INIT_VER"*/
                r1 >>= 4_u32; /*"    mov   r1, r1, lsr #4"*/
                print_hex_char!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    print_hex_char"*/
                r1 = ASTMMC_INIT_VER as u32; /*"    mov   r1, #ASTMMC_INIT_VER"*/
                print_hex_char!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    print_hex_char"*/
                r1 = 0x2D_u32; /*"    mov   r1, #0x2D                              @ '-'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e784014_u32; /*"    ldr   r0, =0x1e784014"*/
                State::WaitPrint
            }
            State::WaitPrint => {
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == 0x40_u32; /*"    tst   r1, #0x40"*/
                if z {
                    s = State::WaitPrint;
                    continue;
                } /*"    beq   WaitPrint"*/
                r0 = 0x1e784000_u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x44_u32; /*"    mov   r1, #0x44                              @ 'D'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x44_u32; /*"    mov   r1, #0x44                              @ 'D'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x52_u32; /*"    mov   r1, #0x52                              @ 'R'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Debug - UART console message */

                /******************************************************************************
                Init DRAM common registers
                ******************************************************************************/
                r0 = 0x1e6e0034_u32; /*"    ldr   r0, =0x1e6e0034                        @ disable SDRAM reset"*/
                r1 = 0x00020080_u32; /*"    ldr   r1, =0x00020080"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0008_u32; /*"    ldr   r0, =0x1e6e0008"*/
                r1 = 0x2003000F_u32; /*"    ldr   r1, =0x2003000F                        /* VGA */
"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0038_u32; /*"    ldr   r0, =0x1e6e0038                        @ disable all DRAM requests except CPU during PHY init"*/
                r1 = 0xFFFFEBFF_u32; /*"    ldr   r1, =0xFFFFEBFF"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0040_u32; /*"    ldr   r0, =0x1e6e0040"*/
                r1 = 0x88448844_u32; /*"    ldr   r1, =0x88448844"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0044_u32; /*"    ldr   r0, =0x1e6e0044"*/
                r1 = 0x24422288_u32; /*"    ldr   r1, =0x24422288"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0048_u32; /*"    ldr   r0, =0x1e6e0048"*/
                r1 = 0x22222222_u32; /*"    ldr   r1, =0x22222222"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e004c_u32; /*"    ldr   r0, =0x1e6e004c"*/
                r1 = 0x22222222_u32; /*"    ldr   r1, =0x22222222"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0050_u32; /*"    ldr   r0, =0x1e6e0050"*/
                r1 = 0x80000000_u32; /*"    ldr   r1, =0x80000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r1 = 0x00000000_u32; /*"    ldr   r1, =0x00000000"*/
                r0 = 0x1e6e0208_u32; /*"    ldr   r0, =0x1e6e0208                        @ PHY Setting"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e0218_u32; /*"    ldr   r0, =0x1e6e0218"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e0220_u32; /*"    ldr   r0, =0x1e6e0220"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e0228_u32; /*"    ldr   r0, =0x1e6e0228"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e0230_u32; /*"    ldr   r0, =0x1e6e0230"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e02a8_u32; /*"    ldr   r0, =0x1e6e02a8"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e02b0_u32; /*"    ldr   r0, =0x1e6e02b0"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0240_u32; /*"    ldr   r0, =0x1e6e0240"*/
                r1 = 0x86000000_u32; /*"    ldr   r1, =0x86000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0244_u32; /*"    ldr   r0, =0x1e6e0244"*/
                r1 = 0x00008600_u32; /*"    ldr   r1, =0x00008600"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0248_u32; /*"    ldr   r0, =0x1e6e0248"*/
                r1 = 0x80000000_u32; /*"    ldr   r1, =0x80000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e024c_u32; /*"    ldr   r0, =0x1e6e024c"*/
                r1 = 0x80808080_u32; /*"    ldr   r1, =0x80808080"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Check DRAM Type by H/W Trapping */
                r0 = 0x1e6e2070_u32; /*"    ldr   r0, =0x1e6e2070"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r2 = 0x01000000_u32; /*"    ldr   r2, =0x01000000                        @ bit[24]=1 => DDR4"*/
                z = r1 == r2; /*"    tst   r1, r2"*/
                if !z {
                    s = State::Ddr4Init;
                    continue;
                } /*"    bne   Ddr4Init"*/
                s = State::Ddr3Init;
                continue; /*"    b     Ddr3Init"*/

                /******************************************************************************
                DDR3 Init
                ******************************************************************************/
            }
            State::Ddr3Init => {
                /* Debug - UART console message */
                r0 = 0x1e784000_u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x33_u32; /*"    mov   r1, #0x33                              @ '3'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x0D_u32; /*"    mov   r1, #0x0D                              @ '\\r'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x0A_u32; /*"    mov   r1, #0x0A                              @ '\\n'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Debug - UART console message */

                if CONFIG_DRAM_1333 == 1 {
                    // #if   defined (CONFIG_DRAM_1333)
                    tptr = ramtable::TIME_TABLE_DDR3_1333 /*"    adrl  r5, TIME_TABLE_DDR3_1333               @ Init DRAM parameter table"*/
                } else {
                    // #else tptr = ramtable::TIME_TABLE_DDR3_1600/*"    adrl  r5, TIME_TABLE_DDR3_1600"*/
                } // #endif

                r0 = 0x1e6e0004_u32; /*"    ldr   r0, =0x1e6e0004"*/
                if CONFIG_DDR3_8GSTACK == 1 {
                    // #ifdef CONFIG_DDR3_8GSTACK
                    r1 = 0x00000323_u32; /*"    ldr   r1, =0x00000323                        @ Init to 8GB stack"*/
                } else {
                    // #else 	r1 = 0x00000303 as u32;/*"    ldr   r1, =0x00000303                        @ Init to 8GB"*/
                } // #endif
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0010_u32; /*"    ldr   r0, =0x1e6e0010"*/
                r1 = tptr[(ASTMMC_REGIDX_010 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_010]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0014_u32; /*"    ldr   r0, =0x1e6e0014"*/
                r1 = tptr[(ASTMMC_REGIDX_014 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_014]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0018_u32; /*"    ldr   r0, =0x1e6e0018"*/
                r1 = tptr[(ASTMMC_REGIDX_018 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_018]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* DRAM Mode Register Setting */
                r0 = 0x1e6e0020_u32; /*"    ldr   r0, =0x1e6e0020                        @ MRS_4/6"*/
                r1 = tptr[(ASTMMC_REGIDX_020 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_020]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0024_u32; /*"    ldr   r0, =0x1e6e0024                        @ MRS_5"*/
                r1 = tptr[(ASTMMC_REGIDX_024 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_024]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e002c_u32; /*"    ldr   r0, =0x1e6e002c                        @ MRS_0/2"*/
                r1 = tptr[(ASTMMC_REGIDX_02C as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_02C]"*/
                r2 = 0x1_u32; /*"    mov   r2, #0x1"*/
                r1 |= r2 << 8_u32; /*"    orr   r1, r1, r2, lsl #8"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0030_u32; /*"    ldr   r0, =0x1e6e0030                        @ MRS_1/3"*/
                r1 = tptr[(ASTMMC_REGIDX_030 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_030]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Start DDR PHY Setting */
                r0 = 0x1e6e0200_u32; /*"    ldr   r0, =0x1e6e0200"*/
                r1 = 0x02492AAE_u32; /*"    ldr   r1, =0x02492AAE"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0204_u32; /*"    ldr   r0, =0x1e6e0204"*/
                if CONFIG_DDR3_8GSTACK == 1 {
                    // #ifdef CONFIG_DDR3_8GSTACK
                    r1 = 0x10001001_u32; /*"    ldr   r1, =0x10001001"*/
                } else {
                    // #else 	r1 = 0x00001001 as u32;/*"    ldr   r1, =0x00001001"*/
                } // #endif
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e020c_u32; /*"    ldr   r0, =0x1e6e020c"*/
                r1 = 0x55E00B0B_u32; /*"    ldr   r1, =0x55E00B0B"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0210_u32; /*"    ldr   r0, =0x1e6e0210"*/
                r1 = 0x20000000_u32; /*"    ldr   r1, =0x20000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0214_u32; /*"    ldr   r0, =0x1e6e0214"*/
                r1 = tptr[(ASTMMC_REGIDX_214 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_214]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02e0_u32; /*"    ldr   r0, =0x1e6e02e0"*/
                r1 = tptr[(ASTMMC_REGIDX_2E0 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_2E0]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02e4_u32; /*"    ldr   r0, =0x1e6e02e4"*/
                r1 = tptr[(ASTMMC_REGIDX_2E4 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_2E4]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02e8_u32; /*"    ldr   r0, =0x1e6e02e8"*/
                r1 = tptr[(ASTMMC_REGIDX_2E8 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_2E8]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02ec_u32; /*"    ldr   r0, =0x1e6e02ec"*/
                r1 = tptr[(ASTMMC_REGIDX_2EC as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_2EC]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02f0_u32; /*"    ldr   r0, =0x1e6e02f0"*/
                r1 = tptr[(ASTMMC_REGIDX_2F0 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_2F0]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02f4_u32; /*"    ldr   r0, =0x1e6e02f4"*/
                r1 = tptr[(ASTMMC_REGIDX_2F4 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_2F4]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02f8_u32; /*"    ldr   r0, =0x1e6e02f8"*/
                r1 = tptr[(ASTMMC_REGIDX_2F8 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_2F8]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0290_u32; /*"    ldr   r0, =0x1e6e0290"*/
                r1 = 0x00100008_u32; /*"    ldr   r1, =0x00100008"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02c0_u32; /*"    ldr   r0, =0x1e6e02c0"*/
                r1 = 0x00000006_u32; /*"    ldr   r1, =0x00000006"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Controller Setting */
                r0 = 0x1e6e0060_u32; /*"    ldr   r0, =0x1e6e0060                        @ Fire DDRPHY Init"*/
                r1 = 0x00000005_u32; /*"    ldr   r1, =0x00000005"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0034_u32; /*"    ldr   r0, =0x1e6e0034"*/
                r1 = 0x00020091_u32; /*"    ldr   r1, =0x00020091"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Debug - UART console message */
                r0 = 0x1e784000_u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x30_u32; /*"    mov   r1, #0x30                              @ '0'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Debug - UART console message */

                r0 = 0x1e6e0120_u32; /*"    ldr   r0, =0x1e6e0120"*/
                r1 = 0x00_u32; /*"    mov   r1, #0x00"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                s = State::DdrPhyInitProcess;
                continue; /*"    b     DdrPhyInitProcess"*/
            }
            State::Ddr3PhyinitDone => {
                /********************************************
                 Check Read training margin
                ********************************************/
                r0 = 0x1e6e03a0_u32; /*"    ldr   r0, =0x1e6e03a0                        @ check Gate Training Pass Window"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r2 = 0x150_u32; /*"    ldr   r2, =0x150"*/
                r0 = r1 & !0xFF000000 as u32; /*"    bic   r0, r1, #0xFF000000"*/
                r0 &= !0x00FF0000 as u32; /*"    bic   r0, r0, #0x00FF0000"*/
                z = r0 == r2;

                lt = r0 < r2; /*"    cmp   r0, r2"*/
                if lt {
                    s = State::DdrTestFail;
                    continue;
                } /*"    blt   DdrTestFail"*/
                r0 = r1 >> 16_u32; /*"    mov   r0, r1, lsr #16"*/
                z = r0 == r2;

                lt = r0 < r2; /*"    cmp   r0, r2"*/
                if lt {
                    s = State::DdrTestFail;
                    continue;
                } /*"    blt   DdrTestFail"*/

                r0 = 0x1e6e03d0_u32; /*"    ldr   r0, =0x1e6e03d0                        @ check Read Data Eye Training Pass Window"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r2 = 0x90_u32; /*"    ldr   r2, =0x90"*/
                r0 = r1 & !0x0000FF00 as u32; /*"    bic   r0, r1, #0x0000FF00"*/
                z = r0 == r2;

                lt = r0 < r2; /*"    cmp   r0, r2"*/
                if lt {
                    s = State::DdrTestFail;
                    continue;
                } /*"    blt   DdrTestFail"*/
                r0 = r1 >> 8_u32; /*"    mov   r0, r1, lsr #8"*/
                z = r0 == r2;

                lt = r0 < r2; /*"    cmp   r0, r2"*/
                if lt {
                    s = State::DdrTestFail;
                    continue;
                } /*"    blt   DdrTestFail"*/
                /*******************************************/

                /* Debug - UART console message */
                r0 = 0x1e784000_u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x31_u32; /*"    mov   r1, #0x31                              @ '1'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Debug - UART console message */

                r0 = 0x1e6e000c_u32; /*"    ldr   r0, =0x1e6e000c"*/
                r1 = 0x00000040_u32; /*"    ldr   r1, =0x00000040"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                if CONFIG_DDR3_8GSTACK == 1 {
                    // #ifdef CONFIG_DDR3_8GSTACK
                    r0 = 0x1e6e0028_u32; /*"    ldr   r0, =0x1e6e0028"*/
                    r1 = 0x00000025_u32; /*"    ldr   r1, =0x00000025"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/

                    r0 = 0x1e6e0028_u32; /*"    ldr   r0, =0x1e6e0028"*/
                    r1 = 0x00000027_u32; /*"    ldr   r1, =0x00000027"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/

                    r0 = 0x1e6e0028_u32; /*"    ldr   r0, =0x1e6e0028"*/
                    r1 = 0x00000023_u32; /*"    ldr   r1, =0x00000023"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/

                    r0 = 0x1e6e0028_u32; /*"    ldr   r0, =0x1e6e0028"*/
                    r1 = 0x00000021_u32; /*"    ldr   r1, =0x00000021"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                } // #endif

                r0 = 0x1e6e0028_u32; /*"    ldr   r0, =0x1e6e0028"*/
                r1 = 0x00000005_u32; /*"    ldr   r1, =0x00000005"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0028_u32; /*"    ldr   r0, =0x1e6e0028"*/
                r1 = 0x00000007_u32; /*"    ldr   r1, =0x00000007"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0028_u32; /*"    ldr   r0, =0x1e6e0028"*/
                r1 = 0x00000003_u32; /*"    ldr   r1, =0x00000003"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0028_u32; /*"    ldr   r0, =0x1e6e0028"*/
                r1 = 0x00000011_u32; /*"    ldr   r1, =0x00000011"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e000c_u32; /*"    ldr   r0, =0x1e6e000c"*/
                r1 = 0x00005C41_u32; /*"    ldr   r1, =0x00005C41"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0034_u32; /*"    ldr   r0, =0x1e6e0034"*/
                r2 = 0x70000000_u32; /*"    ldr   r2, =0x70000000"*/
                State::Ddr3CheckDllrdy
            }
            State::Ddr3CheckDllrdy => {
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == r2; /*"    tst   r1, r2"*/
                if !z {
                    s = State::Ddr3CheckDllrdy;
                    continue;
                } /*"    bne   Ddr3CheckDllrdy"*/

                r0 = 0x1e6e000c_u32; /*"    ldr   r0, =0x1e6e000c"*/
                if CONFIG_DRAM_EXT_TEMP == 1 {
                    // #ifdef CONFIG_DRAM_EXT_TEMP
                    r1 = 0x42AA2F81_u32; /*"    ldr   r1, =0x42AA2F81"*/
                } else {
                    // #else 	r1 = 0x42AA5C81 as u32;/*"    ldr   r1, =0x42AA5C81"*/
                } // #endif
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0034_u32; /*"    ldr   r0, =0x1e6e0034"*/
                r1 = 0x0001AF93_u32; /*"    ldr   r1, =0x0001AF93"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0120_u32; /*"    ldr   r0, =0x1e6e0120                        @ VGA Compatible Mode"*/
                r1 = tptr[(ASTMMC_REGIDX_PLL as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_PLL]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                s = State::CalibrationEnd;
                continue; /*"    b     CalibrationEnd"*/
                /******************************************************************************
                End DDR3 Init
                ******************************************************************************/
                /******************************************************************************
                DDR4 Init
                ******************************************************************************/
            }
            State::Ddr4Init => {
                /* Debug - UART console message */
                r0 = 0x1e784000_u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x34_u32; /*"    mov   r1, #0x34                              @ '4'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x0D_u32; /*"    mov   r1, #0x0D                              @ '\\r'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x0A_u32; /*"    mov   r1, #0x0A                              @ '\\n'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Debug - UART console message */

                if CONFIG_DRAM_1333 == 1 {
                    // #if   defined (CONFIG_DRAM_1333)
                    tptr = ramtable::TIME_TABLE_DDR4_1333 /*"    adrl  r5, TIME_TABLE_DDR4_1333               @ Init DRAM parameter table"*/
                } else {
                    // #else tptr = ramtable::TIME_TABLE_DDR4_1600/*"    adrl  r5, TIME_TABLE_DDR4_1600"*/
                } // #endif

                r0 = 0x1e6e0004_u32; /*"    ldr   r0, =0x1e6e0004"*/
                if CONFIG_DDR4_4GX8 == 1 {
                    // #ifdef CONFIG_DDR4_4GX8
                    r1 = 0x00002313_u32; /*"    ldr   r1, =0x00002313                        @ Init to 8GB"*/
                } else {
                    // #else 	r1 = 0x00000313 as u32;/*"    ldr   r1, =0x00000313                        @ Init to 8GB"*/
                } // #endif
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0010_u32; /*"    ldr   r0, =0x1e6e0010"*/
                r1 = tptr[(ASTMMC_REGIDX_010 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_010]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0014_u32; /*"    ldr   r0, =0x1e6e0014"*/
                r1 = tptr[(ASTMMC_REGIDX_014 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_014]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0018_u32; /*"    ldr   r0, =0x1e6e0018"*/
                r1 = tptr[(ASTMMC_REGIDX_018 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_018]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* DRAM Mode Register Setting */
                r0 = 0x1e6e0020_u32; /*"    ldr   r0, =0x1e6e0020                        @ MRS_4/6"*/
                r1 = tptr[(ASTMMC_REGIDX_020 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_020]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0024_u32; /*"    ldr   r0, =0x1e6e0024                        @ MRS_5"*/
                r1 = tptr[(ASTMMC_REGIDX_024 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_024]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e002c_u32; /*"    ldr   r0, =0x1e6e002c                        @ MRS_0/2"*/
                r1 = tptr[(ASTMMC_REGIDX_02C as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_02C]"*/
                r2 = 0x1_u32; /*"    mov   r2, #0x1"*/
                r1 |= r2 << 8_u32; /*"    orr   r1, r1, r2, lsl #8"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0030_u32; /*"    ldr   r0, =0x1e6e0030                        @ MRS_1/3"*/
                r1 = tptr[(ASTMMC_REGIDX_030 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_030]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Start DDR PHY Setting */
                r0 = 0x1e6e0200_u32; /*"    ldr   r0, =0x1e6e0200"*/
                r1 = 0x42492AAE_u32; /*"    ldr   r1, =0x42492AAE"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0204_u32; /*"    ldr   r0, =0x1e6e0204"*/
                r1 = 0x09002800_u32; /*"    ldr   r1, =0x09002800"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e020c_u32; /*"    ldr   r0, =0x1e6e020c"*/
                r1 = 0x55E00B0B_u32; /*"    ldr   r1, =0x55E00B0B"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0210_u32; /*"    ldr   r0, =0x1e6e0210"*/
                r1 = 0x20000000_u32; /*"    ldr   r1, =0x20000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0214_u32; /*"    ldr   r0, =0x1e6e0214"*/
                r1 = tptr[(ASTMMC_REGIDX_214 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_214]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02e0_u32; /*"    ldr   r0, =0x1e6e02e0"*/
                r1 = tptr[(ASTMMC_REGIDX_2E0 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_2E0]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02e4_u32; /*"    ldr   r0, =0x1e6e02e4"*/
                r1 = tptr[(ASTMMC_REGIDX_2E4 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_2E4]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02e8_u32; /*"    ldr   r0, =0x1e6e02e8"*/
                r1 = tptr[(ASTMMC_REGIDX_2E8 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_2E8]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02ec_u32; /*"    ldr   r0, =0x1e6e02ec"*/
                r1 = tptr[(ASTMMC_REGIDX_2EC as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_2EC]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02f0_u32; /*"    ldr   r0, =0x1e6e02f0"*/
                r1 = tptr[(ASTMMC_REGIDX_2F0 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_2F0]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02f4_u32; /*"    ldr   r0, =0x1e6e02f4"*/
                r1 = tptr[(ASTMMC_REGIDX_2F4 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_2F4]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02f8_u32; /*"    ldr   r0, =0x1e6e02f8"*/
                r1 = tptr[(ASTMMC_REGIDX_2F8 as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_2F8]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0290_u32; /*"    ldr   r0, =0x1e6e0290"*/
                r1 = 0x00100008_u32; /*"    ldr   r1, =0x00100008"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02c4_u32; /*"    ldr   r0, =0x1e6e02c4"*/
                r1 = 0x3C183C3C_u32; /*"    ldr   r1, =0x3C183C3C"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e02c8_u32; /*"    ldr   r0, =0x1e6e02c8"*/
                r1 = 0x00631E0E_u32; /*"    ldr   r1, =0x00631E0E"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0034_u32; /*"    ldr   r0, =0x1e6e0034"*/
                r1 = 0x0001A991_u32; /*"    ldr   r1, =0x0001A991"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Debug - UART console message */
                r0 = 0x1e784000_u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x30_u32; /*"    mov   r1, #0x30                              @ '0'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Debug - UART console message */

                /********************************************
                Set Ron value to manual mode
                Target to fix DDR CK Vix issue
                Set Ron_pu = 0, Ron_pd = trained value
                *******************************************/
                if ASTMMC_DDR4_MANUAL_RPU == 1 {
                    // #ifdef ASTMMC_DDR4_MANUAL_RPU
                    r0 = 0x1e6e02c0_u32; /*"    ldr   r0, =0x1e6e02c0"*/
                    r1 = 0x00001806_u32; /*"    ldr   r1, =0x00001806"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e6e02cc_u32; /*"    ldr   r0, =0x1e6e02cc"*/
                    r1 = 0x00005050_u32; /*"    ldr   r1, =0x00005050"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e6e0120_u32; /*"    ldr   r0, =0x1e6e0120"*/
                    r1 = 0x04_u32; /*"    mov   r1, #0x04"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e6e0060_u32; /*"    ldr   r0, =0x1e6e0060                        @ Fire DDRPHY Init"*/
                    r1 = 0x05_u32; /*"    mov   r1, #0x05"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    s = State::DdrPhyInitProcess;
                    continue; /*"    b     DdrPhyInitProcess"*/
                } // #endif // place here by ron

                State::Ddr4RonPhyinitDone
            }
            State::Ddr4RonPhyinitDone => {
                r0 = 0x1e6e0300_u32; /*"    ldr   r0, =0x1e6e0300                        @ read calibrated Ron_pd"*/
                r3 = peek(r0); /*"    ldr   r3, [r0]"*/
                r3 &= !0xFFFFFF0F as u32; /*"    bic   r3, r3, #0xFFFFFF0F"*/
                r0 = 0x1e6e0240_u32; /*"    ldr   r0, =0x1e6e0240"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 &= !0xFF000000 as u32; /*"    bic   r1, r1, #0xFF000000"*/
                r2 = ASTMMC_DDR4_MANUAL_RPU as u32; /*"    mov   r2, #ASTMMC_DDR4_MANUAL_RPU"*/
                r1 |= r2 << 24_u32; /*"    orr   r1, r1, r2, lsl #24"*/
                if ASTMMC_DDR4_MANUAL_RPD == 1 {
                    // #ifdef ASTMMC_DDR4_MANUAL_RPD
                    r2 = ASTMMC_DDR4_MANUAL_RPD as u32; /*"    mov   r2, #ASTMMC_DDR4_MANUAL_RPD"*/
                    r1 |= r2 << 28_u32; /*"    orr   r1, r1, r2, lsl #28"*/
                } else {
                    // #else r1 = r1 | (r3 << 24 as u32);/*"    orr   r1, r1, r3, lsl #24"*/
                } // #endif
                r1 |= 0x02_u32; /*"    orr   r1, r1, #0x02"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0060_u32; /*"    ldr   r0, =0x1e6e0060                        @ Reset PHY"*/
                r1 = 0x00_u32; /*"    mov   r1, #0x00"*/
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
                r0 = 0x1e720000_u32; /*"    ldr   r0, =0x1e720000                        @ retry count"*/
                r1 = 0x5_u32; /*"    mov   r1, #0x5"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                State::Ddr4VrefPhyCalStart
            }
            State::Ddr4VrefPhyCalStart => {
                r7 = 0x0_u32; /*"    mov   r7, #0x0"*/
                r8 = 0x0_u32; /*"    mov   r8, #0x0"*/
                r10 = 0x3F_u32; /*"    mov   r10, #0x3F"*/

                r0 = 0x1e720000_u32; /*"    ldr   r0, =0x1e720000"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 -= 0x01_u32; /*"    subs  r1, r1, #0x01"*/
                if z {
                    s = State::DdrTestFail;
                    continue;
                } /*"    beq   DdrTestFail"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0120_u32; /*"    ldr   r0, =0x1e6e0120"*/
                r1 = 0x00000001_u32; /*"    ldr   r1, =0x00000001"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Debug - UART console message */
                r0 = 0x1e784000_u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x61_u32; /*"    mov   r1, #0x61                              @ 'a'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Debug - UART console message */

                r0 = 0x1e6e02c0_u32; /*"    ldr   r0, =0x1e6e02c0"*/
                r1 = 0x00001C06_u32; /*"    ldr   r1, =0x00001C06"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                State::Ddr4VrefPhyLoop
            }
            State::Ddr4VrefPhyLoop => {
                r0 = 0x1e6e0060_u32; /*"    ldr   r0, =0x1e6e0060"*/
                r1 = 0x00000000_u32; /*"    ldr   r1, =0x00000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r10 += 0x01_u32; /*"    add   r10, r10, #0x01"*/
                z = r10 == 0x80_u32;

                if z {
                    s = State::Ddr4VrefPhyTestFail;
                    continue;
                } /*"    beq   Ddr4VrefPhyTestFail                @ no valid margin and retry"*/

                r0 = 0x1e6e02cc_u32; /*"    ldr   r0, =0x1e6e02cc"*/
                r1 = r10 | (r10 << 8_u32); /*"    orr   r1, r10, r10, lsl #8"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0060_u32; /*"    ldr   r0, =0x1e6e0060"*/
                r1 = 0x00000005_u32; /*"    ldr   r1, =0x00000005"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                s = State::DdrPhyInitProcess;
                continue; /*"    b     DdrPhyInitProcess"*/
            }
            State::Ddr4VrefPhyPhyinitDone => {
                s = State::CbrTestStart;
                continue; /*"    b     CbrTestStart"*/
            }
            State::Ddr4VrefPhyCbrtestDone => {
                r0 = 0x1e6e03d0_u32; /*"    ldr   r0, =0x1e6e03d0                        @ read eye pass window"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r0 = 0x1e720000_u32; /*"    ldr   r0, =0x1e720000"*/
                r0 += r10; /*"    add   r0, r0, r10, lsl #2"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                z = r9 == 0x01_u32;

                if !z {
                    s = State::Ddr4VrefPhyTestFail;
                    continue;
                } /*"    bne   Ddr4VrefPhyTestFail"*/
                r8 += 0x01_u32; /*"    add   r8, r8, #0x01"*/
                r0 = 0x1e6e03d0_u32; /*"    ldr   r0, =0x1e6e03d0                        @ read eye pass window"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r2 = r1 >> 8_u32; /*"    mov   r2, r1, lsr #8                         @ r2 = DQH"*/
                r1 |= 0xFF_u32; /*"    and   r1, r1, #0xFF                          @ r1 = DQL"*/

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
                s = State::Ddr4VrefPhyLoop;
                continue; /*"    b     Ddr4VrefPhyLoop"*/
            }
            State::Ddr4VrefPhyTestFail => {
                z = r8 == 0x0_u32;

                if !z {
                    s = State::Ddr4VrefPhyLoopEnd;
                    continue;
                } /*"    bne   Ddr4VrefPhyLoopEnd"*/
                z = r10 == 0x80_u32;

                if z {
                    s = State::Ddr4VrefPhyCalStart;
                    continue;
                } /*"    beq   Ddr4VrefPhyCalStart"*/
                s = State::Ddr4VrefPhyLoop;
                continue; /*"    b     Ddr4VrefPhyLoop"*/
            }
            State::Ddr4VrefPhyLoopEnd => {
                z = r8 == 16_u32;

                lt = r8 < 16_u32; /*"    cmp   r8, #16                                @ check phyvref margin >= 16"*/
                if lt {
                    s = State::DdrTestFail;
                    continue;
                } /*"    blt   DdrTestFail"*/
                r0 = 0x1e6e02cc_u32; /*"    ldr   r0, =0x1e6e02cc"*/
                r1 = r6 | (r6 << 8_u32); /*"    orr   r1, r6, r6, lsl #8"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e720010_u32; /*"    ldr   r0, =0x1e720010"*/
                r1 = r6 | (r7 << 8_u32); /*"    orr   r1, r6, r7, lsl #8"*/
                r1 |= r8 << 16_u32; /*"    orr   r1, r1, r8, lsl #16"*/
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
                r0 = 0x1e720000_u32; /*"    ldr   r0, =0x1e720000                        @ retry count"*/
                r1 = 0x5_u32; /*"    mov   r1, #0x5"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                State::Ddr4VrefDdrCalStart
            }
            State::Ddr4VrefDdrCalStart => {
                r6 = 0xFF_u32; /*"    mov   r6, #0xFF"*/
                r7 = 0x0_u32; /*"    mov   r7, #0x0"*/
                r8 = 0x0_u32; /*"    mov   r8, #0x0"*/
                r10 = 0x0_u32; /*"    mov   r10, #0x0"*/

                r0 = 0x1e720000_u32; /*"    ldr   r0, =0x1e720000"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 -= 0x01_u32; /*"    subs  r1, r1, #0x01"*/
                if z {
                    s = State::DdrTestFail;
                    continue;
                } /*"    beq   DdrTestFail"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0120_u32; /*"    ldr   r0, =0x1e6e0120"*/
                r1 = 0x00000002_u32; /*"    ldr   r1, =0x00000002"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Debug - UART console message */
                r0 = 0x1e784000_u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x62_u32; /*"    mov   r1, #0x62                              @ 'b'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Debug - UART console message */

                State::Ddr4VrefDdrLoop
            }
            State::Ddr4VrefDdrLoop => {
                r0 = 0x1e6e0060_u32; /*"    ldr   r0, =0x1e6e0060"*/
                r1 = 0x00000000_u32; /*"    ldr   r1, =0x00000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r10 += 0x01_u32; /*"    add   r10, r10, #0x01"*/
                z = r10 == 0x40_u32;

                if z {
                    s = State::Ddr4VrefDdrTestFail;
                    continue;
                } /*"    beq   Ddr4VrefDdrTestFail                @ no valid margin and retry"*/

                r0 = 0x1e6e02c0_u32; /*"    ldr   r0, =0x1e6e02c0"*/
                r1 = 0x06_u32; /*"    mov   r1, #0x06"*/
                r1 |= r10 << 8_u32; /*"    orr   r1, r1, r10, lsl #8"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0060_u32; /*"    ldr   r0, =0x1e6e0060"*/
                r1 = 0x00000005_u32; /*"    ldr   r1, =0x00000005"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                s = State::DdrPhyInitProcess;
                continue; /*"    b     DdrPhyInitProcess"*/
            }
            State::Ddr4VrefDdrPhyinitDone => {
                s = State::CbrTestStart;
                continue; /*"    b     CbrTestStart"*/
            }
            State::Ddr4VrefDdrCbrtestDone => {
                z = r9 == 0x01_u32;

                if !z {
                    s = State::Ddr4VrefDdrTestFail;
                    continue;
                } /*"    bne   Ddr4VrefDdrTestFail"*/
                r8 += 0x01_u32; /*"    add   r8, r8, #0x01"*/

                gt = r6 > r10;

                if gt {
                    r6 = r10;
                } /*"    movgt r6, r10"*/
                z = r7 == r10;

                lt = r7 < r10; /*"    cmp   r7, r10"*/
                if lt {
                    r7 = r10;
                } /*"    movlt r7, r10"*/
                s = State::Ddr4VrefDdrLoop;
                continue; /*"    b     Ddr4VrefDdrLoop"*/
            }
            State::Ddr4VrefDdrTestFail => {
                z = r8 == 0x0_u32;

                if !z {
                    s = State::Ddr4VrefDdrLoopEnd;
                    continue;
                } /*"    bne   Ddr4VrefDdrLoopEnd"*/
                z = r10 == 0x40_u32;

                if z {
                    s = State::Ddr4VrefDdrCalStart;
                    continue;
                } /*"    beq   Ddr4VrefDdrCalStart"*/
                s = State::Ddr4VrefDdrLoop;
                continue; /*"    b     Ddr4VrefDdrLoop"*/
            }
            State::Ddr4VrefDdrLoopEnd => {
                r0 = 0x1e6e0060_u32; /*"    ldr   r0, =0x1e6e0060"*/
                r1 = 0x00000000_u32; /*"    ldr   r1, =0x00000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                z = r8 == 16_u32;

                lt = r8 < 16_u32; /*"    cmp   r8, #16                                @ check ddrvref margin >= 16"*/
                if lt {
                    s = State::DdrTestFail;
                    continue;
                } /*"    blt   DdrTestFail"*/
                r0 = 0x1e6e02c0_u32; /*"    ldr   r0, =0x1e6e02c0"*/
                r1 = r6 + r7; /*"    add   r1, r6, r7"*/
                r1 += 0x01_u32; /*"    add   r1, r1, #0x01"*/
                r2 = r1 >> 1_u32; /*"    mov   r2, r1, lsr #1"*/
                r1 = r2 << 8_u32; /*"    mov   r1, r2, lsl #8"*/
                r1 |= 0x06_u32; /*"    orr   r1, r1, #0x06"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e720014_u32; /*"    ldr   r0, =0x1e720014"*/
                r1 = r6 | (r7 << 8_u32); /*"    orr   r1, r6, r7, lsl #8"*/
                r1 |= r8 << 16_u32; /*"    orr   r1, r1, r8, lsl #16"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Debug - UART console message */
                r0 = 0x1e784000_u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x63_u32; /*"    mov   r1, #0x63                              @ 'c'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Debug - UART console message */

                r0 = 0x1e6e0120_u32; /*"    ldr   r0, =0x1e6e0120"*/
                r1 = 0x00000003_u32; /*"    ldr   r1, =0x00000003"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0060_u32; /*"    ldr   r0, =0x1e6e0060                        @ Fire DDRPHY Init"*/
                r1 = 0x00000005_u32; /*"    ldr   r1, =0x00000005"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                s = State::DdrPhyInitProcess;
                continue; /*"    b     DdrPhyInitProcess"*/
            }
            State::Ddr4PhyinitDone => {
                /********************************************
                 Check Read training margin
                ********************************************/
                r0 = 0x1e6e03a0_u32; /*"    ldr   r0, =0x1e6e03a0                        @ check Gate Training Pass Window"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r2 = 0x150_u32; /*"    ldr   r2, =0x150"*/
                r0 = r1 & !0xFF000000 as u32; /*"    bic   r0, r1, #0xFF000000"*/
                r0 &= !0x00FF0000 as u32; /*"    bic   r0, r0, #0x00FF0000"*/
                z = r0 == r2;

                lt = r0 < r2; /*"    cmp   r0, r2"*/
                if lt {
                    s = State::DdrTestFail;
                    continue;
                } /*"    blt   DdrTestFail"*/
                r0 = r1 >> 16_u32; /*"    mov   r0, r1, lsr #16"*/
                z = r0 == r2;

                lt = r0 < r2; /*"    cmp   r0, r2"*/
                if lt {
                    s = State::DdrTestFail;
                    continue;
                } /*"    blt   DdrTestFail"*/

                r0 = 0x1e6e03d0_u32; /*"    ldr   r0, =0x1e6e03d0                        @ check Read Data Eye Training Pass Window"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r2 = 0x90_u32; /*"    ldr   r2, =0x90"*/
                r0 = r1 & !0x0000FF00 as u32; /*"    bic   r0, r1, #0x0000FF00"*/
                z = r0 == r2;

                lt = r0 < r2; /*"    cmp   r0, r2"*/
                if lt {
                    s = State::DdrTestFail;
                    continue;
                } /*"    blt   DdrTestFail"*/
                r0 = r1 >> 8_u32; /*"    mov   r0, r1, lsr #8"*/
                z = r0 == r2;

                lt = r0 < r2; /*"    cmp   r0, r2"*/
                if lt {
                    s = State::DdrTestFail;
                    continue;
                } /*"    blt   DdrTestFail"*/
                /*******************************************/

                /*******************************************/
                /* Debug - UART console message */
                r0 = 0x1e784000_u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x31_u32; /*"    mov   r1, #0x31                              @ '1'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Debug - UART console message */

                r0 = 0x1e6e000c_u32; /*"    ldr   r0, =0x1e6e000c"*/
                if CONFIG_DRAM_EXT_TEMP == 1 {
                    // #ifdef CONFIG_DRAM_EXT_TEMP
                    r1 = 0x42AA2F81_u32; /*"    ldr   r1, =0x42AA2F81"*/
                } else {
                    // #else 	r1 = 0x42AA5C81 as u32;/*"    ldr   r1, =0x42AA5C81"*/
                } // #endif
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0034_u32; /*"    ldr   r0, =0x1e6e0034"*/
                r1 = 0x0001AF93_u32; /*"    ldr   r1, =0x0001AF93"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0120_u32; /*"    ldr   r0, =0x1e6e0120                        @ VGA Compatible Mode"*/
                r1 = tptr[(ASTMMC_REGIDX_PLL as u32) as usize]; /*"    ldr   r1, [r5, #ASTMMC_REGIDX_PLL]"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                s = State::CalibrationEnd;
                continue; /*"    b     CalibrationEnd"*/

                /******************************************************************************
                End DDR4 Init
                ******************************************************************************/
                /******************************************************************************
                Global Process
                ******************************************************************************/
                /********************************************
                 DDRPHY Init Process
                ********************************************/
            }
            State::DdrPhyInitProcess => {
                clear_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    clear_delay_timer"*/
                /* Wait DDR PHY init done - timeout 300 ms */
                r2 = 0x000493E0_u32; /*"    ldr   r2, =0x000493E0                        @ Set Timer3 Reload = 300 ms"*/
                init_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    init_delay_timer"*/
                r3 = 0x1e6e0060_u32; /*"    ldr   r3, =0x1e6e0060"*/
                State::DdrPhyInit
            }
            State::DdrPhyInit => {
                check_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    check_delay_timer"*/
                if z {
                    s = State::DdrPhyInitTimeout;
                    continue;
                } /*"    beq   DdrPhyInitTimeout"*/
                r1 = peek(r3); /*"    ldr   r1, [r3]"*/
                z = r1 == 0x01_u32; /*"    tst   r1, #0x01"*/
                if !z {
                    s = State::DdrPhyInit;
                    continue;
                } /*"    bne   DdrPhyInit"*/

                /* Check DDR PHY init status */
                r0 = 0x1e6e0300_u32; /*"    ldr   r0, =0x1e6e0300"*/
                r2 = 0x000A0000_u32; /*"    ldr   r2, =0x000A0000"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == r2; /*"    tst   r1, r2"*/
                if z {
                    s = State::DdrPhyInitSuccess;
                    continue;
                } /*"    beq   DdrPhyInitSuccess"*/

                State::DdrPhyInitTimeout
            }
            State::DdrPhyInitTimeout => {
                r0 = 0x1e6e0060_u32; /*"    ldr   r0, =0x1e6e0060                        @ Reset PHY"*/
                r1 = 0x00_u32; /*"    mov   r1, #0x00"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Debug - UART console message */
                r0 = 0x1e784000_u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x2E_u32; /*"    mov   r1, #0x2E                              @ '.'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Debug - UART console message */

                clear_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    clear_delay_timer"*/
                /* Delay about 10us */
                r2 = 0x0000000A_u32; /*"    ldr   r2, =0x0000000A                        @ Set Timer3 Reload = 10 us"*/
                init_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    init_delay_timer"*/
                State::DdrPhyInitDelay0
            }
            State::DdrPhyInitDelay0 => {
                check_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    check_delay_timer"*/
                if !z {
                    s = State::DdrPhyInitDelay0;
                    continue;
                } /*"    bne   DdrPhyInitDelay0"*/
                clear_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    clear_delay_timer"*/
                /* end delay 10us */

                r0 = 0x1e6e0060_u32; /*"    ldr   r0, =0x1e6e0060                        @ Fire PHY Init"*/
                r1 = 0x05_u32; /*"    mov   r1, #0x05"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                s = State::DdrPhyInitProcess;
                continue; /*"    b     DdrPhyInitProcess"*/
            }
            State::DdrPhyInitSuccess => {
                clear_delay_timer!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    clear_delay_timer"*/
                r0 = 0x1e6e0060_u32; /*"    ldr   r0, =0x1e6e0060"*/
                r1 = 0x06_u32; /*"    mov   r1, #0x06"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0120_u32; /*"    ldr   r0, =0x1e6e0120"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == 0_u32;

                if z {
                    s = State::Ddr3PhyinitDone;
                    continue;
                } /*"    beq   Ddr3PhyinitDone"*/
                z = r1 == 1_u32;

                if z {
                    s = State::Ddr4VrefPhyPhyinitDone;
                    continue;
                } /*"    beq   Ddr4VrefPhyPhyinitDone"*/
                z = r1 == 2_u32;

                if z {
                    s = State::Ddr4VrefDdrPhyinitDone;
                    continue;
                } /*"    beq   Ddr4VrefDdrPhyinitDone"*/
                if ASTMMC_DDR4_MANUAL_RPU == 1 {
                    // #ifdef ASTMMC_DDR4_MANUAL_RPU
                    z = r1 == 4_u32;

                    if z {
                        s = State::Ddr4RonPhyinitDone;
                        continue;
                    } /*"    beq   Ddr4RonPhyinitDone"*/
                } // #endif
                s = State::Ddr4PhyinitDone;
                continue; /*"    b     Ddr4PhyinitDone"*/

                /********************************************
                 CBRTest
                ********************************************/
            }
            State::CbrTestStart => {
                r0 = 0x1e6e000c_u32; /*"    ldr   r0, =0x1e6e000c"*/
                r1 = 0x00005C01_u32; /*"    ldr   r1, =0x00005C01"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e0074_u32; /*"    ldr   r0, =0x1e6e0074"*/
                r1 = 0x0000FFFF_u32; /*"    ldr   r1, =0x0000FFFF                        @ test size = 64KB"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e007c_u32; /*"    ldr   r0, =0x1e6e007c"*/
                r1 = 0xFF00FF00_u32; /*"    ldr   r1, =0xFF00FF00"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                State::CbrTestSingle
            }
            State::CbrTestSingle => {
                r0 = 0x1e6e0070_u32; /*"    ldr   r0, =0x1e6e0070"*/
                r1 = 0x00000000_u32; /*"    ldr   r1, =0x00000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x00000085_u32; /*"    ldr   r1, =0x00000085"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r3 = 0x3000_u32; /*"    ldr   r3, =0x3000"*/
                //r11 = 0x50000_u32; /*"    ldr   r11, =0x50000"*/
                State::CbrWaitEngineIdle0
            }
            State::CbrWaitEngineIdle0 => {
                //r11 -= 1_u32; /*"    subs  r11, r11, #1"*/
                if z {
                    s = State::CbrTestFail;
                    continue;
                } /*"    beq   CbrTestFail"*/
                r2 = peek(r0); /*"    ldr   r2, [r0]"*/
                z = r2 == r3; /*"    tst   r2, r3                                 @ D[12] = idle bit"*/
                if z {
                    s = State::CbrWaitEngineIdle0;
                    continue;
                } /*"    beq   CbrWaitEngineIdle0"*/

                r0 = 0x1e6e0070_u32; /*"    ldr   r0, =0x1e6e0070                        @ read fail bit status"*/
                r3 = 0x2000_u32; /*"    ldr   r3, =0x2000"*/
                r2 = peek(r0); /*"    ldr   r2, [r0]"*/
                z = r2 == r3; /*"    tst   r2, r3                                 @ D[13] = fail bit"*/
                if !z {
                    s = State::CbrTestFail;
                    continue;
                } /*"    bne   CbrTestFail"*/

                State::CbrTestBurst
            }
            State::CbrTestBurst => {
                r1 = 0x00_u32; /*"    mov   r1, #0x00                              @ initialize loop index, r1 is loop index"*/
                State::CbrTestBurstLoop
            }
            State::CbrTestBurstLoop => {
                r0 = 0x1e6e0070_u32; /*"    ldr   r0, =0x1e6e0070"*/
                r2 = 0x00000000_u32; /*"    ldr   r2, =0x00000000"*/
                poke(r2, r0); /*"    str   r2, [r0]"*/
                r2 = r1 << 3_u32; /*"    mov   r2, r1, lsl #3"*/
                r2 |= 0xC1_u32; /*"    orr   r2, r2, #0xC1                          @ test command = 0xC1 | (datagen << 3)"*/
                poke(r2, r0); /*"    str   r2, [r0]"*/
                r3 = 0x3000_u32; /*"    ldr   r3, =0x3000"*/
                //r11 = 0x20000_u32; /*"    ldr   r11, =0x20000"*/
                State::CbrWaitEngineIdle1
            }
            State::CbrWaitEngineIdle1 => {
                //r11 -= 1_u32; /*"    subs  r11, r11, #1"*/
                if z {
                    s = State::CbrTestFail;
                    continue;
                } /*"    beq   CbrTestFail"*/
                r2 = peek(r0); /*"    ldr   r2, [r0]"*/
                z = r2 == r3; /*"    tst   r2, r3                                 @ D[12] = idle bit"*/
                if z {
                    s = State::CbrWaitEngineIdle1;
                    continue;
                } /*"    beq   CbrWaitEngineIdle1"*/

                r0 = 0x1e6e0070_u32; /*"    ldr   r0, =0x1e6e0070                        @ read fail bit status"*/
                r3 = 0x2000_u32; /*"    ldr   r3, =0x2000"*/
                r2 = peek(r0); /*"    ldr   r2, [r0]"*/
                z = r2 == r3; /*"    tst   r2, r3                                 @ D[13] = fail bit"*/
                if !z {
                    s = State::CbrTestFail;
                    continue;
                } /*"    bne   CbrTestFail"*/

                r1 += 1_u32; /*"    add   r1, r1, #1                             @ increase the test mode index"*/
                z = r1 == 0x04_u32;

                if !z {
                    s = State::CbrTestBurstLoop;
                    continue;
                } /*"    bne   CbrTestBurstLoop"*/

                r0 = 0x1e6e0070_u32; /*"    ldr   r0, =0x1e6e0070"*/
                r1 = 0x00000000_u32; /*"    ldr   r1, =0x00000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r9 = 0x1_u32; /*"    mov   r9, #0x1"*/
                s = State::CbrTestPatternEnd;
                continue; /*"    b     CbrTestPatternEnd                   @ CBRTest() return(1)"*/
            }
            State::CbrTestFail => {
                r0 = 0x1e6e0070_u32; /*"    ldr   r0, =0x1e6e0070"*/
                r1 = 0x00000000_u32; /*"    ldr   r1, =0x00000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r9 = 0x0_u32; /*"    mov   r9, #0x0                               @ CBRTest() return(0)"*/

                State::CbrTestPatternEnd
            }
            State::CbrTestPatternEnd => {
                r0 = 0x1e6e000c_u32; /*"    ldr   r0, =0x1e6e000c"*/
                r1 = 0x00000000_u32; /*"    ldr   r1, =0x00000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e0120_u32; /*"    ldr   r0, =0x1e6e0120"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == 1_u32;

                if z {
                    s = State::Ddr4VrefPhyCbrtestDone;
                    continue;
                } /*"    beq   Ddr4VrefPhyCbrtestDone"*/
                s = State::Ddr4VrefDdrCbrtestDone;
                continue; /*"    b     Ddr4VrefDdrCbrtestDone"*/

                /******************************************************************************
                Other features configuration
                *****************************************************************************/
            }
            State::CalibrationEnd => {
                /*******************************
                     Check DRAM Size
                //Can't find instruction for      1Gb : 0x80000000 ~ 0x87FFFFFF/*"     1Gb : 0x80000000 ~ 0x87FFFFFF"*/
                //Can't find instruction for      2Gb : 0x80000000 ~ 0x8FFFFFFF/*"     2Gb : 0x80000000 ~ 0x8FFFFFFF"*/
                //Can't find instruction for      4Gb : 0x80000000 ~ 0x9FFFFFFF/*"     4Gb : 0x80000000 ~ 0x9FFFFFFF"*/
                //Can't find instruction for      8Gb : 0x80000000 ~ 0xBFFFFFFF/*"     8Gb : 0x80000000 ~ 0xBFFFFFFF"*/
                    *******************************/
                r0 = 0x1e6e0004_u32; /*"    ldr   r0, =0x1e6e0004"*/
                r6 = peek(r0); /*"    ldr   r6, [r0]"*/
                r6 &= !0x00000003 as u32; /*"    bic   r6, r6, #0x00000003                    @ record MCR04"*/
                r7 = tptr[(ASTMMC_REGIDX_RFC as u32) as usize]; /*"    ldr   r7, [r5, #ASTMMC_REGIDX_RFC]"*/

                State::CheckDramSize
            }
            State::CheckDramSize => {
                r0 = 0xA0100000_u32; /*"    ldr   r0, =0xA0100000"*/
                r1 = 0x41424344_u32; /*"    ldr   r1, =0x41424344"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x90100000_u32; /*"    ldr   r0, =0x90100000"*/
                r1 = 0x35363738_u32; /*"    ldr   r1, =0x35363738"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x88100000_u32; /*"    ldr   r0, =0x88100000"*/
                r1 = 0x292A2B2C_u32; /*"    ldr   r1, =0x292A2B2C"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x80100000_u32; /*"    ldr   r0, =0x80100000"*/
                r1 = 0x1D1E1F10_u32; /*"    ldr   r1, =0x1D1E1F10"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0xA0100000_u32; /*"    ldr   r0, =0xA0100000"*/
                r1 = 0x41424344_u32; /*"    ldr   r1, =0x41424344"*/
                r2 = peek(r0); /*"    ldr   r2, [r0]"*/
                z = r2 == r1;

                if z {
                    r6 |= 0x03_u32;
                } /*"    orreq r6, r6, #0x03"*/
                if z {
                    r7 >>= 24_u32;
                } /*"    moveq r7, r7, lsr #24"*/
                r3 = 0x38_u32; /*"    mov   r3, #0x38                              @ '8'"*/
                if z {
                    s = State::CheckDramSizeEnd;
                    continue;
                } /*"    beq   CheckDramSizeEnd"*/
                r0 = 0x90100000_u32; /*"    ldr   r0, =0x90100000"*/
                r1 = 0x35363738_u32; /*"    ldr   r1, =0x35363738"*/
                r2 = peek(r0); /*"    ldr   r2, [r0]"*/
                z = r2 == r1;

                if z {
                    r6 |= 0x02_u32;
                } /*"    orreq r6, r6, #0x02"*/
                if z {
                    r7 >>= 16_u32;
                } /*"    moveq r7, r7, lsr #16"*/
                r3 = 0x34_u32; /*"    mov   r3, #0x34                              @ '4'"*/
                if z {
                    s = State::CheckDramSizeEnd;
                    continue;
                } /*"    beq   CheckDramSizeEnd"*/
                r0 = 0x88100000_u32; /*"    ldr   r0, =0x88100000"*/
                r1 = 0x292A2B2C_u32; /*"    ldr   r1, =0x292A2B2C"*/
                r2 = peek(r0); /*"    ldr   r2, [r0]"*/
                z = r2 == r1;

                if z {
                    r6 |= 0x01_u32;
                } /*"    orreq r6, r6, #0x01"*/
                if z {
                    r7 >>= 8_u32;
                } /*"    moveq r7, r7, lsr #8"*/
                r3 = 0x32_u32; /*"    mov   r3, #0x32                              @ '2'"*/
                if z {
                    s = State::CheckDramSizeEnd;
                    continue;
                } /*"    beq   CheckDramSizeEnd"*/
                r3 = 0x31_u32; /*"    mov   r3, #0x31                              @ '1'"*/

                State::CheckDramSizeEnd
            }
            State::CheckDramSizeEnd => {
                r0 = 0x1e6e0004_u32; /*"    ldr   r0, =0x1e6e0004"*/
                poke(r6, r0); /*"    str   r6, [r0]"*/
                r0 = 0x1e6e0014_u32; /*"    ldr   r0, =0x1e6e0014"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 &= !0x000000FF as u32; /*"    bic   r1, r1, #0x000000FF"*/
                r7 |= 0xFF_u32; /*"    and   r7, r7, #0xFF"*/
                r1 |= r7; /*"    orr   r1, r1, r7"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Version Number */
                r0 = 0x1e6e0004_u32; /*"    ldr   r0, =0x1e6e0004"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r2 = ASTMMC_INIT_VER as u32; /*"    mov   r2, #ASTMMC_INIT_VER"*/
                r1 |= r2 << 20_u32; /*"    orr   r1, r1, r2, lsl #20"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0088_u32; /*"    ldr   r0, =0x1e6e0088"*/
                r1 = ASTMMC_INIT_DATE as u32; /*"    ldr   r1, =ASTMMC_INIT_DATE"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Debug - UART console message */
                r0 = 0x1e784000_u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x2D_u32; /*"    mov   r1, #0x2D                              @ '-'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                poke(r3, r0); /*"    str   r3, [r0]"*/
                r1 = 0x47_u32; /*"    mov   r1, #0x47                              @ 'G'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x62_u32; /*"    mov   r1, #0x62                              @ 'b'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x2D_u32; /*"    mov   r1, #0x2D                              @ '-'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Debug - UART console message */

                /* Enable DRAM Cache */
                r0 = 0x1e6e0004_u32; /*"    ldr   r0, =0x1e6e0004"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r2 = 1_u32; /*"    mov   r2, #1"*/
                r2 = r1 | (r2 << 12_u32); /*"    orr   r2, r1, r2, lsl #12"*/
                poke(r2, r0); /*"    str   r2, [r0]"*/
                r3 = 0x00080000_u32; /*"    ldr   r3, =0x00080000"*/
                State::DramCacheInit
            }
            State::DramCacheInit => {
                r2 = peek(r0); /*"    ldr   r2, [r0]"*/
                z = r2 == r3; /*"    tst   r2, r3"*/
                if z {
                    s = State::DramCacheInit;
                    continue;
                } /*"    beq   DramCacheInit"*/
                r2 = 1_u32; /*"    mov   r2, #1"*/
                r1 |= r2 << 10_u32; /*"    orr   r1, r1, r2, lsl #10"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Set DRAM requests threshold */
                r0 = 0x1e6e001c_u32; /*"    ldr   r0, =0x1e6e001c"*/
                r1 = 0x00000008_u32; /*"    ldr   r1, =0x00000008"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e0038_u32; /*"    ldr   r0, =0x1e6e0038"*/
                r1 = 0xFFFFFF00_u32; /*"    ldr   r1, =0xFFFFFF00"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /********************************************
                 DDRTest
                ********************************************/
                State::DdrTestStart
            }
            State::DdrTestStart => {
                r0 = 0x1e6e0074_u32; /*"    ldr   r0, =0x1e6e0074"*/
                r1 = 0x0000FFFF_u32; /*"    ldr   r1, =0x0000FFFF                        @ test size = 64KB"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e007c_u32; /*"    ldr   r0, =0x1e6e007c"*/
                r1 = 0xFF00FF00_u32; /*"    ldr   r1, =0xFF00FF00"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                State::DdrTestBurst
            }
            State::DdrTestBurst => {
                r1 = 0x00_u32; /*"    mov   r1, #0x00                              @ initialize loop index, r1 is loop index"*/
                State::DdrTestBurstLoop
            }
            State::DdrTestBurstLoop => {
                r0 = 0x1e6e0070_u32; /*"    ldr   r0, =0x1e6e0070"*/
                r2 = 0x00000000_u32; /*"    ldr   r2, =0x00000000"*/
                poke(r2, r0); /*"    str   r2, [r0]"*/
                r2 = r1 << 3_u32; /*"    mov   r2, r1, lsl #3"*/
                r2 |= 0xC1_u32; /*"    orr   r2, r2, #0xC1                          @ test command = 0xC1 | (datagen << 3)"*/
                poke(r2, r0); /*"    str   r2, [r0]"*/
                r3 = 0x3000_u32; /*"    ldr   r3, =0x3000"*/
                //r11 = 0x20000_u32; /*"    ldr   r11, =0x20000"*/
                State::DdrWaitEngineIdle1
            }
            State::DdrWaitEngineIdle1 => {
                //r11 -= 1_u32; /*"    subs  r11, r11, #1"*/
                if z {
                    s = State::DdrTestFail;
                    continue;
                } /*"    beq   DdrTestFail"*/
                r2 = peek(r0); /*"    ldr   r2, [r0]"*/
                z = r2 == r3; /*"    tst   r2, r3                                 @ D[12] = idle bit"*/
                if z {
                    s = State::DdrWaitEngineIdle1;
                    continue;
                } /*"    beq   DdrWaitEngineIdle1"*/

                r0 = 0x1e6e0070_u32; /*"    ldr   r0, =0x1e6e0070                        @ read fail bit status"*/
                r3 = 0x2000_u32; /*"    ldr   r3, =0x2000"*/
                r2 = peek(r0); /*"    ldr   r2, [r0]"*/
                z = r2 == r3; /*"    tst   r2, r3                                 @ D[13] = fail bit"*/
                if !z {
                    s = State::DdrTestFail;
                    continue;
                } /*"    bne   DdrTestFail"*/

                r1 += 1_u32; /*"    add   r1, r1, #1                             @ increase the test mode index"*/
                z = r1 == 0x01_u32;

                if !z {
                    s = State::DdrTestBurstLoop;
                    continue;
                } /*"    bne   DdrTestBurstLoop"*/

                r0 = 0x1e6e0070_u32; /*"    ldr   r0, =0x1e6e0070"*/
                r1 = 0x00000000_u32; /*"    ldr   r1, =0x00000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                s = State::SetScratch;
                continue; /*"    b     SetScratch                            @ CBRTest() return(1)"*/
            }
            State::DdrTestFail => {
                /* Debug - UART console message */
                r0 = 0x1e784000_u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x46_u32; /*"    mov   r1, #0x46                              @ 'F'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x61_u32; /*"    mov   r1, #0x61                              @ 'a'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x69_u32; /*"    mov   r1, #0x69                              @ 'i'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6C_u32; /*"    mov   r1, #0x6C                              @ 'l'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x0D_u32; /*"    mov   r1, #0x0D                              @ '\\r'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x0A_u32; /*"    mov   r1, #0x0A                              @ '\\n'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e784014_u32; /*"    ldr   r0, =0x1e784014"*/
                State::WaitPrint0
            }
            State::WaitPrint0 => {
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == 0x40_u32; /*"    tst   r1, #0x40"*/
                if z {
                    s = State::WaitPrint0;
                    continue;
                } /*"    beq   WaitPrint0"*/
                /* Debug - UART console message */
                s = State::ResetMmc;
                continue; /*"    b     ResetMmc"*/
            }
            State::SetScratch => {
                /*Set Scratch register Bit 6 after ddr initial finished */
                r0 = 0x1e6e2040_u32; /*"    ldr   r0, =0x1e6e2040"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 |= 0x41_u32; /*"    orr   r1, r1, #0x41"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Debug - UART console message */
                r0 = 0x1e784000_u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x44_u32; /*"    mov   r1, #0x44                              @ 'D'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6F_u32; /*"    mov   r1, #0x6F                              @ 'o'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6E_u32; /*"    mov   r1, #0x6E                              @ 'n'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x65_u32; /*"    mov   r1, #0x65                              @ 'e'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x0D_u32; /*"    mov   r1, #0x0D                              @ '\\r'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x0A_u32; /*"    mov   r1, #0x0A                              @ '\\n'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Debug - UART console message */

                /* Enable VGA display */
                r0 = 0x1e6e202c_u32; /*"    ldr   r0, =0x1e6e202c"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 &= !0x40 as u32; /*"    bic   r1, r1, #0x40"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Debug - UART console message */
                /* Print PHY timing information */
                r0 = 0x1e784014_u32; /*"    ldr   r0, =0x1e784014"*/
                State::WaitPrint1
            }
            State::WaitPrint1 => {
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == 0x40_u32; /*"    tst   r1, #0x40"*/
                if z {
                    s = State::WaitPrint1;
                    continue;
                } /*"    beq   WaitPrint1"*/

                r0 = 0x1e784000_u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x52_u32; /*"    mov   r1, #0x52                              @ 'R'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x65_u32; /*"    mov   r1, #0x65                              @ 'e'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x61_u32; /*"    mov   r1, #0x61                              @ 'a'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x64_u32; /*"    mov   r1, #0x64                              @ 'd'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x20_u32; /*"    mov   r1, #0x20                              @ ' '"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6D_u32; /*"    mov   r1, #0x6D                              @ 'm'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x61_u32; /*"    mov   r1, #0x61                              @ 'a'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x72_u32; /*"    mov   r1, #0x72                              @ 'r'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x67_u32; /*"    mov   r1, #0x67                              @ 'g'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x69_u32; /*"    mov   r1, #0x69                              @ 'i'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6E_u32; /*"    mov   r1, #0x6E                              @ 'n'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x2D_u32; /*"    mov   r1, #0x2D                              @ '-'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x44_u32; /*"    mov   r1, #0x44                              @ 'D'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x4C_u32; /*"    mov   r1, #0x4C                              @ 'L'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x3A_u32; /*"    mov   r1, #0x3A                              @ ':'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e784014_u32; /*"    ldr   r0, =0x1e784014"*/
                State::WaitPrint2
            }
            State::WaitPrint2 => {
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == 0x40_u32; /*"    tst   r1, #0x40"*/
                if z {
                    s = State::WaitPrint2;
                    continue;
                } /*"    beq   WaitPrint2"*/

                r7 = 0x000001FE_u32; /*"    ldr   r7, =0x000001FE                        @ divide by 510"*/
                r8 = 10_u32; /*"    mov   r8, #10                                @ multiply by 10"*/
                r9 = 0_u32; /*"    mov   r9, #0                                 @ record violation"*/
                r0 = 0x1e6e0004_u32; /*"    ldr   r0, =0x1e6e0004"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == 0x10_u32; /*"    tst   r1, #0x10                              @ bit[4]=1 => DDR4"*/
                if !z {
                    r10 = 0x9A_u32;
                } /*"    movne r10, #0x9A                             @ DDR4 min = 0x99 (0.30)"*/
                if z {
                    r10 = 0xB3_u32;
                } /*"    moveq r10, #0xB3                             @ DDR3 min = 0xB3 (0.35)"*/
                State::PrintDQLEyeMargin
            }
            State::PrintDQLEyeMargin => {
                r0 = 0x1e6e03d0_u32; /*"    ldr   r0, =0x1e6e03d0"*/
                r2 = peek(r0); /*"    ldr   r2, [r0]"*/
                r2 |= 0xFF_u32; /*"    and   r2, r2, #0xFF"*/
                z = r2 == r10;

                lt = r2 < r10; /*"    cmp   r2, r10                                @ check violation"*/
                if lt {
                    r9 = 1_u32;
                } /*"    movlt r9, #1"*/
                r0 = 0x1e784000_u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x30_u32; /*"    mov   r1, #0x30                              @ '0'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x2E_u32; /*"    mov   r1, #0x2E                              @ '.'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r3 = 0x4_u32; /*"    mov   r3, #0x4                               @ print 4 digits"*/
                State::PrintDQLDivLoop
            }
            State::PrintDQLDivLoop => {
                r2 *= r8; /*"    mul   r2, r8, r2"*/
                z = r2 == r7;

                lt = r2 < r7; /*"    cmp   r2, r7"*/
                if lt {
                    s = State::PrintDQLDiv0;
                    continue;
                } /*"    blt   PrintDQLDiv0"*/
                r6 = 0x0_u32; /*"    mov   r6, #0x0"*/
                State::PrintDQLDivDigit
            }
            State::PrintDQLDivDigit => {
                r2 -= r7; /*"    sub   r2, r2, r7"*/
                r6 += 0x1_u32; /*"    add   r6, r6, #0x1"*/
                z = r2 == r7;
                gt = r2 > r7;

                if gt || z {
                    s = State::PrintDQLDivDigit;
                    continue;
                } /*"    bge   PrintDQLDivDigit"*/
                s = State::PrintDQLDivN;
                continue; /*"    b     PrintDQLDivN"*/
            }
            State::PrintDQLDiv0 => {
                r1 = 0x30_u32; /*"    mov   r1, #0x30                              @ '0'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                s = State::PrintDQLNext;
                continue; /*"    b     PrintDQLNext"*/
            }
            State::PrintDQLDivN => {
                r1 = r6 + 0x30_u32; /*"    add   r1, r6, #0x30                          @ print n"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                State::PrintDQLNext
            }
            State::PrintDQLNext => {
                r3 -= 1_u32; /*"    subs  r3, r3, #1"*/
                if z {
                    s = State::PrintDQHEyeMargin;
                    continue;
                } /*"    beq   PrintDQHEyeMargin"*/
                z = r2 == 0x0_u32;

                if z {
                    s = State::PrintDQHEyeMargin;
                    continue;
                } /*"    beq   PrintDQHEyeMargin"*/
                s = State::PrintDQLDivLoop;
                continue; /*"    b     PrintDQLDivLoop"*/
            }
            State::PrintDQHEyeMargin => {
                r1 = 0x2F_u32; /*"    mov   r1, #0x2F                              @ '/'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x44_u32; /*"    mov   r1, #0x44                              @ 'D'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x48_u32; /*"    mov   r1, #0x48                              @ 'H'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x3A_u32; /*"    mov   r1, #0x3A                              @ ':'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e784014_u32; /*"    ldr   r0, =0x1e784014"*/
                State::WaitPrint3
            }
            State::WaitPrint3 => {
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == 0x40_u32; /*"    tst   r1, #0x40"*/
                if z {
                    s = State::WaitPrint3;
                    continue;
                } /*"    beq   WaitPrint3"*/

                r0 = 0x1e6e03d0_u32; /*"    ldr   r0, =0x1e6e03d0"*/
                r2 = peek(r0); /*"    ldr   r2, [r0]"*/
                r2 >>= 8_u32; /*"    mov   r2, r2, lsr #8"*/
                r2 |= 0xFF_u32; /*"    and   r2, r2, #0xFF"*/
                z = r2 == r10;

                lt = r2 < r10; /*"    cmp   r2, r10                                @ check violation"*/
                if lt {
                    r9 = 1_u32;
                } /*"    movlt r9, #1"*/
                r0 = 0x1e784000_u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x30_u32; /*"    mov   r1, #0x30                              @ '0'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x2E_u32; /*"    mov   r1, #0x2E                              @ '.'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r3 = 0x4_u32; /*"    mov   r3, #0x4                               @ print 4 digits"*/
                State::PrintDQHDivLoop
            }
            State::PrintDQHDivLoop => {
                r2 *= r8; /*"    mul   r2, r8, r2"*/
                z = r2 == r7;

                lt = r2 < r7; /*"    cmp   r2, r7"*/
                if lt {
                    s = State::PrintDQHDiv0;
                    continue;
                } /*"    blt   PrintDQHDiv0"*/
                r6 = 0x0_u32; /*"    mov   r6, #0x0"*/
                State::PrintDQHDivDigit
            }
            State::PrintDQHDivDigit => {
                r2 -= r7; /*"    sub   r2, r2, r7"*/
                r6 += 0x1_u32; /*"    add   r6, r6, #0x1"*/
                z = r2 == r7;
                gt = r2 > r7;

                if gt || z {
                    s = State::PrintDQHDivDigit;
                    continue;
                } /*"    bge   PrintDQHDivDigit"*/
                s = State::PrintDQHDivN;
                continue; /*"    b     PrintDQHDivN"*/
            }
            State::PrintDQHDiv0 => {
                r1 = 0x30_u32; /*"    mov   r1, #0x30                              @ '0'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                s = State::PrintDQHNext;
                continue; /*"    b     PrintDQHNext"*/
            }
            State::PrintDQHDivN => {
                r1 = r6 + 0x30_u32; /*"    add   r1, r6, #0x30                          @ print n"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                State::PrintDQHNext
            }
            State::PrintDQHNext => {
                r3 -= 1_u32; /*"    subs  r3, r3, #1"*/
                if z {
                    s = State::PrintDQEyeMarginLast;
                    continue;
                } /*"    beq   PrintDQEyeMarginLast"*/
                z = r2 == 0x0_u32;

                if z {
                    s = State::PrintDQEyeMarginLast;
                    continue;
                } /*"    beq   PrintDQEyeMarginLast"*/
                s = State::PrintDQHDivLoop;
                continue; /*"    b     PrintDQHDivLoop"*/
            }
            State::PrintDQEyeMarginLast => {
                r1 = 0x20_u32; /*"    mov   r1, #0x20                              @ ' '"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x43_u32; /*"    mov   r1, #0x43                              @ 'C'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x4B_u32; /*"    mov   r1, #0x4B                              @ 'K'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0004_u32; /*"    ldr   r0, =0x1e6e0004"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == 0x10_u32; /*"    tst   r1, #0x10                              @ bit[4]=1 => DDR4"*/
                if !z {
                    r10 = 0x30_u32;
                } /*"    movne r10, #0x30                             @ DDR4 min = 0.30"*/
                if z {
                    r10 = 0x35_u32;
                } /*"    moveq r10, #0x35                             @ DDR4 min = 0.35"*/

                r0 = 0x1e784014_u32; /*"    ldr   r0, =0x1e784014"*/
                State::WaitPrint4
            }
            State::WaitPrint4 => {
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == 0x40_u32; /*"    tst   r1, #0x40"*/
                if z {
                    s = State::WaitPrint4;
                    continue;
                } /*"    beq   WaitPrint4"*/

                r0 = 0x1e784000_u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x20_u32; /*"    mov   r1, #0x20                              @ ' '"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x28_u32; /*"    mov   r1, #0x28                              @ '('"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6D_u32; /*"    mov   r1, #0x6D                              @ 'm'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x69_u32; /*"    mov   r1, #0x69                              @ 'i'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6E_u32; /*"    mov   r1, #0x6E                              @ 'n'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x3A_u32; /*"    mov   r1, #0x3A                              @ ':'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x30_u32; /*"    mov   r1, #0x30                              @ '0'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x2E_u32; /*"    mov   r1, #0x2E                              @ '.'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x33_u32; /*"    mov   r1, #0x33                              @ '3'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                poke(r10, r0); /*"    str   r10, [r0]"*/
                r1 = 0x29_u32; /*"    mov   r1, #0x29                              @ ')'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                z = r9 == 0_u32;

                if z {
                    s = State::PrintDQMarginLast;
                    continue;
                } /*"    beq   PrintDQMarginLast"*/
                r1 = 0x20_u32; /*"    mov   r1, #0x20                              @ ' '"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e784014_u32; /*"    ldr   r0, =0x1e784014"*/
                State::WaitPrint5
            }
            State::WaitPrint5 => {
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == 0x40_u32; /*"    tst   r1, #0x40"*/
                if z {
                    s = State::WaitPrint5;
                    continue;
                } /*"    beq   WaitPrint5"*/

                r0 = 0x1e784000_u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x57_u32; /*"    mov   r1, #0x57                              @ 'W'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x61_u32; /*"    mov   r1, #0x61                              @ 'a'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x72_u32; /*"    mov   r1, #0x72                              @ 'r'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6E_u32; /*"    mov   r1, #0x6E                              @ 'n'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x69_u32; /*"    mov   r1, #0x69                              @ 'i'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6E_u32; /*"    mov   r1, #0x6E                              @ 'n'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x67_u32; /*"    mov   r1, #0x67                              @ 'g'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x3A_u32; /*"    mov   r1, #0x3A                              @ ':'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x20_u32; /*"    mov   r1, #0x20                              @ ' '"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x4D_u32; /*"    mov   r1, #0x4D                              @ 'M'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x61_u32; /*"    mov   r1, #0x61                              @ 'a'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x72_u32; /*"    mov   r1, #0x72                              @ 'r'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x67_u32; /*"    mov   r1, #0x67                              @ 'g'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x69_u32; /*"    mov   r1, #0x69                              @ 'i'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6E_u32; /*"    mov   r1, #0x6E                              @ 'n'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e784014_u32; /*"    ldr   r0, =0x1e784014"*/
                State::WaitPrint6
            }
            State::WaitPrint6 => {
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == 0x40_u32; /*"    tst   r1, #0x40"*/
                if z {
                    s = State::WaitPrint6;
                    continue;
                } /*"    beq   WaitPrint6"*/
                r0 = 0x1e784000_u32; /*"    ldr   r0, =0x1e784000"*/
                r1 = 0x20_u32; /*"    mov   r1, #0x20                              @ ' '"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x74_u32; /*"    mov   r1, #0x74                              @ 't'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6F_u32; /*"    mov   r1, #0x6F                              @ 'o'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6F_u32; /*"    mov   r1, #0x6F                              @ 'o'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x20_u32; /*"    mov   r1, #0x20                              @ ' '"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x73_u32; /*"    mov   r1, #0x73                              @ 's'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6D_u32; /*"    mov   r1, #0x6D                              @ 'm'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x61_u32; /*"    mov   r1, #0x61                              @ 'a'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6C_u32; /*"    mov   r1, #0x6C                              @ 'l'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x6C_u32; /*"    mov   r1, #0x6C                              @ 'l'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                State::PrintDQMarginLast
            }
            State::PrintDQMarginLast => {
                r1 = 0x0D_u32; /*"    mov   r1, #0x0D                              @ '\\r'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r1 = 0x0A_u32; /*"    mov   r1, #0x0A                              @ '\\n'"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Debug - UART console message */

                State::PlatformExit
            }
            State::PlatformExit => {
                if CONFIG_DRAM_ECC == 1 {
                    // #ifdef CONFIG_DRAM_ECC
                    r0 = 0x1e6e0004_u32; /*"    ldr   r0, =0x1e6e0004"*/
                    r2 = 0x00000880_u32; /*"    ldr   r2, =0x00000880                        @ add cache range control, 2016.09.02"*/
                    r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                    r1 |= r2; /*"    orr   r1, r1, r2"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/

                    r0 = 0x1e6e0054_u32; /*"    ldr   r0, =0x1e6e0054"*/
                    r1 = CONFIG_DRAM_ECC_SIZE as u32; /*"    ldr   r1, =CONFIG_DRAM_ECC_SIZE              /* ECC protected memory size */
"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/

                    r0 = 0x1e6e007c_u32; /*"    ldr   r0, =0x1e6e007C"*/
                    r1 = 0x00000000_u32; /*"    ldr   r1, =0x00000000"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/
                    r0 = 0x1e6e0074_u32; /*"    ldr   r0, =0x1e6e0074"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/

                    r0 = 0x1e6e0070_u32; /*"    ldr   r0, =0x1e6e0070"*/
                    r1 = 0x00000221_u32; /*"    ldr   r1, =0x00000221"*/
                    poke(r1, r0); /*"    str   r1, [r0]"*/

                    r2 = 0x00001000_u32; /*"    ldr   r2, =0x00001000"*/
                } // #endif
                s = State::EccInitFlag;
                continue; /*"b EccInitFlag"*/
            }
            State::EccInitFlag => {
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == r2; /*"    tst   r1, r2                                 @ D[12] = 1, Done"*/
                if z {
                    s = State::EccInitFlag;
                    continue;
                } /*"    beq   EccInitFlag"*/

                r1 = 0x00000000_u32; /*"    ldr   r1, =0x00000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0050_u32; /*"    ldr   r0, =0x1e6e0050"*/
                r1 = 0x80000000_u32; /*"    ldr   r1, =0x80000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0050_u32; /*"    ldr   r0, =0x1e6e0050"*/
                r1 = 0x00000000_u32; /*"    ldr   r1, =0x00000000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e0070_u32; /*"    ldr   r0, =0x1e6e0070"*/
                r1 = 0x00000400_u32; /*"    ldr   r1, =0x00000400                        @ Enable ECC auto-scrubbing"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                //#endif

                /******************************************************************************
                SPI Timing Calibration
                ******************************************************************************/
                r2 = 0x0_u32; /*"    mov   r2, #0x0"*/
                r6 = 0x0_u32; /*"    mov   r6, #0x0"*/
                r7 = 0x0_u32; /*"    mov   r7, #0x0"*/
                init_spi_checksum!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    init_spi_checksum"*/
                State::SpiChecksumWait0
            }
            State::SpiChecksumWait0 => {
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == r2; /*"    tst   r1, r2"*/
                if z {
                    s = State::SpiChecksumWait0;
                    continue;
                } /*"    beq   SpiChecksumWait0"*/
                r0 = 0x1e620090_u32; /*"    ldr   r0, =0x1e620090"*/
                r5 = peek(r0); /*"    ldr   r5, [r0]                               @ record golden checksum"*/
                r0 = 0x1e620080_u32; /*"    ldr   r0, =0x1e620080"*/
                r1 = 0x0_u32; /*"    mov   r1, #0x0"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e620010_u32; /*"    ldr   r0, =0x1e620010                        @ set to fast read mode"*/
                r1 = 0x000B0041_u32; /*"    ldr   r1, =0x000B0041"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r6 = 0x00F7E6D0_u32; /*"    ldr   r6, =0x00F7E6D0                        @ Init spiclk loop"*/
                r8 = 0x0_u32; /*"    mov   r8, #0x0                               @ Init delay record"*/

                State::SpiCbrNextClkrate
            }
            State::SpiCbrNextClkrate => {
                r6 >>= 0x4_u32; /*"    mov   r6, r6, lsr #0x4"*/
                z = r6 == 0x0_u32;

                if z {
                    s = State::SpiCbrEnd;
                    continue;
                } /*"    beq   SpiCbrEnd"*/

                r7 = 0x0_u32; /*"    mov   r7, #0x0                               @ Init delay loop"*/
                r8 <<= 4_u32; /*"    mov   r8, r8, lsl #4"*/

                State::SpiCbrNextDelayS
            }
            State::SpiCbrNextDelayS => {
                r2 = 0x8_u32; /*"    mov   r2, #0x8"*/
                init_spi_checksum!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    init_spi_checksum"*/
                State::SpiChecksumWait1
            }
            State::SpiChecksumWait1 => {
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == r2; /*"    tst   r1, r2"*/
                if z {
                    s = State::SpiChecksumWait1;
                    continue;
                } /*"    beq   SpiChecksumWait1"*/
                r0 = 0x1e620090_u32; /*"    ldr   r0, =0x1e620090"*/
                r2 = peek(r0); /*"    ldr   r2, [r0]                               @ read checksum"*/
                r0 = 0x1e620080_u32; /*"    ldr   r0, =0x1e620080"*/
                r1 = 0x0_u32; /*"    mov   r1, #0x0"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                z = r2 == r5;

                if !z {
                    s = State::SpiCbrNextDelayE;
                    continue;
                } /*"    bne   SpiCbrNextDelayE"*/

                r2 = 0x0_u32; /*"    mov   r2, #0x0"*/
                init_spi_checksum!(r0, r1, r2, r3, r4, r5, r6, r7, z, gt, lt); /*"    init_spi_checksum"*/
                State::SpiChecksumWait2
            }
            State::SpiChecksumWait2 => {
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                z = r1 == r2; /*"    tst   r1, r2"*/
                if z {
                    s = State::SpiChecksumWait2;
                    continue;
                } /*"    beq   SpiChecksumWait2"*/
                r0 = 0x1e620090_u32; /*"    ldr   r0, =0x1e620090"*/
                r2 = peek(r0); /*"    ldr   r2, [r0]                               @ read checksum"*/
                r0 = 0x1e620080_u32; /*"    ldr   r0, =0x1e620080"*/
                r1 = 0x0_u32; /*"    mov   r1, #0x0"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                z = r2 == r5;

                if !z {
                    s = State::SpiCbrNextDelayE;
                    continue;
                } /*"    bne   SpiCbrNextDelayE"*/

                r8 |= r7; /*"    orr   r8, r8, r7                             @ record passed delay"*/
                s = State::SpiCbrNextClkrate;
                continue; /*"    b     SpiCbrNextClkrate"*/
            }
            State::SpiCbrNextDelayE => {
                r7 += 0x1_u32; /*"    add   r7, r7, #0x1"*/
                z = r7 == 0x6_u32;

                lt = r7 < 0x6_u32; /*"    cmp   r7, #0x6"*/
                if lt {
                    s = State::SpiCbrNextDelayS;
                    continue;
                } /*"    blt   SpiCbrNextDelayS"*/
                s = State::SpiCbrNextClkrate;
                continue; /*"    b     SpiCbrNextClkrate"*/
            }
            State::SpiCbrEnd => {
                r0 = 0x1e620094_u32; /*"    ldr   r0, =0x1e620094"*/
                poke(r8, r0); /*"    str   r8, [r0]"*/
                r0 = 0x1e620010_u32; /*"    ldr   r0, =0x1e620010"*/
                r1 = 0x0_u32; /*"    mov   r1, #0x0"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /******************************************************************************
                Miscellaneous Setting
                ******************************************************************************/
                /* Set UART DMA as AHB high priority master */
                r0 = 0x1e600000_u32; /*"    ldr   r0, =0x1e600000"*/
                r1 = 0xAEED1A03_u32; /*"    ldr   r1, =0xAEED1A03"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e600080_u32; /*"    ldr   r0, =0x1e600080"*/
                r2 = 0x100_u32; /*"    ldr   r2, =0x100"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 |= r2; /*"    orr   r1, r1, r2"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Enable UART3/4 clock and disable LHCLK */
                r0 = 0x1e6e200c_u32; /*"    ldr   r0, =0x1e6e200c"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r2 = 0xF9FFFFFF_u32; /*"    ldr   r2, =0xF9FFFFFF"*/
                r1 |= r2; /*"    and   r1, r1, r2"*/
                r2 = 0x10000000_u32; /*"    ldr   r2, =0x10000000"*/
                r1 |= r2; /*"    orr   r1, r1, r2"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e2008_u32; /*"    ldr   r0, =0x1e6e2008                        @ Set Video ECLK phase"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r2 = 0x0ffffff3_u32; /*"    ldr   r2, =0x0ffffff3"*/
                r1 |= r2; /*"    and   r1, r1, r2"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e2004_u32; /*"    ldr r0, =0x1e6e2004                          @ Enable JTAG Master, solve ARM stucked by JTAG issue"*/
                r1 = peek(r0); /*"    ldr r1, [r0]"*/
                r1 &= !0x00400000 as u32; /*"    bic r1, r1, #0x00400000"*/
                poke(r1, r0); /*"    str r1, [r0]"*/

                /******************************************************************************
                Configure MAC timing
                ******************************************************************************/
                /* Enable D2PLL and set to 250MHz */
                r0 = 0x1e6e213c_u32; /*"    ldr   r0, =0x1e6e213c"*/
                r1 = 0x00000585_u32; /*"    ldr   r1, =0x00000585                        @ Reset D2PLL"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e202c_u32; /*"    ldr   r0, =0x1e6e202c"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 &= !0x10 as u32; /*"    bic   r1, r1, #0x10                          @ Enable D2PLL"*/
                r2 = 0x00200000_u32; /*"    ldr   r2, =0x00200000                        @ Set CRT = 40MHz"*/
                r1 |= r2; /*"    orr   r1, r1, r2"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r2 = 0x8E00A17C_u32; /*"    ldr   r2, =0x8E00A17C                        @ Set to 250MHz"*/

                r0 = 0x1e6e2070_u32; /*"    ldr   r0, =0x1e6e2070                        @ Check CLKIN = 25MHz"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 >>= 23_u32; /*"    mov   r1, r1, lsr #23"*/
                z = r1 == 0x01_u32; /*"    tst   r1, #0x01"*/
                if z {
                    s = State::SetD2PLL;
                    continue;
                } /*"    beq   SetD2PLL"*/
                r2 = 0x8E00A177_u32; /*"    ldr   r2, =0x8E00A177"*/

                State::SetD2PLL
            }
            State::SetD2PLL => {
                r0 = 0x1e6e201c_u32; /*"    ldr   r0, =0x1e6e201c"*/
                poke(r2, r0); /*"    str   r2, [r0]"*/
                r0 = 0x1e6e213c_u32; /*"    ldr   r0, =0x1e6e213c                        @ Enable D2PLL"*/
                r1 = 0x00000580_u32; /*"    ldr   r1, =0x00000580"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e204c_u32; /*"    ldr   r0, =0x1e6e204c"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 &= !0xFF0000 as u32; /*"    bic   r1, r1, #0xFF0000"*/
                r2 = 0x00040000_u32; /*"    ldr   r2, =0x00040000                        @ Set divider ratio"*/
                r1 |= r2; /*"    orr   r1, r1, r2"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e2048_u32; /*"    ldr   r0, =0x1e6e2048                        @ Set MAC interface delay timing = 1G"*/
                r1 = 0x80082208_u32; /*"    ldr   r1, =0x80082208                        @ Select internal 125MHz"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e20b8_u32; /*"    ldr   r0, =0x1e6e20b8                        @ Set MAC interface delay timing = 100M"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e6e20bc_u32; /*"    ldr   r0, =0x1e6e20bc                        @ Set MAC interface delay timing = 10M"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e2070_u32; /*"    ldr   r0, =0x1e6e2070                        @ Set MAC AHB bus clock"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r2 = 0x04_u32; /*"    mov   r2, #0x04                              @ Default RMII, set MHCLK = HPLL/10"*/
                z = r1 == 0xC0_u32; /*"    tst   r1, #0xC0"*/
                if !z {
                    r2 = 0x02_u32;
                } /*"    movne r2, #0x02                              @ if RGMII,     set MHCLK = HPLL/6"*/
                r0 = 0x1e6e2008_u32; /*"    ldr   r0, =0x1e6e2008"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 &= !0x00070000 as u32; /*"    bic   r1, r1, #0x00070000"*/
                r1 |= r2 << 16_u32; /*"    orr   r1, r1, r2, lsl #16"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e21dc_u32; /*"    ldr   r0, =0x1e6e21dc                        @ Set MAC duty"*/
                r1 = 0x00666400_u32; /*"    ldr   r1, =0x00666400"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                r0 = 0x1e6e2090_u32; /*"    ldr   r0, =0x1e6e2090                        @ Enable MAC interface pull low"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r1 &= !0x0000F000 as u32; /*"    bic   r1, r1, #0x0000F000"*/
                r1 &= !0x20000000 as u32; /*"    bic   r1, r1, #0x20000000                    @ Set USB portA as Device mode"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Test - DRAM initial time */
                r0 = 0x1e782040_u32; /*"    ldr   r0, =0x1e782040"*/
                r1 = peek(r0); /*"    ldr   r1, [r0]"*/
                r0 = 0xFFFFFFFF_u32; /*"    ldr   r0, =0xFFFFFFFF"*/
                r1 = r0 - r1; /*"    sub   r1, r0, r1"*/
                r0 = 0x1e6e008c_u32; /*"    ldr   r0, =0x1e6e008c"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                r0 = 0x1e78203c_u32; /*"    ldr   r0, =0x1e78203c"*/
                r1 = 0x0000F000_u32; /*"    ldr   r1, =0x0000F000"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/
                /* Test - DRAM initial time */

                r0 = 0x1e6e0000_u32; /*"    ldr   r0, =0x1e6e0000                        @ disable MMC password"*/
                r1 = 0x0_u32; /*"    mov   r1, #0x0"*/
                poke(r1, r0); /*"    str   r1, [r0]"*/

                /* Disable Timer separate mode */
                r0 = 0x1e782038_u32; /*"    ldr   r0, =0x1e782038"*/
                r1 = 0xEA_u32; /*"    ldr   r1, =0xEA"*/
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
    InitDram = 0,
    StartFirstReset = 1,
    WaitFirstReset = 2,
    BypassFirstReset = 3,
    WaitUsbInit = 4,
    BypassUSBInit = 5,
    BypassMpllHynixMode1 = 6,
    BypassMpllHynixMode2 = 7,
    SetMPLL = 8,
    WaitMpllInit = 9,
    ResetMmc = 10,
    WaitMmcReset = 11,
    WaitMmcResetDone = 12,
    WaitDdrReset = 13,
    WaitPrint = 14,
    Ddr3Init = 15,
    Ddr3PhyinitDone = 16,
    Ddr3CheckDllrdy = 17,
    Ddr4Init = 18,
    Ddr4RonPhyinitDone = 19,
    Ddr4VrefPhyCalStart = 20,
    Ddr4VrefPhyLoop = 21,
    Ddr4VrefPhyPhyinitDone = 22,
    Ddr4VrefPhyCbrtestDone = 23,
    Ddr4VrefPhyTestFail = 24,
    Ddr4VrefPhyLoopEnd = 25,
    Ddr4VrefDdrCalStart = 26,
    Ddr4VrefDdrLoop = 27,
    Ddr4VrefDdrPhyinitDone = 28,
    Ddr4VrefDdrCbrtestDone = 29,
    Ddr4VrefDdrTestFail = 30,
    Ddr4VrefDdrLoopEnd = 31,
    Ddr4PhyinitDone = 32,
    DdrPhyInitProcess = 33,
    DdrPhyInit = 34,
    DdrPhyInitTimeout = 35,
    DdrPhyInitDelay0 = 36,
    DdrPhyInitSuccess = 37,
    CbrTestStart = 38,
    CbrTestSingle = 39,
    CbrWaitEngineIdle0 = 40,
    CbrTestBurst = 41,
    CbrTestBurstLoop = 42,
    CbrWaitEngineIdle1 = 43,
    CbrTestFail = 44,
    CbrTestPatternEnd = 45,
    CalibrationEnd = 46,
    CheckDramSize = 47,
    CheckDramSizeEnd = 48,
    DramCacheInit = 49,
    DdrTestStart = 50,
    DdrTestBurst = 51,
    DdrTestBurstLoop = 52,
    DdrWaitEngineIdle1 = 53,
    DdrTestFail = 54,
    WaitPrint0 = 55,
    SetScratch = 56,
    WaitPrint1 = 57,
    WaitPrint2 = 58,
    PrintDQLEyeMargin = 59,
    PrintDQLDivLoop = 60,
    PrintDQLDivDigit = 61,
    PrintDQLDiv0 = 62,
    PrintDQLDivN = 63,
    PrintDQLNext = 64,
    PrintDQHEyeMargin = 65,
    WaitPrint3 = 66,
    PrintDQHDivLoop = 67,
    PrintDQHDivDigit = 68,
    PrintDQHDiv0 = 69,
    PrintDQHDivN = 70,
    PrintDQHNext = 71,
    PrintDQEyeMarginLast = 72,
    WaitPrint4 = 73,
    WaitPrint5 = 74,
    WaitPrint6 = 75,
    PrintDQMarginLast = 76,
    PlatformExit = 77,
    EccInitFlag = 78,
    SpiChecksumWait0 = 79,
    SpiCbrNextClkrate = 80,
    SpiCbrNextDelayS = 81,
    SpiChecksumWait1 = 82,
    SpiChecksumWait2 = 83,
    SpiCbrNextDelayE = 84,
    SpiCbrEnd = 85,
    SetD2PLL = 86,
    UartSetup,
    PowerOn,
    Exit,
}
