use crate::util::{
    compile_board_dt, dist_dir, find_binutils_prefix_or_fail, get_cargo_cmd_in, objcopy,
    project_root,
};
use crate::{layout_flash, Commands, Env};
// use fdt;
use log::{error, info, trace};
use std::{
    fs::{self, File},
    io,
    path::Path,
    process,
};

extern crate layoutflash;
use layoutflash::areas::{create_areas, Area};

// const SRAM0_SIZE = 128 * 1024;
const SRAM0_SIZE: u64 = 32 * 1024;

const ARCH: &str = "riscv64";
const TARGET: &str = "riscv64imac-unknown-none-elf";

const MAIN_BIN: &str = "emulation-qemu-riscv-main.bin";
const MAIN_ELF: &str = "emulation-qemu-riscv-main";

const BOARD_DTB: &str = "emulation-qemu-riscv-board.dtb";

const FDT_BIN: &str = "emulation-qemu-riscv-board.fdtbin";

const IMAGE_BIN: &str = "emulation-qemu-riscv.bin";

pub(crate) fn execute_command(args: &crate::Cli, _features: Vec<String>) {
    match args.command {
        Commands::Make => {
            info!("building QEMU RiscV");
            let binutils_prefix = &find_binutils_prefix_or_fail(ARCH);
            // Build the stages - should we parallelize this?
            xtask_build_qemu_riscv_flash_main(&args.env);

            objcopy(&args.env, binutils_prefix, TARGET, ARCH, MAIN_ELF, MAIN_BIN);
            xtask_concat_flash_binaries(&args.env);

            // dtb
            compile_board_dt(&args.env, TARGET, &board_project_root(), BOARD_DTB);
            xtask_build_dtb_image(&args.env);
        }
        _ => {
            error!("command {:?} not implemented", args.command);
        }
    }
}

fn xtask_build_qemu_riscv_flash_main(env: &Env) {
    trace!("build QEMU RiscV flash main");
    let mut command = get_cargo_cmd_in(env, board_project_root(), "main", "build");
    let status = command.status().unwrap();
    trace!("cargo returned {}", status);
    if !status.success() {
        error!("cargo build failed with {}", status);
        process::exit(1);
    }
}

fn xtask_concat_flash_binaries(env: &Env) {
    let dist_dir = dist_dir(env, TARGET);
    let mut main_file = File::options()
        .read(true)
        .open(dist_dir.join(MAIN_BIN))
        .expect("open main binary file");

    let output_file_path = dist_dir.join(IMAGE_BIN);
    let mut output_file = File::options()
        .write(true)
        .create(true)
        .open(&output_file_path)
        .expect("create output binary file");

    output_file.set_len(SRAM0_SIZE).unwrap(); // FIXME: depend on storage
    io::copy(&mut main_file, &mut output_file).expect("copy main binary");

    println!("======= DONE =======");
    println!("Output file: {:?}", &output_file_path.into_os_string());
}

fn xtask_build_dtb_image(env: &Env) {
    let dist_dir = dist_dir(env, TARGET);
    let dtb_path = dist_dir.join(BOARD_DTB);
    let dtb = fs::read(dtb_path).expect("dtb");

    let output_file_path = dist_dir.join(FDT_BIN);
    let output_file = File::options()
        .write(true)
        .create(true)
        .open(&output_file_path)
        .expect("create output binary file");

    output_file.set_len(SRAM0_SIZE).unwrap(); // FIXME: depend on storage

    let fdt = fdt::Fdt::new(&dtb).unwrap();
    let mut areas: Vec<Area> = vec![];
    areas.resize(
        16,
        Area {
            name: "",
            offset: None,
            size: 0,
            file: None,
        },
    );
    let areas = create_areas(&fdt, &mut areas);

    layout_flash(
        Path::new(&dist_dir),
        Path::new(&output_file_path),
        areas.to_vec(),
    )
    .unwrap();
    println!("======= DONE =======");
    println!("Output file: {:?}", &output_file_path.into_os_string());
}

// FIXME: factor out, rework, share!
fn board_project_root() -> std::path::PathBuf {
    project_root().join("src/mainboard/emulation/qemu-riscv")
}
