//! SD/MMC Host Controller

use super::ccu::{Clocks, Gating, Reset};
use super::gpio::{portf::*, Function};
use super::time::Hz;
use d1_pac::ccu::smhc0_clk::FACTOR_N_A;
use d1_pac::{smhc, CCU};

use embedded_sdmmc::sdcard::proto::*;
pub use embedded_sdmmc::Block;

pub mod card;
use card::{CardStatus, CardType};

mod idmac;
use idmac::{Descriptor, DescriptorConfig};

pub struct Smhc<SMHC> {
    /// The underlying SMHC peripheral
    inner: SMHC,
}

pub struct Card<'a, SMHC> {
    /// Reference to the controller
    smhc: &'a Smhc<SMHC>,
    /// The remote card address
    rca: u32,
}

#[derive(Debug, PartialEq)]
pub enum Interrupt {
    ResponseError,
    CommandComplete,
    DataTransferComplete,
    DataTransmitRequest,
    DataReceiveRequest,
    ResponseCrcError,
    DataCrcError,
    ResponseTimeout,       // or Boot ACK Received
    DataTimeout,           // or Boot Data Start
    DataStarvationTimeout, // or Voltage Switch Done
    FifoUnderrunOverflow,
    CommandBusyIllegalWrite,
    DataStartError,
    AutoCommandDone,
    DataEndError,
    SDIO,
    CardInserted,
    CardRemoved,
}

impl Into<u32> for Interrupt {
    fn into(self) -> u32 {
        match self {
            Self::ResponseError => 1 << 1,
            Self::CommandComplete => 1 << 2,
            Self::DataTransferComplete => 1 << 3,
            Self::DataTransmitRequest => 1 << 4,
            Self::DataReceiveRequest => 1 << 5,
            Self::ResponseCrcError => 1 << 6,
            Self::DataCrcError => 1 << 7,
            Self::ResponseTimeout => 1 << 8,
            Self::DataTimeout => 1 << 9,
            Self::DataStarvationTimeout => 1 << 10,
            Self::FifoUnderrunOverflow => 1 << 11,
            Self::CommandBusyIllegalWrite => 1 << 12,
            Self::DataStartError => 1 << 13,
            Self::AutoCommandDone => 1 << 14,
            Self::DataEndError => 1 << 15,
            Self::SDIO => 1 << 16,
            Self::CardInserted => 1 << 30,
            Self::CardRemoved => 1 << 31,
        }
    }
}

#[derive(Debug)]
pub enum Error {
    /// A transmit bit error, end bit error, or CMD index error has occurred.
    Response,
    /// Invalid CRC in response.
    ResponseCrc,
    /// When receiving data, this means that the received data has data CRC error.
    /// When transmitting data, this means that the received CRC status taken is negative.
    DataCrc,
    /// Did not receive a response in time.
    ResponseTimeout,
    /// Did not receive data in time.
    DataTimeout,
    /// Data starvation detected.
    DataStarvationTimeout,
    /// FIFO underrun or overflow.
    FifoUnderrunOverflow,
    /// Command busy and illegal write. TODO: understand this + add better explanation
    CommandBusyIllegalWrite,
    /// When receiving data, this means that the host controller found an error start bit.
    /// When transmitting data, this means that the busy signal is cleared after the last block.
    DataStart,
    /// When receiving data, this means that we did not receive a valid data end bit.
    /// When transmitting data, this means that we did not receive the CRC status token.
    DataEnd,
    /// An error occurred in the internal DMA controller.
    Dma(idmac::Error),
}

