#![no_std]
#![deny(warnings)]

pub fn init() {
    arch::init()
}

// There is no ibex-specific way of halting yet.
pub use arch::halt;
