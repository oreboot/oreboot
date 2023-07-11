//! Internal DMA controller

#[repr(C, align(4))]
#[derive(Default, Debug)]
pub struct Descriptor {
    des0: u32,
    des1: u32,
    des2: u32,
    des3: u32,
}

impl Descriptor {
    /// Indicates whether the descriptor is currently owned by the IDMAC.
    pub fn is_owned(&self) -> bool {
        self.des0 & (1 << 31) != 0
    }

    /// Indicates whether an error happened during transfer.
    pub fn is_err(&self) -> bool {
        self.des0 & (1 << 30) != 0
    }
}

pub struct DescriptorConfig {
    /// When true, this will prevent the setting of the TX/RX interrupt
    /// bit of the IDMAC status register for data that ends in the buffer the
    /// descriptor points to.
    pub disable_int_on_complete: bool,
    /// If this is the first descriptor.
    pub first: bool,
    /// If this is the last descriptor.
    pub last: bool,
    /// Buffer data size in bytes, must be a multiple of 4.
    pub buff_size: u16,
    /// The physical address of the data buffer.
    pub buff_addr: core::ptr::NonNull<[u8]>,
    /// The physical address of the next descriptor.
    pub next_desc: Option<*const ()>,
}

#[derive(Debug)]
pub enum Error {
    InvalidBufferSize,
}

impl TryFrom<DescriptorConfig> for Descriptor {
    type Error = Error;

    fn try_from(value: DescriptorConfig) -> Result<Self, Self::Error> {
        let mut descriptor = Descriptor {
            des0: 0,
            des1: 0,
            des2: 0,
            des3: 0,
        };

        if value.disable_int_on_complete {
            descriptor.des0 |= 1 << 1;
        }
        if value.last {
            descriptor.des0 |= 1 << 2;
        }
        if value.first {
            descriptor.des0 |= 1 << 3;
        }

        // These always need to be set
        descriptor.des0 |= 1 << 4;
        descriptor.des0 |= 1 << 31;

        if value.buff_size < (1 << 13) && (value.buff_size & 0b11) == 0 {
            descriptor.des1 = value.buff_size as u32;
        } else {
            return Err(Error::InvalidBufferSize);
        }

        // Right-shift by 2 because it is a *word-address*.
        descriptor.des2 = (value.buff_addr.as_ptr() as *const () as u32) >> 2;

        if let Some(next) = value.next_desc {
            // Right-shift by 2 because it is a *word-address*.
            descriptor.des3 = (next as u32) >> 2;
        }

        Ok(descriptor)
    }
}