impl<SMHC: Instance> Smhc<SMHC> {
    pub fn new<PINS>(smhc: SMHC, _pins: PINS, _freq: Hz, _clocks: &Clocks) -> Self
    where
        PINS: Pins<SMHC>,
    {
        let ccu = unsafe { &*CCU::ptr() };
        SMHC::assert_reset(ccu);
        SMHC::gating_mask(ccu);
        // TODO: generic over all 3 SMHC interfaces
        #[rustfmt::skip]
        ccu.smhc0_clk.write(|w| w
            .clk_src_sel().hosc()
            .factor_n().variant(FACTOR_N_A::N1)
            .factor_m().variant(1)
            .clk_gating().set_bit()
        );
        SMHC::deassert_reset(ccu);
        SMHC::gating_pass(ccu);

        smhc.smhc_ctrl.modify(|_, w| w.soft_rst().reset());
        while smhc.smhc_ctrl.read().soft_rst().is_reset() {}

        smhc.smhc_ctrl.modify(|_, w| w.fifo_rst().reset());
        while smhc.smhc_ctrl.read().fifo_rst().is_reset() {}

        smhc.smhc_ctrl.modify(|_, w| w.ine_enb().disable());
        // smhc.smhc_intmask.write(|w| unsafe { w.bits(0xFFCE) });

        #[rustfmt::skip]
        smhc.smhc_clkdiv.write(|w| {
            w.cclk_enb().off()
        });

        Self::update_clk(&smhc);

        smhc.smhc_clkdiv.modify(|_, w| w.cclk_div().variant(2));

        smhc.smhc_smap_dl.write(|w| w.samp_dl_sw_en().set_bit());

        #[rustfmt::skip]
        smhc.smhc_clkdiv.modify(|_, w| {
            w.cclk_enb().on()
        });

        Self::update_clk(&smhc);

        // Default bus width after power up or idle is 1-bit
        smhc.smhc_ctype.write(|w| w.card_wid().b1());
        // Blocksize is fixed to 512
        smhc.smhc_blksiz
            .write(|w| unsafe { w.bits(Block::LEN_U32) });

        Self { inner: smhc }
    }

    fn update_clk(smhc: &SMHC) {
        #[rustfmt::skip]
        smhc.smhc_cmd.write(|w| {
            w.wait_pre_over().wait()
             .prg_clk().change()
             .cmd_load().set_bit()
        });

        while smhc.smhc_cmd.read().cmd_load().bit_is_set() {
            core::hint::spin_loop()
        }

        smhc.smhc_rintsts.write(|w| unsafe { w.bits(0xFFFFFFFF) });
    }

    fn reset_fifo(&self) {
        self.inner.smhc_ctrl.modify(|_, w| w.fifo_rst().set_bit());
        while self.inner.smhc_ctrl.read().fifo_rst().is_reset() {}
    }

    /// Send a command to the card
    fn card_command(&self, opt: CommandOptions) {
        self.inner.smhc_cmdarg.write(|w| unsafe { w.bits(opt.arg) });

        let (data_trans, trans_dir) = if let Some(transfer) = opt.data {
            match transfer {
                DataTransfer::Read => (true, false),
                DataTransfer::Write => (true, true),
            }
        } else {
            (false, false)
        };

        let (resp_recv, resp_size) = if let Some(response) = opt.response {
            match response {
                Response::Short => (true, false),
                Response::Long => (true, true),
            }
        } else {
            (false, false)
        };

        #[rustfmt::skip]
        self.inner.smhc_cmd.write(|w| {
            w.cmd_load().set_bit()
             .wait_pre_over().wait()
             .stop_cmd_flag().bit(opt.auto_stop)
             .data_trans().bit(data_trans)
             .trans_dir().bit(trans_dir)
             .chk_resp_crc().bit(opt.check_crc)
             .long_resp().bit(resp_size)
             .resp_rcv().bit(resp_recv)
             .cmd_idx().variant(opt.cmd)
        });
    }

    /// Wait for specified interrupt status bit.
    ///
    /// If the interrupt is considered an *error*, the result will be `Err` instead of `Ok`.
    /// This function will also clear the interrupt status
    fn wait_for_interrupt(&self, int: Interrupt) -> Result<(), Error> {
        let int: u32 = int.into();

        let result = loop {
            let rint = self.inner.smhc_rintsts.read();

            if rint.rto_back().bit_is_set() {
                break Err(Error::ResponseTimeout);
            }
            if rint.rce().bit_is_set() {
                break Err(Error::ResponseCrc);
            }
            if rint.re().bit_is_set() {
                break Err(Error::Response);
            }
            if rint.dsto_vsd().bit_is_set() {
                break Err(Error::DataStarvationTimeout);
            }
            if rint.dto_bds().bit_is_set() {
                break Err(Error::DataTimeout);
            }
            if rint.dce().bit_is_set() {
                break Err(Error::DataCrc);
            }
            if rint.fu_fo().bit_is_set() {
                break Err(Error::FifoUnderrunOverflow);
            }
            if rint.cb_iw().bit_is_set() {
                break Err(Error::CommandBusyIllegalWrite);
            }
            if rint.dse_bc().bit_is_set() {
                break Err(Error::DataStart);
            }
            if rint.dee().bit_is_set() {
                break Err(Error::DataEnd);
            }
            if (rint.bits() & int) == int {
                break Ok(());
            }
        };

        self.inner.smhc_rintsts.write(|w| unsafe { w.bits(int) });

        result
    }

