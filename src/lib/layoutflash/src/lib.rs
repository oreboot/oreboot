use std::io::{self, Seek, SeekFrom, Write};
use std::{env, fs, path::Path};

// NOTE: we don't use u32. At the rate that SPI flash is expanding, we're going to see
// 5B addressing soon I bet. The size limitation should be a function of the destination,
// not this program. This problem should just stupidly arrange things.
#[derive(Debug, PartialEq)]
struct Area {
    pub name: String,
    pub offset: Option<usize>,
    pub size: usize,
    pub file: Option<String>,
}

fn find_fdt<'a>(data: &'a [u8]) -> Result<fdt::Fdt, fdt::FdtError> {
    // The informal standard is that the fdt must be on a 0x1000
    // boundary. It is a fine line between too coarse a boundary
    // and falling into an false positive.
    // yuck. Make a better iterator.
    for pos in 0..data.len() - 0x1000 {
        match fdt::Fdt::new(&data[pos..]) {
            Err(_) => {}
            Ok(fdt) => {
                return Ok(fdt);
            }
        };
    }

    Err(fdt::FdtError::BadMagic)
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
fn layout_flash(path: &Path, areas: Vec<Area>) -> io::Result<()> {
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
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "File {path} is too big to fit into the flash area, file size: {} area size: {}",
                        data.len(),
                        a.size
                    ),
                ));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_create() {
        static DATA: &'static [u8] = include_bytes!("testdata/test.dtb");
        let fdt = fdt::Fdt::new(&DATA).unwrap();
        let areas = create_areas(&fdt).unwrap();

        let want: Vec<Area> = vec![
            Area {
                name: "area@0".to_string(),
                offset: Some(0),
                size: 524288,
                file: None,
            },
            Area {
                name: "area@1".to_string(),
                offset: Some(524288),
                size: 524288,
                file: Some("src/testdata/test.dtb".to_string()),
            },
            Area {
                name: "area@2".to_string(),
                offset: Some(1048576),
                size: 524288,
                file: None,
            },
            Area {
                name: "area@3".to_string(),
                offset: Some(1572864),
                size: 524288,
                file: None,
            },
            Area {
                name: "area@4".to_string(),
                offset: Some(2097152),
                size: 1048576,
                file: None,
            },
            Area {
                name: "area@5".to_string(),
                offset: Some(3145728),
                size: 1048576,
                file: None,
            },
            Area {
                name: "area@6".to_string(),
                offset: Some(4194304),
                size: 6291456,
                file: None,
            },
            Area {
                name: "area@7".to_string(),
                offset: Some(10485760),
                size: 6291456,
                file: None,
            },
        ];
        assert_eq!(areas.len(), want.len());
        for i in 0..areas.len() {
            println!("Check element {i}");
            assert_eq!(
                areas[i].name, want[i].name,
                "Element {i}: name {:?} != {:?}",
                areas[i].name, want[i].name
            );
            assert_eq!(
                areas[i].offset, want[i].offset,
                "Element {i}: offset {:?} , {:?}",
                areas[i].offset, want[i].offset
            );
            assert_eq!(
                areas[i].size, want[i].size,
                "Element {i}: size {:?} , {:?}",
                areas[i].size, want[i].size
            );
            assert_eq!(
                areas[i].file, want[i].file,
                "Element {i}: file {:?} , {:?}",
                areas[i].file, want[i].file
            );
        }

        layout_flash(Path::new("out"), areas).unwrap();
        // Make sure we can read what we wrote.
        let data = fs::read("out").expect("Unable to read file produced by layout");
        let reference = fs::read("src/testdata/test.out").expect("Unable to read testdata file");
        assert_eq!(data, reference, "Data and reference differ");
        let mut vec = Vec::with_capacity(16384);
        vec.resize(16384, 0u8);
        match find_fdt(&vec) {
            Err(_) => {}
            Ok(_) => {
                panic!("Unpacked an FDT from a block of zeros!");
            }
        }
        let fdt = find_fdt(&data).unwrap();
    }
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
