use std::{
    path::{Path, PathBuf},
    process::{self, Command, Stdio},
};

use log::{debug, error, info, trace};
use serde::{Deserialize, Serialize};

use crate::Env;

// These utilities help find and run external commands.
// Those are mostly toolchain components and vendor specific tools.

// TODO: change into `"src/platform"`.
pub const PLATFORM_BASE_PATH: &str = "src/mainboard";

/// This gets you the `cargo` command in a specific directory.
/// Use it to build a stage of a mainboard, which is a board's subdirectory.
pub fn get_cargo_cmd_in(env: &Env, plat_dir: &PathBuf, stage: &str, command: &str) -> Command {
    let cargo = std::env::var("CARGO").unwrap_or("cargo".to_string());
    trace!("found cargo at {cargo}");
    let d = platform_dir(plat_dir).join(stage);
    let mut cmd = Command::new(cargo);
    cmd.current_dir(d);
    cmd.arg(command);
    if env.release {
        cmd.arg("--release");
    }
    cmd
}

/// Use this to define a per-platform struct of named binaries for the stages.
/// I.e., something along the lines of:
/// ```rs
/// struct Stages {
///     bt0: Bin,
///     main: Bin,
/// }
/// ```
pub struct Bin {
    pub elf_name: String,
    pub bin_name: String,
    pub target: String,
}

const CARGO_CFG: &str = ".cargo/config.toml";
const CARGO_TOML: &str = "Cargo.toml";

/// For a given platform directory and stage, get a [Bin].
///
/// See <https://doc.rust-lang.org/cargo/reference/config.html>
/// and <https://doc.rust-lang.org/cargo/reference/manifest.html>
pub fn get_bin_for(plat_dir: &PathBuf, stage: &str) -> Bin {
    let f = platform_dir(plat_dir).join(stage).join(CARGO_TOML);
    let m = cargo_toml::Manifest::from_path(&f).unwrap();
    trace!("{f:?}: {m:#?}");
    let elf_name = m.bin.first().unwrap().name.clone().unwrap();
    let bin_name = format!("{elf_name}.bin");
    let f = platform_dir(plat_dir).join(stage).join(CARGO_CFG);
    let settings = config::Config::builder()
        .add_source(config::File::with_name(f.to_str().unwrap()))
        .build()
        .unwrap();
    let target: String = settings.get("build.target").unwrap();
    Bin {
        elf_name,
        bin_name,
        target,
    }
}

const PLATFORM_DTS: &str = "platform.dts";
const PLATFORM_DTB: &str = "platform.dtb";

/// Compile the platform device tree.
pub fn compile_platform_dt(plat_dir: &PathBuf) -> PathBuf {
    trace!("compile platform device tree for {plat_dir:?}");
    let cwd = platform_dir(plat_dir);
    let dtb_path = cwd.join(PLATFORM_DTB);
    let mut command = Command::new("dtc");
    command.current_dir(&cwd);
    command.arg("-o");
    command.arg(&dtb_path);
    command.arg(cwd.join(PLATFORM_DTS));
    let status = command.status().unwrap();
    trace!("dtc returned {status}");
    if !status.success() {
        error!("dtc build failed with {status}");
        process::exit(1);
    }
    dtb_path
}

/// Create a raw binary from an ELF.
pub fn objcopy(env: &Env, bin: &Bin, prefix: &str, arch: &str) {
    trace!("objcopy binary, prefix: '{prefix}'");
    let dir = target_dir(env, &bin.target);
    let mut cmd = Command::new(format!("{prefix}objcopy"));
    cmd.current_dir(dir);
    cmd.arg(&bin.elf_name);
    cmd.arg(format!("--binary-architecture={arch}"));
    cmd.arg("--strip-all");
    cmd.args(["-O", "binary", &bin.bin_name]);
    let status = cmd.status().unwrap();
    trace!("objcopy returned {status}");
    if !status.success() {
        error!("objcopy failed with {status}");
        process::exit(1);
    }
}

