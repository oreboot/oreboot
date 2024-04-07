use layoutflash::areas::{find_fdt, Fdt, FdtIterator};
use std::{error::Error, fs::read};

const DEBUG: bool = false;

fn dump_fdt_nodes(fdt: &Fdt, path: &str) {
    let nodes = &mut fdt.find_all_nodes(path);
    println!(" {path}");
    for n in FdtIterator::new(nodes) {
        for c in n.children() {
            let cname = c.name;
            println!("    ↪ {cname}");
            for cc in c.children() {
                let ccname = cc.name;
                println!("      ↪ {ccname}");
                for ccc in cc.children() {
                    let cccname = ccc.name;
                    println!("        ↪ {cccname}");
                    for p in ccc.properties() {
                        let pname = p.name;
                        match pname {
                            "size" => {
                                let size = p.as_usize().unwrap_or(0);
                                println!("          {pname}: {size} (0x{size:x})");
                            }
                            "addr" => {
                                let size = p.as_usize().unwrap_or(0);
                                println!("          {pname}: {size:08x}");
                            }
                            _ => {
                                let str = p.as_str().unwrap_or("[empty]");
                                println!("          {pname}: {str}");
                            }
                        }
                    }
                }
                for p in cc.properties() {
                    let pname = p.name;
                    match pname {
                        "size" => {
                            let size = p.as_usize().unwrap_or(0);
                            println!("        {pname}: {size} (0x{size:x})");
                        }
                        "addr" => {
                            let size = p.as_usize().unwrap_or(0);
                            println!("        {pname}: {size:08x}");
                        }
                        _ => {
                            let str = p.as_str().unwrap_or("[empty]");
                            println!("        {pname}: {str}");
                        }
                    }
                }
            }
            for p in c.properties() {
                let pname = p.name;
                match pname {
                    "size" => {
                        let size = p.as_usize().unwrap_or(0);
                        println!("      {pname}: {size} (0x{size:x})");
                    }
                    "addr" => {
                        let size = p.as_usize().unwrap_or(0);
                        println!("      {pname}: {size:08x}");
                    }
                    "harts" => {
                        let l = p.value.len();
                        // This is std...
                        /*
                        let vals: Vec<u32> = p
                            .value
                            .chunks(4)
                            .map(|c| {
                                let mut v: u32 = 0;
                                // These are always 4 u8 each.
                                for (i, e) in c.iter().enumerate() {
                                    if i < 4 {
                                        v |= (*e as u32) << (24 - i * 8);
                                    }
                                }
                                v
                            })
                            .collect();
                        */
                        let (pre, v, suf) = unsafe { p.value.align_to::<u32>() };
                        let vals: Vec<u32> = p
                            .value
                            .chunks(4)
                            .map(|c| {
                                let mut v: u32 = 0;
                                // These are always 4 u8 each.
                                for (i, e) in c.iter().enumerate() {
                                    if i < 4 {
                                        v |= (*e as u32) << (24 - i * 8);
                                    }
                                }
                                v
                            })
                            .collect();
                        println!("      {pname}: {vals:?}");
                    }
                    _ => {
                        let str = p.as_str().unwrap_or("[empty]");
                        println!("      {pname}: {str}");
                    }
                }
            }
        }
    }
}

fn get_uboot_offset_and_size(fdt: &Fdt) -> (usize, usize) {
    let mut offset = 0;
    let mut found = false;
    let mut size = 0;
    let areas = &mut fdt.find_all_nodes("/flash-info/areas");
    // TODO: make finding more sophisticated
    for a in FdtIterator::new(areas) {
        for c in a.children() {
            let cname = c.name;
            for p in c.properties() {
                let pname = p.name;
                match pname {
                    "size" => {
                        let psize = p.as_usize().unwrap_or(0);
                        if !found {
                            if DEBUG {
                                println!("No U-Boot yet, inc offset by 0x{psize:x}");
                            }
                            offset += psize;
                        }
                        if found && size == 0 {
                            size = psize;
                        }
                    }
                    _ => {
                        let s = p.as_str().unwrap_or("[empty]");
                        if pname == "compatible" && s == "uboot-main" {
                            found = true;
                        }
                    }
                }
            }
        }
    }
    // NOTE: When in SRAM, the header is cut off!
    offset = offset - 0x400;
    (offset, size)
}

fn dump_fdt_board_info(fdt: &Fdt) {
    let nodes = &mut fdt.find_all_nodes("/board-info");
    println!("ℹ️ Board information");
    for n in FdtIterator::new(nodes) {
        for p in n.properties() {
            let pname = p.name;
            let s = p.as_str().unwrap_or("[empty]");
            println!("  {pname}: {s}");
        }
    }
}

fn find_and_process_dtfs(slice: &[u8]) -> Result<(usize, usize), &str> {
    if let Ok(fdt) = find_fdt(slice) {
        dump_fdt_board_info(&fdt);
        println!("💾 DTFS");
        dump_fdt_nodes(&fdt, "/flash-info/areas");
        dump_fdt_nodes(&fdt, "/load-info");
        dump_fdt_nodes(&fdt, "/load-info/clusters");
        let (offset, size) = get_uboot_offset_and_size(&fdt);
        Ok((offset, size))
    } else {
        Err("DTFS blob not found")
    }
}

fn main() {
    let f = read("./fixture.dtb").unwrap();
    let s = f.as_slice();
    let r = find_and_process_dtfs(s).unwrap();
    println!("{r:08x?}");
}
