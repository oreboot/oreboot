mod gdb_detect;
mod target;
mod util;

mod starfive;
mod sunxi;

use std::{
    env, fs,
    io::{self, Seek, SeekFrom, Write},
    path::Path,
    process,
    str::FromStr,
};

use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use log::{error, info};

use layoutflash::areas::Area;

#[derive(Parser)]
#[clap(name = "xtask")]
#[clap(about = "Program that help you build and debug Oreboot project", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
    #[clap(flatten)]
    env: Env,
    #[clap(flatten)]
    verbose: Verbosity,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Make this project
    Make,
    /// Build flash and burn into target
    Flash,
    /// View assembly code
    Asm,
    /// Debug code using gdb
    Gdb,
}

#[derive(Clone)]
enum Memory {
    /// Operate on NAND flash
    Nand,
    /// Operate on NOR flash
    Nor,
}

impl FromStr for Memory {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "nand" => Ok(Self::Nand),
            "nor" => Ok(Self::Nor),
            others => Err(format!("unknown memory type {others}")),
        }
    }
}

#[derive(Args)]
struct Env {
    #[clap(
        long = "release",
        global = true,
        help = "Build in release mode",
        long_help = None,
    )]
    release: bool,
    #[clap(
        long = "supervisor",
        global = true,
        help = "Build with arch-specific supervisor support",
        long_help = None,
    )]
    supervisor: bool,
    #[clap(long, global = true, help = "Mainboard to build")]
    mainboard: Option<String>,
    #[clap(long, global = true, help = "Target memory description")]
    memory: Option<Memory>,
    #[clap(long, global = true, help = "Board variant")]
    variant: Option<String>,
    #[clap(long, global = true, help = "Path to raw payload")]
    payload: Option<String>,
    #[clap(long, global = true, help = "Path to dtb")]
    dtb: Option<String>,
}

fn main() {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();
    info!("=== oreboot build system ===");
    let cur_path = env::current_dir().unwrap();
    let target = if let Some(target) = target::parse_target(
        &cur_path,
        args.env.mainboard.as_deref(),
        args.env.variant.as_deref(),
    ) {
        target
    } else {
        error!(
            "can't decide target for task
    Change directory to mainboard and run again, or
    use `--mainboard <VENDOR/BOARD>` and `--variant <VARIANT>` when necessary."
        );
        process::exit(1)
    };
    target.execute_command(&args);
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
fn layout_flash(dir: &Path, path: &Path, areas: Vec<Area>) -> io::Result<()> {
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
            if path.starts_with("$(") && path.ends_with(")") {
                continue;
            }

            f.seek(SeekFrom::Start(offset as u64))?;

            let data = {
                if !Path::new(&path).is_absolute() {
                    fs::read(&dir.join(&path))
                } else {
                    fs::read(&path)
                }
            };
            let data = match data {
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

#[test]
fn read_create() {
    static DATA: &'static [u8] = include_bytes!("testdata/test.dtb");
    let fdt = fdt::Fdt::new(&DATA).unwrap();
    let mut areas: Vec<Area> = vec![];
    areas.resize(
        8,
        Area {
            name: "",
            offset: None,
            size: 0,
            file: None,
        },
    );
    let areas = create_areas(&fdt, &mut areas);
    let want: Vec<Area> = vec![
        Area {
            name: "area@0",
            offset: Some(0),
            size: 524288,
            file: None,
        },
        Area {
            name: "area@1",
            offset: Some(524288),
            size: 524288,
            file: Some("src/testdata/test.dtb"),
        },
        Area {
            name: "area@2",
            offset: Some(1048576),
            size: 524288,
            file: None,
        },
        Area {
            name: "area@3",
            offset: Some(1572864),
            size: 524288,
            file: None,
        },
        Area {
            name: "area@4",
            offset: Some(2097152),
            size: 1048576,
            file: None,
        },
        Area {
            name: "area@5",
            offset: Some(3145728),
            size: 1048576,
            file: None,
        },
        Area {
            name: "area@6",
            offset: Some(4194304),
            size: 6291456,
            file: None,
        },
        Area {
            name: "area@7",
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

    layout_flash(Path::new("."), Path::new("out"), areas.to_vec()).unwrap();
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
