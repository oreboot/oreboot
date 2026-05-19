use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Seek, SeekFrom},
    path::PathBuf,
    process,
};

use fdt::Fdt;
use log::{error, info, trace};

use layoutflash::layout::{create_areas, layout_flash};

use crate::util::{
    compile_platform_dt, find_binutils_prefix_or_fail, get_bin_for, get_cargo_cmd_in, objcopy,
    platform_dir, target_bin, target_dir, Bin,
};
use crate::{Cli, Commands, Env};

// const SRAM0_SIZE = 128 * 1024;
const SRAM0_SIZE: u64 = 32 * 1024;

// TODO: detect architecture for binutils
const ARCH: &str = "riscv64";
// TODO: instead of hardcoding, create one binary per feature set.
const IMAGE_BIN: &str = "starfive-visionfive1.bin";
const FDT_BIN: &str = "starfive-visionfive1-dtfs.bin";

const BT0_STAGE: &str = "bt0";
const MAIN_STAGE: &str = "main";
struct Stages {
    bt0: Bin,
    main: Bin,
}

pub(crate) fn execute_command(args: &Cli, dir: &PathBuf, features: Vec<String>) {
    let bt0 = get_bin_for(dir, BT0_STAGE);
    let main = get_bin_for(dir, MAIN_STAGE);
    let stages = Stages { bt0, main };

    match args.command {
        Commands::Make => {
            info!("Build oreboot image for VisionFive1");
            build(&args.env, dir, &stages, &features);
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
    let img_path = plat_dir.join(IMAGE_BIN);
    println!("Stitching final image 🏗️ {img_path:?}");

    let mut bt0_file = File::options()
        .read(true)
        .open(target_dir(env, &stages.bt0.target).join(&stages.bt0.bin_name))
        .expect("open bt0 binary file");
    let mut main_file = File::options()
        .read(true)
        .open(target_dir(env, &stages.main.target).join(&stages.main.bin_name))
        .expect("open main binary file");

    let mut img_file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&img_path)
        .expect("create output binary file");

    // FIXME: depend on storage
    img_file.set_len(SRAM0_SIZE).unwrap();

    let bt0_len = 30 * 1024;
    io::copy(&mut bt0_file, &mut img_file).expect("copy bt0 binary");
    img_file
        .seek(SeekFrom::Start(bt0_len))
        .expect("seek after bt0 copy");
    io::copy(&mut main_file, &mut img_file).expect("copy main binary");

    println!("======= DONE =======");
    println!("Output file: {:?}", &img_path.into_os_string());
}

fn build_image(env: &Env, dir: &PathBuf, stages: &Stages) {
    let plat_dir = platform_dir(dir);
    let main_target_dir = target_dir(env, &stages.main.target);

    let dtb_path = compile_platform_dt(&plat_dir);
    let dtb = fs::read(dtb_path).expect("dtb");

    let output_path = plat_dir.join(FDT_BIN);
    let output_file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&output_path)
        .expect("create output binary file");

    output_file.set_len(SRAM0_SIZE).unwrap(); // FIXME: depend on storage

    let fdt = Fdt::new(&dtb).unwrap();
    let areas = create_areas(&fdt).unwrap();

    let stage_bin_map = HashMap::from([
        (BT0_STAGE, target_bin(env, &stages.bt0)),
        (MAIN_STAGE, target_bin(env, &stages.main)),
        // TODO: use actual payload, this is a hack
        ("payload", target_bin(env, &stages.main)),
    ]);

    if let Err(e) = layout_flash(&main_target_dir, &output_path, areas, stage_bin_map) {
        error!("layoutflash fail: {e}");
        process::exit(1);
    }

    println!("======= DONE =======");
    println!("Output file: {:?}", &output_path.into_os_string());
}

fn build(env: &Env, dir: &PathBuf, stages: &Stages, features: &[String]) {
    // Build the stages - should we parallelize this?
    build_bt0(env, dir, &stages.bt0, features);
    build_main(env, dir, &stages.main);
    concat_binaries(env, dir, stages);
    build_image(env, dir, stages);
}
