use crate::intel::apollolake::meminit::Lpddr4Cfg;
use log::error;

pub const FSP_SMBIOS_MEMORY_INFO_GUID: [u8; 16] = [
    0x8c, 0x10, 0xa1, 0x01, 0xee, 0x9d, 0x84, 0x49, 0x88, 0xc3, 0xee, 0xe8, 0xc4, 0x9e, 0xfb, 0x89,
];

// FIXME: implement after importing FSP sources and impl of CBMEM
pub fn save_lpddr4_dimm_info_part_num(_dram_part_num: &str) {
    /*
    let smbios_memory_info_guid = FSP_SMBIOS_MEMORY_INFO_GUID;

    let full_dram_part_num = if dram_part_num = "" {
        "Unknown"
    } else {
        dram_part_num
    };

    // Locate the memory info HOB
    let memory_info_hob = fsp_find_extension_hob_by_guid(smbios_memory_info_guid.as_ref());

    if memory_info_hob.len() == 0 {
        error!("SMBIOS memory info HOB is missing\r\n");
        return;
    }

    // Allocate CBMEM area for DIMM information used to populate SMBIOS
    // table 17
    let mut mem_info = cbmem_add(CBMEM_ID_MEMINFO, size_of::<MemoryInfo>());
    if mem_info {
         error!("CBMEM entry for DIMM info missing\r\n");
         return;
    }
    for m in mem_info.iter_mut() {
        *m = 0;
    }
    let mut index = 0;
    let dimm_max = mem_info.dimm.len();

    for node in 0..MAX_NODE_NUM {
        let ctrl_info = &memory_info_hob.Controller[node];
        for channel in 0..ctrl_info.ChannelCount {
            if index >= dimm_max {
                break;
            }

            let channel_info = &ctrl_info->ChannelInfo[channel];

            for dimm in 0..channel_info.DimmCount {
                if index >= dimm_max {
                    break;
                }
                let src_dimm = &channel_info.DimmInfo[dimm];
                let dest_dim = &mem_info.dimm[index];

                if src_dimm.DimmCapacity == 0 {
                    continue;
                }

                dimm_info_fill(dest_dimm,
                               src_dimm.DimmCapacity,
                               memory_info_hob.MemoryType,
                               memory_info_hob.ConfiureMemoryClockSpeed,
                               src_dimm.RankInDimm,
                               channel_info.ChannelId,
                               src_simm.DimmId,
                               dram_part_num,
                               dram_part_num.len(),
                               src_dimm.SpdSave + SPD_SAVE_OFFSET_SERIAL,
                               memory_info_hob.DataWidth,
                               0,
                               0,
                               src_dimm.MfgId,
                               src_dimm.SpdModuleType,
                               node);
                index += 1;
            }
        }
    }
    mem_info.dimm_cnt = index;
    debug!("{} DIMMs found\r\n", mem_info.dimm_cnt);
    */
}

pub fn save_lpddr4_dimm_info(lp4cfg: &Lpddr4Cfg, mem_sku: usize) {
    let mut part_num = "";

    if mem_sku >= lp4cfg.skus.len() {
        error!(
            "Too few LPDDR4 SKUs: 0x{:x}/0x{:x}\r\n",
            mem_sku,
            lp4cfg.skus.len()
        );
    } else {
        part_num = lp4cfg.skus[mem_sku].part_num;
    }

    save_lpddr4_dimm_info_part_num(part_num);
}
