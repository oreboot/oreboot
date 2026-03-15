#![no_std]
#![no_main]

#[cfg_attr(not(test), panic_handler)]
fn panic(info: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
