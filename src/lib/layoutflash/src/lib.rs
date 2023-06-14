use std::io::{self, Seek, SeekFrom, Write};
use std::{env, fs, path::Path};

// NOTE: we don't use u32. At the rate that SPI flash is expanding, we're going to see
// 5B addressing soon I bet. The size limitation should be a function of the destination,
// not this program. This problem should just stupidly arrange things.
struct Area {
    pub name: String,
    pub offset: Option<usize>,
    pub size: usize,
    pub file: Option<String>,
}

// In earlier versions of this function, we assumed all Areas had a non-zero
// offset. There was a sort step to sort by offset as a first step.
// Requiring users to compute all the offsets, and adjust them every time
// something changed size, was incredibly inconvenient.
// Now, we allow a zero offset, but it turns out that sorting is the wrong
// thing to do: users tend to specify the areas in the dts in the order
// they need to be in ROM; further, the rule of "put this area here,
// and everything after it, after it" will not work if we sort: all
// offset 0 items will be placed first!
// So, new rules:
// This method assumes that the areas are specified in order for a reason.
// In most cases, the offset is 0, meaning "use the previous area end+1
// as our offset". In rare cases, an offset is specified; for that, ensure
// there is no overlap with previous Areas. All Areas after this Area will
// be placed after it. That way, only a limited number of offsets need
// to be specified, possibly even 0, and the order in the ROM image will be the order
// specified in the DTS.
fn layout_flash(path: &Path, areas: &mut Vec<Area>) -> io::Result<()> {
    let mut f = fs::File::create(path)?;
    let mut last_area_end = 0;
    for a in areas {
        println!("Area {:?}: @{:?}, size {:?}", a.name, a.offset, a.size);
        let offset = match a.offset {
            Some(x) => x,
            None => last_area_end,
        };
        if offset < last_area_end {
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Areas are overlapping, last area finished at offset {}, next area '{}' starts at {}", last_area_end, a.name, offset)));
        }
        last_area_end = offset + a.size;

        println!("<{}> @ 0x{:x}", a.name, last_area_end);
        // First fill with 0xff.
        let mut v = Vec::new();
        v.resize(a.size as usize, 0xff);
        f.seek(SeekFrom::Start(offset as u64))?;
        f.write_all(&v)?;

        // If a file is specified, write the file.
        if let Some(path) = &a.file {
            let mut path = path.to_string();
            // Allow environment variables in the path.
            for (key, value) in env::vars() {
                path = str::replace(&path, &format!("$({})", key), &value);
            }

            // If the path is an unused environment variable, skip it.
            if path.starts_with("$(") && path.ends_with(')') {
                continue;
            }

            f.seek(SeekFrom::Start(offset as u64))?;
            let data = match fs::read(&path) {
                Err(e) => {
                    return Err(io::Error::new(
                        e.kind(),
                        format!("Could not open: {}", path),
                    ))
                }
                Ok(data) => data,
            };
            if data.len() > a.size as usize {
                return Err(io::Error::new(io::ErrorKind::InvalidData, format!("File {} is too big to fit into the flash area, file size: {}, area size: {}", path, data.len(), a.size)));
            }
            f.write_all(&data)?;
        }
    }
    Ok(())
}

fn create_areas(fdt: &fdt::Fdt) -> io::Result<Vec<Area>> {
    // Assemble the bits of the fdt we care about into Areas.
    let mut areas: Vec<Area> = vec![];

    for node in fdt.find_all_nodes("/flash-info/areas") {
        println!("{:?}", node.name);
        for child in node.children() {
            println!("    {}", child.name);
            let mut a: Area = Area {
                name: child.name.to_string(),
                offset: None,
                size: 0,
                file: None,
            };
            for p in child.properties() {
                println!(" {:?} {:?}, {:?}", p.name, p.as_str(), p.as_usize());

                // There can be all kinds of properties in a node.
                // we only care about file, size, and offset.
                // Not that we remove any, just that those relate
                // to data we put in the image.

                match p.name {
                    "file" => {
                        a.file = Some(
                            p.as_str()
                                .expect(
                                    format!("Child {}: \"file\" needs a name", child.name).as_str(),
                                )
                                .to_string(),
                        );
                    }
                    "offset" => {
                        a.offset = Some(p.as_usize().unwrap());
                    }
                    "size" => {
                        a.size = p.as_usize().unwrap();
                    }
                    _ => {}
                }
            }
            areas.push(a);
        }
    }

    Ok(areas)
}

#[test]
fn readit() {
    static DATA: &'static [u8] = include_bytes!("testdata/test.dtb");
    fdt::Fdt::new(&DATA).unwrap();
}

/*
fn main() {
    let args = Opts::parse();

    println!("Read in {:?}", args.in_fdt);
    let data = fs::read(&args.in_fdt).unwrap();

    create_areas(&fdt)
        .and_then(|mut areas| layout_flash(&args.out_firmware, &mut areas))
        .unwrap_or_else(|err| {
            eprintln!("failed: {}", err);
            exit(1);
        });
}
*/
