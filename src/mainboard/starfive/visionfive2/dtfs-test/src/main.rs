use layoutflash::areas::{find_fdt, Fdt, FdtIterator, FdtNode};
use std::fs::read;

const DEBUG: bool = false;

fn dump_props(n: &FdtNode, pre: &str) {
    for p in n.properties() {
        let pname = p.name;
        match pname {
            "addr" => {
                let addr = p.as_usize().unwrap_or(0);
                println!("{pre}{pname}: {addr:08x}");
            }
            "size" => {
                let size = p.as_usize().unwrap_or(0);
                println!("{pre}{pname}: {size} (0x{size:x})");
            }
            // Previous attempt: `harts = <1, 2, 3, 4>;`
            // However, the parser turns that into [u8;n*4], BE
            // i.e. [1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0]
            "harts" => {
                // let l = p.value.len();
                // let (_pre, v, _suf) = unsafe { p.value.align_to::<u32>() };
                // This is std...
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
                println!("{pre}{pname}: {vals:?}");
            }
            _ => {
                let str = p.as_str().unwrap_or("[empty]");
                println!("{pre}{pname}: {str}");
            }
        }
    }
}

fn dump_fdt_nodes(fdt: &Fdt, path: &str) {
    let nodes = &mut fdt.find_all_nodes(path);
    println!(" {path}");
    for n in FdtIterator::new(nodes) {
        for c in n.children() {
            let cname = c.name;
            println!("    ↪ {cname}");
            dump_props(&c, "      ");
            for cc in c.children() {
                let ccname = cc.name;
                println!("      ↪ {ccname}");
                dump_props(&cc, "        ");
                for ccc in cc.children() {
                    let cccname = ccc.name;
                    println!("        ↪ {cccname}");
                    dump_props(&ccc, "          ");
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
            for p in c.properties() {
                match p.name {
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
                        if p.name == "compatible" && s == "uboot-main" {
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
    println!("U-Boot @ {r:08x?}");
}
