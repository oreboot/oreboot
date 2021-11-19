#![no_std]

use tock_registers::interfaces::{Readable, Writeable};
use tock_registers::register_bitfields;
use tock_registers::registers::{ReadOnly, ReadWrite};
/*
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program; if not, write to the Free Software
 *  Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 */
/*
 * Board specific setup info
 *
 ******************************************************************************
 * ASPEED Technology Inc.
 * AST25x0 DDR3/DDR4 SDRAM controller initialization sequence
 *
 * Gary Hsu, <gary_hsu@aspeedtech.com>
 *
 * Version     : 18
 * Release date: 2017.10.27
 *
 * Priority of fix item:
 * [P1] = critical
 * [P2] = nice to have
 * [P3] = minor
 *
 * Change List :
 * V2 |2014.07.25 : 1.[P1] Modify HPLL config sequence
 * V2 |2014.07.30 : 1.[P1] Modify DDR3 AC parameters table
 *    |             2.[P1] Turn on ZQCS mode
 * V2 |2014.08.13 : 1.[P1] Add disable XDMA
 * V2 |2014.09.09 : 1.[P1] Disable CKE dynamic power down
 * V2 |2014.10.31 : 1.[P2] Enable VGA wide screen support (SCU40[0]=1)
 * V2 |2015.03.26 : 1.[P1] Revise AC timing table
 *    |             2.[P1] Add check code to bypass A0 patch
 *    |             3.[P1] Add MPLL parameter of A1
 *    |             4.[P1] Set X-DMA into VGA memory domain
 * V2 |2015.04.24 : 1.[P1] Add disabling all DRAM requests during PHY init
 *    |             2.[P1] Set MCR1C & MCR38
 * V3 |2015.05.13 : 1.[P1] Modify DDR4 PHY Vref training algorithm
 *    |             2.[P2] Enable CKE dynamic power down
 * V4 |2015.06.15 : 1.[P1] Add MAC timing setting
 * V5 |2015.07.09 : 1.[P1] Modify MHCLK divider ratio
 *    |             2.[P2] Add DDR read margin report
 * V6 |2015.08.13 : 1.[P3] Disable MMC password before exit
 * V6 |2015.08.24 : 1.[P1] Fix SCU160 parameter value for CLKIN=25MHz condition
 * V7 |2015.09.18 : 1.[P1] Clear AHB bus lock condition at power up time
 *    |             2.[P1] Add reset MMC controller to solve init DRAM again during VGA ON
 * V7 |2015.09.22 : 1.[P1] Add watchdog full reset for resolving reset incomplete issue at fast reset condition
 *    |             2.[P1] Add DRAM stress test after train complete, and redo DRAM initial if stress fail
 *    |             3.[P2] Enable JTAG master mode
 *    |             4.[P2] Add DDR4 Vref trainig retry timeout
 * V8 |2015.11.02 : 1.[P2] Clear software strap flag before doing watchdog full reset
 *    |2015.12.10 : 1.[P1] Add USB PHY initial code
 *    |2016.01.27 : 1.[P3] Modify the first reset from full chip reset to SOC reset
 *    |             2.[P3] Remove HPLL/MPLL patch code for revision A0
 *    |             3.[P2] Move the reset_mmc code to be after MPLL initialized
 * V9 |2016.02.19 : 1.[P3] Remove definition "CONFIG_FIRMWARE_2ND_BOOT"
 * V10|2016.04.21 : 1.[P1] Add USB PHY initial code - port B, to prevent wrong state on USB pins
 * V11|2016.05.10 : 1.[P3] Add DRAM Extended temperature range support
 * V12|2016.06.24 : 1.[P1] Modify LPC Reset input source when eSPI mode enabled
 *    |2016.07.12 : 2.[P1] Modify DDR4 read path ODT from 60 ohm to 48 ohm, at address 0x1e6e0204
 *    |           : 3.[P1] Modify DDR4 Ron calibration to manual mode to fix Vix issue, set Ron_pu = 0
 *    |           : 4.[P2] Modify read timing margin report policy, change DDR4 min value from 0.35 to 0.3. Add "Warning" while violated.
 * V13|2016.08.29 : 1.[P3] Add option to route debug message output port from UART5 to UART1
 *    |2016.09.02 : 2.[P2] Add range control for cache function when ECC enabled
 *    |2016.09.06 : 3.[P1] Enable full mask setting for first SOC reset, since the coverage of original default setting is not enough
 * V14|2016.10.25 : 1.[P2] Change Ron manual calibration to default OFF, customer can enable it to do fine-tuning of the Vix issue
 *    |2016.11.07 : 2.[P3] Add log information of DDR4 PHY Vref training
 * V15|2017.04.06 : 1.[P1] Modify USB portA initial sequence, this is to prevent DMA lock condition of USB Virtual Hub device for some chips.
 *    |2017.04.13 : 2.[P2] Add initial sequence for LPC controller
 * V16|2017.06.15 : 1.[P1] Add margin check/retry for DDR4 Vref training margin.
 *    |2017.06.15 : 2.[P1] Add margin check/retry for DDR3/DDR4 read timing training margin.
 *    |2017.06.19 : 3.[P2] Add initial sequence for LPC controller
 *    |2017.06.19 : 4.[P2] Add initial full-chip reset option
 *    |2017.06.19 : 5.[P3] Add 10ms delay after DDR reset
 * V17|2017.09.25 : 1.[P1] Modify DDR4 side ODT value from 60ohm to 48ohm.
 *    |2017.09.25 : 2.[P1] Add Hynix DDR4 frequency slow down option.
 * V18|2017.10.26 : 1.[P3] Include the modification of DDR4 side ODT value in V17 into the option of Hynix DDR4 configuration.
 *    |2017.10.26 : 2.[P2] Enhance initial sequence for LPC controller
 * Note: Read timing report is only a reference, it is not a solid rule for stability.
 *
 * Optional define variable
 * 1. DRAM Speed                  //
 *    CONFIG_DRAM_1333            //
 *    CONFIG_DRAM_1600            // (default)
 * 2. ECC Function enable
 *    CONFIG_DRAM_ECC             //const to enable ECC: u32 = function;
 *    CONFIG_DRAM_ECC_SIZE        //const the ECC protected memory: u32 = size;
 * 3. UART5 message output        //
 *    CONFIG_DRAM_UART_38400      // set the UART baud rate to 38400, default is 115200
 *    CONFIG_DRAM_UART_TO_UART1   // route UART5 to UART port1
 * 4. DRAM Type
 *    CONFIG_DDR3_8GSTACK         // DDR3 8Gbit Stack die
 *    CONFIG_DDR4_4GX8            // DDR4 4Gbit X8 dual part
 * 5. Firmware 2nd boot flash
 *    CONFIG_FIRMWARE_2ND_BOOT (Removed)
 * 6. Enable DRAM extended temperature range mode
 *    CONFIG_DRAM_EXT_TEMP
 * 7. Select WDT_Full mode for power up initial reset
 *    ASTMMC_INIT_RESET_MODE_FULL
 * 8. Hynix DDR4 options
 *    CONFIG_DDR4_SUPPORT_HYNIX   // Enable this when Hynix DDR4 included in the BOM
 *    CONFIG_DDR4_HYNIX_SET_1536
 *    CONFIG_DDR4_HYNIX_SET_1488
 *    CONFIG_DDR4_HYNIX_SET_1440  // Default
 ******************************************************************************
 */

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

