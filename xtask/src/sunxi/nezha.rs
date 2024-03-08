use crate::util::{
    dist_dir, find_binutils_prefix_or_fail, get_cargo_cmd_in, objcopy, objdump, project_root,
};
use crate::{gdb_detect, Cli, Commands, Env, Memory};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use log::{error, info, trace};
use std::{
    fs::File,
    io::{self, ErrorKind, Seek, SeekFrom, Write},
    process::{self, Command, Stdio},
};

use oreboot_compression::OreLzss;

const ARCH: &str = "riscv64";
const TARGET: &str = "riscv64imac-unknown-none-elf";
const BOARD_DIR: &str = "src/mainboard/sunxi/nezha";

const BT0_ELF: &str = "oreboot-nezha-bt0";
const BT0_BIN: &str = "oreboot-nezha-bt0.bin";

const MAIN_ELF: &str = "oreboot-nezha-main";
const MAIN_BIN: &str = "oreboot-nezha-main.bin";

const IMAGE_BIN: &str = "oreboot-nezha.bin";

pub(crate) fn execute_command(args: &Cli, features: Vec<String>) {
    match args.command {
        Commands::Make => {
            info!("Build oreboot image for D1");
            build_image(&args.env, &features);
        }
        Commands::Flash => {
            // TODO: print out variant etc
            info!("Build and flash oreboot image for D1");
            let xfel = find_xfel();
            xfel_find_connected_device(xfel);
            build_image(&args.env, &features);
            burn_d1_bt0(xfel, &args.env);
        }
        Commands::Asm => {
            info!("Build bt0 and view assembly for D1");
            let binutils_prefix = &find_binutils_prefix_or_fail(ARCH);
            build_d1_bt0(&args.env, &features);
            objdump(&args.env, binutils_prefix, TARGET, BT0_ELF);
        }
        Commands::Gdb => {
            info!("Debug bt0 for D1 using gdb");
            build_d1_bt0(&args.env, &features);
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
            debug_gdb(&gdb_path, &gdb_server, &args.env);
        }
    }
}

fn build_image(env: &Env, features: &[String]) {
    // Get binutils first so we can fail early
    let binutils_prefix = &find_binutils_prefix_or_fail(ARCH);
    // Build the stages - should we parallelize this?
    build_d1_bt0(env, features);
    build_d1_main(env);
    objcopy(env, binutils_prefix, TARGET, ARCH, BT0_ELF, BT0_BIN);
    objcopy(env, binutils_prefix, TARGET, ARCH, MAIN_ELF, MAIN_BIN);
    bt0_egon_header(env);
    concat_binaries(env);
}

fn build_d1_bt0(env: &Env, features: &[String]) {
    trace!("build D1 bt0");
    let mut command = get_cargo_cmd_in(env, board_project_root(), "bt0", "build");
    if !features.is_empty() {
        let command_line_features = features.join(",");
        trace!("append command line features: {command_line_features}");
        command.arg("--no-default-features");
        command.args(["--features", &command_line_features]);
    } else {
        trace!("no command line features appended");
    }
    trace!("run D1 bt0 build command: {command:?}");
    let status = command.status().unwrap();
    trace!("cargo returned {status}");
    if !status.success() {
        error!("cargo build failed with {status}");
        process::exit(1);
    }
}

fn build_d1_main(env: &Env) {
    trace!("build D1 main");
    let mut command = get_cargo_cmd_in(env, board_project_root(), "main", "build");
    if env.supervisor {
        command.arg("--features");
        command.arg("supervisor");
    }
    let status = command.status().unwrap();
    trace!("cargo returned {status}");
    if !status.success() {
        error!("cargo build failed with {status}");
        process::exit(1);
    }
}

const EGON_HEAD_LENGTH: u64 = 0x60;

// This function does:
// 1. fill in binary length of bt0
// 2. calculate checksum of bt0 image
// NOTE: old checksum value must be filled as stamp value
fn bt0_egon_header(env: &Env) {
    println!("Filling eGON header...");
    let path = dist_dir(env, TARGET);
    let mut bt0_bin = File::options()
        .read(true)
        .write(true)
        .open(path.join(BT0_BIN))
        .expect("open bt0 binary file");
    let bt0_len = bt0_bin.metadata().unwrap().len();
    if bt0_len < EGON_HEAD_LENGTH {
        error!("bt0 size {bt0_len} less than minimal header length {EGON_HEAD_LENGTH}");
    }
    let new_len = align_up_to(bt0_len, 16 * 1024); // align up to 16KB
    bt0_bin.set_len(new_len).unwrap();
    bt0_bin.seek(SeekFrom::Start(0x10)).unwrap();
    bt0_bin.write_u32::<LittleEndian>(new_len as u32).unwrap();

    // fill in checksum
    bt0_bin.seek(SeekFrom::Start(0x0C)).unwrap();
    let stamp = bt0_bin.read_u32::<LittleEndian>().unwrap();
    if stamp != 0x5F0A6C39 {
        error!("wrong stamp value; check your generated blob and try again")
    }
    let mut checksum: u32 = 0;
    bt0_bin.rewind().unwrap();
    loop {
        match bt0_bin.read_u32::<LittleEndian>() {
            Ok(val) => checksum = checksum.wrapping_add(val),
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => break,
            Err(e) => error!("calculating checksum: {e:?}"),
        }
    }
    bt0_bin.seek(SeekFrom::Start(0x0C)).unwrap();
    bt0_bin.write_u32::<LittleEndian>(checksum).unwrap();
    bt0_bin.sync_all().unwrap();
}

