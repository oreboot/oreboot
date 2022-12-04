use crate::google::chromeec::{ec_commands::*, ec_spi::google_chromeec_command};
use core::mem::size_of;
use drivers::{
    context::Context,
    spi::spi_generic::{Error as SPIError, SPICtrlrBuses},
};

pub const EC_HOST_PARAM_SIZE: usize = 0xfc;
pub const HEADER_BYTES: usize = 8;
pub const MSG_HEADER: usize = 0xec;
pub const MSG_HEADER_BYTES: usize = 3;
pub const MSG_TRAILER_BYTES: usize = 2;
pub const MSG_PROTO_BYTES: usize = MSG_HEADER_BYTES + MSG_TRAILER_BYTES;
pub const MSG_BYTES: usize = EC_HOST_PARAM_SIZE + MSG_PROTO_BYTES;
pub const EC_HOST_REQUEST_HEADER_BYTES: usize = 8;
pub const EC_HOST_RESPONSE_HEADER_BYTES: usize = 8;
pub const EC_HOST_REQUEST_VERSION: u8 = 3;
pub const EC_HOST_RESPONSE_VERSION: u8 = 3;

#[derive(Debug)]
pub enum Error {
    ECResRequestTruncated,
    ECResResponseTooBig,
    ECResInvalidResponse,
    ECResInvalidChecksum,
    ECResResponse(i32),
    ECResError,
    ECSPIError(SPIError),
    ECFailedContextDowncast,
}

/* internal structure to send a command to the EC and wait for response. */
#[repr(C, packed)]
pub struct ChromeECCommand {
    /// command code in, status out
    cmd_code: u16,
    /// command version
    cmd_version: u8,
    /// command_data, if any
    cmd_data_in: *const u8,
    /// command response, if any
    cmd_data_out: *mut u8,
    /// size of command data
    cmd_size_in: u16,
    /// expected size of command response in, actual received size out
    cmd_size_out: u16,
    /// device index for passthru
    cmd_dev_index: i32,
}

impl ChromeECCommand {
    pub fn new() -> Self {
        Self {
            cmd_code: 0,
            cmd_version: 3,
            cmd_data_in: core::ptr::null(),
            cmd_data_out: core::ptr::null_mut(),
            cmd_size_in: 0,
            cmd_size_out: 0,
            cmd_dev_index: 0,
        }
    }

    pub fn cmd_code(&self) -> u16 {
        self.cmd_code
    }

    pub fn set_cmd_code(&mut self, code: u16) {
        self.cmd_code = code;
    }

    pub fn cmd_version(&self) -> u8 {
        self.cmd_version
    }

    pub fn set_cmd_version(&mut self, version: u8) {
        self.cmd_version = version;
    }

    /// safety: caller must ensure the struct memory cast to byte pointer
    /// has valid liveness and size
    pub unsafe fn data_in(&self) -> &[u8] {
        core::slice::from_raw_parts(self.cmd_data_in, self.cmd_size_in as usize)
    }

    /// safety: caller must ensure the struct memory cast to byte pointer
    /// has valid liveness and size
    pub unsafe fn data_out(&self) -> &[u8] {
        core::slice::from_raw_parts(self.cmd_data_out, self.cmd_size_out as usize)
    }

    /// safety: caller must ensure the struct memory cast to byte pointer
    /// has valid liveness and size
    pub unsafe fn data_out_mut(&mut self) -> &mut [u8] {
        core::slice::from_raw_parts_mut(self.cmd_data_out, self.cmd_size_out as usize)
    }

    pub fn size_in(&self) -> u16 {
        self.cmd_size_in
    }

    pub fn set_size_in(&mut self, size: u16) {
        self.cmd_size_in = size;
    }

    pub fn size_out(&self) -> u16 {
        self.cmd_size_out
    }

    pub fn set_size_out(&mut self, size: u16) {
        self.cmd_size_out = size;
    }

    pub fn dev_index(&self) -> i32 {
        self.cmd_dev_index
    }

    pub fn set_dev_index(&mut self, idx: i32) {
        self.cmd_dev_index = idx;
    }
}

/**
 * struct ec_host_request - Version 3 request from host.
 * @struct_version: Should be 3. The EC will return EC_RES_INVALID_HEADER if it
 *                  receives a header with a version it doesn't know how to
 *                  parse.
 * @checksum: Checksum of request and data; sum of all bytes including checksum
 *            should total to 0.
 * @command: Command to send (EC_CMD_...)
 * @command_version: Command version.
 * @reserved: Unused byte in current protocol version; set to 0.
 * @data_len: Length of data which follows this header.
 */
#[repr(C, packed)]
pub struct ECHostRequest {
    struct_version: u8,
    checksum: u8,
    command: u16,
    command_version: u8,
    reserved: u8,
    data_len: u16,
}