static TIME_TABLE_DDR3_1333: [u32; 17] = [
    0x53503C37, //       @ 0x010
    0xF858D47F, //       @ 0x014
    0x00010000, //       @ 0x018
    0x00000000, //       @ 0x020
    0x00000000, //       @ 0x024
    0x02101C60, //       @ 0x02C
    0x00000040, //       @ 0x030
    0x00000020, //       @ 0x214
    0x02001000, //       @ 0x2E0
    0x0C000085, //       @ 0x2E4
    0x000BA018, //       @ 0x2E8
    0x2CB92104, //       @ 0x2EC
    0x07090407, //       @ 0x2F0
    0x81000700, //       @ 0x2F4
    0x0C400800, //       @ 0x2F8
    0x7F5E3A27, //       @ tRFC
    0x00005B80, //       @ PLL
];
static TIME_TABLE_DDR3_1600: [u32; 17] = [
    0x64604D38, //       @ 0x010
    0x29690599, //       @ 0x014
    0x00000300, //       @ 0x018
    0x00000000, //       @ 0x020
    0x00000000, //       @ 0x024
    0x02181E70, //       @ 0x02C
    0x00000040, //       @ 0x030
    0x00000024, //       @ 0x214
    0x02001300, //       @ 0x2E0
    0x0E0000A0, //       @ 0x2E4
    0x000E001B, //       @ 0x2E8
    0x35B8C105, //       @ 0x2EC
    0x08090408, //       @ 0x2F0
    0x9B000800, //       @ 0x2F4
    0x0E400A00, //       @ 0x2F8
    0x9971452F, //       @ tRFC
    0x000071C1, //       @ PLL
];
static TIME_TABLE_DDR4_1333: [u32; 17] = [
    0x53503D26, //       @ 0x010
    0xE878D87F, //       @ 0x014
    0x00019000, //       @ 0x018
    0x08000000, //       @ 0x020
    0x00000400, //       @ 0x024
    0x00000200, //       @ 0x02C
    0x00000101, //       @ 0x030
    0x00000020, //       @ 0x214
    0x03002200, //       @ 0x2E0
    0x0C000085, //       @ 0x2E4
    0x000BA01A, //       @ 0x2E8
    0x2CB92106, //       @ 0x2EC
    0x07060606, //       @ 0x2F0
    0x81000700, //       @ 0x2F4
    0x0C400800, //       @ 0x2F8
    0x7F5E3A3A, //       @ tRFC
    0x00005B80, //       @ PLL
];
static TIME_TABLE_DDR4_1600: [u32; 17] = [
    0x63604E37, //       @ 0x010
    0xE97AFA99, //       @ 0x014
    0x00019000, //       @ 0x018
    0x08000000, //       @ 0x020
    0x00000400, //       @ 0x024
    0x00000410, //       @ 0x02C
    //#ifdef CONFIG_DDR5_SUPPORT_HYNIX
    //	0x030     , //        @ ODT = 48 ohm
    //#else
    0x030, //        @ ODT = 60 ohm
    //#endif
    0x00000024, //       @ 0x214
    0x03002900, //       @ 0x2E0
    0x0E0000A0, //       @ 0x2E4
    0x000E001C, //       @ 0x2E8
    0x35B8C106, //       @ 0x2EC
    0x08080607, //       @ 0x2F0
    0x9B000900, //       @ 0x2F4
    0x0E400A00, //       @ 0x2F8
    0x99714545, //       @ tRFC
    0x000071C1, //       @ PLL
];

