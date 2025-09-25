use std::{fs::File, io::Write, path::PathBuf, process};

use log::{error, info, trace};

use crate::util::{get_bin_for, get_cargo_cmd_in, objcopy, target_dir, Bin};
use crate::{Cli, Commands, Env};

use super::{egon, fel};

const ARCH: &str = "arm";

const BT0_ADDR: usize = 0x20000;
const BT32_STAGE: &str = "bt32";
// TODO: Determine in execute_command() based on dir and features
// We could drop the final image into the vendor/platform directory under `build/`,
// and call it `oreboot[_features...].bin` or something like that.
// We should be able to produce multiple binaries per vendor/platform/features.
const IMAGE_BIN: &str = "oreboot_allwinner_h616.bin";

struct Stages {
    bt32: Bin,
}

pub(crate) fn execute_command(args: &Cli, dir: &PathBuf, features: Vec<String>) {
    let bt32 = get_bin_for(dir, BT32_STAGE);
    let stages = Stages { bt32 };

    match args.command {
        Commands::Make => {
            info!("Build oreboot image for H616");
            build_image(&args.env, dir, &stages, &features);
        }
        Commands::Flash => {
            // TODO: print out variant etc
            info!("Build and flash oreboot image for H616");
            if false {
                fel::xfel_find_connected_device();
                build_image(&args.env, dir, &stages, &features);
                fel::flash_image(&args.env, &dir, IMAGE_BIN);
            }
            todo!();
        }
        Commands::Run => {
            // TODO: print out variant etc
            info!("Run image on H616 via FEL");
            let _ = fel::find_xfel();
            fel::xfel_find_connected_device();
            build_image(&args.env, dir, &stages, &features);
            if false {
                fel::xfel_run(&args.env, &stages.bt32, BT0_ADDR);
            } else {
                fel::sunxi_fel_run(&args.env, &stages.bt32);
            }
        }
        Commands::Asm => {
            todo!("Build bt0 and view assembly for H616");
            /*
            let binutils_prefix = &find_binutils_prefix_or_fail(ARCH);
            build_H616_bt0(&args.env, &features);
            objdump(&args.env, binutils_prefix, TARGET, BT0_ELF);
            */
        }
        Commands::Gdb => {
            todo!("Debug bt0 for H616 using gdb");
        }
    }
}

fn build_bt32(env: &Env, dir: &PathBuf, bin: &Bin, features: &[String]) {
    trace!("build H616 bt32");
    // Get binutils first so we can fail early
    let binutils_prefix = "arm-linux-gnueabi-";
    let mut command = get_cargo_cmd_in(env, dir, BT32_STAGE, "build");
    if !features.is_empty() {
        let command_line_features = features.join(",");
        trace!("append command line features: {command_line_features}");
        command.arg("--no-default-features");
        command.args(["--features", &command_line_features]);
    } else {
        trace!("no command line features appended");
    }
    trace!("run H616 bt0 build command: {command:?}");
    let status = command.status().unwrap();
    trace!("cargo returned {status}");
    if !status.success() {
        error!("cargo build failed with {status}");
        process::exit(1);
    }
    objcopy(env, &bin, binutils_prefix, ARCH);
    let bin_file = target_dir(env, &bin.target).join(&bin.bin_name);
    let bt32 = std::fs::read(&bin_file).expect("opening bt32 binary file");
    let egon_bin = egon::add_header(&bt32, egon::Arch::Arm32);
    let mut output_file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&bin_file)
        .expect("patch bt32 binary file");
    output_file.write_all(&egon_bin).unwrap();
}

fn build_image(env: &Env, dir: &PathBuf, stages: &Stages, features: &[String]) {
    // Build the stages - should we parallelize this?
    build_bt32(env, dir, &stages.bt32, features);
}