fn align_up_to(len: u64, target_align: u64) -> u64 {
    let (div, rem) = (len / target_align, len % target_align);
    if rem != 0 {
        (div + 1) * target_align
    } else {
        len
    }
}

const FLASH_IMG_SIZE: u64 = 16 * 1024 * 1024;
// 4 bytes describe the payload size
const MAX_COMPRESSED_SIZE: usize = 0x00fe_0000 - 4;

fn concat_binaries(env: &Env) {
    println!("Stitching final image 🏗️");
    let dist_dir = dist_dir(env, TARGET);
    let mut bt0_file = File::options()
        .read(true)
        .open(dist_dir.join(BT0_BIN))
        .expect("open bt0 binary file");
    let mut main_file = File::options()
        .read(true)
        .open(dist_dir.join(MAIN_BIN))
        .expect("open main binary file");

    // TODO: evaluate flash layout
    let bt0_len = 32 * 1024;
    let max_main_len = 96 * 1024;
    let dtfs_len = 64 * 1024;

    const M_MODE_PAYLOAD_SIZE: u64 = 2 * 1024 * 1024;
    let payload_offset = bt0_len + max_main_len + dtfs_len;
    let dtb_len = 64 * 1024;

    let output_file_path = dist_dir.join(IMAGE_BIN);
    let mut output_file = File::options()
        .write(true)
        .create(true)
        .open(&output_file_path)
        .expect("create output binary file");

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
    output_file.set_len(img_size).unwrap(); // FIXME: depend on storage

    println!("bt0 stage\n  Size: {bt0_len} bytes");
    io::copy(&mut bt0_file, &mut output_file).expect("copying bt0 stage");

    let pos = SeekFrom::Start(bt0_len);
    output_file.seek(pos).expect("seek after bt0");
    io::copy(&mut main_file, &mut output_file).expect("copying main stage");
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
        output_file.seek(pos).expect("seek after main stage");
        println!("  Compressing...");
        let mut compressed = vec![0; MAX_COMPRESSED_SIZE];
        let result = OreLzss::compress_heap(
            lzss::SliceReader::new(&payload),
            lzss::SliceWriter::new(&mut compressed),
        );
        match result {
            Ok(r) => {
                println!("  Compressed size: {r}");
                output_file
                    .write_u32::<LittleEndian>(r as u32)
                    .expect("writing compressed payload size");
                output_file
                    .write_all(&compressed[..r])
                    .expect("copying payload");
            }
            Err(e) => {
                error!("compression failed, payload too large?\n  {e}\n");
                process::exit(1);
            }
        }
        if let Some(dtb) = env.dtb.as_deref() {
            println!("DTB\n  File: {dtb}");
            let mut dtb_file = File::options().read(true).open(dtb).expect("open dtb file");
            let pos = SeekFrom::Start(FLASH_IMG_SIZE - dtb_len);
            output_file.seek(pos).expect("seek to DTB position");
            io::copy(&mut dtb_file, &mut output_file).expect("copy dtb");
            let dtb_len = dtb_file.metadata().unwrap().len();
            println!("  Size: {dtb_len} bytes");
        }
    }

    if let Some(o) = output_file_path.into_os_string().to_str() {
        println!("Output\n  File: {o}",);
    } else {
        panic!("Could not get final output file.");
    }
    println!("======= DONE =======");
}

fn burn_d1_bt0(xfel: &str, env: &Env) {
    println!("Write to flash with {xfel}");
    let mut cmd = Command::new(xfel);
    cmd.current_dir(dist_dir(env, TARGET));
    match env.memory {
        Some(Memory::Nand) => cmd.arg("spinand"),
        Some(Memory::Nor) => cmd.arg("spinor"),
        // FIXME: error early, not here after minutes of build time!
        None => {
            error!("no memory parameter found; use --memory nand or --memory nor");
            process::exit(1);
        }
    };
    cmd.args(["write", "0"]);
    cmd.arg(IMAGE_BIN);
    println!("Command: {cmd:?}");
    let status = cmd.status().unwrap();
    trace!("xfel returned {status}");
    if !status.success() {
        error!("xfel failed with {status}");
        process::exit(1);
    }
}

fn debug_gdb(gdb_path: &str, gdb_server: &str, env: &Env) {
    let mut command = Command::new(gdb_path);
    command.current_dir(dist_dir(env, TARGET));
    command.args(["--eval-command", &format!("file {BT0_ELF}")]);
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

// TODO: factor out generic command detection function
const CMD: &str = "xfel";

fn find_xfel() -> &'static str {
    let mut command = Command::new(CMD);
    command.stdout(Stdio::null());
    match command.status() {
        Ok(status) if status.success() => return CMD,
        Ok(status) => match status.code() {
            Some(code) => {
                error!("{CMD} command failed with code {code}");
                process::exit(code)
            }
            None => error!("xfel command terminated by signal"),
        },
        Err(e) if e.kind() == ErrorKind::NotFound => error!(
            "{CMD} not found
    install xfel from: https://github.com/xboot/xfel"
        ),
        Err(e) => error!(
            "I/O error occurred when detecting xfel: {e}.
    Please check your xfel program and try again."
        ),
    }
    process::exit(1)
}

fn xfel_find_connected_device(xfel: &str) {
    let mut command = Command::new(xfel);
    command.arg("version");
    let output = command.output().unwrap();
    if !output.status.success() {
        error!("xfel failed with code {}", output.status);
        error!("Is your device in FEL mode?");
        process::exit(1);
    }
    info!("Found {}", String::from_utf8_lossy(&output.stdout).trim());
}

fn board_project_root() -> std::path::PathBuf {
    project_root().join(BOARD_DIR)
}
