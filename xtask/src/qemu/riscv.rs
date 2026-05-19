use std::{
    collections::HashMap,
    fs::{self},
    path::PathBuf,
    process,
};

use fdt::Fdt;
use log::{error, info, trace};

use layoutflash::layout::{create_areas, layout_flash};

use crate::util::{
    analyze, compile_platform_dt, find_binutils_prefix_or_fail, get_bin_for, get_cargo_cmd_in,
    objcopy, platform_dir, target_bin, Bin,
};
use crate::{Cli, Commands, Env};

const ARCH: &str = "riscv64";

// TODO: instead of hardcoding, create one binary per feature set.
const IMAGE_BIN: &str = "oreboot-emulation-qemu-riscv.bin";

const MAIN_STAGE: &str = "main";
struct Stages {
    main: Bin,
}

pub(crate) fn execute_command(args: &Cli, dir: &PathBuf, features: Vec<String>) {
    let main = get_bin_for(dir, MAIN_STAGE);
    let stages = Stages { main };

    match args.command {
        Commands::Make => {
            info!("Build oreboot image for QEMU RISC-V");
            build(&args.env, dir, &stages, &features);
        }
        Commands::Analyze => {
            info!("Build and analyze main stage");
            build_main(&args.env, dir, &stages.main);
            analyze(&args.env, &stages.main);
        }
        _ => {
            error!("command {:?} not implemented", args.command);
        }
    }
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

fn build_image(env: &Env, dir: &PathBuf, stages: &Stages) {
    let plat_dir = platform_dir(dir);
    let img_path = plat_dir.join(IMAGE_BIN);

    let dtb_path = compile_platform_dt(&plat_dir);
    let dtb = fs::read(dtb_path).expect("platform DTB");

    let fdt = Fdt::new(&dtb).unwrap();
    let areas = create_areas(&fdt).unwrap();

    let stage_bin_map = HashMap::from([
        (MAIN_STAGE, target_bin(env, &stages.main)), //
    ]);

    if let Err(e) = layout_flash(&plat_dir, &img_path, areas, stage_bin_map) {
        error!("Error building image: {e}");
        process::exit(1);
    }

    println!("======= DONE =======");
    println!("Output file: {:?}", &img_path.into_os_string());
}

fn build(env: &Env, dir: &PathBuf, stages: &Stages, _features: &[String]) {
    build_main(env, dir, &stages.main);
    build_image(env, dir, stages);
}
