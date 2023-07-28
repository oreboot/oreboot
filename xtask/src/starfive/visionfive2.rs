use crate::dist_dir;
use crate::layout_flash;
use crate::project_root;
use crate::{Commands, Env};
use fdt;
use log::{error, info, trace};
use std::fs::File;
use std::io::{self, Seek, SeekFrom};
use std::process::{self, Command, Stdio};

use std::{env, fs, path::Path};
extern crate layoutflash;
use layoutflash::areas::{create_areas, Area};

use super::visionfive2_hdr::spl_create_hdr;

const SRAM0_SIZE: u64 = 64 * 1024;

const DEFAULT_TARGET: &str = "riscv64imac-unknown-none-elf";

pub(crate) fn execute_command(args: &crate::Cli, features: Vec<String>) {
    match args.command {
        Commands::Make => {
            info!("building VisionFive2");
            let binutils_prefix = find_binutils_prefix_or_fail();
            // bt0 stage
            xtask_build_jh7110_flash_bt0(&args.env, &features);
            xtask_binary_jh7110_flash_bt0(binutils_prefix, &args.env);
            // main stage
            //  xtask_build_jh7110_flash_main(&args.env);
            //  xtask_binary_jh7110_flash_main(binutils_prefix, &args.env);
            //  xtask_concat_flash_binaries(&args.env);
            // dtb
            xtask_build_dtb(&args.env);
            xtask_build_dtb_image(&args.env);
            // add funny header :-)
            xtask_add_bt0_header(&args.env);
        }
        _ => {
            error!("command {:?} not implemented", args.command);
        }
    }
}

fn xtask_build_dtb(env: &Env) {
    trace!("build dtb");
    let cwd = dist_dir(env, DEFAULT_TARGET);
    let mut command = Command::new("dtc");
    command.current_dir(cwd);
    command.arg("-o");
    command.arg(BOARD_DTB);
    command.arg(board_project_root().join("board.dts"));
    let status = command.status().unwrap();
    trace!("dtc returned {}", status);
    if !status.success() {
        error!("dtc build failed with {}", status);
        process::exit(1);
    }
}

const BT0_BIN: &str = "starfive-visionfive2-bt0.bin";
const BT0_STAGE: &str = "starfive-visionfive2-bt0";

const MAIN_BIN: &str = "starfive-visionfive2-main.bin";
const MAIN_STAGE: &str = "starfive-visionfive2-main";

const BOARD_DTB: &str = "starfive-visionfive2-board.dtb";
const FDT_BIN: &str = "starfive-visionfive2-board.fdtbin";

const SRAM_IMAGE: &str = "starfive-visionfive2-sram.bin";

const IMAGE_BIN: &str = "starfive-visionfive2.bin";

fn xtask_add_bt0_header(env: &Env) {
    let cwd = dist_dir(env, DEFAULT_TARGET);
    let fdt_bin_path = cwd.join(FDT_BIN);
    trace!("add wacky header to {fdt_bin_path:?}");
    let dat = fs::read(fdt_bin_path).expect("SRAM image");
    let out = spl_create_hdr(dat);
    let out_path = cwd.join(SRAM_IMAGE);
    fs::write(out_path.clone(), out).expect("writing output");

    println!("======= DONE =======");
    println!("Output file: {:?}", &out_path.into_os_string());
}

fn xtask_build_jh7110_flash_bt0(env: &Env, features: &Vec<String>) {
    trace!("build JH7110 flash bt0");
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    trace!("found cargo at {}", cargo);
    let mut command = Command::new(cargo);
    command.current_dir(board_project_root().join("bt0"));
    command.arg("build");
    if env.release {
        command.arg("--release");
    }
    if features.len() != 0 {
        let command_line_features = features.join(",");
        trace!("append command line features: {}", command_line_features);
        command.arg("--no-default-features");
        command.args(&["--features", &command_line_features]);
    } else {
        trace!("no command line features appended");
    }
    let status = command.status().unwrap();
    trace!("cargo returned {}", status);
    if !status.success() {
        error!("cargo build failed with {}", status);
        process::exit(1);
    }
}

fn xtask_binary_jh7110_flash_bt0(prefix: &str, env: &Env) {
    trace!("objcopy binary, prefix: '{}'", prefix);
    let status = Command::new(format!("{}objcopy", prefix))
        .current_dir(dist_dir(env, DEFAULT_TARGET))
        .arg(BT0_STAGE)
        .arg("--binary-architecture=riscv64")
        .arg("--strip-all")
        .args(["-O", "binary", BT0_BIN])
        .status()
        .unwrap();

    trace!("objcopy returned {}", status);
    if !status.success() {
        error!("objcopy failed with {}", status);
        process::exit(1);
    }
}

