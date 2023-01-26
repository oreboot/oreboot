use crate::{
    pad_cfg_gpi, pad_iosstate, pad_pull, pad_buf, pad_reset, pad_func,
    intel::{
        apollolake::{
            cpu::cpu_soc_is_in_untrusted_mode,
            gpio_glk::{soc_gpio_get_community, GPIO_NUM_PAD_CFG_REGS, NUM_GPI_STATUS_REGS},
        },
        common::block::{
            gpio::gpio_defs::*,
            itss::itss_set_irq_polarity,
            p2sb::{p2sb_hide, p2sb_unhide, PCH_DEV_P2SB},
            pcr::{
                pcr_execute_sideband_msg, pcr_or32, pcr_read32, pcr_rmw32, pcr_write32, PcrSbiMsg,
                PcrSbiOpcode,
            },
        },
        Error,
    },
};
use alloc::vec::Vec;
use consts::{ENV_ROMSTAGE_OR_BEFORE, ENV_SMM};
use core::mem::size_of;

use security::vboot::vboot_common::vboot_recovery_mode_enabled;
use spin::rwlock::RwLock;
use types::bit;

pub mod gpio_defs;

pub type Gpio = u32;

static GPIO_IOAPIC_IRQS_USED: RwLock<[u32; GPIO_IOAPIC_IRQS_USED_LEN]> =
    RwLock::new([0; GPIO_IOAPIC_IRQS_USED_LEN]);

pub const GPIO_IOAPIC_IRQS_USED_LEN: usize = 120 / size_of::<u32>() * BITS_PER_BYTE + 1;
pub const BITS_PER_BYTE: usize = 8;
pub const PAD_BASE_NONE: i32 = -1;
pub const PAD_DW0_MASK: u32 = PadCfg0TxState::State as u32
    | PadCfg0Tx::Disable as u32
    | PadCfg0Rx::Disable as u32
    | PadCfg0Mask::Mode as u32
    | PadCfg0Mask::Route as u32
    | PadCfg0Mask::RxTenCfg as u32
    | PadCfg0Mask::RxInv as u32
    | PAD_CFG0_PREGFRXSEL
    | PadCfg0Mask::Trig as u32
    | PadCfg0Mask::RxRaw1 as u32
    | PAD_CFG0_NAFVWE_ENABLE
    | PadCfg0Mask::RxPadstsel as u32
    | PadCfg0Mask::Reset as u32;
pub const PAD_DW1_MASK: u32 =
    PadCfg1Mask::Iosterm as u32 | PadCfg1Mask::Pull as u32 | PadCfg1Mask::Iosstate as u32;
pub const PAD_DW2_MASK: u32 = PadCfg2Mask::Debounce as u32;
pub const PAD_DW3_MASK: u32 = 0;

pub const MASK: [i32; 4] = [
    PAD_DW0_MASK as i32,
    PAD_DW1_MASK as i32,
    PAD_DW2_MASK as i32,
    PAD_DW3_MASK as i32,
];

pub fn gpio_dwx_size(x: u32) -> u32 {
    size_of::<u32>() as u32 * x
}

pub fn pad_cfg_offset(x: u32, dw_num: u32) -> u32 {
    x + gpio_dwx_size(dw_num)
}

pub fn pad_cfg1_offset(x: u32) -> u32 {
    pad_cfg_offset(x, 1)
}

pub fn gpio_lock_pad(pad: Gpio, lock_action: GpioLockAction) -> Result<(), Error> {
    if ENV_ROMSTAGE_OR_BEFORE || vboot_recovery_mode_enabled() {
        return Err(Error::SkipGpioPad);
    }

    let pads = GpioLockConfig { pad, lock_action };

    if !ENV_SMM && !cfg!(soc_intel_common_block_smm_lock_gpio_pads) {
        return pads.gpio_non_smm_lock_pad();
    }

    gpio_lock_pads(&[pads])
}

