use std::{
    fs::{self, File},
    path::PathBuf,
    process,
};

use fdt::Fdt;

use cast_iron::layout::{create_areas, layout_flash};
use log::{error, info, trace};

use crate::util::{
    compile_platform_dt, find_binutils_prefix_or_fail, get_bin_for, get_cargo_cmd_in, objcopy,
    platform_dir, Bin,
};
use crate::{Cli, Commands, Env};

// const SRAM0_SIZE = 128 * 1024;
const SRAM0_SIZE: u64 = 32 * 1024;

const ARCH: &str = "riscv64";
// TODO: instead of hardcoding, create one binary per feature set.
const IMAGE_BIN: &str = "emulation-qemu-riscv.bin";
const DTFS_IMAGE: &str = "starfive-visionfive2-dtfs.bin";

const MAIN_STAGE: &str = "main";
struct Stages {
    main: Bin,
}

pub(crate) fn execute_command(args: &Cli, dir: &PathBuf, features: Vec<String>) {
    let main = get_bin_for(dir, MAIN_STAGE);
    let stages = Stages { main };

    match args.command {
        Commands::Make => {
            build_image(&args.env, dir, &stages, &features);
        }
        Commands::Run => {
            build_image(&args.env, dir, &stages, &features);
            todo!("Run in QEMU");
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
    let mut command = get_cargo_cmd_in(env, dir, "main", "build");
    let status = command.status().unwrap();
    trace!("cargo returned {}", status);
    if !status.success() {
        error!("cargo build failed with {}", status);
        process::exit(1);
    }
    objcopy(env, bin, binutils_prefix, ARCH);
}

fn build_dtfs_image(dir: &PathBuf) {
    let plat_dir = platform_dir(dir);
    let dtfs_img = plat_dir.join(DTFS_IMAGE);

    let dtb_path = compile_platform_dt(dir);
    let dtfs_dtb = plat_dir.join(dtb_path);

    let dtfs_bin = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&dtfs_img)
        .expect("create output binary file");
    // FIXME: depend on storage
    dtfs_bin.set_len(SRAM0_SIZE).unwrap();

    let dtb = fs::read(dtfs_dtb).expect("DTFS DTB");
    let dtfs = Fdt::new(&dtb).unwrap();
    info!("{dtfs:#?}");
    let areas = create_areas(&dtfs).unwrap();
    info!("{areas:#?}");

    if let Err(e) = layout_flash(&plat_dir, &dtfs_img, areas.to_vec()) {
        error!("layoutflash fail: {e}");
        process::exit(1);
    }

    println!("Output\n  File: {dtfs_img:?}",);
    println!("======= DONE =======");
}

fn build_image(env: &Env, dir: &PathBuf, stages: &Stages, _features: &[String]) {
    info!("Build oreboot image for QEMU RISC-V");
    build_main(env, dir, &stages.main);
    build_dtfs_image(dir);
}
