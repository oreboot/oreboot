use crate::util::{
    compile_platform_dt, find_binutils_prefix_or_fail, get_bin_for, get_cargo_cmd_in, objcopy,
    platform_dir, target_dir, Bin,
};
use crate::{layout_flash, Commands, Env};
use fdt::Fdt;
use log::{error, info, trace};
use std::path::PathBuf;
use std::{
    fs::{self, File},
    io::{self, Seek, SeekFrom},
    path::Path,
    process,
};

use layoutflash::areas::{create_areas, Area};

// const SRAM0_SIZE = 128 * 1024;
const SRAM0_SIZE: u64 = 32 * 1024;

// TODO: detect architecture for binutils
const ARCH: &str = "riscv64";
// TODO: instead of hardcoding, create one binary per feature set.
const IMAGE_BIN: &str = "starfive-visionfive1.bin";
const DTFS_IMAGE: &str = "starfive-visionfive1-dtfs.bin";

const BT0_STAGE: &str = "bt0";
const MAIN_STAGE: &str = "main";
struct Stages {
    bt0: Bin,
    main: Bin,
}

pub(crate) fn execute_command(args: &crate::Cli, dir: &PathBuf, features: Vec<String>) {
    let bt0 = get_bin_for(dir, BT0_STAGE);
    let main = get_bin_for(dir, MAIN_STAGE);
    let stages = Stages { bt0, main };

    match args.command {
        Commands::Make => {
            info!("Build oreboot image for VisionFive1");
            build_image(&args.env, dir, &stages, &features);
        }
        _ => {
            error!("command {:?} not implemented", args.command);
        }
    }
}

fn build_bt0(env: &Env, dir: &PathBuf, bin: &Bin, features: &[String]) {
    trace!("build {BT0_STAGE}");
    // Get binutils first so we can fail early
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
    objcopy(env, bin, binutils_prefix, ARCH);
}

fn build_main(env: &Env, dir: &PathBuf, bin: &Bin) {
    trace!("build {MAIN_STAGE}");
    // Get binutils first so we can fail early
    let binutils_prefix = &find_binutils_prefix_or_fail(ARCH);
    let mut command = get_cargo_cmd_in(env, dir, MAIN_STAGE, "build");
    let status = command.status().unwrap();
    trace!("cargo returned {}", status);
    if !status.success() {
        error!("cargo build failed with {}", status);
        process::exit(1);
    }
    objcopy(env, bin, binutils_prefix, ARCH);
}

fn concat_binaries(env: &Env, dir: &PathBuf, stages: &Stages) {
    let plat_dir = platform_dir(dir);
    let image_path = plat_dir.join(IMAGE_BIN);
    println!("Stitching final image 🏗️ {image_path:?}");

    let mut bt0_file = File::options()
        .read(true)
        .open(target_dir(env, &stages.bt0.target).join(&stages.bt0.bin_name))
        .expect("open bt0 binary file");
    let mut main_file = File::options()
        .read(true)
        .open(target_dir(env, &stages.main.target).join(&stages.main.bin_name))
        .expect("open main binary file");

    let mut image_file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&image_path)
        .expect("create output binary file");

    // FIXME: depend on storage
    image_file.set_len(SRAM0_SIZE).unwrap();

    let bt0_len = 30 * 1024;
    io::copy(&mut bt0_file, &mut image_file).expect("copy bt0 binary");
    image_file
        .seek(SeekFrom::Start(bt0_len))
        .expect("seek after bt0 copy");
    io::copy(&mut main_file, &mut image_file).expect("copy main binary");

    println!("Output\n  File: {image_path:?}",);
    println!("======= DONE =======");
}

fn build_dtfs_image(dir: &PathBuf) {
    let plat_dir = platform_dir(dir);
    let dtfs_img = plat_dir.join(DTFS_IMAGE);

    let dtb_path = compile_platform_dt(dir);
    let dtfs_dtb = plat_dir.join(dtb_path);

    let dtb = fs::read(dtfs_dtb).expect("DTFS DTB");
    let dtfs = Fdt::new(&dtb).unwrap();
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
    let areas = create_areas(&dtfs, &mut areas);

    let dtfs_bin = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&dtfs_img)
        .expect("create output binary file");
    // FIXME: depend on storage
    dtfs_bin.set_len(SRAM0_SIZE).unwrap();

    if let Err(e) = layout_flash(&plat_dir, &dtfs_img, areas.to_vec()) {
        error!("layoutflash fail: {e}");
        process::exit(1);
    }

    println!("Output\n  File: {dtfs_img:?}",);
    println!("======= DONE =======");
}

fn build_image(env: &Env, dir: &PathBuf, stages: &Stages, features: &[String]) {
    // Build the stages - should we parallelize this?
    build_bt0(env, dir, &stages.bt0, features);
    build_main(env, dir, &stages.main);
    concat_binaries(env, dir, stages);
    build_dtfs_image(dir);
}
