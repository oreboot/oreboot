use crate::util::dist_dir;
use crate::{Env, Memory};
use log::{error, info, trace};
use std::{
    io::ErrorKind,
    process::{self, Command, Stdio},
};

// TODO: factor out generic command detection function
const CMD: &str = "xfel";

pub(crate) fn find_xfel() -> &'static str {
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

pub(crate) fn xfel_find_connected_device(xfel: &str) {
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

pub(crate) fn flash_image(xfel: &str, env: &Env, image_bin: &str, target_dir: &str) {
    println!("Write to flash with {xfel}");
    let mut cmd = Command::new(xfel);
    cmd.current_dir(dist_dir(env, target_dir));
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
    cmd.arg(image_bin);
    println!("Command: {cmd:?}");
    let status = cmd.status().unwrap();
    trace!("xfel returned {status}");
    if !status.success() {
        error!("xfel failed with {status}");
        process::exit(1);
    }
}

pub(crate) fn run(xfel: &str, env: &Env, target_dir: &str, image_bin: &str, addr: usize) {
    println!("Run with {xfel}");
    let mut cmd = Command::new(xfel);
    cmd.current_dir(dist_dir(env, target_dir));
    cmd.args(["write", format!("0x{addr}").as_str()]);
    cmd.arg(image_bin);
    println!("Command: {cmd:?}");
    let status = cmd.status().unwrap();
    trace!("xfel returned {status}");
    if !status.success() {
        error!("xfel failed with {status}");
        process::exit(1);
    }
    let mut cmd = Command::new(xfel);
    cmd.current_dir(dist_dir(env, target_dir));
    cmd.args(["exec", format!("0x{addr}").as_str()]);
    println!("Command: {cmd:?}");
    let status = cmd.status().unwrap();
    trace!("xfel returned {status}");
    if !status.success() {
        error!("xfel failed with {status}");
        process::exit(1);
    }
}
