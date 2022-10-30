/*!

  # SD Card for D1

  ## D1 Specific

  * Datasheet says: "SD v3.0, SDIO v3.0, eMMC v5.0"
  * 3 Host Controller interfaces
    * SMHC0 => SD mem-version v3.0 (for SD Card) Base Address is 0x0402_0000 (SMHC0_BASE)
      * SDC0-CMD
      * SDC0-CLK
      * SDC0-D[3:0]
      * SDC0-RST
    * SMHC1 => SDIO v3 (SDIO WiFi)
      * SDC1-CMD
      * SDC1-CLK
      * SDC1-D[3:0]
    * SMHC2 => eMMC v5 (eMMC)
      * SDC2-CMD
      * SDC2-CLK
      * SDC2-D[3:0]
  * Performance:
    * SDR => 150 Mhz @ 1.8 volt IO Pad 
    * DDR => 100 Mhz @ 1.8 volt IO Pad 
    * DDR =>  50 Mhz @ 3.3 volt IO Pad
  * 1 or 4 bit data width
  * 1-65535 bytes block size
  *  1024 bytes Rx & TX FIFO
  * Interrupts => card insert/remove
  * CRC generation + checking
  * descriptor-based DMA controller (DMAC aka IDMAC). See 7.2.3.9.
   * A patch for DMA is not yet upstream:
<https://github.com/orangecms/linux/commit/512f5679c938d09eb623ef629e1a9ddc9b15a587>

  ## ClockworkPi Specific

  * D1
    * Clockwork Core R01
      * Clockwork Main
        * TF-Card (SMHC0)
        * On/Off
        * GPIOs
        * (etc)
        * Clockwork Ext
          * UART
          * Printer
          * Camera
          * 2x USB
          * Fan
          * (etc)
  
  ## Actors

  * memory card
  * memory card slot
  * host controller interface
    * 
  *

  ## FSM:

  Host Controller <- Reset

  Card Eject -> Stop Polling
  Card Insert -> Start Polling

  Poll ->
    * Card Info capacity
    * Detect Capacity


  # References
  * <https://github.com/DongshanPI/Awesome_RISCV-AllwinnerD1/blob/master/Tina-SDK/Hardware%E7%A1%AC%E4%BB%B6%E7%B1%BB%E6%96%87%E6%A1%A3/%E8%8A%AF%E7%89%87%E6%89%8B%E5%86%8C/D1-H_Datasheet_V1.0.pdf>
  * <https://github.com/DongshanPI/Awesome_RISCV-AllwinnerD1/blob/master/Tina-SDK/Hardware%E7%A1%AC%E4%BB%B6%E7%B1%BB%E6%96%87%E6%A1%A3/%E8%8A%AF%E7%89%87%E6%89%8B%E5%86%8C/D1-H_User%20Manual_V1.0.pdf>
  * <https://github.com/orangecms/linux/blob/5.19-smaeul-plus-dts/drivers/mmc/host/sunxi-mmc.c>
  * <https://gitlab.com/pnru/xv6-d1>

*/

/* ======= General SD stuff ===========

/// Constuct a new HCI
fn hci_new();

/// Bring up a host controller from unknown state
fn hci_init() {};

/// Issue a command to the host controller
fn issue_command();

/// 
fn read();

/// Card Detect
///
*/


const MMC_STATUS_MASK: u32 = (!0x0206BF7F);
const MMC_STATUS_RDY_FOR_DATA: u32 = (1 << 8);

const CMD_GO_IDLE_STATE: u32 = 0;
const CMD_SEND_OP_COND: u32 = 1;
const CMD_ALL_SEND_CID: u32 = 2;
const CMD_SEND_RELATIVE_ADDR: u32 = 3;
const CMD_SWITCH_FUNC: u32 = 6;
const CMD_SELECT_CARD: u32 = 7;
const CMD_SEND_IF_COND: u32 = 8;
const CMD_SEND_CSD: u32 = 9;
const CMD_SEND_CID: u32 = 10;
const CMD_STOP_TRANSMISSION: u32 = 12;
const CMD_SEND_STATUS: u32 = 13;
const CMD_READ_SINGLE_BLOCK: u32 = 17;
const CMD_READ_MULTIPLE_BLOCK: u32 = 18;
const CMD_WRITE_SINGLE_BLOCK: u32 = 24;
const CMD_WRITE_MULTIPLE_BLOC: u32 = 25;
const CMD_APP_CMD: u32 = 55;
const CMD_SPI_READ_OCR: u32 = 58;

const CMDA_SET_BUS_WIDTH: u32 = 6;


//! SMHC base address. See section 7.5.2 of D1 manual.

/// SD mem-version v3.0 (for SD Card)
/// NOTE: Oreboot only cares about this SMHC0 for the D1
const SMHC0_BASE: usize = 0x0402_0000;
/// SDIO Device (ex. WiFi on Clockwork Pi Main board)
const SMHC1_BASE: usize = 0x0402_1000;
/// eMMC v5 device
const SMHC2_BASE: usize = 0x0402_2000;

