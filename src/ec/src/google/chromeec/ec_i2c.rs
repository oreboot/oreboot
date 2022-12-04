use spin::rwlock::RwLock;

pub const PROTO3_FRAMING_BYTES: usize = 4;
pub const PROTO3_MAX_PACKET_SIZE: usize = 268;

#[repr(C, align(4))]
pub struct Proto3I2CBuf {
    pub framing_bytes: [u8; PROTO3_FRAMING_BYTES],
    pub data: [u8; PROTO3_MAX_PACKET_SIZE],
}

impl Proto3I2CBuf {
    pub const fn new() -> Self {
        Self {
            framing_bytes: [0u8; PROTO3_FRAMING_BYTES],
            data: [0u8; PROTO3_MAX_PACKET_SIZE],
        }
    }
}

pub static REQ_BUF: RwLock<Proto3I2CBuf> = RwLock::new(Proto3I2CBuf::new());
pub static RESP_BUF: RwLock<Proto3I2CBuf> = RwLock::new(Proto3I2CBuf::new());
