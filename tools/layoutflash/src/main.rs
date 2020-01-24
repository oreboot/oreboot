#![deny(warnings)]
use device_tree::{infer_type, Entry, FdtReader, Type, MAX_NAME_SIZE};
use model::Driver;
use std::io;
use std::io::{Seek, SeekFrom, Write};
use std::process::exit;
use std::{env, fs};
use wrappers::SliceReader;

// TODO: Move this struct to lib so it can be used at runtime.
#[derive(Default, Debug)]
struct Area {
    description: String,
    compatible: String,
    offset: u32,
    size: u32,
    file: Option<String>,
}

// TODO: Move to some common library.
fn read_all(d: &dyn Driver) -> Vec<u8> {
    let mut v = Vec::new();
    v.resize(MAX_NAME_SIZE, 0);
    // Safe to unwrap because SliceReader does not return an error.
    let size = d.pread(v.as_mut_slice(), 0).unwrap();
    v.truncate(size);
    v
}

// TODO: Move this function to lib so it can be used at runtime.
fn read_fixed_fdt(path: &str) -> Vec<Area> {
    let data = fs::read(path).expect(&format!("could not read {}", path)[..]);
    let driver = SliceReader::new(data.as_slice());

    let mut areas = Vec::new();
    for item in FdtReader::new(&driver).unwrap().walk() {
        // TODO: We really need a better iterator for this.
        match item {
            Entry::Node { path } => {
                if path.name().starts_with("area@") {
                    areas.push(Area { ..Default::default() });
                }
            }
            Entry::Property { path, value } => {
                let data = read_all(&value);
                if let Some(a) = areas.last_mut() {
                    match (path.name(), infer_type(data.as_slice())) {
                        ("description", Type::String(x)) => a.description = String::from(x),
                        ("compatible", Type::String(x)) => a.compatible = String::from(x),
                        ("offset", Type::U32(x)) => a.offset = x,
                        ("size", Type::U32(x)) => a.size = x,
                        ("file", Type::String(x)) => a.file = Some(String::from(x)),
                        (_, _) => {}
                    }
                }
            }
        }
    }
    areas
}

fn layout_flash(path: &str, areas: &Vec<Area>) -> io::Result<()> {
    let mut f = fs::File::create(path).expect(&format!("Can not create {}", path)[..]);
    for a in areas {
        // First fill with 0xff.
        let mut v = Vec::new();
        v.resize(a.size as usize, 0xff);
        f.seek(SeekFrom::Start(a.offset as u64))?;
        f.write(&v)?;

        // If a file is specified, write the file.
        if let Some(path) = &a.file {
            f.seek(SeekFrom::Start(a.offset as u64))?;
            let mut data = fs::read(path).expect(&format!("Can not read {}", path)[..]);
            if data.len() > a.size as usize {
                eprintln!("warning: truncating {}", a.description);
                data.truncate(a.size as usize);
            }
            f.write(&data)?;
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (in_path, out_path) = match args.as_slice() {
        [_, in_path, out_path] => (in_path, out_path),
        _ => {
            eprintln!("Usage: layoutflash <in-fdt> <out-firmware>");
            std::process::exit(1);
        }
    };

    let areas = read_fixed_fdt(in_path);
    if let Err(err) = layout_flash(out_path, &areas) {
        eprintln!("failed: {}", err);
        exit(1);
    }
}
