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
        println!("Failed to clean FSP");
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

    // Rebase the FSP binary in place.
    // See QemuFspPkg/QemuFspPkg.fdf, but ultimately FLASH_BASE must be 0xFFF80000
    //
    //   - FSP-S starts at 0xFFF80000 (gQemuFspPkgTokenSpaceGuid.PcdFlashFvFspsBase = 0x00000000)
    //   - FSP-M starts at 0xFFF95000 (gQemuFspPkgTokenSpaceGuid.PcdFlashFvFspmBase = 0x00015000)
    //   - FSP-T starts at 0xFFFB7000 (gQemuFspPkgTokenSpaceGuid.PcdFlashFvFsptBase = 0x00037000)
    //
    // TODO: We may want to parse the file to derive these, or at the very least verify they match
    // up. Also, there's a slight difference in the binary output between changing FLASH_BASE and
    // building vs. building and rebasing. It appears not to be impactful, and we're assuming that
    // using FSP tools unpatched is the safer of the two.
    let status = Command::new("python3")
        .args(&[
            "IntelFsp2Pkg/Tools/SplitFspBin.py",
            "rebase",
            "-c",
            "s",
            "m",
            "t",
            "-b",
            "0xFFF80000",
            "0xFFF95000",
            "0xFFFB7000",
            "-f",
            "Build/QemuFspPkg/DEBUG_GCC5/FV/QEMUFSP.fd",
            "-o",
            "Build/QemuFspPkg/DEBUG_GCC5/FV",
            "-n",
            "QEMUFSP.fd",
        ])
        .current_dir(root_path.join("3rdparty/fspsdk"))
        .status()
        .expect("failed to rebase FSP");
    if !status.success() {
        println!("Failed to rebase FSP");
        exit(1)
    }

    // The build command above creates each FSP component (S, M, and T) and combines them
    // into one binary. The rebase then operates on that combined binary, so we split here
    // to get back the components. This way, the whole and the parts alongside it will have
    // the same base address.
    let status = Command::new("python3")
        .args(&[
            "IntelFsp2Pkg/Tools/SplitFspBin.py",
            "split",
            "-f",
            "Build/QemuFspPkg/DEBUG_GCC5/FV/QEMUFSP.fd",
            "-o",
            "Build/QemuFspPkg/DEBUG_GCC5/FV",
            "-n",
            "FSP.Fv",
        ])
        .current_dir(root_path.join("3rdparty/fspsdk"))
        .status()
        .expect("failed to split rebased FSP");
    if !status.success() {
        println!("Failed to split rebased FSP");
        exit(1)
    }

    // Copy FSP binaries to output path.
    let edk2_fv_dir = root_path.join("3rdparty/fspsdk/Build/QemuFspPkg/DEBUG_GCC5/FV");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::copy(edk2_fv_dir.join("QEMUFSP.fd"), out_dir.join("QEMUFSP.fd"))?;

    Ok(())
}
