use crate::google::chromeec::{
    ec::{Error, MEC_EMI_BASE, MEC_EMI_SIZE},
    ec_commands::{
        HostEventCode, EC_CMD_ACPI_QUERY_EVENT, EC_HOST_CMD_REGION0, EC_HOST_CMD_REGION_SIZE,
        EC_LPC_ADDR_ACPI_CMD, EC_LPC_ADDR_ACPI_DATA, EC_LPC_ADDR_MEMMAP, EC_LPC_CMDR_BUSY,
        EC_LPC_CMDR_DATA, EC_LPC_CMDR_PENDING, EC_MEMMAP_SIZE, EC_MEMMAP_SWITCHES,
    },
};
use log::error;
use util::{
    cpuio::{inb, outb},
    timer::{udelay, Stopwatch, USECS_PER_SEC},
};

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

/// Write bytes to a given LPC-mapped address.
///
/// @port: Base write address
/// @length: Number of bytes to write
/// @msg: Write data buffer
/// @csum: Optional parameter, sums data written
pub fn write_bytes(port: u16, msg: &[u8], csum: &mut u8) {
    for (i, &b) in msg.iter().enumerate() {
        unsafe { outb(b, port + i as u16) };
        *csum += b;
    }
}

/// Write single byte and return byte written and checksum
pub fn write_byte(val: u8, port: u16) -> (u8, u8) {
    let mut csum = 0;
    write_bytes(port, &[val], &mut csum);
    (val, csum)
}

/// Return the byte of EC switch states
pub fn get_switches() -> u8 {
    read_byte(EC_LPC_ADDR_MEMMAP + EC_MEMMAP_SWITCHES)
}

pub fn status_check(port: u16, mask: u8, cond: u8) -> Result<(), Error> {
    let mut sw = Stopwatch::new();

    // One second is more than plenty for any EC operation to complete
    let ec_status_timeout_us = 1 * USECS_PER_SEC;

    // Wait 1 usec between read attempts
    let ec_status_read_period_us = 1;

    sw.init_usecs_expire(ec_status_timeout_us);
    while !sw.expired() {
        if read_byte(port) & mask == cond {
            return Ok(());
        }
        udelay(ec_status_read_period_us);
    }

    Err(Error::Generic)
}

pub fn wait_ready(port: u16) -> Result<(), Error> {
    status_check(port, EC_LPC_CMDR_PENDING as u8, EC_LPC_CMDR_BUSY as u8)
}

pub fn data_ready(port: u16) -> Result<(), Error> {
    status_check(port, EC_LPC_CMDR_DATA as u8, EC_LPC_CMDR_DATA as u8)
}

pub fn get_event() -> HostEventCode {
    if wait_ready(EC_LPC_ADDR_ACPI_CMD).is_err() {
        error!("Timeout waiting for EC ready!\r\n");
        return HostEventCode::None;
    }

    // Issue the ACPI query-event command
    let _ = write_byte(EC_CMD_ACPI_QUERY_EVENT as u8, EC_LPC_ADDR_ACPI_CMD);

    if wait_ready(EC_LPC_ADDR_ACPI_CMD).is_err() {
        error!("Timeout waiting for EC QUERY_EVENT!\r\n");
        return HostEventCode::None;
    }

    if data_ready(EC_LPC_ADDR_ACPI_CMD).is_err() {
        error!("Timeout waiting for data ready!\r\n");
        return HostEventCode::None;
    }

    // Event (or 0 if none) is returned directly in the data byte
    HostEventCode::from(read_byte(EC_LPC_ADDR_ACPI_DATA))
}

pub fn ioport_range() -> (u16, usize) {
    if cfg!(feature = "ec_google_chromeec_mec") {
        (MEC_EMI_BASE, MEC_EMI_SIZE)
    } else {
        let size = 2 * EC_HOST_CMD_REGION_SIZE;
        // Make sure MEMMAP region follows host cmd region.
        assert_eq!(EC_HOST_CMD_REGION0 + size as u16, EC_LPC_ADDR_MEMMAP);
        (EC_HOST_CMD_REGION0, size + EC_MEMMAP_SIZE)
    }
}
