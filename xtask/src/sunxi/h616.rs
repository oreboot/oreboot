use std::{fs::File, io::Write, path::PathBuf, process};

use log::{error, info, trace};

use crate::{
    sunxi::{egon, fel},
    util::{dist_dir, get_cargo_cmd_in, get_manifest_in, objcopy, Bin},
    Cli, Commands, Env,
};

const ARCH: &str = "arm";
const TARGET: &str = "armv7a-none-eabi";

const BT0_ADDR: usize = 0x20000;
const BT32_STAGE: &str = "bt32";
// TODO: Determine in execute_command() based on dir and features
// We could drop the final image into the vendor/platform directory under `build/`,
// and call it `oreboot[_features...].bin` or something like that.
// We should be able to produce multiple binaries per vendor/platform/features.
const FULL_BIN: &str = "oreboot_allwinner_h616.bin";

struct Bins<'a> {
    bt32: &'a Bin<'a>,
}

pub(crate) fn execute_command(args: &Cli, dir: &PathBuf, features: Vec<String>) {
    let m = get_manifest_in(dir, BT32_STAGE);
    let bt32_elf = m.bin.first().unwrap().name.clone().unwrap();
    let bt32_bin = format!("{bt32_elf}.bin");
    let bins = Bins {
        bt32: &Bin {
            elf_name: bt32_elf.as_str(),
            bin_name: bt32_bin.as_str(),
        },
    };
    match args.command {
        Commands::Make => {
            info!("Build oreboot image for H616");
            build_image(&args.env, dir, &bins, &features);
        }
        Commands::Flash => {
            // TODO: print out variant etc
            info!("Build and flash oreboot image for H616");
            if false {
                fel::xfel_find_connected_device();
                build_image(&args.env, dir, &bins, &features);
                fel::flash_image(&args.env, TARGET, FULL_BIN);
            }
            todo!();
        }
        Commands::Run => {
            // TODO: print out variant etc
            info!("Run image on H616 via FEL");
            let _ = fel::find_xfel();
            fel::xfel_find_connected_device();
            build_image(&args.env, dir, &bins, &features);
            if false {
                fel::xfel_run(&args.env, TARGET, &bt32_bin, BT0_ADDR);
            } else {
                fel::sunxi_fel_run(&args.env, TARGET, &bt32_bin);
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

fn build_bt32(env: &Env, dir: &PathBuf, bins: &Bins, features: &[String]) {
    trace!("build H616 bt32");
    let stage = "bt32";
    let dist_dir = dist_dir(env, TARGET);
    // Get binutils first so we can fail early
    let binutils_prefix = "arm-linux-gnueabi-";
    let mut command = get_cargo_cmd_in(env, dir, stage, "build");
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
    objcopy(
        env,
        binutils_prefix,
        TARGET,
        ARCH,
        bins.bt32.elf_name,
        bins.bt32.bin_name,
    );
    let elf_file = dist_dir.join(bins.bt32.elf_name);
    let bin_file = dist_dir.join(bins.bt32.bin_name);
    let bt32 = std::fs::read(&bin_file).expect("opening bt32 binary file");
    let egon_bin = egon::add_header(&bt32, egon::Arch::Arm32);
    let mut output_file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&bin_file)
        .expect("create bt0 binary file");
    output_file.write_all(&egon_bin).unwrap();
}

fn build_image(env: &Env, dir: &PathBuf, bins: &Bins, features: &[String]) {
    // Build the stages - should we parallelize this?
    build_bt32(env, dir, bins, features);
}
