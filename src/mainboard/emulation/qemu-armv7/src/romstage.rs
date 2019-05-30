use crate::halt;

//mod console;

pub fn romstage() -> ! {
    //console_init();
    halt()
}
