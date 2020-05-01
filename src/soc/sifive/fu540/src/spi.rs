use spi::*;

pub const FU540_SPI_CONFIG: SiFiveSpiConfig =
    SiFiveSpiConfig { freq: 33_330_000, phase: SiFiveSpiPhase::SampleLeading, polarity: SiFiveSpiPolarity::InactiveLow, protocol: SiFiveSpiProtocol::Single, endianness: SiFiveSpiEndianness::BigEndian, bits_per_frame: 8 };

pub const FU540_SPI_MMAP_CONFIG: SiFiveSpiMmapConfig =
    SiFiveSpiMmapConfig { command_enable: true, address_len: 4, pad_count: 6, command_protocol: SiFiveSpiProtocol::Single, address_protocol: SiFiveSpiProtocol::Quad, data_protocol: SiFiveSpiProtocol::Quad, command_code: 0xec, pad_code: 0 };