pub fn gpio_lock_pads(pad_list: &[GpioLockConfig]) -> Result<(), Error> {
    if !cfg!(soc_intel_common_block_smm_lock_gpio_pads) {
        return Err(Error::NoSmm);
    }

    /*
     * FSP-S will unlock all the GPIO pads and hide the P2SB device.  With
     * the device hidden, we will not be able to send the sideband interface
     * message to lock the GPIO configuration. Therefore, we need to unhide
     * the P2SB device which can only be done in SMM requiring that this
     * function is called from SMM.
     */
    if !ENV_SMM {
        //error!("{}: Error: must be called from SMM!", "gpio_lock_pads");
        return Err(Error::NoSmm);
    }

    if pad_list.len() == 0 {
        //error!("{}: Error: pad_list count = 0!", "gpio_lock_pads");
        return Err(Error::NullGpioPads);
    }

    p2sb_unhide();

    for gpio_pad in pad_list.iter() {
        let pad = gpio_pad.pad;
        let community = &soc_gpio_get_community();
        let comm = gpio_get_community(pad, community)?;
        let rel_pad = comm.relative_pad_in_comm(pad);
        let mut offset = comm.pad_cfg_lock_offset;

        if offset == 0 {
            //error!(
            //    "{}: Error: offset not defined for pad {}",
            //    "gpio_lock_pads", pad
            //);
            continue;
        }

        offset += comm.gpio_group_index_scaled(rel_pad as u32, 2 * size_of::<u32>())? as u16;

        let bit_mask = comm.gpio_bitmask_within_group(rel_pad as u32)?;
        gpio_pad.gpio_pad_config_lock_using_sbi(comm.port, offset, bit_mask)?;
    }

    p2sb_hide();

    Ok(())
}

