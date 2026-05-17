use std::collections::HashMap;
use std::{fs, path::PathBuf, process};

use fdt::Fdt;
use log::{error, info, trace, warn};

use layoutflash::layout::{create_areas, layout_flash};

use crate::util::{
    compile_platform_dt, find_binutils_prefix_or_fail, get_cargo_cmd_in, get_stage_for, objcopy,
    platform_dir, target_bin, Stage,
};
use crate::{Cli, Commands, Env};

use super::visionfive2_hdr::{spl_create_hdr, HEADER_SIZE};

const SRAM_SIZE: usize = 0x20_0000;

const ARCH: &str = "riscv64";

// TODO: instead of hardcoding, create one binary per feature set.
const IMAGE: &str = "starfive-visionfive2.bin";

const BT0_STAGE: &str = "bt0";
const MAIN_STAGE: &str = "main";
struct Stages {
    bt0: Stage,
    main: Stage,
}

pub(crate) fn execute_command(args: &Cli, dir: &PathBuf, features: Vec<String>) {
    let bt0 = get_stage_for(dir, BT0_STAGE);
    let main = get_stage_for(dir, MAIN_STAGE);
    let stages = Stages { bt0, main };

    match args.command {
        Commands::Make => {
            info!("Build oreboot image for VisionFive2");
            build(&args.env, dir, &stages, &features);
        }
        _ => {
            error!("command {:?} not implemented", args.command);
        }
    }
}

fn build_bt0(env: &Env, dir: &PathBuf, stage: &Stage, features: &[String]) {
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
    objcopy(env, stage, binutils_prefix, ARCH);
}

fn build_main(env: &Env, dir: &PathBuf, stage: &Stage) {
    trace!("build {MAIN_STAGE}");
    let binutils_prefix = &find_binutils_prefix_or_fail(ARCH);
    let mut command = get_cargo_cmd_in(env, dir, MAIN_STAGE, "build");
    let status = command.status().unwrap();
    trace!("cargo returned {status}");
    if !status.success() {
        error!("cargo build failed with {status}");
        process::exit(1);
    }
    objcopy(env, stage, binutils_prefix, ARCH);
}

fn build_image(env: &Env, dir: &PathBuf, stages: &Stages) {
    let plat_dir = platform_dir(dir);
    let img_path = plat_dir.join(IMAGE);

    let dtb_path = compile_platform_dt(&plat_dir);
    let dtb = fs::read(dtb_path).expect("platform DTB");

    let fdt = Fdt::new(&dtb).unwrap();
    let areas = create_areas(&fdt).unwrap();

    let stage_bin_map = HashMap::from([
        (BT0_STAGE, target_bin(env, &stages.bt0)),
        (MAIN_STAGE, target_bin(env, &stages.main)),
    ]);

    if let Err(e) = layout_flash(&plat_dir, &img_path, areas, stage_bin_map) {
        error!("layoutflash fail: {e}");
        process::exit(1);
    }

    // TODO: how else do we do layoutflash + header?
    trace!("add header to {img_path:?}");
    let dat = fs::read(&img_path).expect("platform image");
    // HACK: omit LinuxBoot etc so we fit in SRAM
    let cut = core::cmp::min(SRAM_SIZE, dat.len());
    trace!("image size {:08x} cut down to {cut:08x}", dat.len());
    let out = spl_create_hdr(dat[HEADER_SIZE as usize..cut].to_vec());
    trace!("final size {:08x}", out.len());
    fs::write(img_path.clone(), out).expect("writing final image");

    println!("======= DONE =======");
    println!("Output file: {:?}", &img_path.into_os_string());
}

fn build(env: &Env, dir: &PathBuf, stages: &Stages, features: &[String]) {
    // Build the stages - should we parallelize this?
    build_bt0(env, dir, &stages.bt0, features);
    build_main(env, dir, &stages.main);
    build_image(env, dir, stages);
}
