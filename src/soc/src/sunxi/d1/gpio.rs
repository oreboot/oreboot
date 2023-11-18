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
        PB0:  (pb0,   0, Disabled), ("PB0",  "J16"), ("PWM3", "IR-TX", "TWI2-SCK", "SPI1-WP", "UART0-TX", "UART2-TX", "OWA-OUT"),
        PB1:  (pb1,   1, Disabled), ("PB1",  "J17"), ("PWM4", "I2S2-DOUT3", "TWI2-SDA", "I2S2-DIN3", "UART0-RX", "UART2-RX", "IR-RX"),
        PB2:  (pb2,   2, Disabled), ("PB2",  "M16"), ("LCD0-D0", "I2S2-DOUT2", "TWI0-SDA", "I2S2-DIN2", "LCD0-D18", "UART4-TX", x),
        PB3:  (pb3,   3, Disabled), ("PB3",  "M15"), ("LCD0-D1", "I2S2-DOUT1", "TWI0-SCK", "I2S2-DIN0", "LCD0-D19", "UART4-RX", x),
        PB4:  (pb4,   4, Disabled), ("PB4",  "K16"), ("LCD0-D8", "I2S2-DOUT0", "TWI1-SCK", "I2S2-DIN1", "LCD0-D20", "UART5-TX", x),
        PB5:  (pb5,   5, Disabled), ("PB5",  "K15"), ("LCD0-D9", "I2S2-BCLK", "TWI1-SDA", "PWM0", "LCD0-D21", "UART5-RX", x),
        PB6:  (pb6,   6, Disabled), ("PB6",  "K17"), ("LCD0-D16", "I2S2-LRCK", "TWI3-SCK", "PWM1", "LCD0-D22", "UART3-TX", "CPUBIST0"),
        PB7:  (pb7,   7, Disabled), ("PB7",  "J15"), ("LCD0-D17", "I2S2-MCLK", "TWI3-SDA", "IR-RX", "LCD0-D23", "UART3-RX", "CPUBIST1"),
        PB8:  (pb8,   8, Disabled), ("PB8",  "G15"), ("DMIC-DATA3", "PWM5", "TWI2-SCK", "SPI1-HOLD/DBI-DCX/DBI-WRX", "UART0-TX", "UART1-TX", x),
        PB9:  (pb9,   9, Disabled), ("PB9",  "G16"), ("DMIC-DATA2", "PWM6", "TWI2-SDA", "SPI1-MISO/DBI-SDI/DBI-TE/DBI-DCX", "UART0-RX", "UART1-RX", x),
        PB10: (pb10, 10, Disabled), ("PB10", "F17"), ("DMIC-DATA1", "PWM7", "TWI0-SCK", "SPI1-MOSI", "CLK-FANOUT0", "UART1-RTS", x),
        PB11: (pb11, 11, Disabled), ("PB11", "F15"), ("DMIC-DATA0", "PWM2", "TWI0-SDA", "SPI1-CLK", "CLK-FANOUT1", "UART1-CTS", x),
        PB12: (pb12, 12, Disabled), ("PB12", "F16"), ("DMIC-CLK", "PWM0", "OWA-IN", "SPI1-CS", "CLK-FANOUT2", "IR-RX", x),
    ]
    PortC, portc, 'C', [
        PC0: (pc0, 0, Disabled), ("PC0", "F2"), ("UART2-TX", "TWI2-SCK", "LEDC-DO", x, x, x, x),
        PC1: (pc1, 1, Disabled), ("PC1", "F1"), ("UART2-RX", "TWI2-SDA", x, x, x, x, x),
        PC2: (pc2, 2, Disabled), ("PC2", "G3"), ("SPI0-CLK", "SDC2-CLK", x, x, x, x, x),
        PC3: (pc3, 3, Disabled), ("PC3", "G2"), ("SPI0-CS0", "SDC2-CMD", x, x, x, x, x),
        PC4: (pc4, 4, Disabled), ("PC4", "H3"), ("SPI0-MOSI", "SDC2-D2", "BOOT-SEL0", x, x, x, x),
        PC5: (pc5, 5, Disabled), ("PC5", "F5"), ("SPI0-MISO", "SDC2-D1", "BOOT-SEL1", x, x, x, x),
        PC6: (pc6, 6, Disabled), ("PC6", "G6"), ("SPI0-WP", "SDC2-D0", "UART3-TX", "TWI3-SCK", "DBG-CLK", x, x),
        PC7: (pc7, 7, Disabled), ("PC7", "G5"), ("SPI0-HOLD", "SDC2-D3", "UART3-RX", "TWI3-SDA", "TCON-TRIG", x, x),
    ]
    PortD, portd, 'D', [
        PD0:  (pd0,   0, Disabled), ("PD0",  "W19"), ("LCD0-D2", "LVDS0-V0P", "DSI-D0P", "TWI0-SCK", x, x, x),
        PD1:  (pd1,   1, Disabled), ("PD1",  "V20"), ("LCD0-D3", "LVDS0-V0N", "DSI-D0N", "UART2-TX", x, x, x),
        PD2:  (pd2,   2, Disabled), ("PD2",  "V19"), ("LCD0-D4", "LVDS0-V1P", "DSI-D1P", "UART2-RX", x, x, x),
        PD3:  (pd3,   3, Disabled), ("PD3",  "U20"), ("LCD0-D5", "LVDS0-V1N", "DSI-D1N", "UART2-RTS", x, x, x),
        PD4:  (pd4,   4, Disabled), ("PD4",  "U19"), ("LCD0-D6", "LVDS0-V2P", "DSI-CKP", "UART2-CTS", x, x, x),
        PD5:  (pd5,   5, Disabled), ("PD5",  "U18"), ("LCD0-D7", "LVDS0-V2N", "DSI-CKN", "UART5-TX", x, x, x),
        PD6:  (pd6,   6, Disabled), ("PD6",  "T19"), ("LCD0-D10", "LVDS0-CKP", "DSI-D2P", "UART5-RX", x, x, x),
        PD7:  (pd7,   7, Disabled), ("PD7",  "T18"), ("LCD0-D11", "LVDS0-CKN", "DSI-D2N", "UART4-TX", x, x, x),
        PD8:  (pd8,   8, Disabled), ("PD8",  "R20"), ("LCD0-D12", "LVDS0-V3P", "DSI-D3P", "UART4-RX", x, x, x),
        PD9:  (pd9,   9, Disabled), ("PD9",  "R19"), ("LCD0-D13", "LVDS0-V3N", "DSI-D3N", "PWM6", x, x, x),
        PD10: (pd10, 10, Disabled), ("PD10", "T17"), ("LCD0-D14", "LVDS1-V0P", "SPI1-CS/DBI-CSX", "UART3-TX", x, x, x),
        PD11: (pd11, 11, Disabled), ("PD11", "R17"), ("LCD0-D15", "LVDS1-V0N", "SPI1-CLK/DBI-SCLK", "UART3-RX", x, x, x),
        PD12: (pd12, 12, Disabled), ("PD12", "P19"), ("LCD0-D18", "LVDS1-V1P", "SPI1-MOSI/DBI-SDO", "TWI0-SDA", x, x, x),
        PD13: (pd13, 13, Disabled), ("PD13", "P18"), ("LCD0-D19", "LVDS1-V1N", "SPI1-MISO/DBI-SDI/DBI-TE/DBI-DCX", "UART3-RTS", x, x, x),
        PD14: (pd14, 14, Disabled), ("PD14", "N17"), ("LCD0-D20", "LVDS1-V2P", "SPI1-HOLD/DBI-DCX/DBI-WRX", "UART3-CTS", x, x, x),
        PD15: (pd15, 15, Disabled), ("PD15", "N16"), ("LCD0-D21", "LVDS1-V2N", "SPI1-WP/DBI-TE", "IR-RX", x, x, x),
        PD16: (pd16, 16, Disabled), ("PD16", "N20"), ("LCD0-D22", "LVDS1-CKP", "DMIC-DATA3", "PWM0", x, x, x),
        PD17: (pd17, 17, Disabled), ("PD17", "N19"), ("LCD0-D23", "LVDS1-CKN", "DMIC-DATA2", "PWM1", x, x, x),
        PD18: (pd18, 18, Disabled), ("PD18", "M19"), ("LCD0-CLK", "LVDS1-V3P", "DMIC-DATA1", "PWM2", x, x, x),
        PD19: (pd19, 19, Disabled), ("PD19", "M18"), ("LCD0-DE", "LVDS1-V3N", "DMIC-DATA0", "PWM3", x, x, x),
        PD20: (pd20, 20, Disabled), ("PD20", "W18"), ("LCD0-HSYNC", "TWI2-SCK", "DMIC-CLK", "PWM4", x, x, x),
        PD21: (pd21, 21, Disabled), ("PD21", "V18"), ("LCD0-VSYNC", "TWI2-SDA", "UART1-TX", "PWM5", x, x, x),
        PD22: (pd22, 22, Disabled), ("PD22", "Y18"), ("OWA-OUT", "IR-RX", "UART1-RX", "PWM7", x, x, x),
    ]
    PortE, porte, 'E', [
        PE2:  (pe2,   2, Disabled), ("PE2",  "???"), ("DMIC-DATA3", "PWM5", "TWI2-SCK", "SPI1-HOLD/DBI-DCX/DBI-WRX", "UART0-TX", "UART1-TX", x),
        PE3:  (pe3,   3, Disabled), ("PE3",  "???"), ("DMIC-DATA2", "PWM6", "TWI2-SDA", "SPI1-MISO/DBI-SDI/DBI-TE/DBI-DCX", "UART0-RX", "UART1-RX", x),
    ]
    PortF, portf, 'F', [
        PF0: (pf0, 0, Disabled), ("PF0", "C2"), ("SDC0-D1", "JTAG-MS", "R-JTAG-MS", "I2S2-DOUT1", "I2S2-DIN0", x, x),
        PF1: (pf1, 1, Disabled), ("PF1", "C1"), ("SDC0-D0", "JTAG-DI", "R-JTAG-DI", "I2S2-DOUT0", "I2S2-DIN1", x, x),
        PF2: (pf2, 2, Disabled), ("PF2", "D2"), ("SDC0-CLK", "UART0-TX", "TWI0-SCK", "LEDC-DO", "OWA-IN", x, x),
        PF3: (pf3, 3, Disabled), ("PF3", "D1"), ("SDC0-CMD", "JTAG-DO", "R-JTAG-DO", "I2S2-BCLK", x, x, x),
        PF4: (pf4, 4, Disabled), ("PF4", "E3"), ("SDC0-D3", "UART0-RX", "TWI0-SDA", "PWM6", "IR-TX", x, x),
        PF5: (pf5, 5, Disabled), ("PF5", "E2"), ("SDC0-D2", "JTAG-CK", "R-JTAG-CK", "I2S2-LRCK", x, x, x),
        PF6: (pf6, 6, Disabled), ("PF6", "D3"), (x, "OWA-OUT", "IR-RX", "I2S2-MCLK", "PWM5", x, x),
    ]
    PortG, portg, 'G', [
        PG0:  (pg0,   0, Disabled), ("PG0",  "B2"), ("SDC1-CLK", "UART3-TX", "RGMII-RXCTRL", "PWM7", x, x, x),
        PG1:  (pg1,   1, Disabled), ("PG1",  "B3"), ("SDC1-CMD", "UART3-RX", "RGMII-RXD0", "PWM6", x, x, x),
        PG2:  (pg2,   2, Disabled), ("PG2",  "A3"), ("SDC1-D0", "UART3-RTS", "RGMII-RXD1", "UART4-TX", x, x, x),
        PG3:  (pg3,   3, Disabled), ("PG3",  "C3"), ("SDC1-D1", "UART3-CTS", "RGMII-TXCK", "UART4-RX", x, x, x),
        PG4:  (pg4,   4, Disabled), ("PG4",  "A4"), ("SDC1-D2", "UART5-TX", "RGMII-TXD0", "PWM5", x, x, x),
        PG5:  (pg5,   5, Disabled), ("PG5",  "B4"), ("SDC1-D3", "UART5-RX", "RGMII-TXD1", "PWM4", x, x, x),
        PG6:  (pg6,   6, Disabled), ("PG6",  "B5"), ("UART1-TX", "TWI2-SCK", "RGMII-TXD2", "PWM1", x, x, x),
        PG7:  (pg7,   7, Disabled), ("PG7",  "C6"), ("UART1-RX", "TWI2-SDA", "RGMII-TXD3", "OWA-IN", x, x, x),
        PG8:  (pg8,   8, Disabled), ("PG8",  "A6"), ("UART1-RTS", "TWI1-SCK", "RGMII-RXD2", "UART3-TX", x, x, x),
        PG9:  (pg9,   9, Disabled), ("PG9",  "B6"), ("UART1-CTS", "TWI1-SDA", "RGMII-RXD3", "UART3-RX", x, x, x),
        PG10: (pg10, 10, Disabled), ("PG10", "C6"), ("PWM3", "TWI3-SCK", "RGMII-RXCK", "CLK-FANOUT0", "IR-RX", x, x),
        PG11: (pg11, 11, Disabled), ("PG11", "D4"), ("I2S1-MCLK", "TWI3-SDA", "EPHY-25M", "CLK-FANOUT1", "TCON-TRIG", x, x),
        PG12: (pg12, 12, Disabled), ("PG12", "D5"), ("I2S1-LRCK", "TWI0-SCK", "RGMII-TXCTRL", "CLK-FANOUT2", "PWM0", "UART1-TX", x),
        PG13: (pg13, 13, Disabled), ("PG13", "D6"), ("I2S1-BCLK", "TWI0-SDA", "RGMII-CLKIN", "PWM2", "LEDC-DO", "UART1-RX", x),
        PG14: (pg14, 14, Disabled), ("PG14", "E6"), ("I2S1-DIN0", "TWI2-SCK", "MDC", "I2S1-DOUT1", "SPI0-WP", "UART1-RTS", x),
        PG15: (pg15, 15, Disabled), ("PG15", "F6"), ("I2S1-DOUT0", "TWI2-SDA", "MDIO", "I2S1-DIN1", "SPI0-HOLD", "UART1-CTS", x),
        PG16: (pg16, 16, Disabled), ("PG16", "F7"), ("IR-RX", "TCON-TRIG", "PWM5", "CLK-FANOUT2", "OWA-IN", "LEDC-DO", x),
        PG17: (pg17, 17, Disabled), ("PG17", "E7"), ("UART2-TX", "TWI3-SCK", "PWM7", "CLK-FANOUT0", "IR-TX", "UART0-TX",  x),
        PG18: (pg18, 18, Disabled), ("PG18", "D7"), ("UART2-RX", "TWI3-SDA", "PWM6", "CLK-FANOUT1", "SPDIF-OUT", "UART0-RX", x),
    ]
}

impl<const P: char, const N: u8> embedded_hal::digital::ErrorType for Pin<P, N, Input> {
    type Error = core::convert::Infallible;
}

impl<const P: char, const N: u8> embedded_hal::digital::InputPin for Pin<P, N, Input> {
    #[inline]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(unsafe { read_volatile(Self::DATA_REG) } & (1 << N) != 0)
    }
    #[inline]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
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
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(unsafe { read_volatile(Self::DATA_REG) } & (1 << N) != 0)
    }
    #[inline]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
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
