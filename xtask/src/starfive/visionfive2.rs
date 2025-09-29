use std::{
    fs::{self, File},
    path::PathBuf,
    process,
};

use fdt::Fdt;

use cast_iron::layout::{create_areas, layout_flash};
use log::{error, info, trace, warn};

use crate::util::{
    compile_platform_dt, find_binutils_prefix_or_fail, get_bin_for, get_cargo_cmd_in, objcopy,
    platform_dir, Bin,
};
use crate::{Cli, Commands, Env};

use super::visionfive2_hdr::{spl_create_hdr, HEADER_SIZE};

const SRAM_SIZE: usize = 0x20_0000;

const ARCH: &str = "riscv64";
// TODO: instead of hardcoding, create one binary per feature set.
const IMAGE_BIN: &str = "starfive-visionfive2.bin";
const DTFS_IMAGE: &str = "starfive-visionfive2-dtfs.bin";

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
    let dram_size = match env.dram_size {
        Some(crate::DramSize::TwoG) => 2,
        Some(crate::DramSize::FourG) => 4,
        Some(crate::DramSize::EightG) => 8,
        None => {
            warn!("no DRAM size provided, falling back to 4G");
            4
        }
    };
    let rustflags_key = "target.riscv64imac-unknown-none-elf.rustflags";
    let rustflags_val = &format!("['--cfg=dram_size=\"{dram_size}G\"']");
    command.args(["--config", &format!("{rustflags_key}={rustflags_val}")]);
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
    let binutils_prefix = &find_binutils_prefix_or_fail(ARCH);
    let mut command = get_cargo_cmd_in(env, dir, MAIN_STAGE, "build");
    let status = command.status().unwrap();
    trace!("cargo returned {status}");
    if !status.success() {
        error!("cargo build failed with {status}");
        process::exit(1);
    }
    objcopy(env, bin, binutils_prefix, ARCH);
}

fn build_dtfs_image(dir: &PathBuf) {
    let plat_dir = platform_dir(dir);
    let dtfs_img = plat_dir.join(DTFS_IMAGE);

    let dtb_path = compile_platform_dt(&plat_dir);
    let dtfs_dtb = plat_dir.join(dtb_path);

    let dtb = fs::read(dtfs_dtb).expect("DTFS DTB");
    let dtfs = Fdt::new(&dtb).unwrap();
    let areas = create_areas(&dtfs).unwrap();
    trace!("{areas:#?}");

    // preallocate image file
    let dtfs_bin = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&dtfs_img)
        .expect("create image file");
    // FIXME: depend on storage
    dtfs_bin.set_len(SRAM_SIZE as u64).unwrap();

    if let Err(e) = layout_flash(&plat_dir, &dtfs_img, areas.to_vec()) {
        error!("layoutflash fail: {e}");
        process::exit(1);
    }

    // TODO: how else do we do layoutflash + header?
    trace!("add header to {dtfs_img:?}");
    let dat = fs::read(dtfs_img).expect("DTFS image");
    // HACK: omit LinuxBoot etc so we fit in SRAM
    let cut = core::cmp::min(SRAM_SIZE, dat.len());
    trace!("image size {:08x} cut down to {cut:08x}", dat.len());
    let img = spl_create_hdr(dat[HEADER_SIZE as usize..cut].to_vec());
    trace!("final size {:08x}", img.len());
    let img_path = dir.join(IMAGE_BIN);
    fs::write(&img_path, img).expect("writing final image");

    println!("Output\n  File: {img_path:?}",);
    println!("======= DONE =======");
}

fn build_image(env: &Env, dir: &PathBuf, stages: &Stages, features: &[String]) {
    info!("Build oreboot image for VisionFive2");
    // Build the stages - should we parallelize this?
    build_bt0(env, dir, &stages.bt0, features);
    build_main(env, dir, &stages.main);
    build_dtfs_image(dir);
}
