use crate::dist_dir;
use crate::project_root;
use crate::{Commands, Env};
use log::{error, info, trace};
use std::env;
use std::fs::File;
use std::io::{self, Seek, SeekFrom};
use std::process::{self, Command, Stdio};

// const SRAM0_SIZE = 128 * 1024;
const SRAM0_SIZE: u64 = 32 * 1024;

const DEFAULT_TARGET: &str = "riscv64imac-unknown-none-elf";

pub(crate) fn execute_command(args: &crate::Cli, features: Vec<String>) {
    match args.command {
        Commands::Make => {
            info!("building VisionFive1");
            let binutils_prefix = find_binutils_prefix_or_fail();
	    // dtb
	    xtask_build_dtb(env: &Env);
            // bt0 stage
            xtask_build_jh7100_flash_bt0(&args.env, &features);
            xtask_binary_jh7100_flash_bt0(binutils_prefix, &args.env);
            // main stage
            xtask_build_jh7100_flash_main(&args.env);
            xtask_binary_jh7100_flash_main(binutils_prefix, &args.env);
            xtask_concat_flash_binaries(&args.env);
	    xtask_build_dtb_image(&args.env);
        }
        _ => {
            error!("command {:?} not implemented", args.command);
        }
    }
}

fn xtask_build_dtb(env: &Env) {
    trace!("build dtb");
    let mut command = Command::new("dtc");
    command.current_dir(board_project_root());
    command.arg("-o");
    command.arg("board.dtb");
    command.arg("board.dts");
    let status = command.status().unwrap();
    trace!("dtc returned {}", status);
    if !status.success() {
        error!("dtc build failed with {}", status);
        process::exit(1);
    }
}

fn xtask_build_jh7100_flash_bt0(env: &Env, features: &Vec<String>) {
    trace!("build JH7100 flash bt0");
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

fn xtask_binary_jh7100_flash_bt0(prefix: &str, env: &Env) {
    trace!("objcopy binary, prefix: '{}'", prefix);
    let status = Command::new(format!("{}objcopy", prefix))
        .current_dir(dist_dir(env, DEFAULT_TARGET))
        .arg("starfive-visionfive1-bt0")
        .arg("--binary-architecture=riscv64")
        .arg("--strip-all")
        .args(["-O", "binary", "starfive-visionfive1-bt0.bin"])
        .status()
        .unwrap();

    trace!("objcopy returned {}", status);
    if !status.success() {
        error!("objcopy failed with {}", status);
        process::exit(1);
    }
}

fn xtask_build_jh7100_flash_main(env: &Env) {
    trace!("build JH7100 flash main");
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

fn xtask_binary_jh7100_flash_main(prefix: &str, env: &Env) {
    trace!("objcopy binary, prefix: '{}'", prefix);
    let status = Command::new(format!("{}objcopy", prefix))
        .current_dir(dist_dir(env, DEFAULT_TARGET))
        .arg("starfive-visionfive1-main")
        .arg("--binary-architecture=riscv64")
        .arg("--strip-all")
        .args(["-O", "binary", "starfive-visionfive1-main.bin"])
        .status()
        .unwrap();

    trace!("objcopy returned {}", status);
    if !status.success() {
        error!("objcopy failed with {}", status);
        process::exit(1);
    }
}

fn xtask_concat_flash_binaries(env: &Env) {
    let dist_dir = dist_dir(env, DEFAULT_TARGET);
    let mut bt0_file = File::options()
        .read(true)
        .open(dist_dir.join("starfive-visionfive1-bt0.bin"))
        .expect("open bt0 binary file");
    let mut main_file = File::options()
        .read(true)
        .open(dist_dir.join("starfive-visionfive1-main.bin"))
        .expect("open main binary file");

    let output_file_path = dist_dir.join("starfive-visionfive1.bin");
    let mut output_file = File::options()
        .write(true)
        .create(true)
        .open(&output_file_path)
        .expect("create output binary file");

    output_file.set_len(SRAM0_SIZE).unwrap(); // FIXME: depend on storage

    let bt0_len = 30 * 1024;
    io::copy(&mut bt0_file, &mut output_file).expect("copy bt0 binary");
    output_file
        .seek(SeekFrom::Start(bt0_len))
        .expect("seek after bt0 copy");
    io::copy(&mut main_file, &mut output_file).expect("copy main binary");

    println!("======= DONE =======");
    println!("Output file: {:?}", &output_file_path.into_os_string());
}

fn xtask_build_dtb_image(env: &Env) {
    let dist_dir = dist_dir(env, DEFAULT_TARGET);
    let mut bt0_file = File::options()
        .read(true)
        .open(dist_dir.join("starfive-visionfive1-bt0.bin"))
        .expect("open bt0 binary file");
    let mut main_file = File::options()
        .read(true)
        .open(dist_dir.join("starfive-visionfive1-main.bin"))
        .expect("open main binary file");

    let output_file_path = dist_dir.join("starfive-visionfive1.bin");
    let mut output_file = File::options()
        .write(true)
        .create(true)
        .open(&output_file_path)
        .expect("create output binary file");

    output_file.set_len(SRAM0_SIZE).unwrap(); // FIXME: depend on storage

    let bt0_len = 30 * 1024;
    io::copy(&mut bt0_file, &mut output_file).expect("copy bt0 binary");
    output_file
        .seek(SeekFrom::Start(bt0_len))
        .expect("seek after bt0 copy");
    io::copy(&mut main_file, &mut output_file).expect("copy main binary");

    println!("======= DONE =======");
    println!("Output file: {:?}", &output_file_path.into_os_string());
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

// FIXME: factor out, rework, share!
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

// FIXME: factor out, rework, share!
fn board_project_root() -> std::path::PathBuf {
    project_root().join("src/mainboard/starfive/visionfive1")
}