pub fn gpio_input_pulldown(gpio: Gpio) {
    let cfg = pad_cfg_gpi!(gpio, Dn20k, Deep);
    let _ = cfg.configure_pad();
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GpioLockAction {
    Unlock = 0x0,
    LockConfig = 0x1,
    LockTx = 0x2,
    LockFull = Self::LockConfig as isize | Self::LockTx as isize,
}

#[repr(C)]
pub struct GpioLockConfig {
    pub pad: Gpio,
    pub lock_action: GpioLockAction,
}

impl GpioLockConfig {
    pub fn gpio_pad_config_lock_using_pcr(&self, pid: u8, offset: u16, bit_mask: u32) {
        //const FUNC_NAME: &str = "gpio_pad_config_lock_using_pcr";

        if self.lock_action == GpioLockAction::LockConfig {
            if cfg!(DEBUG_GPIO) {
                //debug!("{}: Locking pad {} configuration", FUNC_NAME, self.pad);
            }
            pcr_or32(pid, offset, bit_mask);
        }

        if self.lock_action == GpioLockAction::LockTx {
            if cfg!(DEBUG_GPIO) {
                //debug!("{}: Locking pad {} TX state", FUNC_NAME, self.pad);
            }
            pcr_or32(pid, offset + size_of::<u32>() as u16, bit_mask);
        }
    }

    pub fn gpio_pad_config_lock_using_sbi(
        &self,
        pid: u8,
        offset: u16,
        bit_mask: u32,
    ) -> Result<(), Error> {
        //const FUNC_NAME: &str = "gpio_pad_config_lock_using_sbi";

        let mut msg = PcrSbiMsg {
            pid,
            offset: offset as u32,
            opcode: PcrSbiOpcode::GpioLockUnlock,
            is_posted: false,
            fast_byte_enable: 0xf,
            bar: 0,
            fid: 0,
        };

        if self.lock_action as u32 & GpioLockAction::LockFull as u32 == 0 {
            //error!(
            //    "{}: Error: no lock_action specified for pad {}",
            //    FUNC_NAME, self.pad
            //);
            return Err(Error::MissingLockAction);
        }

        if self.lock_action == GpioLockAction::LockConfig {
            if cfg!(debug_gpio) {
                //debug!("{}: Locking pad {} configuration", FUNC_NAME, self.pad);
            }
            let mut data = pcr_read32(pid, offset) | bit_mask;
            let mut response = 0;
            pcr_execute_sideband_msg(PCH_DEV_P2SB, &mut msg, &mut data, &mut response)?;
            if response != 0 {
                //error!("Failed to lock GPIO PAD Tx state, response = {}", response);
            }
        }

        Ok(())
    }

    pub fn gpio_non_smm_lock_pad(&self) -> Result<(), Error> {
        //const FUNC_NAME: &str = "gpio_non_smm_lock_pad";

        let community = &soc_gpio_get_community();
        let comm = gpio_get_community(self.pad, community)?;

        if cpu_soc_is_in_untrusted_mode() {
            //error!(
            //    "{}: Error: IA Untrusted Mode enabled, can't lock pad!",
            //    FUNC_NAME
            //);
            return Err(Error::UntrustedMode);
        }

        let rel_pad = comm.relative_pad_in_comm(self.pad);
        let mut offset = comm.pad_cfg_lock_offset;
        if offset == 0 {
            //error!(
            //    "{}: Error: offset not defined for pad {}",
            //    FUNC_NAME, self.pad
            //);
            return Err(Error::UndefinedOffset);
        }

        offset += comm.gpio_group_index_scaled(rel_pad as u32, 2 * size_of::<u32>())? as u16;
        let bit_mask = comm.gpio_bitmask_within_group(rel_pad as u32)?;

        if cfg!(feature = "lock_using_pcr") {
            if cfg!(DEBUG_GPIO) {
                //debug!("Locking pad configuration using PCR");
            }
            self.gpio_pad_config_lock_using_pcr(comm.port, offset, bit_mask);
        } else if cfg!(feature = "lock_using_sbi") {
            if cfg!(DEBUG_GPIO) {
                //debug!("Locking pad configuration using SBI");
            }
            self.gpio_pad_config_lock_using_sbi(comm.port, offset, bit_mask)?;
        } else {
            //error!(
            //    "{}: Error: No pad configuration lock method is selected!",
            //    FUNC_NAME
            //);
        }

        Ok(())
    }
}

#[repr(C)]
pub struct PadConfig {
    /// Offset of pad within community
    pub pad: Gpio,
    /// Pad config data corresponding to DW0, DW1, ...
    pub pad_config: [u32; GPIO_NUM_PAD_CFG_REGS],
    /// Pad lock configuration
    pub lock_action: GpioLockAction,
}

pub fn set_ioapic_used(irq: u32) {
    let word_offset = irq / 32;
    let bit_offset = irq % 32;
    assert!(word_offset < GPIO_IOAPIC_IRQS_USED_LEN as u32);
    (*GPIO_IOAPIC_IRQS_USED.write())[word_offset as usize] |= bit(bit_offset as u64) as u32;
}

impl PadConfig {
    pub const fn new() -> Self {
        Self {
            pad: 0,
            pad_config: [0; GPIO_NUM_PAD_CFG_REGS],
            lock_action: GpioLockAction::Unlock,
        }
    }

    pub const fn create(pad: Gpio, config0: u32, config1: u32) -> Self {
        Self {
            pad,
            pad_config: [config0, config1, 0, 0],
            lock_action: GpioLockAction::Unlock,
        }
    }

    pub const fn create_lock(
        pad: Gpio,
        config0: u32,
        config1: u32,
        lock_action: GpioLockAction,
    ) -> Self {
        Self {
            pad,
            pad_config: [config0, config1, 0, 0],
            lock_action,
        }
    }

    pub fn configure_pad(&self) -> Result<(), Error> {
        let community = &soc_gpio_get_community();
        let comm = gpio_get_community(self.pad, community)?;
        let config_offset = comm.pad_config_offset(self.pad);
        let pin = comm.relative_pad_in_comm(self.pad);
        let group = comm.gpio_group_index(pin as u32)?;

        for i in 0..GPIO_NUM_PAD_CFG_REGS {
            let pad_conf = pcr_read32(
                comm.port,
                pad_cfg_offset(config_offset as u32, i as u32) as u16,
            );

            let mut soc_pad_conf = self.pad_config[i];
            if i == 0 {
                soc_pad_conf = comm.gpio_pad_reset_config_override(soc_pad_conf);
            }
            soc_pad_conf &= MASK[i] as u32;
            soc_pad_conf |= pad_conf & (!MASK[i]) as u32;

            soc_pad_conf = self.soc_gpio_pad_config_fixup(i as i32, soc_pad_conf);
            if cfg!(feature = "debug_gpio") {
                //debug!(
                //    "gpio_padcfg [0x{:02x}, {:02}] DW{} [0x{:08x} : 0x{:08x} : 0x{:08x}]",
                //    comm.port, pin, i, pad_conf, self.pad_config[i], soc_pad_conf
                //);
            }
            pcr_write32(
                comm.port,
                pad_cfg_offset(config_offset as u32, i as u32) as u16,
                soc_pad_conf,
            );
        }

        self.gpio_configure_itss(comm.port as u16, config_offset);
        self.gpio_configure_owner(comm)?;
        self.gpi_enable_smi(comm, group as i32, pin as i32)?;
        self.gpi_enable_nmi(comm, group as i32, pin as i32)?;
        self.gpi_enable_gpe(comm, group as i32, pin as i32)?;
        if self.lock_action as u32 != 0 {
            gpio_lock_pad(self.pad, self.lock_action)?;
        }

        Ok(())
    }

    pub fn gpio_get_config<'a>(&self, override_cfg: &'a [PadConfig]) -> Option<&'a PadConfig> {
        for pad in override_cfg.iter() {
            if self.pad == pad.pad {
                return Some(pad);
            }
        }
        None
    }

    pub fn soc_gpio_pad_config_fixup(&self, _dw_reg: i32, reg_val: u32) -> u32 {
        reg_val
    }

    pub fn gpio_configure_itss(&self, port: u16, pad_cfg_offset: u16) {
        /* No ITSS configuration in SMM. */
        if ENV_SMM {
            return;
        }

        /* Set up ITSS polarity if pad is routed to APIC.
         *
         * The ITSS takes only active high interrupt signals. Therefore,
         * if the pad configuration indicates an inversion assume the
         * intent is for the ITSS polarity. Before forwarding on the
         * request to the APIC there's an inversion setting for how the
         * signal is forwarded to the APIC. Honor the inversion setting
         * in the GPIO pad configuration so that a hardware active low
         * signal looks that way to the APIC (double inversion).
         */
        if self.pad_config[0] & (PadCfg0Route::Ioapic as u32) == 0 {
            return;
        }

        let mut irq = pcr_read32(port as u8, pad_cfg1_offset(pad_cfg_offset as u32) as u16);
        irq &= PadCfg1Mask::Irq as u32;

        if irq == 0 {
            //error!("GPIO {} doesn't support routing.", self.pad);
        }

        if cfg!(feature = "itss_pol_cfg") {
            itss_set_irq_polarity(
                irq as i32,
                !!(self.pad_config[0] as i32 & PadCfg0RxPol::Invert as i32),
            );
        }

        set_ioapic_used(irq);
    }

    pub fn gpio_configure_owner(&self, comm: &PadCommunity) -> Result<(), Error> {
        let pin = comm.relative_pad_in_comm(self.pad);

        // Based on the gpio pin number configure the corresponding bit in
        // HOSTSW_OWN register. Value of 0x1 indicates GPIO Driver onwership.
        let mut hostsw_own_offset = comm.host_own_reg_0;
        hostsw_own_offset += comm.gpio_group_index_scaled(pin as u32, size_of::<u32>())? as u16;

        let mut hostsw_own = pcr_read32(comm.port, hostsw_own_offset);

        /* The 4th bit in pad_config 1 (RO) is used to indicate if the pad
         * needs GPIO driver ownership.  Set the bit if GPIO driver ownership
         * requested, otherwise clear the bit.
         */
        if self.pad_config[1] & PAD_CFG_OWN_GPIO_DRIVER as u32 != 0 {
            hostsw_own |= comm.gpio_bitmask_within_group(pin as u32)? as u32;
        } else {
            hostsw_own &= (!comm.gpio_bitmask_within_group(pin as u32)?) as u32;
        }

        pcr_write32(comm.port, hostsw_own_offset, hostsw_own);

        Ok(())
    }

    pub fn gpi_enable_smi(&self, comm: &PadCommunity, group: i32, pin: i32) -> Result<(), Error> {
        if self.pad_config[0] & (PadCfg0Route::Smi as u32) != PadCfg0Route::Smi as u32 {
            return Ok(());
        }

        let sts_reg = comm.gpi_smi_sts_offset(group as u32);
        let en_reg = comm.gpi_smi_en_offset(group as u32);
        let en_value = comm.gpio_bitmask_within_group(pin as u32)?;

        /* Write back 1 to reset the sts bit */
        pcr_rmw32(comm.port, sts_reg as u16, en_value, 0);

        /* Set enable bits */
        pcr_or32(comm.port, en_reg as u16, en_value);

        Ok(())
    }

    pub fn gpi_enable_nmi(&self, comm: &PadCommunity, group: i32, pin: i32) -> Result<(), Error> {
        if self.pad_config[0] & (PadCfg0Route::Nmi as u32) != PadCfg0Route::Nmi as u32 {
            return Ok(());
        }

        /* Do not configure NMI if the platform doesn't support it */
        if comm.gpi_nmi_sts_reg_0 == 0 || comm.gpi_nmi_en_reg_0 == 0 {
            return Ok(());
        }

        let sts_reg = comm.gpi_nmi_sts_offset(group as u32);
        let en_reg = comm.gpi_nmi_en_offset(group as u32);
        let en_value = comm.gpio_bitmask_within_group(pin as u32)?;

        /* Write back 1 to reset the sts bit */
        pcr_rmw32(comm.port, sts_reg as u16, en_value, 0);

        /* Set enable bits */
        pcr_or32(comm.port, en_reg as u16, en_value);

        Ok(())
    }

    pub fn gpi_enable_gpe(&self, comm: &PadCommunity, group: i32, pin: i32) -> Result<(), Error> {
        /* Do not configure GPE_EN if PAD is not configured for SCI/wake */
        if self.pad_config[0] & (PadCfg0Route::Sci as u32) != PadCfg0Route::Sci as u32 {
            return Ok(());
        }

        let en_reg = comm.gpi_gpe_en_offset(group as u32);
        let en_value = comm.gpio_bitmask_within_group(pin as u32)?;

        pcr_or32(comm.port, en_reg as u16, en_value);

        if cfg!(debug_gpio) {
            //debug!(
            //    "GPE_EN[0x{:02x}, {:02}]: Reg: 0x{:x}, Value = 0x{:x}",
            //    comm.port,
            //    comm.relative_pad_in_comm(self.pad),
            //    en_reg,
            //    pcr_read32(comm.port, en_reg as u16)
            //);
        }

        Ok(())
    }
}

