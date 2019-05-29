use crate::halt;

pub fn romstage() -> ! {
    console_init();
    halt()
}

fn console_init() {}
