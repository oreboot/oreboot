use crate::dist_dir;
use crate::layout_flash;
use crate::project_root;
use crate::{Commands, Env};
use fdt;
use log::{error, info, trace};
use std::fs::File;
use std::io::{self, Seek, SeekFrom};
use std::process::{self, Command, Stdio};

use std::{env, fs, path::Path};
extern crate layoutflash;
use layoutflash::areas::{create_areas, Area};

// It is not clear what CRC starfive had in mind, they wrote their own ...
fn crc32_reverse(x: u32) -> u32 {
    let mut x = x;
    x = ((x & 0x55555555) << 1) | ((x >> 1) & 0x55555555);
    x = ((x & 0x33333333) << 2) | ((x >> 2) & 0x33333333);
    x = ((x & 0x0F0F0F0F) << 4) | ((x >> 4) & 0x0F0F0F0F);
    (x << 24) | ((x & 0xFF00) << 8) | ((x >> 8) & 0xFF00) | (x >> 24)
}

fn crc32(iv: u32, sv: u32, data: Vec<u8>) -> u32 {
    let mut crc = iv;
    for v in data {
        let mut byte = crc32_reverse(v.into());
        for _x in 0..8 {
            crc = if ((crc ^ byte) & 0x80000000u32) != 0 {
                (crc << 1) ^ sv
            } else {
                crc << 1
            };
            byte = byte << 1;
        }
    }

    crc
}

fn crc32_final(iv: u32) -> u32 {
    crc32_reverse(iv ^ !0u32)
}

use crc::Crc;
use std::mem::transmute;

// The use of a packed struct is kind of pointless. Just emit the proper things in the proper order.
// Push them into the output.
// Also, let's get real: there are no big-endian machines left. Assume LE.
//        uint32_t sofs;          /* offset of spl header: 64+256+256 = 0x240 */
//        uint32_t bofs;          /* SBL_BAK_OFFSET: Offset of backup SBL from Flash info start (from input_sbl_normal.cfg) */
//        uint8_t  zro2[636];
//        uint32_t vers;          /* version: shall be 0x01010101
//                                 * (from https://doc-en.rvspace.org/VisionFive2/SWTRM/VisionFive2_SW_TRM/create_spl.html) */
//        uint32_t fsiz;          /* u-boot-spl.bin size in bytes */
//        uint32_t res1;          /* Offset from HDR to SPL_IMAGE, 0x400 (00 04 00 00) currently */
//        uint32_t crcs;          /* CRC32 of u-boot-spl.bin */
//        uint8_t  zro3[364];

fn spl_create_hdr(dat: Vec<u8>) -> Vec<u8> {
    if false {
        // need to find out which one to use, but it's not this one.
        let rc32 = Crc::<u32>::new(&crc::CRC_32_ISCSI);
        let mut digest = rc32.digest();
        digest.update(&dat);
        let crcout = digest.finalize();
    }
    let v = crc32(!0, 0x04c11db7u32, dat.clone());
    let fv = crc32_final(v);
    //println!(fv, crcout, "crcout {crcout:x}, v {v:x}, fv {fv:x}");

    /* version: shall be 0x01010101
     * (from https://doc-en.rvspace.org/VisionFive2/SWTRM/VisionFive2_SW_TRM/create_spl.html) */
    let default_version_id: [u8; 4] = unsafe { transmute(0x01010101u32.to_le()) };
    let default_backup: [u8; 4] = unsafe { transmute(0x20_00_00u32.to_le()) };
    let data_offset: [u8; 4] = unsafe { transmute(0x400u32.to_le()) }; /* Offset from HDR to SPL_IMAGE, 0x400 (00 04 00 00) currently */
    // let CRCFAILED: [u8; 4] = unsafe { transmute(0x5A5A5A5Au32.to_le()) };
    let spl_header_offset: [u8; 4] = unsafe { transmute(0x240u32.to_le()) }; // offset of spl header: 64+256+256 = 0x240

    let mut hdr = vec![];
    hdr.extend_from_slice(&spl_header_offset);
    hdr.extend_from_slice(&default_backup); // Offset of backup SBL from Flash info start (from input_sbl_normal.cfg)
    hdr.resize(hdr.len() + 636, 0);
    hdr.extend_from_slice(&default_version_id);

    let l: [u8; 4] = unsafe { transmute({ dat.len() as u32 }.to_le()) };
    hdr.extend_from_slice(&l); /* u-boot-spl.bin size in bytes */
    hdr.extend_from_slice(&data_offset);
    let l: [u8; 4] = unsafe { transmute(fv.to_le()) };
    hdr.extend_from_slice(&l); /* CRC32 of u-boot-spl.bin */
    hdr.resize(hdr.len() + 364, 0);
    assert!(hdr.len() == 1024, "hdr is {:?} bytes, not 1024", hdr.len());
    hdr.extend(&dat);
    hdr
}

