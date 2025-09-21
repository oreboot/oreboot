use log::{error, info, trace};

use crate::{
    sunxi::{egon, xfel},
    util::{find_binutils_prefix_or_fail, objcopy},
    Cli, Commands, Env, Memory,
};

const ARCH: &str = "arm32";
const TARGET: &str = "armv7a-none-eabi";
const BOARD_DIR: &str = "src/mainboard/sunxi/H616";

const BT0_ELF: &str = "oreboot-allwinner-h616-bt0";
const BT0_BIN: &str = "oreboot-allwinner-h616-bt0.bin";
const BT0_ADDR: usize = 0x20000;

pub(crate) fn execute_command(args: &Cli, features: Vec<String>) {
    match args.command {
        Commands::Make => {
            info!("Build oreboot image for H616");
            todo!();
            // build_image(&args.env, &features);
        }
        Commands::Flash => {
            // TODO: print out variant etc
            todo!("Build and flash oreboot image for H616");
            /*
            let xfel = find_xfel();
            xfel_find_connected_device(xfel);
            build_image(&args.env, &features);
            burn_H616_bt0(xfel, &args.env);
            */
        }
        Commands::Run => {
            // TODO: print out variant etc
            info!("Run image on H616 via FEL");
            let xfel = xfel::find_xfel();
            xfel::xfel_find_connected_device(xfel);
            build_image(&args.env, &features);
            xfel::run(xfel, &args.env, TARGET, BT0_BIN, BT0_ADDR);
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

fn build_bt0(env: &Env, features: &[String]) {
    todo!()
}

fn build_image(env: &Env, features: &[String]) {
    // Get binutils first so we can fail early
    let binutils_prefix = &find_binutils_prefix_or_fail(ARCH);
    // Build the stages - should we parallelize this?
    build_bt0(env, features);
    objcopy(env, binutils_prefix, TARGET, ARCH, BT0_ELF, BT0_BIN);
    let bt0_bin = vec![]; // TODO
    egon::add_header(&bt0_bin, egon::Arch::Arm32);
}
