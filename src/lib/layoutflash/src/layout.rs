use std::prelude::rust_2021::*;

use std::io::{self, Seek, SeekFrom, Write};
use std::{env, fs, path::Path};

use crate::areas::Area;

/// Create the areas from the FDT.
pub fn create_areas<'a>(fdt: &'a fdt::Fdt<'a>) -> Result<Vec<Area<'a>>, String> {
    let mut areas: Vec<Area> = vec![];
    let mut offset = 0;
    for node in fdt.find_all_nodes("/flash-info/areas") {
        for c in node.children() {
            let Some(size) = c.property("size").map_or_else(|| None, |e| e.as_usize()) else {
                return Err("a size MUST be provided".to_string());
            };

            areas.push(Area {
                name: c.name,
                stage: c.property("stage").map_or_else(|| None, |e| e.as_str()),
                file: c.property("file").map_or_else(|| None, |e| e.as_str()),
                offset: c
                    .property("offset")
                    .map_or_else(|| Some(offset), |e| e.as_usize()),
                size,
            });
            if let Some(o) = c.property("offset") {
                offset = o.as_usize().unwrap();
            }
            offset += c.property("size").unwrap().as_usize().unwrap();
        }
    }
    Ok(areas)
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
pub fn layout_flash(dir: &Path, path: &Path, areas: Vec<Area>) -> io::Result<()> {
    let mut f = fs::File::create(path)?;
    let mut last_area_end = 0;
    for a in areas {
        println!("{a:#?}");
        let name = a.name;
        let size = a.size;
        let offset = a.offset.unwrap_or(last_area_end);
        if offset < last_area_end {
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Areas are overlapping, last area finished at offset {last_area_end}, next area '{name}' starts at {offset}")));
        }
        last_area_end = offset + size;

        println!("<{name}> @ 0x{last_area_end:x}");
        // First fill with 0xff.
        let mut v = Vec::new();
        v.resize(size, 0xff);
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
            if path.starts_with("$(") && path.ends_with(")") {
                continue;
            }

            f.seek(SeekFrom::Start(offset as u64))?;

            let path = Path::new(&path);
            let image_path = if path.is_absolute() {
                path.to_path_buf()
            } else {
                dir.join(path)
            };
            println!("Read file {image_path:?}");
            let data = match fs::read(&image_path) {
                Err(e) => {
                    return Err(io::Error::new(
                        e.kind(),
                        format!("Could not open: {image_path:?}"),
                    ))
                }
                Ok(data) => data,
            };
            let file_size = data.len();
            if file_size > size {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "File {image_path:?} is too big to fit into the flash area, file size: {file_size} area size: {size}",
                    ),
                ));
            }
            f.write_all(&data)?;
        }
    }
    Ok(())
}

#[test]
fn no_file() {
    let dtfs = include_bytes!("testdata/no_file.dtb");
    let fdt = fdt::Fdt::new(dtfs).unwrap();
    let areas = create_areas(&fdt);
    assert!(areas.is_ok())
}

#[test]
fn no_offset() {
    let dtfs = include_bytes!("testdata/no_offset.dtb");
    let fdt = fdt::Fdt::new(dtfs).unwrap();
    let areas = create_areas(&fdt);
    assert!(areas.is_ok())
}

/// Size is a MUST.
#[test]
fn no_size() {
    let dtfs = include_bytes!("testdata/no_size.dtb");
    let fdt = fdt::Fdt::new(dtfs).unwrap();
    let areas = create_areas(&fdt);
    assert!(areas.is_err())
}

// This is the same as in `test.dtb` itself (see `./testdata/test.dts`).
#[cfg(test)]
const AREAS_FIXTURE: [Area; 4] = [
    Area {
        name: "area@0",
        stage: Some("bt0"),
        offset: Some(0),
        size: 512 * 1024,
        file: None,
    },
    Area {
        name: "area@1",
        stage: None,
        offset: Some(512 * 1024),
        size: 512 * 1024,
        file: Some("src/testdata/test.dtb"),
    },
    Area {
        name: "area@2",
        stage: Some("main"),
        offset: Some(1024 * 1024),
        size: 1024 * 1024,
        file: None,
    },
    Area {
        name: "area@3",
        stage: Some("payload"),
        offset: Some(2 * 1024 * 1024),
        size: 14 * 1024 * 1024,
        file: None,
    },
];

/// Fixtures generated via:
/// ```sh
/// `dtc -o src/testdata/test.dt{b,s}`
/// # https://remysharp.com/2020/06/17/how-to-pad-a-file-with-specific-bytes
/// `tr '\0' '\377' < /dev/zero | head -c 16777216 > src/testdata/test.out`
/// `dd if=src/testdata/test.dtb bs=1024 seek=512 conv=notrunc of=src/testdata/test.out`
/// ```
#[test]
fn read_create() {
    use crate::areas::find_fdt;
    let fdt = fdt::Fdt::new(crate::areas::TEST_DTB).unwrap();
    let areas = create_areas(&fdt).unwrap();
    let want = AREAS_FIXTURE;
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

    // This is relative to from where `cargo test` is run.
    let image = Path::new("out.bin");
    layout_flash(Path::new("."), image, areas.to_vec()).unwrap();

    // Make sure we can read what we wrote and got the expected result.
    let data = fs::read(&image).expect("Unable to read file produced by layout");
    // NOTE: Do NOT use assert_eq! here. If it fails, it prints all the bytes.
    assert!(
        data.eq(crate::areas::IMAGE_FIXTURE),
        "Data and reference differ"
    );
    let mut vec = Vec::with_capacity(16384);
    vec.resize(16384, 0u8);
    match find_fdt(&vec) {
        Err(_) => {}
        Ok(_) => {
            panic!("Unpacked an FDT from a block of zeros!");
        }
    }
    let fdt = find_fdt(&data);
    assert!(fdt.is_ok());
}
