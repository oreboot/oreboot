use std::{fs::File, io::Write, process};

use log::{error, info, trace};

use crate::{
    sunxi::{egon, fel},
    util::{dist_dir, get_cargo_cmd_in, objcopy},
    Cli, Commands, Env,
};

const ARCH: &str = "arm";
const TARGET: &str = "armv7a-none-eabi";

// const BT0_ELF: &str = "oreboot-allwinner-h616-bt0";
// const BT0_BIN: &str = "oreboot-allwinner-h616-bt0.bin";
const BT0_ADDR: usize = 0x20000;

const BT32_ELF: &str = "oreboot-allwinner-h616-bt32";
const BT32_BIN: &str = "oreboot-allwinner-h616-bt32.bin";

pub(crate) fn execute_command(args: &Cli, features: Vec<String>) {
    match args.command {
        Commands::Make => {
            info!("Build oreboot image for H616");
            build_image(&args.env, &features);
        }
        Commands::Flash => {
            // TODO: print out variant etc
            info!("Build and flash oreboot image for H616");
            todo!();
            /*
            fel::xfel_find_connected_device();
            build_image(&args.env, &features);
            fel::flash_image(&args.env, TARGET, FULL_BIN);
            */
        }
        Commands::Run => {
            // TODO: print out variant etc
            info!("Run image on H616 via FEL");
            let _ = fel::find_xfel();
            fel::xfel_find_connected_device();
            build_image(&args.env, &features);
            if false {
                fel::xfel_run(&args.env, TARGET, BT32_BIN, BT0_ADDR);
            } else {
                fel::sunxi_fel_run(&args.env, TARGET, BT32_BIN);
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

fn build_bt32(env: &Env, features: &[String]) {
    trace!("build H616 bt32");
    let dist_dir = dist_dir(env, TARGET);
    // Get binutils first so we can fail early
    let binutils_prefix = "arm-linux-gnueabi-";
    let mut command = get_cargo_cmd_in(env, "bt32", "build");
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
    objcopy(env, binutils_prefix, TARGET, ARCH, BT32_ELF, BT32_BIN);
    let f = dist_dir.join(BT32_BIN);
    let bt32 = std::fs::read(&f).expect("opening bt32 binary file");
    let egon_bin = egon::add_header(&bt32, egon::Arch::Arm32);
    let mut output_file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&f)
        .expect("create bt0 binary file");
    output_file.write_all(&egon_bin).unwrap();
}

fn build_image(env: &Env, features: &[String]) {
    // Build the stages - should we parallelize this?
    build_bt32(env, features);
}
