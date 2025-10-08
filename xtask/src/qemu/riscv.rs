use std::{
    fs::{self, File},
    path::{Path, PathBuf},
    process,
};

use fdt::Fdt;
use log::{error, info, trace};

use layoutflash::layout::{create_areas, layout_flash};

use crate::util::{
    compile_board_dt, find_binutils_prefix_or_fail, get_bin_for, get_cargo_cmd_in, objcopy,
    platform_dir, target_dir, Bin,
};
use crate::{Commands, Env};

// const SRAM0_SIZE = 128 * 1024;
const SRAM0_SIZE: u64 = 32 * 1024;

const ARCH: &str = "riscv64";

const BOARD_DTB: &str = "emulation-qemu-riscv-board.dtb";

const IMAGE_BIN: &str = "oreboot-emulation-qemu-riscv.bin";

const MAIN_STAGE: &str = "main";
struct Stages {
    main: Bin,
}

pub(crate) fn execute_command(args: &crate::Cli, dir: &PathBuf, _features: Vec<String>) {
    let main = get_bin_for(dir, MAIN_STAGE);
    let stages = Stages { main };

    match args.command {
        Commands::Make => {
            info!("Build oreboot image for QEMU RISC-V");
            build_image(&args.env, dir, &stages);
        }
        _ => {
            error!("command {:?} not implemented", args.command);
        }
    }
}

fn xtask_build_qemu_riscv_flash_main(env: &Env, dir: &PathBuf, bin: &Bin) {
    trace!("build {MAIN_STAGE}");
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

fn xtask_build_dtb_image(env: &Env, dir: &PathBuf, stages: &Stages) {
    let plat_dir = platform_dir(dir);
    let target_dir = target_dir(env, &stages.main.target);

    let dtb_path = target_dir.join(BOARD_DTB);
    compile_board_dt(
        env,
        &stages.main.target,
        &plat_dir,
        dtb_path.to_str().unwrap(),
    );
    let dtb = fs::read(dtb_path).expect("dtb");

    let output_file_path = target_dir.join(IMAGE_BIN);
    let output_file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&output_file_path)
        .expect("create output binary file");

    output_file.set_len(SRAM0_SIZE).unwrap(); // FIXME: depend on storage

    let fdt = Fdt::new(&dtb).unwrap();
    let areas = create_areas(&fdt).unwrap();

    layout_flash(&target_dir, Path::new(&output_file_path), areas).unwrap();
    println!("======= DONE =======");
    println!("Output file: {:?}", &output_file_path.into_os_string());
}

fn build_image(env: &Env, dir: &PathBuf, stages: &Stages) {
    xtask_build_qemu_riscv_flash_main(env, dir, &stages.main);
    xtask_build_dtb_image(env, dir, stages);
}