fn xtask_build_jh7110_flash_main(env: &Env) {
    trace!("build JH7110 flash main");
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    trace!("found cargo at {}", cargo);
    let mut command = Command::new(cargo);
    command.current_dir(board_project_root().join("main"));
    command.arg("build");
    if env.release {
        command.arg("--release");
    }
    let status = command.status().unwrap();
    trace!("cargo returned {}", status);
    if !status.success() {
        error!("cargo build failed with {}", status);
        process::exit(1);
    }
}

fn xtask_binary_jh7110_flash_main(prefix: &str, env: &Env) {
    trace!("objcopy binary, prefix: '{}'", prefix);
    let status = Command::new(format!("{}objcopy", prefix))
        .current_dir(dist_dir(env, DEFAULT_TARGET))
        .arg(MAIN_STAGE)
        .arg("--binary-architecture=riscv64")
        .arg("--strip-all")
        .args(["-O", "binary", MAIN_BIN])
        .status()
        .unwrap();

    trace!("objcopy returned {}", status);
    if !status.success() {
        error!("objcopy failed with {}", status);
        process::exit(1);
    }
}

fn xtask_concat_flash_binaries(env: &Env) {
    let dist_dir = dist_dir(env, DEFAULT_TARGET);
    let mut bt0_file = File::options()
        .read(true)
        .open(dist_dir.join(BT0_BIN))
        .expect("open bt0 binary file");
    let mut main_file = File::options()
        .read(true)
        .open(dist_dir.join(MAIN_BIN))
        .expect("open main binary file");

    let output_file_path = dist_dir.join(IMAGE_BIN);
    let mut output_file = File::options()
        .write(true)
        .create(true)
        .open(&output_file_path)
        .expect("create output binary file");

    output_file.set_len(SRAM0_SIZE).unwrap(); // FIXME: depend on storage

    let bt0_len = 30 * 1024;
    io::copy(&mut bt0_file, &mut output_file).expect("copy bt0 binary");
    output_file
        .seek(SeekFrom::Start(bt0_len))
        .expect("seek after bt0 copy");
    io::copy(&mut main_file, &mut output_file).expect("copy main binary");

    println!("======= DONE =======");
    println!("Output file: {:?}", &output_file_path.into_os_string());
}

fn xtask_build_dtb_image(env: &Env) {
    let dist_dir = dist_dir(env, DEFAULT_TARGET);

    let dtb_path = dist_dir.join(BOARD_DTB);
    let dtb = fs::read(dtb_path).expect("dtb");

    let output_file_path = dist_dir.join(FDT_BIN);
    let output_file = File::options()
        .write(true)
        .create(true)
        .open(&output_file_path)
        .expect("create output binary file");

    output_file.set_len(SRAM0_SIZE).unwrap(); // FIXME: depend on storage

    let fdt = fdt::Fdt::new(&dtb).unwrap();
    let mut areas: Vec<Area> = vec![];
    areas.resize(
        16,
        Area {
            name: "",
            offset: None,
            size: 0,
            file: None,
        },
    );
    let areas = create_areas(&fdt, &mut areas);

    layout_flash(
        Path::new(&dist_dir),
        Path::new(&output_file_path),
        areas.to_vec(),
    )
    .unwrap();
}

fn find_binutils_prefix() -> Option<&'static str> {
    for prefix in ["rust-", "riscv64-unknown-elf-", "riscv64-linux-gnu-"] {
        let mut command = Command::new(format!("{}objcopy", prefix));
        command.arg("--version");
        command.stdout(Stdio::null());
        let status = command.status().unwrap();
        if status.success() {
            return Some(prefix);
        }
    }
    None
}

// FIXME: factor out, rework, share!
fn find_binutils_prefix_or_fail() -> &'static str {
    trace!("find binutils");
    if let Some(ans) = find_binutils_prefix() {
        trace!("found binutils, prefix is '{}'", ans);
        return ans;
    }
    error!(
        "no binutils found, try install using:
    rustup component add llvm-tools-preview
    cargo install cargo-binutils"
    );
    process::exit(1)
}

// FIXME: factor out, rework, share!
fn board_project_root() -> std::path::PathBuf {
    project_root().join("src/mainboard/starfive/visionfive2")
}
