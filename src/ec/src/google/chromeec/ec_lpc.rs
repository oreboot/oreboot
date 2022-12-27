use crate::google::chromeec::ec_commands::{EC_LPC_ADDR_MEMMAP, EC_MEMMAP_SWITCHES};
use util::cpuio::inb;

/// Read bytes from a given LPC-mapped address.
///
/// @port: Base read address
/// @dest: Destination buffer
pub fn read_bytes(port: u16, dest: &mut [u8]) {
    if cfg!(feature = "ec_google_chromeec_mec") {
        unimplemented!("MEC is unimplemented");
    }

    for (i, b) in dest.iter_mut().enumerate() {
        *b = unsafe { inb(port + i as u16) };
    }
}

/// Read single byte and return byte read
pub fn read_byte(port: u16) -> u8 {
    let mut byte = [0];
    read_bytes(port, byte.as_mut());
    byte[0]
}

/// Return the byte of EC switch states
pub fn get_switches() -> u8 {
    read_byte(EC_LPC_ADDR_MEMMAP + EC_MEMMAP_SWITCHES)
}
