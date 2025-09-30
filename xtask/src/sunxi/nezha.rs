use std::{
    fs::File,
    io::{self, Seek, SeekFrom, Write},
    path::PathBuf,
    process::{self, Command},
};

use log::{error, info, trace};

use crate::util::{
    find_binutils_prefix_or_fail, get_bin_for, get_cargo_cmd_in, objcopy, objdump, platform_dir,
    target_dir, Bin,
};
use crate::{
    gdb_detect,
    sunxi::{egon, fel},
    Cli, Commands, Env,
};

// TODO: detect architecture for binutils
const ARCH: &str = "riscv64";
// TODO: instead of hardcoding, create one binary per feature set.
const IMAGE_BIN: &str = "oreboot-nezha.bin";
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
            info!("Build oreboot image for D1");
            build_image(&args.env, dir, &stages, &features);
        }
        Commands::Run => {
            todo!("implement {:?} command", args.command);
        }
        Commands::Flash => {
            // TODO: print out variant etc
            info!("Build and flash oreboot image for D1");
            fel::xfel_find_connected_device();
            build_image(&args.env, dir, &stages, &features);
            fel::flash_image(&args.env, dir, IMAGE_BIN);
        }
        Commands::Asm => {
            info!("Build bt0 and view assembly for D1");
            let binutils_prefix = &find_binutils_prefix_or_fail(ARCH);
            build_bt0(&args.env, dir, &stages.bt0, &features);
            objdump(&args.env, &stages.bt0, binutils_prefix);
        }
        Commands::Gdb => {
            info!("Debug bt0 for D1 using gdb");
            build_bt0(&args.env, dir, &stages.bt0, &features);
            let gdb_path = if let Ok(ans) = gdb_detect::load_gdb_path_from_file() {
                ans
            } else {
                let ans = gdb_detect::detect_gdb_path();
                gdb_detect::save_gdb_path_to_file(&ans);
                trace!("saved GDB path");
                ans
            };
            let gdb_server = if let Ok(ans) = gdb_detect::load_gdb_server_from_file() {
                ans
            } else {
                let ans = gdb_detect::detect_gdb_server(&gdb_path);
                gdb_detect::save_gdb_server_to_file(&ans);
                trace!("saved GDB server");
                ans
            };
            debug_gdb(&args.env, &stages.bt0, &gdb_path, &gdb_server);
        }
    }
}

fn build_image(env: &Env, dir: &PathBuf, stages: &Stages, features: &[String]) {
    // Build the stages - should we parallelize this?
    build_bt0(env, dir, &stages.bt0, features);
    build_main(env, dir, &stages.main);
    concat_binaries(env, dir, stages);
}

fn build_bt0(env: &Env, dir: &PathBuf, bin: &Bin, features: &[String]) {
    trace!("build {BT0_STAGE}");
    // Get binutils first so we can fail early
    let binutils_prefix = &find_binutils_prefix_or_fail(ARCH);
    let mut command = get_cargo_cmd_in(env, dir, BT0_STAGE, "build");
    if !features.is_empty() {
        let platform_features = features.join(",");
        trace!("append features: {platform_features}");
        command.arg("--no-default-features");
        command.args(["--features", &platform_features]);
    } else {
        trace!("no features appended");
    }
    trace!("run build command: {command:?}");
    let status = command.status().unwrap();
    trace!("cargo returned {status}");
    if !status.success() {
        error!("cargo build failed with {status}");
        process::exit(1);
    }
    objcopy(env, &bin, binutils_prefix, ARCH);
    let bin_file = target_dir(env, &bin.target).join(&bin.bin_name);
    let bt0 = std::fs::read(&bin_file).expect("opening bt0 binary file");
    let egon_bin = egon::add_header(&bt0, egon::Arch::Riscv64);
    let mut output_file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&bin_file)
        .expect("patch bt0 binary file");
    output_file.write_all(&egon_bin).unwrap();
}

fn build_main(env: &Env, dir: &PathBuf, bin: &Bin) {
    trace!("build D1 main");
    let binutils_prefix = &find_binutils_prefix_or_fail(ARCH);
    let mut command = get_cargo_cmd_in(env, dir, "main", "build");
    if env.supervisor {
        command.args(["--features", "supervisor"]);
    }
    let status = command.status().unwrap();
    trace!("cargo returned {status}");
    if !status.success() {
        error!("cargo build failed with {status}");
        process::exit(1);
    }
    objcopy(env, &bin, binutils_prefix, ARCH);
}