impl ECHostRequest {
    pub fn new() -> Self {
        Self {
            struct_version: EC_HOST_REQUEST_VERSION,
            checksum: 0,
            command: 0,
            command_version: 0,
            reserved: 0,
            data_len: 0,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        // SAFETY: reference to self guaranteed valid
        unsafe {
            core::slice::from_raw_parts((self as *const Self) as *const u8, size_of::<Self>())
        }
    }

    pub fn len(&self) -> usize {
        EC_HOST_REQUEST_HEADER_BYTES
    }

    pub fn struct_version(&self) -> u8 {
        self.struct_version
    }

    pub fn set_struct_version(&mut self, version: u8) {
        self.struct_version = version;
    }

    pub fn checksum(&self) -> u8 {
        self.checksum
    }

    pub fn set_checksum(&mut self, csum: u8) {
        self.checksum = csum;
    }

    pub fn command(&self) -> u16 {
        self.command
    }

    pub fn set_command(&mut self, cmd: u16) {
        self.command = cmd;
    }

    pub fn command_version(&self) -> u8 {
        self.command_version
    }

    pub fn set_command_version(&mut self, version: u8) {
        self.command_version = version;
    }

    pub fn reserved(&self) -> u8 {
        self.reserved
    }

    pub fn set_reserved(&mut self, rsv: u8) {
        self.reserved = rsv;
    }

    pub fn data_len(&self) -> u16 {
        self.data_len
    }

    pub fn set_data_len(&mut self, len: u16) {
        self.data_len = len;
    }
}

/**
 * struct ec_host_response - Version 3 response from EC.
 * @struct_version: Struct version (=3).
 * @checksum: Checksum of response and data; sum of all bytes including
 *            checksum should total to 0.
 * @result: EC's response to the command (separate from communication failure)
 * @data_len: Length of data which follows this header.
 * @reserved: Unused bytes in current protocol version; set to 0.
 */
#[repr(C, packed)]
pub struct ECHostResponse {
    struct_version: u8,
    checksum: u8,
    result: u16,
    data_len: u16,
    reserved: u16,
}

impl ECHostResponse {
    pub fn new() -> Self {
        Self {
            struct_version: EC_HOST_RESPONSE_VERSION,
            checksum: 0,
            result: 0,
            data_len: 0,
            reserved: 0,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        // SAFETY: reference to self guaranteed valid
        unsafe {
            core::slice::from_raw_parts((self as *const Self) as *const u8, size_of::<Self>())
        }
    }

    pub fn len(&self) -> usize {
        EC_HOST_RESPONSE_HEADER_BYTES
    }

    pub fn struct_version(&self) -> u8 {
        self.struct_version
    }

    pub fn checksum(&self) -> u8 {
        self.checksum
    }

    pub fn set_checksum(&mut self, csum: u8) {
        self.checksum = csum;
    }

    pub fn result(&self) -> u16 {
        self.result
    }

    pub fn set_result(&mut self, res: u16) {
        self.result = res;
    }

    pub fn data_len(&self) -> u16 {
        self.data_len
    }

    pub fn set_data_len(&mut self, len: u16) {
        self.data_len = len;
    }

    pub fn reserved(&self) -> u16 {
        self.reserved
    }

    pub fn set_reserved(&mut self, res: u16) {
        self.reserved = res;
    }
}

/* Standard Chrome EC protocol, version 3 */
pub struct ECCommandV3 {
    header: ECHostRequest,
    data: [u8; MSG_BYTES],
}

impl ECCommandV3 {
    pub fn new() -> Self {
        Self {
            header: ECHostRequest::new(),
            data: [0u8; MSG_BYTES],
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        // SAFETY: reference to self guaranteed valid
        unsafe {
            core::slice::from_raw_parts((self as *const Self) as *const u8, size_of::<Self>())
        }
    }

    pub fn len(&self) -> usize {
        self.header.len() + MSG_BYTES
    }

    pub fn header(&self) -> &ECHostRequest {
        &self.header
    }

    pub fn header_mut(&mut self) -> &mut ECHostRequest {
        &mut self.header
    }

    pub fn data(&self) -> &[u8] {
        &self.data[..self.header.data_len as usize]
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data[..self.header.data_len as usize]
    }

    pub fn raw_data(&self) -> &[u8; MSG_BYTES] {
        &self.data
    }
}

pub struct ECResponseV3 {
    header: ECHostResponse,
    data: [u8; MSG_BYTES],
}

impl ECResponseV3 {
    pub fn new() -> Self {
        Self {
            header: ECHostResponse::new(),
            data: [0u8; MSG_BYTES],
        }
    }

    pub fn len(&self) -> usize {
        self.header.len() + self.header.data_len as usize
    }

    pub fn as_bytes(&self) -> &[u8] {
        // SAFETY: reference to self guaranteed valid
        unsafe {
            core::slice::from_raw_parts((self as *const Self) as *const u8, size_of::<Self>())
        }
    }

    pub fn header(&self) -> &ECHostResponse {
        &self.header
    }

    pub fn header_mut(&mut self) -> &mut ECHostResponse {
        &mut self.header
    }

    pub fn data(&self) -> &[u8] {
        &self.data[..self.header.data_len as usize]
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data[..self.header.data_len as usize]
    }

    pub fn raw_data(&self) -> &[u8; MSG_BYTES] {
        &self.data
    }
}

pub type CrosECIO = fn(usize, usize, &mut dyn Context) -> Result<(), Error>;

/**
 * google_chromeec_get_board_version() - Get the board version
 * @version: Out parameter to retrieve the board Version
 *
 * Return: 0 on success or -1 on failure/error.
 *
 * This function is used to get the board version information from EC.
 */
pub fn google_chromeec_get_board_version(
    _version: u32,
    spi_map: &[SPICtrlrBuses],
) -> Result<u32, Error> {
    let resp = ECResponseBoardVersion::new();
    let mut cmd = ChromeECCommand::new();
    cmd.set_cmd_code(EC_CMD_GET_BOARD_VERSION);
    cmd.set_size_out(resp.len() as u16);
    unsafe {
        cmd.data_out_mut().copy_from_slice(&resp.as_bytes());
    }

    google_chromeec_command(&mut cmd, spi_map)?;

    Ok(resp.board_version() as u32)
}