    /// Wait for command complete.
    fn wait_for_cc(&self) -> Result<(), Error> {
        self.wait_for_interrupt(Interrupt::CommandComplete)
    }

    /// Wait for data transfer complete.
    fn wait_for_dtc(&self) -> Result<(), Error> {
        self.wait_for_interrupt(Interrupt::DataTransferComplete)
    }

    /// Wait for auto command done.
    fn wait_for_acd(&self) -> Result<(), Error> {
        self.wait_for_interrupt(Interrupt::AutoCommandDone)
    }

    pub fn reset_card(&self) -> Result<(), Error> {
        // For CMD0, we can use the default options
        self.card_command(CommandOptions::default());
        self.wait_for_cc()?;
        self.reset_fifo();
        Ok(())
    }

    pub fn initialize(&self) -> Result<CardType, Error> {
        /// Request switch to 1.8V
        #[allow(dead_code)]
        const OCR_S18R: u32 = 0x1000000;
        /// Host supports high capacity
        const OCR_HCS: u32 = 0x40000000;
        /// Card has finished power up routine if bit is high
        const OCR_NBUSY: u32 = 0x80000000;
        /// Valid bits for voltage setting
        const OCR_VOLTAGE_MASK: u32 = 0x007FFF80;

        let mut card_type = CardType::SD1;

        self.card_command(CommandOptions::init_cmd(CMD8, 0x1AA, true));
        self.wait_for_cc()?; // TODO: this will return Error(ResponseTimeout) for SD1 cards?
        let data = self.inner.smhc_resp0.read().bits();
        if data == 0x1AA {
            card_type = CardType::SD2;
        }

        let ocr = loop {
            // TODO: limit the number of attempts
            // Go to *APP* mode before sending application command
            self.card_command(CommandOptions::init_cmd(CMD55, 0, true));
            self.wait_for_cc()?;

            let mut op_cond_arg = OCR_VOLTAGE_MASK & 0x00ff8000;
            if card_type != CardType::SD1 {
                op_cond_arg = OCR_HCS | op_cond_arg;
            }
            self.card_command(CommandOptions::init_cmd(ACMD41, op_cond_arg, false));
            self.wait_for_cc()?;

            let data = self.inner.smhc_resp0.read().bits();
            if (data & OCR_NBUSY) == OCR_NBUSY {
                // Card has finished power up, data is valid
                break data;
            }

            // TODO: wait 1ms
            // let clint = unsafe { super::clint::Clint::summon() };
            // let next = clint.get_mtime() + 24_000;
            // while clint.get_mtime() < next {
            //     core::hint::spin_loop();
            // }
        };

        if (ocr & OCR_HCS) == OCR_HCS {
            card_type = CardType::SDHC;
        }

        Ok(card_type)
    }

    /// Get the card identification register
    pub fn get_cid(&self) -> Result<(), Error> {
        const CMD2: u8 = 0x02; // should be added in `embedded_sdmmc`
        self.card_command(CommandOptions::ctrl_cmd(CMD2, 0, Response::Long));
        self.wait_for_cc()?;
        // TODO: return CID
        Ok(())
    }

    /// Get the relative card address, stuff-bits are included in the result
    pub fn get_rca<'a>(&'a self) -> Result<Card<'a, SMHC>, Error> {
        const CMD3: u8 = 0x03; // should be added in `embedded_sdmmc`
        self.card_command(CommandOptions::ctrl_cmd(CMD3, 0, Response::Short));
        self.wait_for_cc()?;
        Ok(Card {
            smhc: self,
            rca: self.inner.smhc_resp0.read().bits(),
        })
    }
}

