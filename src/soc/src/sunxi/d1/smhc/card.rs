/// The different types of cards
#[derive(Debug, PartialEq)]
pub enum CardType {
    SD1,
    SD2,
    SDHC,
}

/// Card status in R1 response format
pub struct CardStatus(u32);

#[derive(Debug, PartialEq)]
pub enum CardCurrentState {
    Idle,
    Ready,
    Ident,
    Stby,
    Tran,
    Data,
    Rcv,
    Prg,
    Dis,
    Reserved,
}

impl From<u32> for CardCurrentState {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Idle,
            1 => Self::Ready,
            2 => Self::Ident,
            3 => Self::Stby,
            4 => Self::Tran,
            5 => Self::Data,
            6 => Self::Rcv,
            7 => Self::Prg,
            8 => Self::Dis,
            _ => Self::Reserved,
        }
    }
}

impl CardStatus {
    pub(crate) fn new(val: u32) -> Self {
        Self(val)
    }

    /// Indicates if there was an error in the sequence of the authentication process.
    pub fn auth_seq_error(&self) -> bool {
        const BIT: u32 = 1 << 3;
        (self.0 & BIT) == BIT
    }

    /// Indicates that the card expects an application command or
    /// that the command has been interpreted as an application command.
    pub fn app_cmd(&self) -> bool {
        const BIT: u32 = 1 << 5;
        (self.0 & BIT) == BIT
    }

    /// Corresponds to buffer empty signaling on the bus
    pub fn ready_for_data(&self) -> bool {
        const BIT: u32 = 1 << 8;
        (self.0 & BIT) == BIT
    }

    /// The state of the card when receiving the command.
    pub fn current_state(&self) -> CardCurrentState {
        let val = (self.0 >> 9) & 0xF;
        val.into()
    }

    /// General or unknown error occurred during operation.
    pub fn error(&self) -> bool {
        const BIT: u32 = 1 << 19;
        (self.0 & BIT) == BIT
    }

    /// Internal card controller error.
    pub fn cc_error(&self) -> bool {
        const BIT: u32 = 1 << 20;
        (self.0 & BIT) == BIT
    }

    /// Signals that the card is locked by the host
    pub fn is_locked(&self) -> bool {
        const BIT: u32 = 1 << 25;
        (self.0 & BIT) == BIT
    }

    // TODO: the rest
}

impl core::fmt::Display for CardStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:x}", self.0)
    }
}
