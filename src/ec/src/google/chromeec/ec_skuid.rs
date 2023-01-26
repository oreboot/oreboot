use crate::google::chromeec::ec::{cbi_get_sku_id, CROS_SKU_UNKNOWN};
use spin::rwlock::RwLock;

pub fn get_board_sku() -> u32 {
    static SKU_ID: RwLock<u32> = RwLock::new(CROS_SKU_UNKNOWN);

    if *SKU_ID.read() != CROS_SKU_UNKNOWN {
        return *SKU_ID.read();
    }

    (*SKU_ID.write()) = cbi_get_sku_id(*SKU_ID.read()).unwrap_or(CROS_SKU_UNKNOWN);

    *SKU_ID.read()
}