impl<'a, SMHC: Instance> Card<'a, SMHC> {
    /// Get the card specific data
    pub fn get_csd(&self) -> Result<Csd, Error> {
        self.smhc
            .card_command(CommandOptions::ctrl_cmd(CMD9, self.rca, Response::Long));
        self.smhc.wait_for_cc()?;
        let d0 = self.smhc.inner.smhc_resp0.read().bits();
        let d1 = self.smhc.inner.smhc_resp1.read().bits();
        let d2 = self.smhc.inner.smhc_resp2.read().bits();
        let d3 = self.smhc.inner.smhc_resp3.read().bits();
        let mut csdv = CsdV2::new();
        csdv.data[0..4].copy_from_slice(&d0.to_le_bytes());
        csdv.data[4..8].copy_from_slice(&d1.to_le_bytes());
        csdv.data[8..12].copy_from_slice(&d2.to_le_bytes());
        csdv.data[12..16].copy_from_slice(&d3.to_le_bytes());
        csdv.data.reverse();
        Ok(Csd::V2(csdv))
    }

    /// Get the card status register
    pub fn get_status(&self) -> Result<CardStatus, Error> {
        self.smhc
            .card_command(CommandOptions::ctrl_cmd(CMD13, self.rca, Response::Short));
        self.smhc.wait_for_cc()?;

        let response = self.smhc.inner.smhc_resp0.read().bits();
        Ok(CardStatus::new(response))
    }

    /// Toggle the card between stand-by and transfer state
    pub fn select(&self) -> Result<CardStatus, Error> {
        const CMD7: u8 = 0x07; // should be added in `embedded_sdmmc`
        self.smhc
            .card_command(CommandOptions::ctrl_cmd(CMD7, self.rca, Response::Short));
        self.smhc.wait_for_cc()?;

        let response = self.smhc.inner.smhc_resp0.read().bits();
        Ok(CardStatus::new(response))
    }

    /// Use 4 data lanes
    pub fn set_wide_bus(&self) -> Result<CardStatus, Error> {
        // Go to *APP* mode before sending application command
        self.smhc
            .card_command(CommandOptions::init_cmd(CMD55, self.rca, true));
        self.smhc.wait_for_cc()?;

        const ACMD6: u8 = 0x06; // should be added in `embedded_sdmmc`
        self.smhc
            .card_command(CommandOptions::ctrl_cmd(ACMD6, 0b10, Response::Short));
        self.smhc.wait_for_cc()?;

        self.smhc.inner.smhc_ctype.write(|w| w.card_wid().b4());

        let response = self.smhc.inner.smhc_resp0.read().bits();
        Ok(CardStatus::new(response))
    }

    /// Prepare the internal DMA controller for data transfer
    fn prepare_for_dma(&self, descriptor: &Descriptor, block_cnt: u32) -> Result<(), Error> {
        #[rustfmt::skip]
        self.smhc.inner.smhc_ctrl.modify(|_, w| {
            w.dma_enb().set_bit()
             .dma_rst().set_bit()
        });
        while self.smhc.inner.smhc_ctrl.read().dma_rst().bit_is_set() {}

        // Configure the address of the first DMA descriptor
        // Right-shift by 2 because it is a *word-address*.
        self.smhc
            .inner
            .smhc_dlba
            .write(|w| unsafe { w.bits((descriptor as *const _ as u32) >> 2) });

        // Set number of bytes that will be read or written.
        self.smhc
            .inner
            .smhc_bytcnt
            .write(|w| unsafe { w.bits(block_cnt * Block::LEN_U32) });

        // Soft reset of DMA controller
        self.smhc
            .inner
            .smhc_idmac
            .write(|w| w.idmac_rst().set_bit());

        // Configure the burst size and TX/RX trigger level
        #[rustfmt::skip]
        self.smhc.inner.smhc_fifoth.write(|w| {
            w.tx_tl().variant(8)
             .rx_tl().variant(7)
             .bsize_of_trans().t8()
        });

        // configure the transfer interrupt, receive interrupt, and abnormal interrupt.
        #[rustfmt::skip]
        self.smhc.inner.smhc_idie.write(|w| {
            w.rx_int_enb().set_bit()
             .tx_int_enb().set_bit()
             .err_sum_int_enb().set_bit()
        });

        // enable the IDMAC and configure burst transfers
        #[rustfmt::skip]
        self.smhc.inner.smhc_idmac.write(|w| {
            w.idmac_enb().set_bit()
             .fix_bust_ctrl().set_bit()
        });

        self.smhc.reset_fifo();

        Ok(())
    }

