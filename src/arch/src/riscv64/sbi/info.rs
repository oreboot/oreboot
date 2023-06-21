use super::csr;
use log::println;

pub fn print_info(platform: &str, version: &str) {
    println!("RustSBI version {}", rustsbi::VERSION);
    println!("{}", rustsbi::LOGO);
    println!("Platform Name: {}", platform);
    println!("Implementation: oreboot version {}", version);
    csr::print_info();
}
