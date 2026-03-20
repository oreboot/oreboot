use std::collections::HashMap;
use std::{fs, path::PathBuf, process};

use fdt::Fdt;
use log::{error, info, trace, warn};

use layoutflash::layout::{create_areas, layout_flash};

use crate::util::{
    compile_platform_dt, find_binutils_prefix_or_fail, get_bin_for, get_cargo_cmd_in, objcopy,
    platform_dir, target_dir, Bin,
};
use crate::{Cli, Commands, Env};

use super::visionfive2_hdr::{spl_create_hdr, HEADER_SIZE};

const SRAM_SIZE: usize = 0x20_0000;

const ARCH: &str = "riscv64";

const IMAGE: &str = "starfive-visionfive2.bin";

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
            info!("Build oreboot image for VisionFive2");
            build_image(&args.env, dir, &stages, &features);
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

    objcopy(env, bin, binutils_prefix, ARCH);
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

    objcopy(env, bin, binutils_prefix, ARCH);
}

fn xtask_build_image(env: &Env, dir: &PathBuf, stages: &Stages) {
    let plat_dir = platform_dir(dir);
    let main_target_dir = target_dir(env, &stages.main.target);

    let dtb_path = compile_platform_dt(&plat_dir);
    let dtb = fs::read(dtb_path).expect("platform DTB");

    let fdt = Fdt::new(&dtb).unwrap();
    let areas = create_areas(&fdt).unwrap();

    let stage_bin_map = HashMap::from([
        (
            BT0_STAGE,
            target_dir(env, &stages.bt0.target).join(&stages.bt0.bin_name),
        ),
        (MAIN_STAGE, main_target_dir.join(&stages.main.bin_name)),
    ]);

    let out_path = plat_dir.join(IMAGE);
    if let Err(e) = layout_flash(&main_target_dir, &out_path, areas, stage_bin_map) {
        error!("layoutflash fail: {e}");
        process::exit(1);
    }

    // TODO: how else do we do layoutflash + header?
    trace!("add header to {out_path:?}");
    let dat = fs::read(&out_path).expect("DTFS image");
    // HACK: omit LinuxBoot etc so we fit in SRAM
    let cut = core::cmp::min(SRAM_SIZE, dat.len());
    trace!("image size {:08x} cut down to {cut:08x}", dat.len());
    let out = spl_create_hdr(dat[HEADER_SIZE as usize..cut].to_vec());
    trace!("final size {:08x}", out.len());
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
