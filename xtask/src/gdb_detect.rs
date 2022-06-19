use log::{error, info};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn detect_gdb_path() -> String {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();
    println!("you need T-Head toolchain installed to debug this program.");
    loop {
        input.clear();
        print!(
            "Please input T-Head GDB toolchain path:
> "
        );
        stdout.flush().unwrap();
        stdin.read_line(&mut input).expect("read line");
        let mut command = Command::new(&input.trim());
        command.arg("--version");
        let output = match command.output() {
            Ok(output) => output,
            Err(e) => {
                error!("io error occurred {}", e);
                continue;
            }
        };
        let info = String::from_utf8_lossy(&output.stdout);
        if !info.starts_with("GNU gdb") {
            error!("not a GNU gdb program");
            continue;
        }
        if info.find("Xuantie-900 elf").is_some() {
            info!("chosen Xuantie-900 ELF GDB program");
            break;
        } else {
            info!("chosen generic GDB program");
            break;
        }
    }
    input.trim().to_string()
}

pub(crate) fn detect_gdb_server(gdb_path: &str) -> String {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();
    println!("an address is needed to connect to GDB server.");
    loop {
        input.clear();
        print!(
            "Please input GDB remote address ([host]:port):
> "
        );
        stdout.flush().unwrap();
        stdin.read_line(&mut input).expect("read line");
        println!("trying gdb connect...");
        let mut command = Command::new(gdb_path);
        command.args(&["--eval-command", "set tcp connect-timeout 5"]);
        command.args(&["--eval-command", &format!("target remote {}", input.trim())]);
        command.arg("--batch-silent");
        let status = match command.status() {
            Ok(status) => status,
            Err(e) => {
                error!("io error occurred when trying to connect: {}", e);
                continue;
            }
        };
        if status.success() {
            break;
        } else {
            error!("GDB connection error: {}", status);
        }
    }
    input.trim().to_string()
}

pub fn save_gdb_path_to_file(gdb_path: &str) {
    fs::create_dir_all(project_root().join("target").join("xtask")).expect("create folder");
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(
            project_root()
                .join("target")
                .join("xtask")
                .join("gdb-path.txt"),
        )
        .expect("create and open file");
    file.write(gdb_path.as_bytes()).expect("write file");
}

pub fn load_gdb_path_from_file() -> io::Result<String> {
    fs::read_to_string(
        project_root()
            .join("target")
            .join("xtask")
            .join("gdb-path.txt"),
    )
}

pub fn save_gdb_server_to_file(gdb_server: &str) {
    fs::create_dir_all(project_root().join("target").join("xtask")).expect("create folder");
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(
            project_root()
                .join("target")
                .join("xtask")
                .join("gdb-server.txt"),
        )
        .expect("create and open file");
    file.write(gdb_server.as_bytes()).expect("write file");
}

pub fn load_gdb_server_from_file() -> io::Result<String> {
    fs::read_to_string(
        project_root()
            .join("target")
            .join("xtask")
            .join("gdb-server.txt"),
    )
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
