use crate::util::{platform_dir, target_dir, Bin};
use crate::{Env, Memory};
use log::{error, info, trace};
use std::path::PathBuf;
use std::{
    io::ErrorKind,
    process::{self, Command, Stdio},
};

const XFEL_CMD: &str = "xfel";
const XFEL_URL: &str = "https://github.com/xboot/xfel";
const SUNXI_FEL_CMD: &str = "sunxi-fel";
const SUNXI_FEL_URL: &str = "https://github.com/linux-sunxi/sunxi-tools";
enum FelCommand {
    Xfel,
    SunxiFel,
}

fn find_cmd(cmd: FelCommand) -> &'static str {
    let (cmd, url) = match cmd {
        FelCommand::SunxiFel => (SUNXI_FEL_CMD, SUNXI_FEL_URL),
        FelCommand::Xfel => (XFEL_CMD, XFEL_URL),
    };
    let mut command = Command::new(cmd);
    command.stdout(Stdio::null());
    match command.status() {
        Ok(status) if status.success() => return cmd,
        Ok(status) => match status.code() {
            Some(code) => {
                error!("{cmd:?} command failed with code {code}");
                process::exit(code)
            }
            None => error!("{cmd:?} command terminated by signal"),
        },
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                error!("{cmd} not found; install from: {url}");
            }
            _ => error!("{cmd}: {e}. Please check the binary and try again."),
        },
    }
    process::exit(1)
}

pub(crate) fn find_xfel() -> &'static str {
    find_cmd(FelCommand::Xfel)
}

pub(crate) fn find_sunxi_fel() -> &'static str {
    find_cmd(FelCommand::SunxiFel)
}

pub(crate) fn xfel_find_connected_device() {
    let xfel = find_cmd(FelCommand::Xfel);
    let mut command = Command::new(xfel);
    command.arg("version");
    let output = command.output().unwrap();
    if !output.status.success() {
        error!("{xfel} failed with code {}", output.status);
        error!("Is your device in FEL mode?");
        process::exit(1);
    }
    info!("Found {}", String::from_utf8_lossy(&output.stdout).trim());
}

pub(crate) fn flash_image(env: &Env, dir: &PathBuf, image_bin: &str) {
    let xfel = find_cmd(FelCommand::Xfel);
    println!("Write to flash with {xfel}");
    let mut cmd = Command::new(xfel);
    cmd.current_dir(platform_dir(dir));
    match env.memory {
        Some(Memory::Nand) => cmd.arg("spinand"),
        Some(Memory::Nor) => cmd.arg("spinor"),
        // FIXME: error early, not here after minutes of build time!
        None => {
            error!("no memory parameter found; use --memory nand or --memory nor");
            process::exit(1);
        }
    };
    cmd.args(["write", "0", image_bin]);
    run_command(&mut cmd);
}

fn run_command(cmd: &mut Command) {
    println!("Command: {cmd:?}");
    let status = cmd.status().unwrap();
    trace!("{cmd:?} returned {status}");
    if !status.success() {
        error!("{cmd:?} failed with {status}");
        process::exit(1);
    }
}

pub(crate) fn xfel_run(env: &Env, bin: &Bin, addr: usize) {
    let xfel = find_cmd(FelCommand::Xfel);
    let target_dir = target_dir(env, &bin.target);
    println!("Run with {xfel}");
    let mut command = Command::new(xfel);
    command.current_dir(&target_dir);
    command.args(["write", format!("0x{addr:x}").as_str(), &bin.bin_name]);
    run_command(&mut command);
    let mut command = Command::new(xfel);
    command.current_dir(&target_dir);
    command.args(["exec", format!("0x{addr:x}").as_str()]);
    run_command(&mut command);
}

pub(crate) fn sunxi_fel_run(env: &Env, bin: &Bin) {
    let cmd = find_sunxi_fel();
    println!("Run with {cmd}");
    let mut command = Command::new(cmd);
    command.current_dir(target_dir(env, &bin.target));
    command.args(["spl", &bin.bin_name]);
    run_command(&mut command);
}