pub fn gpio_get_community<'a>(
    pad: Gpio,
    communities: &'a [PadCommunity],
) -> Result<&'a PadCommunity<'a, 'a, 'a, 'a>, Error> {
    for comm in communities.iter() {
        if pad >= comm.first_pad && pad <= comm.last_pad {
            return Ok(comm);
        }
    }
    //error!("{} pad {} not found", "gpio_get_community", pad);
    Err(Error::MissingCommunityPad)
}

pub fn gpio_output(gpio: Gpio, val: u32) -> Result<(), Error> {
    let cfg = pad_cfg_gpo_deep(gpio, val);
    cfg.configure_pad()
}

pub fn gpio_get(gpio_num: Gpio) -> Result<u32, Error> {
    let community = &soc_gpio_get_community();
    let comm = gpio_get_community(gpio_num, community)?;

    let config_offset = comm.pad_config_offset(gpio_num);
    let reg = pcr_read32(comm.port, config_offset);

    Ok(!!(reg & (PadCfg0RxState::State as u32)))
}

/// Configuration for raw pads. Some pads are designated as only special function
/// pins, and don't have an associated GPIO number, so we need to expose the raw
/// pad configuration functionality.
pub fn gpio_configure_pads(cfg: &[PadConfig]) -> Result<(), Error> {
    for pad in cfg.iter() {
        pad.configure_pad()?;
    }
    Ok(())
}

