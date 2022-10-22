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
        PB10: (pb10, 10, Disabled), ("PB10", "F17"), ("LCD0-D9", "I2S2-BCLK", "TWI1-SDA", "PWM0", "LCD0-D21", "UART5-RX", x),
        PB11: (pb11, 11, Disabled), ("PB11", "F15"), ("LCD0-D9", "I2S2-BCLK", "TWI1-SDA", "PWM0", "LCD0-D21", "UART5-RX", x),
        PB12: (pb12, 12, Disabled), ("PB12", "F16"), ("DMIC-CLK", "PWM0", "SPDIF-IN", "SPI1-CS", "DBI-CSX", "CLK-FANOUT2", "IR-RX"),
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
        PF2: (pf2, 2, Disabled), ("PF2", "D2"), ("SDC0-CLK", "UART0-TX", "TWI0-SCK", "LEDC-DO", "SPDIF-IN", x, x),
        PF3: (pf3, 3, Disabled), ("PF3", "D1"), ("SDC0-CMD", "JTAG-DO", "R-JTAG-DO", "I2S2-BCLK", x, x, x),
        PF4: (pf4, 4, Disabled), ("PF4", "E3"), ("SDC0-D3", "UART0-RX", "TWI0-SDA", "PWM6", "IR-TX", x, x),
        PF5: (pf5, 5, Disabled), ("PF5", "E2"), ("SDC0-D2", "JTAG-CK", "R-JTAG-CK", "I2S2-LRCK", x, x, x),
        PF6: (pf6, 6, Disabled), ("PF6", "D3"), ("SPDIF-OUT", "IR-RX", "I2S2-MCLK", "PWM5", x, x, x),
    ]
    PortG, portg, 'G', [
        PG0: (pg0, 0, Disabled), ("PG0", "B2"), ("SDC1-CLK", "UART3-TX", "RGMII-RXCTRL", "RMII-CRS-DV", "PWM7", x, x),
        PG1: (pg1, 1, Disabled), ("PG1", "B3"), ("SDC1-CMD", "UART3-RX", "RGMII-RXD0", "RMII-RXD0", "PWM6", x, x),
        PG2: (pg2, 2, Disabled), ("PG2", "A3"), ("SDC1-D0", "UART3-RTS", "RGMII-RXD1", "RMII-RXD1", "UART4-TX", x, x),
        PG3: (pg3, 3, Disabled), ("PG3", "C3"), ("SDC1-D1", "UART3-CTS", "RGMII-TXCK", "RMII-TXCK", "UART4-RX", x, x),
        PG4: (pg4, 4, Disabled), ("PG4", "A4"), ("SDC1-D2", "UART5-TX", "RGMII-TXD0", "RMII-TXD0", "PWM5", x, x),
        PG5: (pg5, 5, Disabled), ("PG5", "B4"), ("SDC1-D3", "UART5-RX", "RGMII-TXD1", "RMII-TXD1", "PWM4", x, x),
        PG6: (pg6, 6, Disabled), ("PG6", "B5"), ("UART1-TX", "TWI2-SCK", "RGMII-TXD2", "PWM1", x, x, x),
        PG7: (pg7, 7, Disabled), ("PG7", "C6"), ("UART1-RX", "TWI2-SDA", "RGMII-TXD3", "SPDIF-IN", x, x, x),
        PG8: (pg8, 8, Disabled), ("PG8", "A6"), ("UART1-RTS", "TWI1-SCK", "RGMII-RXD2", "UART3-TX", x, x, x),
        PG9: (pg9, 9, Disabled), ("PG9", "B6"), ("UART1-CTS", "TWI1-SDA", "RGMII-RXD3", "UART3-RX", x, x, x),
        PG10: (pg10, 10, Disabled), ("PG10", "C6"), ("PWM3", "TWI3-SCK", "RGMII-RXCK", "CLK-FANOUT0", "IR-RX", x, x),
        PG11: (pg11, 11, Disabled), ("PG11", "D4"), ("I2S1-MCLK", "TWI3-SDA", "EPHY-25M", "CLK-FANOUT1", "TCON-TRIG", x, x),
        PG12: (pg12, 12, Disabled), ("PG12", "D5"), ("I2S1-LRCK", "TWI0-SCK", "RGMII-TXCTRL", "RMII-TXEN", "CLK-FANOUT2", "PWM0", "UART1-TX"),
        PG13: (pg13, 13, Disabled), ("PG13", "D6"), ("I2S1-BCLK", "TWI0-SDA", "RGMII-CLKIN", "RMII-RXER", "PWM2", "LEDC-DO", "UART1-RX"),
        PG14: (pg14, 14, Disabled), ("PG14", "E6"), ("I2S1-DIN0", "TWI2-SCK", "MDC", "I2S1-DOUT1", "SPI0-WP", "UART1-RTS", x),
        PG15: (pg15, 15, Disabled), ("PG15", "F6"), ("I2S1-DOUT0", "TWI2-SDA", "MDIO", "I2S1-DIN1", "SPI0-HOLD", "UART1-CTS", x),
        PG16: (pg16, 16, Disabled), ("PG16", "F7"), ("IR-RX", "TCON-TRIG", "PWM5", "CLK-FANOUT2", "SPDIF-IN", "LEDC-DO", x),
        PG17: (pg17, 17, Disabled), ("PG17", "E7"), ("TWI3-SCK", "PWM7", "CLK-FANOUT0", "IR-TX", "UART0-TX", x, x),
        PG18: (pg18, 18, Disabled), ("PG18", "D7"), ("TWI3-SDA", "PWM6", "CLK-FANOUT1", "SPDIF-OUT", "UART0-RX", x, x),
    ]
}

impl<const P: char, const N: u8> embedded_hal::digital::ErrorType for Pin<P, N, Input> {
    type Error = core::convert::Infallible;
}

impl<const P: char, const N: u8> embedded_hal::digital::blocking::InputPin for Pin<P, N, Input> {
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

impl<const P: char, const N: u8> embedded_hal::digital::blocking::OutputPin for Pin<P, N, Output> {
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

impl<const P: char, const N: u8> embedded_hal::digital::blocking::StatefulOutputPin
    for Pin<P, N, Output>
{
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
