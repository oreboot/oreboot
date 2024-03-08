use crate::Env;
use log::{error, trace};
use std::{
    path::{Path, PathBuf},
    process::{self, Command, Stdio},
};

/// These utilities help find and run external commands.
/// Those are mostly toolchain components and vendor specific tools.

/// This gets you the `cargo` command in a specific directory.
/// Use it to build a stage of a mainboard, which is a board's subdirectory.
pub fn get_cargo_cmd_in(env: &Env, root: PathBuf, dir: &str, command: &str) -> Command {
    let cargo = std::env::var("CARGO").unwrap_or("cargo".to_string());
    trace!("found cargo at {cargo}");
    let d = root.join(dir);
    let mut cmd = Command::new(cargo);
    cmd.current_dir(d);
    cmd.arg(command);
    if env.release {
        cmd.arg("--release");
    }
    cmd
}

/// Compile the board device tree.
pub fn compile_board_dt(env: &Env, target: &str, root: &Path, dtb: &str) {
    trace!("compile board device tree {dtb}");
    let cwd = dist_dir(env, target);
    let mut command = Command::new("dtc");
    command.current_dir(cwd);
    command.arg("-o");
    command.arg(dtb);
    command.arg(root.join("board.dts"));
    let status = command.status().unwrap();
    trace!("dtc returned {status}");
    if !status.success() {
        error!("dtc build failed with {status}");
        process::exit(1);
    }
}

/// Create a raw binary from an ELF.
pub fn objcopy(env: &Env, prefix: &str, target: &str, arch: &str, elf_path: &str, bin_path: &str) {
    trace!("objcopy binary, prefix: '{prefix}'");
    let dir = dist_dir(env, target);
    let mut cmd = Command::new(format!("{prefix}objcopy"));
    cmd.current_dir(dir);
    cmd.arg(elf_path);
    cmd.arg(format!("--binary-architecture={arch}"));
    cmd.arg("--strip-all");
    cmd.args(["-O", "binary", bin_path]);
    let status = cmd.status().unwrap();
    trace!("objcopy returned {status}");
    if !status.success() {
        error!("objcopy failed with {status}");
        process::exit(1);
    }
}

/// Disssemble an ELF for inspection.
pub fn objdump(env: &Env, prefix: &str, target: &str, elf_path: &str) {
    let mut cmd = Command::new(format!("{prefix}objdump"));
    let dir = dist_dir(env, target);
    cmd.current_dir(dir);
    cmd.arg(elf_path);
    cmd.arg("-d");
    cmd.status().unwrap();
}

/// Figure out the prefix for a toolchain's binutils.
/// We may be able to drop this at some point, since we specify the toolchain
/// components that we need.
fn find_binutils_prefix(arch: &str) -> Option<String> {
    for prefix in [
        "rust-".to_string(),
        format!("{arch}-unknown-elf-"),
        format!("{arch}-linux-gnu-"),
    ] {
        let mut cmd = Command::new(format!("{prefix}objcopy"));
        cmd.arg("--version");
        cmd.stdout(Stdio::null());
        let status = cmd.status().unwrap();
        if status.success() {
            return Some(prefix);
        }
    }
    None
}

/// Find the binutils directory. This only needs to be done once per invocation.
pub fn find_binutils_prefix_or_fail(arch: &str) -> String {
    trace!("find binutils");
    if let Some(ans) = find_binutils_prefix(arch) {
        trace!("found binutils, prefix is '{ans}'");
        return ans;
    }
    error!("No binutils found, try `cargo install cargo-binutils`");
    process::exit(1)
}

/// Get the oreboot root directory.
pub fn project_root() -> PathBuf {
    let d = &env!("CARGO_MANIFEST_DIR");
    Path::new(d).ancestors().nth(1).unwrap().to_path_buf()
}

/// Get the target specific build output directory.
/// Example: `$OREBOOT_ROOT/target/riscv64imac-unknown-none-elf/release
pub fn dist_dir(env: &Env, target: &str) -> PathBuf {
    let target_dir = project_root().join("target").join(target);
    let mode = if env.release { "release" } else { "debug" };
    target_dir.join(mode)
}
