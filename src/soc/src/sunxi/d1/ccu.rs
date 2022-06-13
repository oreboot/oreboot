use super::time::Hz;
use d1_pac::ccu::RegisterBlock as CcuRb;

#[derive(Debug)]
pub struct Clocks {
    pub psi: Hz,
    pub apb1: Hz,
}

pub trait Gating {
    fn gating_pass(ccu: &CcuRb);
    fn gating_mask(ccu: &CcuRb);
}

pub trait Reset {
    fn deassert_reset(ccu: &CcuRb);
    fn assert_reset(ccu: &CcuRb);
}

macro_rules! define_gating_reset {
    ($($PERI: ident: ($bgr: ident, $gating: ident, $rst: ident);)+) => {
$(impl Gating for d1_pac::$PERI {
    #[inline(always)] fn gating_pass(ccu: &CcuRb) {
        ccu.$bgr.modify(|_, w| w.$gating().pass())
    }
    #[inline(always)] fn gating_mask(ccu: &CcuRb) {
        ccu.$bgr.modify(|_, w| w.$gating().mask())
    }
}
impl Reset for d1_pac::$PERI {
    #[inline(always)] fn deassert_reset(ccu: &CcuRb) {
        ccu.$bgr.modify(|_, w| w.$rst().deassert())
    }
    #[inline(always)] fn assert_reset(ccu: &CcuRb) {
        ccu.$bgr.modify(|_, w| w.$rst().assert())
    }
})+
    }
}

define_gating_reset! {
    UART0: (uart_bgr, uart0_gating, uart0_rst);
    SPI0: (spi_bgr, spi0_gating, spi0_rst);
}
