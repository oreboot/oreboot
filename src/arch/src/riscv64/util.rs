use riscv::asm::{ebreak, ecall, fence_i};
use riscv::register::{mtvec, time};

use log::println;

pub fn rdtime() -> u64 {
    time::read64()
}

pub fn delay(t: u64) {
    let later = rdtime() + t;
    while rdtime() < later {}
}

#[rustc_align(4)]
fn trap(a: usize, b: usize) {
    println!("stop it {a:08x} {b:08x}");
}

pub fn test_mtvec() {
    fence_i();
    let a = trap as *const () as usize;
    unsafe {
        mtvec::write(a, mtvec::TrapMode::Direct);
        ecall();
        ebreak();
    };
}
