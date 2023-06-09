use super::hart_csr_utils;
use log::println;

pub fn print_info(platform: &str, version: &str) {
    println!("RustSBI version {}", rustsbi::VERSION);
    println!("{}", rustsbi::LOGO);
    println!("Platform Name: {}", platform);
    println!("Implementation: oreboot version {}", version);
    hart_csr_utils::print_hart_csrs();
    hart_csr_utils::print_hart_pmp();
}
