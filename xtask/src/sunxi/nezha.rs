use crate::dist_dir;
use crate::gdb_detect;
use crate::project_root;
use crate::{Commands, Env, Memory};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use log::{error, info, trace};
use std::env;
use std::fs::File;
use std::io::{self, ErrorKind, Seek, SeekFrom};
use std::process::{self, Command, Stdio};

pub(crate) fn execute_command(args: &crate::Cli, features: Vec<String>) {
    match args.command {
        Commands::Make => {
            info!("make D1 flash binary");
            let binutils_prefix = find_binutils_prefix_or_fail();
            xtask_build_d1_flash_bt0(&args.env, &features);
            xtask_build_d1_flash_main(&args.env);
            xtask_binary_d1_flash_bt0(binutils_prefix, &args.env);
            xtask_binary_d1_flash_main(binutils_prefix, &args.env);
            xtask_finialize_d1_flash(&args.env);
            xtask_concat_flash_binaries(&args.env);
        }
        Commands::Flash => {
            info!("build D1 binary and burn");
            let xfel = find_xfel();
            xfel_find_connected_device(xfel);
            let binutils_prefix = find_binutils_prefix_or_fail();
            xtask_build_d1_flash_bt0(&args.env, &features);
            xtask_build_d1_flash_main(&args.env);
            xtask_binary_d1_flash_bt0(binutils_prefix, &args.env);
            xtask_binary_d1_flash_main(binutils_prefix, &args.env);
            xtask_finialize_d1_flash(&args.env);
            xtask_concat_flash_binaries(&args.env);
            xtask_burn_d1_flash_bt0(xfel, &args.env);
        }
        Commands::Asm => {
            info!("build D1 flash ELF and view assembly");
            let binutils_prefix = find_binutils_prefix_or_fail();
            xtask_build_d1_flash_bt0(&args.env, &features);
            xtask_dump_d1_flash_bt0(binutils_prefix, &args.env);
        }
        Commands::Gdb => {
            info!("debug using gdb");
            xtask_build_d1_flash_bt0(&args.env, &features);
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
            xtask_debug_gdb(&gdb_path, &gdb_server, &args.env);
        }
    }
}

const DEFAULT_TARGET: &'static str = "riscv64imac-unknown-none-elf";

fn xtask_build_d1_flash_bt0(env: &Env, features: &Vec<String>) {
    trace!("build D1 flash bt0");
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    trace!("found cargo at {}", cargo);
    let mut command = Command::new(cargo);
    command.current_dir(board_project_root().join("bt0"));
    command.arg("build");
    if env.release {
        command.arg("--release");
    }
    if features.len() != 0 {
        let command_line_features = features.join(",");
        trace!("append command line features: {}", command_line_features);
        command.arg("--no-default-features");
        command.args(&["--features", &command_line_features]);
    } else {
        trace!("no command line features appended");
    }
    let status = command.status().unwrap();
    trace!("cargo returned {}", status);
    if !status.success() {
        error!("cargo build failed with {}", status);
        process::exit(1);
    }
}

fn xtask_build_d1_flash_main(env: &Env) {
    trace!("build D1 flash main");
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    trace!("found cargo at {}", cargo);
    let mut command = Command::new(cargo);
    command.current_dir(board_project_root().join("main"));
    command.arg("build");
    if env.release {
        command.arg("--release");
    }
    let status = command.status().unwrap();
    trace!("cargo returned {}", status);
    if !status.success() {
        error!("cargo build failed with {}", status);
        process::exit(1);
    }
}

fn xtask_binary_d1_flash_bt0(prefix: &str, env: &Env) {
    trace!("objcopy binary, prefix: '{}'", prefix);
    let status = Command::new(format!("{}objcopy", prefix))
        .current_dir(dist_dir(env, DEFAULT_TARGET))
        .arg("oreboot-nezha-bt0")
        .arg("--binary-architecture=riscv64")
        .arg("--strip-all")
        .args(&["-O", "binary", "oreboot-nezha-bt0.bin"])
        .status()
        .unwrap();

    trace!("objcopy returned {}", status);
    if !status.success() {
        error!("objcopy failed with {}", status);
        process::exit(1);
    }
}