const FLASH_IMG_SIZE: u64 = 16 * 1024 * 1024;
const M_MODE_PAYLOAD_SIZE: u64 = 2 * 1024 * 1024;
const MAX_COMPRESSED_SIZE: usize = 0x00fc_0000;

fn concat_binaries(env: &Env, dir: &PathBuf, stages: &Stages) {
    let plat_dir = platform_dir(dir);
    let image_path = plat_dir.join(IMAGE_BIN);
    println!("Stitching final image 🏗️ {image_path:?}");

    let mut bt0_file = File::options()
        .read(true)
        .open(target_dir(env, &stages.bt0.target).join(&stages.bt0.bin_name))
        .expect("open bt0 binary file");
    let mut main_file = File::options()
        .read(true)
        .open(target_dir(env, &stages.main.target).join(&stages.main.bin_name))
        .expect("open main binary file");

    // TODO: evaluate flash layout
    let bt0_len = 32 * 1024;
    let max_main_len = 96 * 1024;
    let dtfs_len = 64 * 1024;

    let payload_offset = bt0_len + max_main_len + dtfs_len;
    let dtb_len = 64 * 1024;

    let mut image_file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&image_path)
        .expect("create image file");

    let img_size = match (env.payload.as_deref(), env.supervisor) {
        (Some(_), true) => {
            println!("  Full image with oreboot SBI");
            FLASH_IMG_SIZE
        }
        (Some(_), _) => {
            println!("  Image with M-mode payload, no SBI");
            payload_offset + M_MODE_PAYLOAD_SIZE
        }
        (None, _) => {
            println!("No payload, will update oreboot only");
            payload_offset
        }
    };
    println!("  Size: {img_size} bytes");
    image_file.set_len(img_size).unwrap(); // FIXME: depend on storage

    println!("bt0 stage\n  Size: {bt0_len} bytes");
    io::copy(&mut bt0_file, &mut image_file).expect("copying bt0 stage");

    let pos = SeekFrom::Start(bt0_len);
    image_file.seek(pos).expect("seek after bt0");
    io::copy(&mut main_file, &mut image_file).expect("copying main stage");
    let main_len = main_file.metadata().unwrap().len();
    println!("main stage\n  Size: {main_len} bytes");

    if let Some(payload_file) = env.payload.as_deref() {
        if env.supervisor {
            env.dtb.as_deref().expect("provide a DTB for LinuxBoot");
        }
        println!("Payload\n  File: {payload_file}");
        let payload = std::fs::read(payload_file).expect("open payload file");
        println!("  Size: {} bytes", payload.len());
        let pos = SeekFrom::Start(payload_offset);
        image_file.seek(pos).expect("seek after main stage");
        println!("  Compressing...");
        let compressed = miniz_oxide::deflate::compress_to_vec_zlib(&payload, 9);
        let size = compressed.len();
        if size <= MAX_COMPRESSED_SIZE {
            println!("  Compressed size: {size}");
        } else {
            panic!("  Compressed payload too large: {size} (max. {MAX_COMPRESSED_SIZE})");
        }

        println!("{:02x?}", &compressed[..0x20]);

        image_file.write_all(&compressed).expect("copying payload");
        if let Some(dtb) = env.dtb.as_deref() {
            println!("DTB\n  File: {dtb}");
            let mut dtb_file = File::options().read(true).open(dtb).expect("open dtb file");
            let pos = SeekFrom::Start(FLASH_IMG_SIZE - dtb_len);
            image_file.seek(pos).expect("seek to DTB position");
            io::copy(&mut dtb_file, &mut image_file).expect("copy dtb");
            let dtb_len = dtb_file.metadata().unwrap().len();
            println!("  Size: {dtb_len} bytes");
        }
    }

    println!("Output\n  File: {image_path:?}",);
    println!("======= DONE =======");
}

fn debug_gdb(env: &Env, bin: &Bin, gdb_path: &str, gdb_server: &str) {
    let mut command = Command::new(gdb_path);
    command.current_dir(target_dir(env, &bin.target));
    command.args(["--eval-command", &format!("file {}", bin.elf_name)]);
    command.args(["--eval-command", "set architecture riscv:rv64"]);
    command.args(["--eval-command", "mem 0x0 0xffff ro"]);
    command.args(["--eval-command", "mem 0x20000 0x27fff rw"]);
    command.args(["--eval-command", &format!("target remote {gdb_server}")]);
    command.arg("-q");
    ctrlc::set_handler(move || {
        // when ctrl-c, don't exit gdb
    })
    .expect("disable Ctrl-C exit");
    let status = command.status().unwrap();
    if !status.success() {
        error!("gdb failed with {}", status);
        process::exit(1);
    }
}
