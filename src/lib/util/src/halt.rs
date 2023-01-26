use arch::x86_64::hlt;

pub fn halt() {
    loop {
        hlt::hlt();
    }
}