fn xtask_binary_d1_flash_main(prefix: &str, env: &Env) {
    trace!("objcopy binary, prefix: '{}'", prefix);
    let status = Command::new(format!("{}objcopy", prefix))
        .current_dir(dist_dir(env, DEFAULT_TARGET))
        .arg("oreboot-nezha-main")
        .arg("--binary-architecture=riscv64")
        .arg("--strip-all")
        .args(&["-O", "binary", "oreboot-nezha-main.bin"])
        .status()
        .unwrap();

    trace!("objcopy returned {}", status);
    if !status.success() {
        error!("objcopy failed with {}", status);
        process::exit(1);
    }
}

const EGON_HEAD_LENGTH: u64 = 0x60;
const MAIN_STAGE_HEAD_LENGTH: u64 = 0x08;
const TOTAL_HEAD_LENGTH: u64 = EGON_HEAD_LENGTH + MAIN_STAGE_HEAD_LENGTH;

// This function does:
// 1. fill in binary length of bt0
// 2. fill in flash length of main stage
// 3. calculate checksum of bt0 image; old checksum value must be filled as stamp value
fn xtask_finialize_d1_flash(env: &Env) {
    let path = dist_dir(env, DEFAULT_TARGET);
    let mut bt0_file = File::options()
        .read(true)
        .write(true)
        .open(path.join("oreboot-nezha-bt0.bin"))
        .expect("open output binary file");
    let total_length = bt0_file.metadata().unwrap().len();
    if total_length < TOTAL_HEAD_LENGTH {
        error!(
            "objcopy binary size less than minimal header length, expected >= {} but is {}",
            TOTAL_HEAD_LENGTH, total_length
        );
    }
    let main_stage_file = File::options()
        .read(true)
        .write(true)
        .open(path.join("oreboot-nezha-main.bin"))
        .expect("open output binary file");
    let main_stage_length = main_stage_file.metadata().unwrap().len();
    bt0_file.seek(SeekFrom::Start(0x64)).unwrap();
    bt0_file
        .write_u32::<LittleEndian>(main_stage_length as u32)
        .unwrap();
    let new_len = align_up_to(total_length, 16 * 1024); // align up to 16KB
    bt0_file.seek(SeekFrom::Start(0x60)).unwrap();
    bt0_file.write_u32::<LittleEndian>(new_len as u32).unwrap();
    bt0_file.set_len(new_len).unwrap();
    bt0_file.seek(SeekFrom::Start(0x10)).unwrap();
    bt0_file.write_u32::<LittleEndian>(new_len as u32).unwrap();
    bt0_file.seek(SeekFrom::Start(0x0C)).unwrap();
    let stamp = bt0_file.read_u32::<LittleEndian>().unwrap();
    if stamp != 0x5F0A6C39 {
        error!("wrong stamp value; check your generated blob and try again")
    }
    let mut checksum: u32 = 0;
    bt0_file.seek(SeekFrom::Start(0)).unwrap();
    loop {
        match bt0_file.read_u32::<LittleEndian>() {
            Ok(val) => checksum = checksum.wrapping_add(val),
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => break,
            Err(e) => error!("io error while calculating checksum: {:?}", e),
        }
    }
    bt0_file.seek(SeekFrom::Start(0x0C)).unwrap();
    bt0_file.write_u32::<LittleEndian>(checksum).unwrap();
    bt0_file.sync_all().unwrap(); // save file before automatic closing
} // for C developers: files are automatically closed when they're out of scope

fn align_up_to(len: u64, target_align: u64) -> u64 {
    let (div, rem) = (len / target_align, len % target_align);
    if rem != 0 {
        (div + 1) * target_align
    } else {
        len
    }
}

