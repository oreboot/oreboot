use crate::halt;

pub fn romstage() -> ! {
    let p = zimage::PAYLOAD;
    p.load();
    p.run();

    halt()
}