pub fn gpio_configure_pads_with_override(base_cfg: &[PadConfig], override_cfg: &[PadConfig]) -> Result<(), Error> {
    for pad in base_cfg.iter() {
        let c = pad.gpio_get_config(override_cfg).unwrap_or(pad);
        c.configure_pad()?;
    }
    Ok(())
}

/// Structure provides the logical to actual value for PADRSTCFG in DW0. Note
/// that the values are expected to be within the field placement of the register
/// itself. i.e. if the reset field is at 31:30 then the values within logical
/// and chipset should occupy 31:30.
#[repr(C)]
pub struct ResetMapping {
    pub logical: u32,
    pub chipset: u32,
}

/// Structure describes the groups within each community
#[repr(C)]
pub struct PadGroup {
    /// Offset of first pad of the group relative to the community
    pub first_pad: i32,
    /// Size of the group
    pub size: u32,
    /// This is the starting pin number for the pads in this group when
    /// they are used in ACPI.  This is only needed if the pins are not
    /// contiguous across groups, most groups will have this set to
    /// PAD_BASE_NONE and use contiguous numbering for ACPI.
    pub acpi_pad_base: i32,
}

impl PadGroup {
    pub const fn intel_gpp_base(
        first_of_community: i32,
        start_of_group: u32,
        end_of_group: u32,
        group_pad_base: i32,
    ) -> Self {
        Self {
            first_pad: start_of_group as i32 - first_of_community,
            size: end_of_group - start_of_group + 1,
            acpi_pad_base: group_pad_base,
        }
    }

