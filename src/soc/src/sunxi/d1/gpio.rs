//! General Purpose Input-Output

use core::{
    marker::PhantomData,
    mem::transmute,
    ptr::{read_volatile, write_volatile},
};
use d1_pac::GPIO;

/// Individual GPIO pin
pub struct Pin<const P: char, const N: u8, MODE = Disabled> {
    _mode: PhantomData<MODE>,
}

#[allow(unused)] // FIXME
impl<const P: char, const N: u8, MODE> Pin<P, N, MODE> {
    /// Disables the pin
    #[inline]
    pub fn into_disabled(self) -> Pin<P, N, Disabled> {
        self.into_mode()
    }
    /// Configures the pin to operate as an input pin
    #[inline]
    pub fn into_input(self) -> Pin<P, N, Input> {
        self.into_mode()
    }
    /// Configures the pin to operate as an output pin
    #[inline]
    pub fn into_output(self) -> Pin<P, N, Output> {
        self.into_mode()
    }
    /// Configures the pin to operate as an external interrupt
    #[inline]
    pub fn into_external_interrupt(self) -> Pin<P, N, Eint> {
        self.into_mode()
    }
}

#[allow(unused)] // FIXME
impl<const P: char, const N: u8, MODE> Pin<P, N, MODE> {
    // todo: should these be designed as type state?
    /// Enable internal pull-up
    #[inline]
    pub fn set_pull_up(&mut self) {
        let mut val = unsafe { read_volatile(Self::PULL_REG) };
        val |= 0b01 << Self::PULL_IDX;
        unsafe { write_volatile(Self::PULL_REG, val) };
    }
    /// Enable internal pull-down
    #[inline]
    pub fn set_pull_down(&mut self) {
        let mut val = unsafe { read_volatile(Self::PULL_REG) };
        val |= 0b10 << Self::PULL_IDX;
        unsafe { write_volatile(Self::PULL_REG, val) };
    }
    /// Disable internal pulls
    #[inline]
    pub fn set_pull_none(&mut self) {
        let mut val = unsafe { read_volatile(Self::PULL_REG) };
        val |= 0b00 << Self::PULL_IDX;
        unsafe { write_volatile(Self::PULL_REG, val) };
    }
}

impl<const P: char, const N: u8, MODE> Pin<P, N, MODE> {
    #[inline(always)]
    const fn new() -> Self {
        Self { _mode: PhantomData }
    }
    #[inline(always)]
    fn into_mode<M: PinMode>(mut self) -> Pin<P, N, M> {
        self.set_mode::<M>();
        Pin::new()
    }
    // this function violates type parameter rule; caller must ensure
    // a correct type parameter change after calling this function.
    #[inline(always)]
    fn set_mode<M: PinMode>(&mut self) {
        let mut new_cfg = unsafe { read_volatile(Self::CFG_REG) };
        new_cfg &= !(0xF << Self::CFG_IDX);
        new_cfg |= (M::VALUE as u32) << Self::CFG_IDX;
        unsafe { write_volatile(Self::CFG_REG, new_cfg) };
    }
    const PORT_OFFSET_BYTES: usize = (P as usize - b'A' as usize) * 0x30;
    const CFG_REG: *mut u32 = unsafe {
        (transmute::<_, usize>(GPIO::ptr()) + Self::PORT_OFFSET_BYTES + (((N >> 3) as usize) << 2))
            as *mut u32
    };
    const CFG_IDX: u8 = (N & 0x7) << 2;
    const DATA_REG: *mut u32 = unsafe {
        (transmute::<_, usize>(GPIO::ptr()) + Self::PORT_OFFSET_BYTES + 0x10) as *mut u32
    };
    const PULL_REG: *mut u32 = unsafe {
        (transmute::<_, usize>(GPIO::ptr())
            + Self::PORT_OFFSET_BYTES
            + 0x24
            + (((N >> 4) as usize) << 2)) as *mut u32
    };
    const PULL_IDX: u8 = (N & 0xF) << 1;
}

macro_rules! define_gpio {
    ($(
        $PortX: ident, $portx: ident, $P: expr, [
            $($PXi: ident:
                ($pxi: ident, $i: expr, $mode: ty),
                ($doc_name: expr, $pinout: expr),
                ($f2: tt, $f3: tt, $f4: tt, $f5: tt, $f6: tt, $f7: tt, $f8: tt),
            )+
        ]
    )+) => {
/// Gpio peripheral
pub struct Gpio {
    $(pub $portx: $portx::$PortX,)+
    _inner: GPIO,
}

impl Gpio {
    #[inline]
    pub fn new(inner: GPIO) -> Self {
        // todo: ensure APB0 clock okay
        Self {
            $($portx: $portx::$PortX {
                $($pxi: Pin::new(),)+
            },)+
            _inner: inner,
        }
    }
}
$(#[allow(unused)] pub mod $portx {
    use super::*;
    $(
    #[doc = concat!("Pin ",$doc_name," at ",$pinout)]
    pub type $PXi<MODE = $mode> = Pin<$P, $i, MODE>;
    )+
    #[doc = concat!("GPIO port ",$P)]
    pub struct $PortX {
        $(pub $pxi: $PXi,)+
    }
    $(impl $PXi {
        define_gpio!(@func $PXi, into_function_2, 2, $f2);
        define_gpio!(@func $PXi, into_function_3, 3, $f3);
        define_gpio!(@func $PXi, into_function_4, 4, $f4);
        define_gpio!(@func $PXi, into_function_5, 5, $f5);
        define_gpio!(@func $PXi, into_function_6, 6, $f6);
        define_gpio!(@func $PXi, into_function_7, 7, $f7);
        define_gpio!(@func $PXi, into_function_8, 8, $f8);
    })+
})+
    };
    (@func $PXi: ident, $into_fn: ident, $fi: expr, x) => {}; // generate nothing
    (@func $PXi: ident, $into_fn: ident, $fi: expr, $doc: expr) => {
        #[doc = concat!("Configures the pin to operate as alternate function ",$fi,": ",$doc)]
        #[inline] pub fn $into_fn(self) -> $PXi<Function<$fi>> {
            self.into_mode()
        }
    };
}

