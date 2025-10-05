use std::{fs, path::PathBuf, process};

use fdt::Fdt;
use log::{error, info, trace, warn};

use layoutflash::layout::{create_areas, layout_flash};

use crate::util::{
    compile_board_dt, find_binutils_prefix_or_fail, get_bin_for, get_cargo_cmd_in, objcopy,
    platform_dir, target_dir, Bin,
};
use crate::{Cli, Commands, Env};

use super::visionfive2_hdr::{spl_create_hdr, HEADER_SIZE};

const SRAM_SIZE: usize = 0x20_0000;

const ARCH: &str = "riscv64";

const BOARD_DTFS: &str = "starfive-visionfive2-board.dtb";

const DTFS_IMAGE: &str = "starfive-visionfive2-dtfs.bin";

const IMAGE: &str = "starfive-visionfive2.bin";

const BT0_STAGE: &str = "bt0";
const MAIN_STAGE: &str = "main";
struct Stages {
    bt0: Bin,
    main: Bin,
}

const DIR: &str = "starfive/visionfive2";

pub(crate) fn execute_command(args: &Cli, features: Vec<String>) {
    let dir = PathBuf::from(DIR);
    let bt0 = get_bin_for(&dir, BT0_STAGE);
    let main = get_bin_for(&dir, MAIN_STAGE);
    let stages = Stages { bt0, main };

    match args.command {
        Commands::Make => {
            info!("Build oreboot image for VisionFive2");
            build_image(&args.env, &dir, &stages, &features);
        }
        _ => {
            error!("command {:?} not implemented", args.command);
        }
    }
}

fn xtask_build_jh7110_bt0(env: &Env, dir: &PathBuf, bin: &Bin, features: &[String]) {
    trace!("build JH7110 bt0");
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

    objcopy(
        env,
        binutils_prefix,
        &bin.target,
        ARCH,
        &bin.elf_name,
        &bin.bin_name,
    );
}

fn xtask_build_jh7110_main(env: &Env, dir: &PathBuf, bin: &Bin) {
    trace!("build JH7110 main");
    let binutils_prefix = &find_binutils_prefix_or_fail(ARCH);
    let mut command = get_cargo_cmd_in(env, dir, MAIN_STAGE, "build");
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

fn xtask_build_image(env: &Env, dir: &PathBuf, stages: &Stages) {
    let plat_dir = platform_dir(dir);
    let target_dir = target_dir(env, &stages.main.target);

    let dtfs_path = target_dir.join(BOARD_DTFS);
    compile_board_dt(
        env,
        &stages.main.target,
        &plat_dir,
        dtfs_path.to_str().unwrap(),
    );
    let dtfs_file = fs::read(dtfs_path).expect("dtfs");

    let dtfs = Fdt::new(&dtfs_file).unwrap();
    let areas = create_areas(&dtfs).unwrap();

    let dtfs_image_path = target_dir.join(DTFS_IMAGE);
    if let Err(e) = layout_flash(&target_dir, &dtfs_image_path, areas) {
        error!("layoutflash fail: {e}");
        process::exit(1);
    }

    // TODO: how else do we do layoutflash + header?
    trace!("add header to {dtfs_image_path:?}");
    let dat = fs::read(dtfs_image_path).expect("DTFS image");
    // HACK: omit LinuxBoot etc so we fit in SRAM
    let cut = core::cmp::min(SRAM_SIZE, dat.len());
    trace!("image size {:08x} cut down to {cut:08x}", dat.len());
    let out = spl_create_hdr(dat[HEADER_SIZE as usize..cut].to_vec());
    trace!("final size {:08x}", out.len());
    let out_path = plat_dir.join(IMAGE);
    fs::write(out_path.clone(), out).expect("writing final image");

    println!("======= DONE =======");
    println!("Output file: {:?}", &out_path.into_os_string());
}

fn build_image(env: &Env, dir: &PathBuf, stages: &Stages, features: &[String]) {
    // Build the stages - should we parallelize this?
    xtask_build_jh7110_bt0(env, dir, &stages.bt0, features);
    xtask_build_jh7110_main(env, dir, &stages.main);

    xtask_build_image(env, dir, stages);
}
