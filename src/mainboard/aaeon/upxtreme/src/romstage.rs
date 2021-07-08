use crate::halt;
use payloads::external::zimage::PAYLOAD;

pub fn romstage() -> ! {
    let p = PAYLOAD;
    p.load();
    p.run();

    halt()
}
