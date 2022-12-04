#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use x86::{inb, inl, inw, outb, outl, outw};
