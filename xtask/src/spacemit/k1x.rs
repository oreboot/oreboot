use std::{fs::File, io::Write, path::PathBuf, process};

use log::{error, info, trace};

use crate::util::{
    find_binutils_prefix_or_fail, get_cargo_cmd_in, get_stage_for, objcopy, target_bin, Stage,
};
use crate::{Cli, Commands, Env};

use super::k1x_hdr;

// TODO: detect architecture for binutils
const ARCH: &str = "riscv64";
// TODO: instead of hardcoding, create one binary per feature set.
// const IMAGE_BIN: &str = "oreboot-spacemit-k1x.bin";
const BT0_STAGE: &str = "bt0";
// const MAIN_STAGE: &str = "main";
struct Stages {
    bt0: Stage,
    // main: Stage,
}

pub(crate) fn execute_command(args: &Cli, dir: &PathBuf, features: Vec<String>) {
    let bt0 = get_stage_for(dir, BT0_STAGE);
    // let main = get_stage_for(dir, MAIN_STAGE);
    let stages = Stages { bt0 };

    match args.command {
        Commands::Make => {
            info!("Build oreboot image for SpacemiT K1x");
            build(&args.env, dir, &stages, &features);
        }
        Commands::Run => {
            build_bt0(&args.env, dir, &stages.bt0, &features);
            todo!("Run via fastboot");
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
    if features.is_empty() {
        trace!("no command line features appended");
    } else {
        let command_line_features = features.join(",");
        trace!("append command line features: {command_line_features}");
        command.arg("--no-default-features");
        command.args(["--features", &command_line_features]);
    }
    let status = command.status().unwrap();
    trace!("cargo returned {status}");
    if !status.success() {
        error!("cargo build failed with {status}");
        process::exit(1);
    }
    objcopy(env, stage, binutils_prefix, ARCH);

    let bin_file = target_bin(env, stage);
    let bt0 = std::fs::read(&bin_file).expect("opening bt0 binary file");
    let image = k1x_hdr::build(&bt0);
    let mut output_file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&bin_file)
        .expect("patch bt0 binary file");
    output_file.write_all(&image).unwrap();
    info!("{BT0_STAGE}: {}", bin_file.display());
}

fn build(env: &Env, dir: &PathBuf, stages: &Stages, features: &[String]) {
    build_bt0(env, dir, &stages.bt0, features);
    // build_image(dir);
}