fn xtask_concat_flash_binaries(env: &Env) {
    let dist_dir = dist_dir(env, DEFAULT_TARGET);
    let mut bt0_file = File::options()
        .read(true)
        .open(dist_dir.join("oreboot-nezha-bt0.bin"))
        .expect("open bt0 binary file");
    let mut main_file = File::options()
        .read(true)
        .open(dist_dir.join("oreboot-nezha-main.bin"))
        .expect("open main binary file");
    let mut output_file = File::options()
        .write(true)
        .create(true)
        .open(dist_dir.join("oreboot-nezha.bin"))
        .expect("create output binary file");
    let bt0_len = bt0_file.metadata().unwrap().len();
    let new_len = bt0_len + main_file.metadata().unwrap().len();
    output_file.set_len(new_len).unwrap();
    io::copy(&mut bt0_file, &mut output_file).expect("copy bt0 binary");
    output_file
        .seek(SeekFrom::Start(bt0_len))
        .expect("seek after bt0 copy");
    io::copy(&mut main_file, &mut output_file).expect("copy main binary");
}

fn xtask_burn_d1_flash_bt0(xfel: &str, env: &Env) {
    trace!("burn flash with xfel {}", xfel);
    let mut command = Command::new(xfel);
    command.current_dir(dist_dir(env, DEFAULT_TARGET));
    match env.memory {
        Some(Memory::Nand) => command.arg("spinand"),
        Some(Memory::Nor) => command.arg("spinor"),
        None => {
            error!("no memory parameter found; use --memory nand or --memory nor");
            process::exit(1);
        }
    };
    command.args(["write", "0"]);
    command.arg("oreboot-nezha.bin");
    let status = command.status().unwrap();
    trace!("xfel returned {}", status);
    if !status.success() {
        error!("xfel failed with {}", status);
        process::exit(1);
    }
}

fn xtask_dump_d1_flash_bt0(prefix: &str, env: &Env) {
    Command::new(format!("{}objdump", prefix))
        .current_dir(dist_dir(env, DEFAULT_TARGET))
        .arg("oreboot-nezha-bt0")
        .arg("-d")
        .status()
        .unwrap();
}

fn xtask_debug_gdb(gdb_path: &str, gdb_server: &str, env: &Env) {
    let mut command = Command::new(gdb_path);
    command.current_dir(dist_dir(env, DEFAULT_TARGET));
    command.args(&["--eval-command", "file oreboot-nezha-bt0"]);
    command.args(&["--eval-command", "set architecture riscv:rv64"]);
    command.args(&["--eval-command", "mem 0x0 0xffff ro"]);
    command.args(&["--eval-command", "mem 0x20000 0x27fff rw"]);
    command.args(&["--eval-command", &format!("target remote {}", gdb_server)]);
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

fn find_xfel() -> &'static str {
    let mut command = Command::new("xfel");
    command.stdout(Stdio::null());
    match command.status() {
        Ok(status) if status.success() => return "xfel",
        Ok(status) => match status.code() {
            Some(code) => {
                error!("xfel command failed with code {}", code);
                process::exit(code)
            }
            None => error!("xfel command terminated by signal"),
        },
        Err(e) if e.kind() == ErrorKind::NotFound => error!(
            "xfel not found
    install xfel from: https://github.com/xboot/xfel"
        ),
        Err(e) => error!(
            "I/O error occurred when detecting xfel: {}.
    Please check your xfel program and try again.",
            e
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

fn find_binutils_prefix() -> Option<&'static str> {
    for prefix in ["rust-", "riscv64-unknown-elf-", "riscv64-linux-gnu-"] {
        let mut command = Command::new(format!("{}objcopy", prefix));
        command.arg("--version");
        command.stdout(Stdio::null());
        let status = command.status().unwrap();
        if status.success() {
            return Some(prefix);
        }
    }
    None
}

fn find_binutils_prefix_or_fail() -> &'static str {
    trace!("find binutils");
    if let Some(ans) = find_binutils_prefix() {
        trace!("found binutils, prefix is '{}'", ans);
        return ans;
    }
    error!(
        "no binutils found, try install using:
    rustup component add llvm-tools-preview
    cargo install cargo-binutils"
    );
    process::exit(1)
}

fn board_project_root() -> std::path::PathBuf {
    project_root().join("src/mainboard/sunxi/nezha")
}
