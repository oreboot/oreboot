mod gdb_detect;
mod target;

mod sunxi;

use std::{
    env,
    path::{Path, PathBuf},
    process,
    str::FromStr,
};

use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use log::error;

#[derive(Parser)]
#[clap(name = "xtask")]
#[clap(about = "Program that help you build and debug Oreboot project", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
    #[clap(flatten)]
    env: Env,
    #[clap(flatten)]
    verbose: Verbosity,
}

#[derive(Subcommand)]
enum Commands {
    /// Make this project
    Make,
    /// Build flash and burn into target
    Flash,
    /// View assembly code
    Asm,
    /// Debug code using gdb
    Gdb,
}

enum Memory {
    /// Operate on NAND flash
    Nand,
    /// Operate on NOR flash
    Nor,
}

impl FromStr for Memory {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "nand" => Ok(Self::Nand),
            "nor" => Ok(Self::Nor),
            others => Err(format!("unknown memory type {}", others)),
        }
    }
}

#[derive(Args)]
struct Env {
    #[clap(
        long = "release",
        global = true,
        help = "Build in release mode",
        long_help = None,
    )]
    release: bool,
    #[clap(long, global = true, help = "Mainboard to build")]
    mainboard: Option<String>,
    #[clap(long, global = true, help = "Target memory description")]
    memory: Option<Memory>,
    #[clap(long, global = true, help = "Board variant")]
    variant: Option<String>,
}

fn main() {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();
    let cur_path = env::current_dir().unwrap();
    let target = if let Some(target) = target::parse_target(
        &cur_path,
        args.env.mainboard.as_deref(),
        args.env.variant.as_deref(),
    ) {
        target
    } else {
        error!(
            "can't decide target for task
    Change directory to mainboard and run again, or
    use `--mainboard <VENDOR/BOARD>` and `--variant <VARIANT>` when necessary."
        );
        process::exit(1)
    };
    target.execute_command(&args);
}

fn dist_dir(env: &Env, target: &str) -> PathBuf {
    let mut path_buf = project_root().join("target").join(target);
    path_buf = match env.release {
        false => path_buf.join("debug"),
        true => path_buf.join("release"),
    };
    path_buf
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