// These register are all mixed up for now. First things first:
// get dram to work at all
// The fun part is the registers are all over the place,
// so the advantage of a struct is not so obvious as I thought.
// Don't expose this dreck!
// Also, this is not a based register set, there is only
// one set per SoC. For that reason, for now, I'm going to
// skip the register block, since it is giant swathes of
// padding with the occasional register.
// We, further, don't export this stuff since people should not
// go at it open-coding style.

#[repr(C)]
struct TimerReload {
    RELOAD: ReadWrite<u32>,
}
// const TimerReloadBase: StaticRef<TimerReload> = unsafe { StaticRef::new(0x1e78_2024 as *const TimerReload) };

#[repr(C)]
struct TimerEnable {
    ENABLE: ReadWrite<u32>,
}
// const TimerEnableBase: StaticRef<TimerEnable> = unsafe { StaticRef::new(0x1e78_2030 as *const TimerEnable) };

#[repr(C)]
struct TimerControl {
    ENABLE: ReadWrite<u32>,
}
// const TimerControlBase: StaticRef<TimerControl> = unsafe { StaticRef::new(0x1e78_2030 as *const TimerControl) };

#[repr(C)]
struct ISRClear {
    CLEAR: ReadWrite<u32>,
}
// const ISRClearBase: StaticRef<ISRClear> = unsafe { StaticRef::new(0x1e6c_0038 as *const ISRClear) };

#[repr(C)]
struct ISRStatus {
    STATUS: ReadWrite<u32>,
}
// const ISRStatusBase: StaticRef<ISRStatus> = unsafe { StaticRef::new(0x1e6c_0090 as *const ISRStatus) };

struct Timer {
    ISR: ISR,
}

impl Timer {
    pub fn new() -> Timer {
        Timer { ISR: ISR::new() }
    }

    // The question: can this fit into a read-write world?
    pub fn set(&self, t: u32) -> u32 {
        let r = 0x1e78_2024 as *const TimerReload;
        unsafe { (*r).RELOAD.set(t as u32) };
        self.ISR.clear(0x40000);
        self.start(7);
        self.ISR.status()
    }

    pub fn start(&self, t: u32) {
        let r = 0x1e78_2030 as *const TimerEnable;
        let v = t << 8;
        unsafe { (*r).ENABLE.set(t as u32) };
    }

    pub fn done(&self) -> bool {
        let v = self.ISR.status();
        v & 0x40000 == 0x40000
    }

    pub fn enable(&self, t: u32) {
        let r = 0x1e78_2030 as *const TimerEnable;
        unsafe { (*r).ENABLE.set(t as u32) };
    }

    pub fn clear(&self) {
        self.enable(0xf << 8);
        self.ISR.clear(0x40000);
    }
}

#[repr(C)]
struct ISR {}

impl ISR {
    pub fn new() -> ISR {
        ISR {}
    }

    // The question: can this fit into a read-write world?
    pub fn clear(&self, t: u32) {
        let r = 0x1e6c_0038 as *const ISRClear;
        unsafe { (*r).CLEAR.set(t as u32) };
    }

    pub fn status(&self) -> u32 {
        let r = 0x1e6c_0090 as *const ISRStatus;
        unsafe { (*r).STATUS.get() }
    }
}

pub struct Ram {}

impl Ram {
    pub fn new() -> Ram {
        Ram {}
    }
}

pub fn ram() {}
