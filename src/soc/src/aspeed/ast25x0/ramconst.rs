#![allow(unused)] // todo: use this module

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

const ASTMMC_REGIDX_010: u32 = 0x00;
const ASTMMC_REGIDX_014: u32 = 0x04;
const ASTMMC_REGIDX_018: u32 = 0x08;
const ASTMMC_REGIDX_020: u32 = 0x0C;
const ASTMMC_REGIDX_024: u32 = 0x10;
const ASTMMC_REGIDX_02C: u32 = 0x14;
const ASTMMC_REGIDX_030: u32 = 0x18;
const ASTMMC_REGIDX_214: u32 = 0x1C;
const ASTMMC_REGIDX_2E0: u32 = 0x20;
const ASTMMC_REGIDX_2E4: u32 = 0x24;
const ASTMMC_REGIDX_2E8: u32 = 0x28;
const ASTMMC_REGIDX_2EC: u32 = 0x2C;
const ASTMMC_REGIDX_2F0: u32 = 0x30;
const ASTMMC_REGIDX_2F4: u32 = 0x34;
const ASTMMC_REGIDX_2F8: u32 = 0x38;
const ASTMMC_REGIDX_RFC: u32 = 0x3C;
const ASTMMC_REGIDX_PLL: u32 = 0x40;