//! SMHC0_BASE register offset definitions. See section 7.5.2 and
//! <https://github.com/orangecms/linux/blob/5.19-smaeul-plus-dts/drivers/mmc/host/sunxi-mmc.c>.
//! 
//! For convenience the #defines in linux and names in D1 docs are aliased to our names

/// Global Control Register
/// FIXME: This is the same as SMHC0_BASE. We should only have one ref.
#[doc(alias = "SDXC_REG_GCTRL")]
#[doc(alias = "SMHC_CTRL")]
const GCTRL: u32 = SMHC0_BASE + (0x00);
/// SMC Clock Control Register
#[doc(alias = "SDXC_REG_CLKCLR")]
#[doc(alias = "SMHC_CLKDIV")]
const CLKCR: u32 = SMHC0_BASE + (0x04);
const TIMEOUT: u32 = SMHC0_BASE + (0x08); /* SMC Time Out Register */
const WIDTH: u32 = SMHC0_BASE + (0x0C); /* SMC Bus Width Register */
const BLKSZ: u32 = SMHC0_BASE + (0x10); /* SMC Block Size Register */
const BYTECNT: u32 = SMHC0_BASE + (0x14); /* SMC Byte Count Register */
const CMD: u32 = SMHC0_BASE + (0x18); /* SMC Command Register */
const ARG: u32 = SMHC0_BASE + (0x1C); /* SMC Argument Register */
const RESP0: u32 = SMHC0_BASE + (0x20); /* SMC Response Register 0 */
const RESP1: u32 = SMHC0_BASE + (0x24); /* SMC Response Register 1 */
const RESP2: u32 = SMHC0_BASE + (0x28); /* SMC Response Register 2 */
const RESP3: u32 = SMHC0_BASE + (0x2C); /* SMC Response Register 3 */
const IMASK: u32 = SMHC0_BASE + (0x30); /* SMC Interrupt Mask Register */
const MINT: u32 = SMHC0_BASE + (0x34); /* SMC Masked Interrupt Status Register */
const RINT: u32 = SMHC0_BASE + (0x38); /* SMC Raw Interrupt Status Register */
const STATUS: u32 = SMHC0_BASE + (0x3C); /* SMC Status Register */
const FTRGLEVEL: u32 = SMHC0_BASE + (0x40); /* SMC FIFO Threshold Watermark Register */
const FUNCSEL: u32 = SMHC0_BASE + (0x44); /* SMC Function Select Register */
const CBCR: u32 = SMHC0_BASE + (0x48); /* SMC CIU Byte Count Register */
const BBCR: u32 = SMHC0_BASE + (0x4C); /* SMC BIU Byte Count Register */
const DBGC: u32 = SMHC0_BASE + (0x50); /* SMC Debug Enable Register */
const CSDC: u32 = SMHC0_BASE + (0x54); /* CRC status detect control register */
const A12A: u32 = SMHC0_BASE + (0x58); /* Auto command 12 argument */
const NTSR: u32 = SMHC0_BASE + (0x5c); /* SMC2 Newtiming Set Register */
// const RES1[6]: u32	 = SMHC0_BASE + (0x54~0x74); /* */
const HWRST: u32 = SMHC0_BASE + (0x78); /* SMC eMMC Hardware Reset Register */
const RES2: u32 = SMHC0_BASE + (0x7c); /* */
const DMAC: u32 = SMHC0_BASE + (0x80); /* SMC IDMAC Control Register */
const DLBA: u32 = SMHC0_BASE + (0x84); /* SMC IDMAC Descriptor List Base Address Register */
const IDST: u32 = SMHC0_BASE + (0x88); /* SMC IDMAC Status Register */
const IDIE: u32 = SMHC0_BASE + (0x8C); /* SMC IDMAC Interrupt Enable Register */
const CHDA: u32 = SMHC0_BASE + (0x90); /* */
const CBDA: u32 = SMHC0_BASE + (0x94); /* */
// const RES3[26]: u32	 = SMHC0_BASE + (0x98~0xff); /* */
const THLDC: u32 = SMHC0_BASE + (0x100); /* Card Threshold Control Register */
const SFC: u32 = SMHC0_BASE + (0x104); /*  SMC Sample FIFO Control Register */
// const RES4[1]: u32	 = SMHC0_BASE + (0x105-0x10b); /* */
const DSBD: u32 = SMHC0_BASE + (0x10c); /* eMMC4.5 DDR Start Bit Detection Control */
// const RES5[12]: u32	 = SMHC0_BASE + (0x110~0x13c); */
const DRV_DL: u32 = SMHC0_BASE + (0x140); /* drive delay control register */
const SAMP_DL: u32 = SMHC0_BASE + (0x144); /* sample delay control register */
const DS_DL: u32 = SMHC0_BASE + (0x148); /* data strobe delay control register */
// const RES6[45]: u32	 = SMHC0_BASE + (0x149~0x1ff); /* */
const FIFO: u32 = SMHC0_BASE + (0x200); /* SMC FIFO Access Address */