    pub const fn intel_gpp(
        first_of_community: i32,
        start_of_group: u32,
        end_of_group: u32,
    ) -> Self {
        Self::intel_gpp_base(
            first_of_community,
            start_of_group,
            end_of_group,
            PAD_BASE_NONE,
        )
    }
}

/// A range of consecutive virtual-wire entries in a community
#[repr(C)]
pub struct VwEntries {
    pub first_pad: Gpio,
    pub last_pad: Gpio,
}

/// This structure will be used to describe a community or each group within a
/// community when multiple groups exist inside a community
#[repr(C)]
pub struct PadCommunity<'a, 'b, 'c, 'd> {
    pub name: &'a str,
    pub acpi_path: &'b str,
    /// Number of GPI registers in community
    pub num_gpi_regs: usize,
    /// Number of pads in each group
    /// Number of pads bit mapped in each GPI status/en and Host Own Reg
    pub max_pads_per_group: usize,
    /// First pad in community
    pub first_pad: Gpio,
    /// Last pad in community
    pub last_pad: Gpio,
    /// Offset to Host Ownership Reg 0
    pub host_own_reg_0: u16,
    /// Offset to GPI Int STS Reg 0
    pub gpi_int_sts_reg_0: u16,
    /// Offset to GPI Int Enable Reg 0
    pub gpi_int_en_reg_0: u16,
    /// Offset to GPI SMI STS Reg 0
    pub gpi_smi_sts_reg_0: u16,
    /// Offset to GPI SMI EN Reg 0
    pub gpi_smi_en_reg_0: u16,
    /// Offset to GPI GPE STS Reg 0
    pub gpi_gpe_sts_reg_0: u16,
    /// Offset to GPI GPE EN Reg 0
    pub gpi_gpe_en_reg_0: u16,
    /// Offset to GPI NMI STS Reg 0
    pub gpi_nmi_sts_reg_0: u16,
    /// Offset to GPI NMI EN Reg 0
    pub gpi_nmi_en_reg_0: u16,
    /// Offset to first PAD_GFG_DW0 Reg
    pub pad_cfg_base: u16,
    /// Offset to first PADCFGLOCK Reg
    pub pad_cfg_lock_offset: u16,
    /// Specifies offset in struct GpiStatus
    pub gpi_status_offset: u8,
    /// PCR Port ID
    pub port: u8,
    /// CPU Port ID
    pub cpu_port: u8,
    /// PADRSTCFG logical to chipset mapping
    pub reset_map: &'c [ResetMapping],
    pub groups: &'d [PadGroup],
    pub vw_base: u32,
    /// Note: The entries must be in the same order here as the order in
    /// which they map to VW indexes (beginning with VW base)
    pub vw_entries: Vec<VwEntries>,
}

impl<'a, 'b, 'c, 'd> PadCommunity<'a, 'b, 'c, 'd> {
    pub fn pad_config_offset(&self, pad: Gpio) -> u16 {
        let mut offset = self.relative_pad_in_comm(pad) as u16;
        offset *= gpio_dwx_size(GPIO_NUM_PAD_CFG_REGS as u32) as u16;
        offset + self.pad_cfg_base as u16
    }

    pub fn relative_pad_in_comm(&self, gpio: Gpio) -> usize {
        gpio as usize - self.first_pad as usize
    }