define_gpio! {
    PortB, portb, 'B', [
        PB5: (pb5, 5, Disabled), ("PB5", "K15"), ("LCD0-D9", "I2S2-BCLK", "TWI1-SDA", "PWM0", "LCD0-D21", "UART5-RX", x),
        PB8: (pb8, 8, Disabled), ("PB8", "G15"), ("DMIC-DATA3", "PWM5", "TWI2-SCK", "SPI1-HOLD/DBI-DCX/DBI-WRX", "UART0-TX", "UART1-TX", x),
        PB9: (pb9, 9, Disabled), ("PB9", "G16"), ("DMIC-DATA2", "PWM6", "TWI2-SDA", "SPI1-MISO/DBI-SDI/DBI-TE/DBI-DCX", "UART0-RX", "UART1-RX", x),
    ]
    PortC, portc, 'C', [
        PC1: (pc1, 1, Disabled), ("PC1", "F1"), ("UART2-RX", "TWI2-SDA", x, x, x, x, x),
        PC2: (pc2, 2, Disabled), ("PC2", "G3"), ("SPI0-CLK", "SDC2-CLK", x, x, x, x, x),
        PC3: (pc3, 3, Disabled), ("PC3", "G2"), ("SPI0-CS0", "SDC2-CMD", x, x, x, x, x),
        PC4: (pc4, 4, Disabled), ("PC4", "H3"), ("SPI0-MOSI", "SDC2-D2", "BOOT-SEL0", x, x, x, x),
        PC5: (pc5, 5, Disabled), ("PC5", "F5"), ("SPI0-MISO", "SDC2-D1", "BOOT-SEL1", x, x, x, x),
        PC6: (pc6, 6, Disabled), ("PC6", "G6"), ("SPI0-WP", "SDC2-D0", "UART3-TX", "TWI3-SCK", "DBG-CLK", x, x),
        PC7: (pc7, 7, Disabled), ("PC7", "G5"), ("SPI0-HOLD", "SDC2-D3", "UART3-RX", "TWI3-SDA", "TCON-TRIG", x, x),
    ]
    PortF, portf, 'F', [
        PF0: (pf0, 0, Disabled), ("PF0", "C2"), ("SDC0-D1", "JTAG-MS", "R-JTAG-MS", "I2S2-DOUT1", "I2S2-DIN0", x, x),
        PF1: (pf1, 1, Disabled), ("PF1", "C1"), ("SDC0-D0", "JTAG-DI", "R-JTAG-DI", "I2S2-DOUT0", "I2S2-DIN1", x, x),
        PF3: (pf3, 3, Disabled), ("PF3", "D1"), ("SDC0-CMD", "JTAG-DO", "R-JTAG-DO", "I2S2-BCLK", x, x, x),
        PF5: (pf5, 5, Disabled), ("PF5", "E2"), ("SDC0-D2", "JTAG-CK", "R-JTAG-CK", "I2S2-LRCK", x, x, x),
    ]
}

impl<const P: char, const N: u8> embedded_hal::digital::ErrorType for Pin<P, N, Input> {
    type Error = core::convert::Infallible;
}

impl<const P: char, const N: u8> embedded_hal::digital::InputPin for Pin<P, N, Input> {
    #[inline]
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(unsafe { read_volatile(Self::DATA_REG) } & (1 << N) != 0)
    }
    #[inline]
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(unsafe { read_volatile(Self::DATA_REG) } & (1 << N) == 0)
    }
}

impl<const P: char, const N: u8> embedded_hal::digital::ErrorType for Pin<P, N, Output> {
    type Error = core::convert::Infallible;
}

impl<const P: char, const N: u8> embedded_hal::digital::OutputPin for Pin<P, N, Output> {
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        let mut new_data = unsafe { read_volatile(Self::DATA_REG) };
        new_data &= !(1 << N);
        unsafe { write_volatile(Self::DATA_REG, new_data) };
        Ok(())
    }
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        let mut new_data = unsafe { read_volatile(Self::DATA_REG) };
        new_data |= 1 << N;
        unsafe { write_volatile(Self::DATA_REG, new_data) };
        Ok(())
    }
}

impl<const P: char, const N: u8> embedded_hal::digital::StatefulOutputPin for Pin<P, N, Output> {
    #[inline]
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(unsafe { read_volatile(Self::DATA_REG) } & (1 << N) != 0)
    }
    #[inline]
    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(unsafe { read_volatile(Self::DATA_REG) } & (1 << N) == 0)
    }
}

/// Input mode (type state)
pub struct Input;
/// Output mode (type state)
pub struct Output;
/// Function modes (type state)
///
/// N should be in 2..=8.
pub struct Function<const N: u8>;
/// External interrupt mode (type state)
pub struct Eint;
/// Disabled mode (type state)
pub struct Disabled;

pub trait PinMode {
    const VALUE: u8;
}

impl PinMode for Input {
    const VALUE: u8 = 0;
}

impl PinMode for Output {
    const VALUE: u8 = 1;
}

impl<const N: u8> PinMode for Function<N> {
    const VALUE: u8 = N;
}

impl PinMode for Eint {
    const VALUE: u8 = 14;
}

impl PinMode for Disabled {
    const VALUE: u8 = 15;
}
