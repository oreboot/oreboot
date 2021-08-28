use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{exit, Command};

pub enum FspArchitecture {
    X64,
    Ia32,
}

pub fn build_qemu_fsp(oreboot_root: &str, arch: FspArchitecture) -> std::io::Result<()> {
    let arch_name = match arch {
        FspArchitecture::X64 => "x64",
        FspArchitecture::Ia32 => "ia32",
    };

    let root_path = PathBuf::from(oreboot_root);

    // Build FSP binaries and generate header files.
    let status = Command::new("python3")
        .args(&["BuildFsp.py", "clean"])
        .current_dir(root_path.join("3rdparty/fspsdk"))
        .status()
        .expect("failed to clean FSP build");
    if !status.success() {
        println!("Failed to build FSP");
        exit(1);
    }

    let status = Command::new("python3")
        .args(&["BuildFsp.py", "build", "-p", "qemu", "-a", &arch_name])
        .current_dir(root_path.join("3rdparty/fspsdk"))
        .status()
        .expect("failed to build FSP");
    if !status.success() {
        println!("Failed to build FSP");
        exit(1);
    }

    // Copy FSP binaries to output path.
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let edk2_fv_dir = root_path.join("3rdparty/fspsdk/Build/QemuFspPkg/DEBUG_GCC5/FV");
    fs::copy(edk2_fv_dir.join("QEMUFSP.fd"), out_dir.join("QEMUFSP.fd"))?;

    Ok(())
}
