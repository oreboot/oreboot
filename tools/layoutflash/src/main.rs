#![deny(warnings)]
use device_tree::{infer_type, Entry, FdtReader, Type, MAX_NAME_SIZE};
use model::Driver;
use std::io;
use std::io::{Seek, SeekFrom, Write};
use std::process::exit;
use std::{
    env, fs,
    path::{Path, PathBuf},
};
use structopt::StructOpt;
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
fn read_fixed_fdt(path: &Path) -> io::Result<Vec<Area>> {
    let data = match fs::read(path) {
        Err(e) => return Err(io::Error::new(e.kind(), format!("{}{}", "Could not open: ", path.display()))),
        Ok(data) => data,
    };
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

    Ok(areas)
}

// This method assumes that areas are sorted by offset.
fn layout_flash(path: &Path, areas: &mut [Area]) -> io::Result<()> {
    areas.sort_unstable_by_key(|a| a.offset);
    let mut f = fs::File::create(path)?;
    let mut last_area_end = 0;
    for a in areas {
        if a.offset < last_area_end {
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Areas are overlapping, last area finished at offset {}, next area '{}' starts at {}", last_area_end, a.description, a.offset)));
        }
        last_area_end = a.offset + a.size;

        // First fill with 0xff.
        let mut v = Vec::new();
        v.resize(a.size as usize, 0xff);
        f.seek(SeekFrom::Start(a.offset as u64))?;
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

            f.seek(SeekFrom::Start(a.offset as u64))?;
            let data = match fs::read(&path) {
                Err(e) => return Err(io::Error::new(e.kind(), format!("Could not open: {}", path))),
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

#[derive(StructOpt)]
struct Opts {
    /// The path to the firmware device tree file
    in_fdt: PathBuf,
    #[structopt(parse(from_os_str))]
    /// The output path for the firmware
    out_firmware: PathBuf,
}

fn main() {
    let args = Opts::from_args();

    read_fixed_fdt(&args.in_fdt).and_then(|mut areas| layout_flash(&args.out_firmware, &mut areas)).unwrap_or_else(|err| {
        eprintln!("failed: {}", err);
        exit(1);
    });
}
