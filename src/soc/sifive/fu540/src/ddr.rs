use model::*;
use core::ops;

use register::mmio::{ReadOnly, ReadWrite};
use register::{register_bitfields, Field};

#[allow(non_snake_case)]
#[repr(C)]
pub struct RegisterBlock {
}

pub struct ddr {
    base: usize,
}

impl ops::Deref for ddr {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl ddr {
    pub fn new(base: usize, baudrate: u32) -> ddr {
        ddr { base: base}
    }

    /// Returns a pointer to the register block
    fn ptr(&self) -> *const RegisterBlock {
        self.base as *const _
    }
}

impl Driver for ddr {
    fn init(&mut self) {
    }

    fn pread(&self, data: &mut [u8], _offset: usize) -> Result<usize> {
        Ok(0)
    }

    fn pwrite(&mut self, data: &[u8], _offset: usize) -> Result<usize> {
        Ok(0)
    }

    fn shutdown(&mut self) {}
}

// 	ux00ddr_writeregmap(FU540_DDRCTRL, ddr_ctl_settings, ddr_phy_settings);
// 	ux00ddr_disableaxireadinterleave(FU540_DDRCTRL);

// 	ux00ddr_disableoptimalrmodw(FU540_DDRCTRL);

// 	ux00ddr_enablewriteleveling(FU540_DDRCTRL);
// 	ux00ddr_enablereadleveling(FU540_DDRCTRL);
// 	ux00ddr_enablereadlevelinggate(FU540_DDRCTRL);
// 	if (ux00ddr_getdramclass(FU540_DDRCTRL) == DRAM_CLASS_DDR4)
// 		ux00ddr_enablevreftraining(FU540_DDRCTRL);

// 	//mask off interrupts for leveling completion
// 	ux00ddr_mask_leveling_completed_interrupt(FU540_DDRCTRL);

// 	ux00ddr_mask_mc_init_complete_interrupt(FU540_DDRCTRL);
// 	ux00ddr_mask_outofrange_interrupts(FU540_DDRCTRL);
// 	ux00ddr_setuprangeprotection(FU540_DDRCTRL, DDR_SIZE);
// 	ux00ddr_mask_port_command_error_interrupt(FU540_DDRCTRL);

// 	const uint64_t ddr_size = DDR_SIZE;
// 	const uint64_t ddr_end = FU540_DRAM + ddr_size;
// 	ux00ddr_start(FU540_DDRCTRL, FU540_DDRBUSBLOCKER, ddr_end);

// 	ux00ddr_phy_fixup(FU540_DDRCTRL);
// }

// size_t sdram_size_mb(void)
// {
// 	static size_t size_mb = 0;

// 	if (!size_mb) {
// 		// TODO: implement
// 		size_mb = 8 * 1024;
// 	}

// 	return size_mb;
// }