    /// Read a single block at the given sector address
    pub fn read_block(&self, sector: u32, buffer: &mut Block) -> Result<(), Error> {
        let descriptor = Descriptor::try_from(DescriptorConfig {
            disable_int_on_complete: false,
            first: true,
            last: true,
            buff_size: Block::LEN as u16,
            buff_addr: core::ptr::NonNull::from(&buffer.contents),
            next_desc: None,
        })
        .map_err(Error::Dma)?;

        self.prepare_for_dma(&descriptor, 1)?;

        let opts = CommandOptions::data_cmd(CMD17, sector, DataTransfer::Read, false);
        self.smhc.card_command(opts);
        self.smhc.wait_for_cc()?;

        while self.smhc.inner.smhc_idst.read().rx_int().bit_is_clear() {}
        self.smhc.inner.smhc_idst.write(|w| w.rx_int().set_bit());

        self.smhc.wait_for_dtc()?;

        Ok(())
    }

    /// Write a single block to the given sector address
    pub fn write_block(&self, sector: u32, buffer: &Block) -> Result<(), Error> {
        let descriptor = Descriptor::try_from(DescriptorConfig {
            disable_int_on_complete: false,
            first: true,
            last: true,
            buff_size: Block::LEN as u16,
            buff_addr: core::ptr::NonNull::from(&buffer.contents),
            next_desc: None,
        })
        .map_err(Error::Dma)?;

        self.prepare_for_dma(&descriptor, 1)?;

        let opts = CommandOptions::data_cmd(CMD24, sector, DataTransfer::Write, false);
        self.smhc.card_command(opts);
        self.smhc.wait_for_cc()?;

        while self.smhc.inner.smhc_idst.read().tx_int().bit_is_clear() {}
        self.smhc.inner.smhc_idst.write(|w| w.tx_int().set_bit());

        self.smhc.wait_for_dtc()?;

        Ok(())
    }

    fn blocks_to_descriptors(
        blocks: &[Block],
        descriptors: &mut [Descriptor],
    ) -> Result<(), Error> {
        for i in 0..(blocks.len() - 1) {
            descriptors[i] = Descriptor::try_from(DescriptorConfig {
                disable_int_on_complete: false,
                first: i == 0,
                last: false,
                buff_size: Block::LEN as u16,
                buff_addr: core::ptr::NonNull::from(&blocks[i].contents),
                next_desc: Some(&descriptors[i + 1] as *const _ as _),
            })
            .map_err(Error::Dma)?;
        }
        let i = blocks.len() - 1;
        descriptors[i] = Descriptor::try_from(DescriptorConfig {
            disable_int_on_complete: false,
            first: i == 0,
            last: true,
            buff_size: Block::LEN as u16,
            buff_addr: core::ptr::NonNull::from(&blocks[i].contents),
            next_desc: None,
        })
        .map_err(Error::Dma)?;
        Ok(())
    }

