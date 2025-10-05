use std::{
    fs::{self, File},
    io::{self, Seek, SeekFrom},
    path::PathBuf,
    process,
};

use fdt::Fdt;
use log::{error, info, trace};

use layoutflash::layout::{create_areas, layout_flash};

use crate::util::{
    compile_board_dt, find_binutils_prefix_or_fail, get_bin_for, get_cargo_cmd_in, objcopy,
    platform_dir, target_dir, Bin,
};
use crate::{Commands, Env};

// const SRAM0_SIZE = 128 * 1024;
const SRAM0_SIZE: u64 = 32 * 1024;

// TODO: detect architecture for binutils
const ARCH: &str = "riscv64";
// TODO: instead of hardcoding, create one binary per feature set.
const IMAGE_BIN: &str = "starfive-visionfive1.bin";
const FDT_BIN: &str = "starfive-visionfive1-dtfs.bin";

const BOARD_DTB: &str = "starfive-visionfive1-board.dtb";

const BT0_STAGE: &str = "bt0";
const MAIN_STAGE: &str = "main";
struct Stages {
    bt0: Bin,
    main: Bin,
}

const DIR: &str = "starfive/visionfive1";

pub(crate) fn execute_command(args: &crate::Cli, features: Vec<String>) {
    let dir = PathBuf::from(DIR);
    let bt0 = get_bin_for(&dir, BT0_STAGE);
    let main = get_bin_for(&dir, MAIN_STAGE);
    let stages = Stages { bt0, main };

    match args.command {
        Commands::Make => {
            info!("Build oreboot image for VisionFive1");
            build_image(&args.env, &dir, &stages, &features);
        }
        _ => {
            error!("command {:?} not implemented", args.command);
        }
    }
}

fn xtask_build_jh7100_flash_bt0(env: &Env, dir: &PathBuf, bin: &Bin, features: &[String]) {
    trace!("build JH7100 flash bt0");
    let binutils_prefix = &find_binutils_prefix_or_fail(ARCH);
    let mut command = get_cargo_cmd_in(env, dir, BT0_STAGE, "build");
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
    objcopy(
        env,
        binutils_prefix,
        &bin.target,
        ARCH,
        &bin.elf_name,
        &bin.bin_name,
    );
}

fn xtask_build_jh7100_flash_main(env: &Env, dir: &PathBuf, bin: &Bin) {
    trace!("build JH7100 flash main");
    let binutils_prefix = &find_binutils_prefix_or_fail(ARCH);
    let mut command = get_cargo_cmd_in(env, dir, MAIN_STAGE, "build");
    let status = command.status().unwrap();
    trace!("cargo returned {}", status);
    if !status.success() {
        error!("cargo build failed with {}", status);
        process::exit(1);
    }
    objcopy(
        env,
        binutils_prefix,
        &bin.target,
        ARCH,
        &bin.elf_name,
        &bin.bin_name,
    );
}

fn xtask_concat_flash_binaries(env: &Env, dir: &PathBuf, stages: &Stages) {
    let plat_dir = platform_dir(dir);
    let mut bt0_file = File::options()
        .read(true)
        .open(target_dir(env, &stages.bt0.target).join(&stages.bt0.bin_name))
        .expect("open bt0 binary file");
    let mut main_file = File::options()
        .read(true)
        .open(target_dir(env, &stages.main.target).join(&stages.main.bin_name))
        .expect("open main binary file");

    let output_file_path = plat_dir.join(IMAGE_BIN);
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

fn xtask_build_dtb_image(env: &Env, dir: &PathBuf, stages: &Stages) {
    let plat_dir = platform_dir(dir);

    let dtb_path = target_dir(env, &stages.main.target).join(BOARD_DTB);
    compile_board_dt(
        env,
        &stages.main.target,
        &plat_dir,
        dtb_path.to_str().unwrap(),
    );
    let dtb = fs::read(dtb_path).expect("dtb");

    let output_file_path = plat_dir.join(FDT_BIN);
    let output_file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&output_file_path)
        .expect("create output binary file");

    output_file.set_len(SRAM0_SIZE).unwrap(); // FIXME: depend on storage

    let fdt = Fdt::new(&dtb).unwrap();
    let areas = create_areas(&fdt).unwrap();

    layout_flash(
        &target_dir(env, &stages.main.target),
        &output_file_path,
        areas,
    )
    .unwrap();
    println!("======= DONE =======");
    println!("Output file: {:?}", &output_file_path.into_os_string());
}

fn build_image(env: &Env, dir: &PathBuf, stages: &Stages, features: &[String]) {
    // Build the stages - should we parallelize this?
    xtask_build_jh7100_flash_bt0(env, dir, &stages.bt0, features);
    xtask_build_jh7100_flash_main(env, dir, &stages.main);
    xtask_concat_flash_binaries(env, dir, stages);
    xtask_build_dtb_image(env, dir, stages);
}