#[test]
fn test_hdr() {
    static HOSTS: &'static [u8] = include_bytes!("testdata/hosts");
    static HOSTS_OUT: &'static [u8] = include_bytes!("testdata/hosts.normal.out");

    let out = spl_create_hdr(HOSTS.to_vec());
    assert_eq!(HOSTS_OUT.len(), out.len());
    for (x, val) in out.iter().enumerate() {
        if *val != HOSTS_OUT[x] {
            println!("at index {x:#x} got {:#x}, want {:#x}", *val, HOSTS_OUT[x]);
        }
    }
    assert_eq!(HOSTS_OUT, &out);
}

// CRC-32-IEEE being the most commonly used one

// const SRAM0_SIZE = 128 * 1024;
const SRAM0_SIZE: u64 = 32 * 1024;

const DEFAULT_TARGET: &str = "riscv64imac-unknown-none-elf";

pub(crate) fn execute_command(args: &crate::Cli, features: Vec<String>) {
    match args.command {
        Commands::Make => {
            info!("building VisionFive2");
            let binutils_prefix = find_binutils_prefix_or_fail();
            // bt0 stage
            xtask_build_jh7100_flash_bt0(&args.env, &features);
            xtask_binary_jh7100_flash_bt0(binutils_prefix, &args.env);
            xtask_add_bt0_header(&args.env);
            // main stage
            xtask_build_jh7100_flash_main(&args.env);
            xtask_binary_jh7100_flash_main(binutils_prefix, &args.env);
            xtask_concat_flash_binaries(&args.env);
            // dtb
            xtask_build_dtb(&args.env);
            xtask_build_dtb_image(&args.env);
        }
        _ => {
            error!("command {:?} not implemented", args.command);
        }
    }
}

fn xtask_build_dtb(env: &Env) {
    trace!("build dtb");
    let cwd = dist_dir(env, DEFAULT_TARGET);
    let mut command = Command::new("dtc");
    command.current_dir(cwd);
    command.arg("-o");
    command.arg("starfive-visionfive2-board.dtb");
    command.arg(board_project_root().join("board.dts"));
    let status = command.status().unwrap();
    trace!("dtc returned {}", status);
    if !status.success() {
        error!("dtc build failed with {}", status);
        process::exit(1);
    }
}

fn xtask_add_bt0_header(env: &Env) {
    let cwd = dist_dir(env, DEFAULT_TARGET);
    trace!(
        "add wacky header to dtb in {}/starfive-visionfive2-bt0.bin",
        cwd.display()
    );
    let bt0_path = cwd.join("starfive-visionfive2-bt0.bin");
    let bt0 = fs::read(bt0_path).expect("bt0");
    let out = spl_create_hdr(bt0);
    let normal_path = cwd.join("starfive-visionfive2-bt0.bin.normal.out");
    fs::write(normal_path, out).expect("writing output");
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
        .arg("starfive-visionfive2-bt0")
        .arg("--binary-architecture=riscv64")
        .arg("--strip-all")
        .args(["-O", "binary", "starfive-visionfive2-bt0.bin"])
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
        .arg("starfive-visionfive2-main")
        .arg("--binary-architecture=riscv64")
        .arg("--strip-all")
        .args(["-O", "binary", "starfive-visionfive2-main.bin"])
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
        .open(dist_dir.join("starfive-visionfive2-bt0.bin"))
        .expect("open bt0 binary file");
    let mut main_file = File::options()
        .read(true)
        .open(dist_dir.join("starfive-visionfive2-main.bin"))
        .expect("open main binary file");

    let output_file_path = dist_dir.join("starfive-visionfive2.bin");
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
    let dtb_path = dist_dir.join("starfive-visionfive2-board.dtb");
    let dtb = fs::read(dtb_path).expect("dtb");

    let output_file_path = dist_dir.join("starfive-visionfive2-board.fdtbin");
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
    project_root().join("src/mainboard/starfive/visionfive2")
}