    /// Read a series of blocks, starting at the given sector address
    pub fn read_blocks(&self, sector: u32, buffers: &mut [Block]) -> Result<(), Error> {
        let run_chunk = |sector: u32, desc: &Descriptor, block_cnt: u32| -> Result<(), Error> {
            self.prepare_for_dma(desc, block_cnt)?;

            let opts = CommandOptions::data_cmd(CMD18, sector, DataTransfer::Read, true);
            self.smhc.card_command(opts);
            self.smhc.wait_for_cc()?;

            while self.smhc.inner.smhc_idst.read().rx_int().bit_is_clear() {}
            self.smhc.inner.smhc_idst.write(|w| w.rx_int().set_bit());

            self.smhc.wait_for_dtc()?;
            self.smhc.wait_for_acd()?;
            Ok(())
        };

        let mut current_sector = sector;
        let mut descriptors: [Descriptor; 16] = unsafe { core::mem::zeroed() };

        for chunk in buffers.chunks(16) {
            Self::blocks_to_descriptors(chunk, &mut descriptors)?;
            run_chunk(current_sector, &descriptors[0], chunk.len() as u32)?;
            current_sector += chunk.len() as u32;
        }

        Ok(())
    }

    /// Writes a series of blocks, starting at the given sector address
    pub fn write_blocks(&self, sector: u32, buffers: &[Block]) -> Result<(), Error> {
        let run_chunk = |sector: u32, desc: &Descriptor, block_cnt: u32| -> Result<(), Error> {
            self.prepare_for_dma(desc, block_cnt)?;

            let opts = CommandOptions::data_cmd(CMD25, sector, DataTransfer::Write, true);
            self.smhc.card_command(opts);
            self.smhc.wait_for_cc()?;

            while self.smhc.inner.smhc_idst.read().tx_int().bit_is_clear() {}
            self.smhc.inner.smhc_idst.write(|w| w.tx_int().set_bit());

            self.smhc.wait_for_dtc()?;
            self.smhc.wait_for_acd()?;

            Ok(())
        };

        let mut current_sector = sector;
        let mut descriptors: [Descriptor; 16] = unsafe { core::mem::zeroed() };

        for chunk in buffers.chunks(16) {
            Self::blocks_to_descriptors(chunk, &mut descriptors)?;
            run_chunk(current_sector, &descriptors[0], chunk.len() as u32)?;
            current_sector += chunk.len() as u32;
            // wait until write has finished
            loop {
                let card_stat = self.get_status()?;
                if card_stat.current_state() != card::CardCurrentState::Prg {
                    break;
                }
            }
        }

        Ok(())
    }
}

enum DataTransfer {
    Read,
    Write,
}

enum Response {
    /// 48-bit response
    Short,
    /// 136-bit response
    Long,
}

struct CommandOptions {
    cmd: u8,
    arg: u32,
    data: Option<DataTransfer>,
    check_crc: bool,
    response: Option<Response>,
    auto_stop: bool,
}

impl CommandOptions {
    /// A command to be sent during the *initialization* phase.
    fn init_cmd(cmd: u8, arg: u32, check_crc: bool) -> Self {
        Self {
            cmd,
            arg,
            data: None,
            check_crc,
            response: Some(Response::Short),
            auto_stop: false,
        }
    }
    /// A command used after initialization, for *configuration, ...*
    fn ctrl_cmd(cmd: u8, arg: u32, response: Response) -> Self {
        Self {
            cmd,
            arg,
            data: None,
            check_crc: true,
            response: Some(response),
            auto_stop: false,
        }
    }
    /// A command used after initialization, for *data transfer*.
    fn data_cmd(cmd: u8, arg: u32, dir: DataTransfer, auto_stop: bool) -> Self {
        Self {
            cmd,
            arg,
            data: Some(dir),
            check_crc: true,
            response: Some(Response::Short),
            auto_stop,
        }
    }
}

impl Default for CommandOptions {
    /// The default command is **reset**
    fn default() -> Self {
        Self {
            cmd: CMD0,
            arg: 0,
            data: None,
            check_crc: false,
            response: None,
            auto_stop: false,
        }
    }
}

pub trait Instance: core::ops::Deref<Target = smhc::RegisterBlock> + Gating + Reset {}

impl Instance for d1_pac::SMHC0 {}
impl Instance for d1_pac::SMHC1 {}
impl Instance for d1_pac::SMHC2 {}

pub trait Pins<SMHC> {}

impl Pins<d1_pac::SMHC0>
    for (
        PF0<Function<2>>,
        PF1<Function<2>>,
        PF2<Function<2>>,
        PF3<Function<2>>,
        PF4<Function<2>>,
        PF5<Function<2>>,
    )
{
}
