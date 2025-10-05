use std::{
    fs::{self, File},
    io::{self, Seek, SeekFrom},
    path::{Path, PathBuf},
    process,
};

use fdt::Fdt;
use log::{error, info, trace};

use layoutflash::layout::{create_areas, layout_flash};

use crate::util::{
    compile_board_dt, dist_dir, find_binutils_prefix_or_fail, get_cargo_cmd_in, objcopy,
    platform_dir,
};
use crate::{Commands, Env};

// const SRAM0_SIZE = 128 * 1024;
const SRAM0_SIZE: u64 = 32 * 1024;

const ARCH: &str = "riscv64";
const TARGET: &str = "riscv64imac-unknown-none-elf";

const BT0_BIN: &str = "starfive-visionfive1-bt0.bin";
const BT0_ELF: &str = "starfive-visionfive1-bt0";

const MAIN_BIN: &str = "starfive-visionfive1-main.bin";
const MAIN_ELF: &str = "starfive-visionfive1-main";

const BOARD_DTB: &str = "starfive-visionfive1-board.dtb";

const FDT_BIN: &str = "starfive-visionfive1-board.fdtbin";

const IMAGE_BIN: &str = "starfive-visionfive1.bin";

const DIR: &str = "starfive/visionfive1";

pub(crate) fn execute_command(args: &crate::Cli, features: Vec<String>) {
    match args.command {
        Commands::Make => {
            info!("building VisionFive1");
            build_image(&args.env, &features);
        }
        _ => {
            error!("command {:?} not implemented", args.command);
        }
    }
}

fn xtask_build_jh7100_flash_bt0(env: &Env, features: &[String]) {
    trace!("build JH7100 flash bt0");
    let binutils_prefix = &find_binutils_prefix_or_fail(ARCH);
    let mut command = get_cargo_cmd_in(env, &PathBuf::from(DIR), "bt0", "build");
    if !features.is_empty() {
        let command_line_features = features.join(",");
        trace!("append command line features: {command_line_features}");
        command.arg("--no-default-features");
        command.args(["--features", &command_line_features]);
    } else {
        trace!("no command line features appended");
    }
    let status = command.status().unwrap();
    trace!("cargo returned {status}");
    if !status.success() {
        error!("cargo build failed with {status}");
        process::exit(1);
    }

    objcopy(env, binutils_prefix, TARGET, ARCH, BT0_ELF, BT0_BIN);
}

fn xtask_build_jh7100_flash_main(env: &Env) {
    trace!("build JH7100 flash main");
    let binutils_prefix = &find_binutils_prefix_or_fail(ARCH);
    let mut command = get_cargo_cmd_in(env, &PathBuf::from(DIR), "main", "build");
    let status = command.status().unwrap();
    trace!("cargo returned {}", status);
    if !status.success() {
        error!("cargo build failed with {}", status);
        process::exit(1);
    }

    objcopy(env, binutils_prefix, TARGET, ARCH, MAIN_ELF, MAIN_BIN);
}

fn xtask_concat_flash_binaries(env: &Env) {
    let dist_dir = dist_dir(env, TARGET);
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
        .truncate(true)
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
    let dist_dir = dist_dir(env, TARGET);
    let dtb_path = dist_dir.join(BOARD_DTB);
    let dtb = fs::read(dtb_path).expect("dtb");

    let output_file_path = dist_dir.join(FDT_BIN);
    let output_file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&output_file_path)
        .expect("create output binary file");

    output_file.set_len(SRAM0_SIZE).unwrap(); // FIXME: depend on storage

    let fdt = Fdt::new(&dtb).unwrap();
    let areas = create_areas(&fdt).unwrap();

    layout_flash(Path::new(&dist_dir), Path::new(&output_file_path), areas).unwrap();
    println!("======= DONE =======");
    println!("Output file: {:?}", &output_file_path.into_os_string());
}

fn build_image(env: &Env, features: &[String]) {
    // Build the stages - should we parallelize this?
    xtask_build_jh7100_flash_bt0(env, features);
    xtask_build_jh7100_flash_main(env);
    xtask_concat_flash_binaries(env);
    compile_board_dt(env, TARGET, &platform_dir(&PathBuf::from(DIR)), BOARD_DTB);
    xtask_build_dtb_image(env);
}
