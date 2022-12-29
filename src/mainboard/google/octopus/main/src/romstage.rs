use crate::consts::DRAM_PART_IN_CBI_BOARD_ID_MIN;
use consts::memory_info::DIMM_INFO_PART_NUMBER_SIZE;
use ec::google::chromeec::{ec::cbi_get_dram_part_num, ec_boardid::board_id};
use fsp::FSPM_UPD;
use log::error;
use soc::intel::apollolake::meminit_glk::{save_lpddr4_dimm_info, save_lpddr4_dimm_info_part_num};
use variants::baseboard::memory::{lpddr4_config, memory_sku};

pub fn memory_init_params(memupd: &mut FSPM_UPD) {
    lpddr4_config().meminit_lpddr4_by_sku(&mut memupd.FspmConfig, memory_sku());
}

pub fn save_dimm_info_by_sku_config() {
    save_lpddr4_dimm_info(lpddr4_config(), memory_sku());
}

pub fn save_dimm_info() {
    let mut part_num_store = [0u8; DIMM_INFO_PART_NUMBER_SIZE];

    if cfg!(feature = "dram_part_num_not_always_in_cbi") {
        // Fall back on part numbers encoded in lp4cfg array.
        if (board_id() as i32) < DRAM_PART_IN_CBI_BOARD_ID_MIN {
            save_dimm_info_by_sku_config();
            return;
        }
    }

    if cbi_get_dram_part_num(&mut part_num_store).is_err() {
        error!("Couldn't obtain DRAM part number from CBI\r\n");
        return;
    }

    let part_num_store_str = core::str::from_utf8(part_num_store.as_ref()).unwrap_or("");
    save_lpddr4_dimm_info_part_num(part_num_store_str);
}
