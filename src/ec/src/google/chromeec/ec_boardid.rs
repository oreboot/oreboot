use crate::google::chromeec::ec::{google_chromeec_get_board_version, Error};

pub const BOARD_ID_UNKNOWN: u32 = !0; // unsigned equivalent to -1
pub const BOARD_ID_INIT: u32 = !1; // unsigned equivalent to -2

pub fn board_id() -> Result<u32, Error> {
    let mut id = BOARD_ID_INIT;
    if id == BOARD_ID_INIT {
        if google_chromeec_get_board_version(id)? != 0 {
            id = BOARD_ID_UNKNOWN;
        }
    }
    Ok(id)
}
