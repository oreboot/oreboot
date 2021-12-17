use clap::Clap;
use device_tree::area::{Area, read_areas};
use std::io;
use std::io::{Seek, SeekFrom, Write};
use std::process::exit;
use std::{
    env, fs,
    path::{Path, PathBuf},
};
use wrappers::SliceReader;

fn read_fixed_fdt(path: &Path) -> io::Result<Vec<Area>> {
    let data = match fs::read(path) {
        Err(e) => {
            return Err(io::Error::new(
                e.kind(),
                format!("{}{}", "Could not open: ", path.display()),
            ))
        }
        Ok(data) => data,
    };
    let driver = SliceReader::new(data.as_slice());

    // no error returned from helper
    let areas = read_areas(&driver).unwrap();

    Ok(areas.to_vec())
}

// This method assumes that areas are sorted by offset.
fn layout_flash(path: &Path, areas: &mut [Area]) -> io::Result<()> {
    areas.sort_unstable_by_key(|a| a.offset);
    let mut f = fs::File::create(path)?;
    let mut last_area_end = 0;
    for a in areas {
        let offset = match a.offset {
            Some(x) => x,
            None => last_area_end,
        };
        if offset < last_area_end {
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Areas are overlapping, last area finished at offset {}, next area '{}' starts at {}", last_area_end, a.description, offset)));
        }
        last_area_end = offset + a.size;

        println!("<{}> @ 0x{:x}", a.description, last_area_end);
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

#[derive(Clap)]
#[clap(version)]
struct Opts {
    /// The path to the firmware device tree file
    in_fdt: PathBuf,
    #[clap(parse(from_os_str))]
    /// The output path for the firmware
    out_firmware: PathBuf,
}

fn main() {
    let args = Opts::parse();

    read_fixed_fdt(&args.in_fdt)
        .and_then(|mut areas| layout_flash(&args.out_firmware, &mut areas))
        .unwrap_or_else(|err| {
            eprintln!("failed: {}", err);
            exit(1);
        });
}
