use std::{fs::File, io::Write, process};

use log::{error, info, trace};

use crate::{
    sunxi::{egon, fel},
    util::{dist_dir, get_cargo_cmd_in, objcopy, project_root},
    Cli, Commands, Env,
};

const ARCH: &str = "arm";
const TARGET: &str = "armv7a-none-eabi";
const BOARD_DIR: &str = "src/mainboard/sunxi/H616";

// const BT0_ELF: &str = "oreboot-allwinner-h616-bt0";
// const BT0_BIN: &str = "oreboot-allwinner-h616-bt0.bin";
const BT0_ADDR: usize = 0x20000;

const BT32_ELF: &str = "oreboot-allwinner-h616-bt32";
const BT32_BIN: &str = "oreboot-allwinner-h616-bt32.bin";
const BT32_BIN_WITH_HEADER: &str = "oreboot-allwinner-h616-bt32-wheader.bin";

pub(crate) fn execute_command(args: &Cli, features: Vec<String>) {
    match args.command {
        Commands::Make => {
            info!("Build oreboot image for H616");
            todo!();
            // build_image(&args.env, &features);
        }
        Commands::Flash => {
            // TODO: print out variant etc
            info!("Build and flash oreboot image for H616");
            todo!();
            /*
            let xfel = fel::find_xfel();
            fel::xfel_find_connected_device(xfel);
            build_image(&args.env, &features);
            fel::flash_image(xfel, &args.env);
            */
        }
        Commands::Run => {
            // TODO: print out variant etc
            info!("Run image on H616 via FEL");
            let xfel = fel::find_xfel();
            fel::xfel_find_connected_device(xfel);
            build_image(&args.env, &features);
            // fel::xfel_run(xfel, &args.env, TARGET, BT32_BIN_WITH_HEADER, BT0_ADDR);
            fel::run(&args.env, TARGET, BT32_BIN_WITH_HEADER);
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

fn board_project_root() -> std::path::PathBuf {
    project_root().join(BOARD_DIR)
}

fn build_bt32(env: &Env, features: &[String]) {
    trace!("build H616 bt32");
    let mut command = get_cargo_cmd_in(env, board_project_root(), "bt32", "build");
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
}

fn build_image(env: &Env, features: &[String]) {
    let dist_dir = dist_dir(env, TARGET);
    // Get binutils first so we can fail early
    // let binutils_prefix = &find_binutils_prefix_or_fail(ARCH);
    let binutils_prefix = "arm-linux-gnueabi-";
    // Build the stages - should we parallelize this?
    build_bt32(env, features);
    objcopy(env, binutils_prefix, TARGET, ARCH, BT32_ELF, BT32_BIN);
    let bt32 = std::fs::read(dist_dir.join(BT32_BIN)).expect("opening bt32 binary file");
    let egon_bin = egon::add_header(&bt32, egon::Arch::Arm32);
    let output_file_path = dist_dir.join(BT32_BIN_WITH_HEADER);
    info!("{output_file_path:?}");
    println!("{output_file_path:?}");
    let mut output_file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&output_file_path)
        .expect("create output binary file");
    output_file.write_all(&egon_bin).unwrap();
}