    pub fn gpio_group_index(&self, relative_pad: u32) -> Result<usize, Error> {
        for (i, group) in self.groups.iter().enumerate() {
            if relative_pad >= group.first_pad as u32
                && relative_pad < group.first_pad as u32 + group.size
            {
                return Ok(i);
            }
        }
        //error!(
        //    "{}: pad {} is not found in community {}!",
        //    "gpio_group_index", relative_pad, self.name
        //);
        Err(Error::MissingCommunityPad)
    }

    pub fn gpio_pad_reset_config_override(&self, config_value: u32) -> u32 {
        let rst_map = self.reset_map;
        if rst_map.len() == 0 {
            return config_value;
        }

        for rst in rst_map.iter() {
            if config_value & (PadCfg0Mask::Reset as u32) == rst.logical {
                let mut ret = config_value & !(PadCfg0Mask::Reset as u32);
                ret |= rst.chipset;
                return ret;
            }
        }

        //error!(
        //    "{}: Logical to Chipset mapping not found",
        //    "gpio_pad_reset_config_override"
        //);
        config_value
    }

    pub fn gpio_group_index_scaled(&self, relative_pad: u32, scale: usize) -> Result<usize, Error> {
        Ok(self.gpio_group_index(relative_pad)? * scale)
    }

    pub fn gpio_within_group(&self, relative_pad: u32) -> Result<usize, Error> {
        let i = self.gpio_group_index(relative_pad)?;
        Ok(relative_pad as usize - self.groups[i].first_pad as usize)
    }

    pub fn gpio_bitmask_within_group(&self, relative_pad: u32) -> Result<u32, Error> {
        Ok(1 << self.gpio_within_group(relative_pad)? as u32)
    }

    pub fn gpi_smi_sts_offset(&self, group: u32) -> u32 {
        self.gpi_smi_sts_reg_0 as u32 + (group * size_of::<u32>() as u32)
    }

    pub fn gpi_smi_en_offset(&self, group: u32) -> u32 {
        self.gpi_smi_en_reg_0 as u32 + (group * size_of::<u32>() as u32)
    }

    pub fn gpi_nmi_sts_offset(&self, group: u32) -> u32 {
        self.gpi_nmi_sts_reg_0 as u32 + (group * size_of::<u32>() as u32)
    }

    pub fn gpi_nmi_en_offset(&self, group: u32) -> u32 {
        self.gpi_nmi_en_reg_0 as u32 + (group * size_of::<u32>() as u32)
    }

    pub fn gpi_is_offset(&self, group: u32) -> u32 {
        self.gpi_int_sts_reg_0 as u32 + (group * size_of::<u32>() as u32)
    }

    pub fn gpi_ie_offset(&self, group: u32) -> u32 {
        self.gpi_int_en_reg_0 as u32 + (group * size_of::<u32>() as u32)
    }

    pub fn gpi_gpe_sts_offset(&self, group: u32) -> u32 {
        self.gpi_gpe_sts_reg_0 as u32 + (group * size_of::<u32>() as u32)
    }

    pub fn gpi_gpe_en_offset(&self, group: u32) -> u32 {
        self.gpi_gpe_en_reg_0 as u32 + (group * size_of::<u32>() as u32)
    }

    pub fn gpi_status_offset(&self) -> u32 {
        self.gpi_status_offset as u32
    }
}

/// Provides storage for all GPI status registers from all communities
#[repr(C)]
pub struct GpiStatus {
    pub grp: [u32; NUM_GPI_STATUS_REGS as usize],
}

impl GpiStatus {
    pub const fn new() -> Self {
        Self {
            grp: [0; NUM_GPI_STATUS_REGS as usize],
        }
    }

    pub fn get(&self, mut pad: Gpio) -> Result<u32, Error> {
        let community = &soc_gpio_get_community();
        let comm = gpio_get_community(pad, community)?;

        pad = comm.relative_pad_in_comm(pad) as Gpio;
        let mut sts_index = comm.gpi_status_offset() as usize;
        sts_index += comm.gpio_group_index(pad)?;

        Ok(!!(self.grp[sts_index] & comm.gpio_bitmask_within_group(pad)?))
    }
}

/// Structure provides the pmc to gpio group mapping
#[repr(C)]
pub struct PmcToGpioRoute {
    pub pmc: i32,
    pub gpio: i32,
}