/// Disssemble an ELF for inspection.
pub fn objdump(env: &Env, bin: &Bin, prefix: &str) {
    let mut cmd = Command::new(format!("{prefix}objdump"));
    let dir = target_dir(env, &bin.target);
    cmd.current_dir(dir);
    cmd.arg(&bin.elf_name);
    cmd.arg("-d");
    cmd.status().unwrap();
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct FileSummary {
    file: String,
    format: String,
    arch: String,
    address_size: String,
    load_name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct StackSizeEntry {
    functions: Vec<String>,
    size: usize,
}

impl core::fmt::Display for StackSizeEntry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let functions = self.functions.join(", ");
        write!(f, "{:5} bytes; {functions}", self.size)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct StackSize {
    entry: StackSizeEntry,
}

impl core::fmt::Display for StackSize {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "{}", self.entry)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct StackAnalysis {
    file_summary: FileSummary,
    stack_sizes: Vec<StackSize>,
}

/// Analyze an ELF for its stack size using readobj.
///
/// NOTE: If this fails, something is wrong with the command invocation.
/// In that case, check the debug output.
/// TODO: There may be crates for doing this, or programmatic interfaces.
pub fn analyze(env: &Env, bin: &Bin) {
    let mut cmd = Command::new("rust-readobj");
    let dir = target_dir(env, &bin.target);
    cmd.current_dir(dir);
    cmd.arg(&bin.elf_name);
    cmd.arg("--demangle");
    cmd.arg("--stack-sizes");
    cmd.arg("--elf-output-style");
    cmd.arg("JSON");
    debug!("{cmd:?}");
    let output = cmd.output().unwrap();
    let outstr = str::from_utf8(&output.stdout).unwrap();
    debug!("{outstr}");

    let mut parsed: Vec<StackAnalysis> = serde_json::from_str(outstr).unwrap();
    let summary = &parsed[0].file_summary;
    info!("{summary:#?}");

    let entries = &mut parsed[0].stack_sizes;
    entries.sort_by_key(|e| e.entry.size);
    let biggest = entries
        .iter()
        .rev()
        .take(10)
        .map(|e| e.entry.clone())
        .collect::<Vec<StackSizeEntry>>();
    info!("Largest stack size entries:");
    for e in biggest {
        info!("  {e}");
    }
}

/// Figure out the prefix for a toolchain's binutils.
/// We may be able to drop this at some point, since we specify the toolchain
/// components that we need.
fn find_binutils_prefix(arch: &str) -> Option<String> {
    trace!("find binutils prefix");
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
            trace!("found binutils with prefix '{prefix}'");
            return Some(prefix);
        }
    }
    None
}

/// Find the binutils directory. This only needs to be done once per invocation.
pub fn find_binutils_prefix_or_fail(arch: &str) -> String {
    let Some(prefix) = find_binutils_prefix(arch) else {
        error!("No binutils found, try `cargo install cargo-binutils`");
        process::exit(1)
    };
    prefix
}

/// Get the oreboot root directory.
pub fn project_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
}

/// Get the full path of the base directory of all platforms.
pub fn platform_base_dir() -> PathBuf {
    project_root().join(PLATFORM_BASE_PATH)
}

/// Get the full path to a specific platform directory.
/// This is where the final oreboot images for the given platform are found.
pub fn platform_dir(plat_dir: &PathBuf) -> PathBuf {
    platform_base_dir().join(plat_dir)
}

/// Get the target specific build output directory.
/// This is where the ELF and objcopied binaries of the stages are found.
/// Example: `$OREBOOT_ROOT/target/riscv64imac-unknown-none-elf/release/`
pub fn target_dir(env: &Env, target: &str) -> PathBuf {
    let target_dir = project_root().join("target").join(target);
    let mode = if env.release { "release" } else { "debug" };
    target_dir.join(mode)
}

/// Get the target specific build output raw binary file.
pub fn target_bin(env: &Env, bin: &Bin) -> PathBuf {
    target_dir(env, &bin.target).join(&bin.bin_name)
}
