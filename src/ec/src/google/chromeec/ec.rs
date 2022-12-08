use crate::google::chromeec::{ec_commands::*, ec_spi::google_chromeec_command};
use core::mem::size_of;
use drivers::{
    context::Context,
    spi::spi_generic::{Error as SPIError, SPICtrlrBuses},
};
use log::debug;
use spin::rwlock::RwLock;

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
pub const INVALID_HCMD: u8 = 0xff;

#[repr(C)]
pub struct EventInfo {
    pub log_events: u64,
    pub sci_events: u64,
    pub smi_events: u64,
    pub s3_wake_events: u64,
    pub s3_device_events: u64,
    pub s5_wake_events: u64,
    pub s0ix_wake_events: u64,
}

impl EventInfo {
    pub const fn new() -> Self {
        Self {
            log_events: 0u64,
            sci_events: 0u64,
            smi_events: 0u64,
            s3_wake_events: 0u64,
            s3_device_events: 0u64,
            s5_wake_events: 0u64,
            s0ix_wake_events: 0u64,
        }
    }

    pub fn init(&self, is_s3_wakeup: bool, spi_map: &[SPICtrlrBuses]) -> Result<(), Error> {
        if is_s3_wakeup {
            log_events(self.log_events | self.s3_wake_events);

            /* Log and clear device events that may wake the system. */
            log_device_events(self.s3_device_events);

            /* Disable SMI and wake events. */
            set_smi_mask(0, spi_map);

            /* Restore SCI event mask. */
            set_sci_mask(self.sci_events, spi_map);
        } else {
            set_smi_mask(self.smi_events, spi_map);

            log_events(self.log_events | self.s5_wake_events);

            if is_uhepi_supported()? {
                set_lazy_wake_masks(
                    self.s5_wake_events,
                    self.s3_wake_events,
                    self.s0ix_wake_events,
                );
            }
        }

        /* Clear wake event mask. */
        set_wake_mask(0);

        Ok(())
    }
}

#[repr(C)]
pub struct EventMap {
    pub set_cmd: u8,
    pub clear_cmd: u8,
    pub get_cmd: u8,
}

pub const EVENT_MAP: [EventMap; 9] = [
    EventMap {
        set_cmd: INVALID_HCMD,
        clear_cmd: EC_CMD_HOST_EVENT_CLEAR as u8,
        get_cmd: INVALID_HCMD,
    },
    EventMap {
        set_cmd: INVALID_HCMD,
        clear_cmd: EC_CMD_HOST_EVENT_CLEAR_B as u8,
        get_cmd: EC_CMD_HOST_EVENT_GET_B as u8,
    },
    EventMap {
        set_cmd: EC_CMD_HOST_EVENT_SET_SCI_MASK as u8,
        clear_cmd: INVALID_HCMD,
        get_cmd: EC_CMD_HOST_EVENT_GET_SCI_MASK as u8,
    },
    EventMap {
        set_cmd: EC_CMD_HOST_EVENT_SET_SMI_MASK as u8,
        clear_cmd: INVALID_HCMD,
        get_cmd: EC_CMD_HOST_EVENT_GET_SMI_MASK as u8,
    },
    EventMap {
        set_cmd: INVALID_HCMD,
        clear_cmd: INVALID_HCMD,
        get_cmd: INVALID_HCMD,
    },
    EventMap {
        set_cmd: EC_CMD_HOST_EVENT_SET_WAKE_MASK as u8,
        clear_cmd: INVALID_HCMD,
        get_cmd: EC_CMD_HOST_EVENT_GET_WAKE_MASK as u8,
    },
    EventMap {
        set_cmd: EC_CMD_HOST_EVENT_SET_WAKE_MASK as u8,
        clear_cmd: INVALID_HCMD,
        get_cmd: EC_CMD_HOST_EVENT_GET_WAKE_MASK as u8,
    },
    EventMap {
        set_cmd: EC_CMD_HOST_EVENT_SET_WAKE_MASK as u8,
        clear_cmd: INVALID_HCMD,
        get_cmd: EC_CMD_HOST_EVENT_GET_WAKE_MASK as u8,
    },
    EventMap {
        set_cmd: EC_CMD_HOST_EVENT_SET_WAKE_MASK as u8,
        clear_cmd: INVALID_HCMD,
        get_cmd: EC_CMD_HOST_EVENT_GET_WAKE_MASK as u8,
    },
];

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
    UnsupportedCommand,
    UnsupportedFeature,
}

/* internal structure to send a command to the EC and wait for response. */
#[repr(C, packed)]
pub struct ChromeECCommand {
    /// command code in, status out
    pub cmd_code: u16,
    /// command version
    pub cmd_version: u8,
    /// command_data, if any
    pub cmd_data_in: *const u8,
    /// command response, if any
    pub cmd_data_out: *mut u8,
    /// size of command data
    pub cmd_size_in: u16,
    /// expected size of command response in, actual received size out
    pub cmd_size_out: u16,
    /// device index for passthru
    pub cmd_dev_index: i32,
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

/// Query the EC for specified mask indicating enabled events.
/// The EC maintains separate event masks for SMI, SCI and WAKE.
pub fn uhepi_cmd(
    mask: u8,
    action: u8,
    value: &mut u64,
    spi_map: &[SPICtrlrBuses],
) -> Result<(), Error> {
    let mut params = EcParamsHostEvent {
        action: action,
        mask_type: mask,
        reserved: 0,
        value: 0,
    };

    let mut resp = EcResponseHostEvent::new();

    let mut cmd = ChromeECCommand {
        cmd_code: EC_CMD_HOST_EVENT,
        cmd_version: 0,
        cmd_data_in: params.as_byte_ptr(),
        cmd_size_in: params.len() as u16,
        cmd_data_out: resp.as_mut_byte_ptr(),
        cmd_size_out: resp.len() as u16,
        cmd_dev_index: 0,
    };

    if action != EcHostEventAction::Get as u8 {
        params.value = *value;
    } else {
        *value = 0;
    }

    let ret = google_chromeec_command(&mut cmd, spi_map);
    if action != EcHostEventAction::Get as u8 {
        return ret;
    }
    if ret.is_ok() {
        *value = resp.value;
    }
    ret
}

pub fn is_uhepi_supported(spi_map: &[SPICtrlrBuses]) -> Result<bool, Error> {
    static UHEPI_SUPPORT: RwLock<u32> = RwLock::new(0);
    const UHEPI_SUPPORTED: u32 = 1;
    const UHEPI_NOT_SUPPORTED: u32 = 2;

    if *UHEPI_SUPPORT.read() == 0 {
        (*UHEPI_SUPPORT.write()) = if check_feature(EcFeatureCode::UnifiedWakeMasks, spi_map)? > 0 {
            UHEPI_SUPPORTED
        } else {
            UHEPI_NOT_SUPPORTED
        };
        debug!(
            "Chrome EC: UHEPI {}",
            if *UHEPI_SUPPORT.read() == UHEPI_SUPPORTED {
                "supported"
            } else {
                "not supported"
            }
        );
    }

    Ok(*UHEPI_SUPPORT.read() == UHEPI_SUPPORTED)
}

pub fn check_feature(feature: EcFeatureCode, spi_map: &[SPICtrlrBuses]) -> Result<u32, Error> {
    let mut resp = EcResponseGetFeatures::new();
    let mut cmd = ChromeECCommand {
        cmd_code: EC_CMD_GET_FEATURES,
        cmd_version: 0,
        cmd_data_in: core::ptr::null(),
        cmd_size_in: 0,
        cmd_data_out: resp.as_mut_byte_ptr(),
        cmd_size_out: resp.len() as u16,
        cmd_dev_index: 0,
    };

    google_chromeec_command(&mut cmd, spi_map)?;

    if feature as usize >= 8 * resp.flags.len() {
        return Err(Error::UnsupportedFeature);
    }

    Ok(resp.flags[(feature as usize) / 32] & ec_feature_mask_0(feature as u32) as u32)
}

pub fn get_events_b(spi_map: &[SPICtrlrBuses]) -> Result<u64, Error> {
    get_mask(EcHostEventMaskType::B as u8, spi_map)
}

pub fn get_mask(type_: u8, spi_map: &[SPICtrlrBuses]) -> Result<u64, Error> {
    let mut value = 0u64;

    if is_uhepi_supported(spi_map)? {
        uhepi_cmd(type_, EcHostEventAction::Get as u8, &mut value, spi_map)?;
    } else {
        assert!((type_ as usize) < EVENT_MAP.len());
        handle_non_uhepi_cmd(
            EVENT_MAP[type_ as usize].get_cmd,
            EcHostEventAction::Get,
            &mut value,
            spi_map,
        )?;
    }
    Ok(value)
}

pub fn handle_non_uhepi_cmd(
    hcmd: u8,
    action: EcHostEventAction,
    value: &mut u64,
    spi_map: &[SPICtrlrBuses],
) -> Result<(), Error> {
    let mut params = EcParamsHostEventMask::new();
    let mut resp = EcResponseHostEventMask::new();
    let mut cmd = ChromeECCommand {
        cmd_code: hcmd as u16,
        cmd_version: 0,
        cmd_data_in: params.as_byte_ptr(),
        cmd_size_in: params.len() as u16,
        cmd_data_out: resp.as_mut_byte_ptr(),
        cmd_size_out: resp.len() as u16,
        cmd_dev_index: 0,
    };

    if hcmd == INVALID_HCMD {
        return Err(Error::UnsupportedCommand);
    }

    if action != EcHostEventAction::Get {
        params.mask = *value as u32;
    } else {
        *value = 0;
    }

    let ret = google_chromeec_command(&mut cmd, spi_map);

    if action != EcHostEventAction::Get {
        return ret;
    }

    if ret.is_ok() {
        *value = resp.mask as u64;
    }

    ret
}

pub fn set_mask(type_: u8, mut mask: u64, spi_map: &[SPICtrlrBuses]) -> Result<(), Error> {
    if is_uhepi_supported()? {
        return uhepi_cmd(type_, EcHostEventAction::Set, &mut mask, spi_map);
    }

    assert!((type_ as usize) < EVENT_MAP.len());

    handle_non_uhepi_cmd(
        (*EVENT_MAP.read())[type_ as usize].set_cmd,
        EcHostEventAction::Set,
        &mut mask,
    )
}

pub fn set_sci_mask(mut mask: u64, spi_map: &[SPICtrlrBuses]) -> Result<(), Error> {
    debug!("Chrome EC: Set SCI mask to 0x{:16x}", mask);
    set_mask(EcHostEventMaskType::SciMask as u8, mask, spi_map)
}

pub fn set_smi_mask(mut mask: u64, spi_map: &[SPICtrlrBuses]) -> Result<(), Error> {
    debug!("Chrome EC: Set SMI mask to 0x{:16x}", mask);
    set_mask(EcHostEventMaskType::SmiMask as u8, mask, spi_map)
}

pub fn set_wake_mask(mut mask: u64, spi_map: &[SPICtrlrBuses]) -> Result<(), Error> {
    debug!("Chrome EC: Set WAKE mask to 0x{:16x}", mask);
    set_mask(EcHostEventMaskType::WakeMask as u8, mask, spi_map)
}

pub fn set_s3_lazy_wake_mask(mask: u64, spi_map: &[SPICtrlrBuses]) -> Result<(), Error> {
    debug!("Chrome EC: Set S3 LAZY WAKE mask to 0x{:16x}", mask);
    set_mask(EcHostEventMaskType::LazyWakeMaskS3 as u8, mask, spi_map)
}

pub fn set_s5_lazy_wake_mask(mask: u64, spi_map: &[SPICtrlrBuses]) -> Result<(), Error> {
    debug!("Chrome EC: Set S5 LAZY WAKE mask to 0x{:16x}", mask);
    set_mask(EcHostEventMaskType::LazyWakeMaskS5 as u8, mask, spi_map)
}

pub fn set_s0ix_lazy_wake_mask(mask: u64, spi_map: &[SPICtrlrBuses]) -> Result<(), Error> {
    debug!("Chrome EC: Set S0iX LAZY WAKE mask to 0x{:16x}", mask);
    set_mask(EcHostEventMaskType::LazyWakeMaskS0ix as u8, mask, spi_map)
}

pub fn set_lazy_wake_masks(
    s5_mask: u64,
    s3_mask: u64,
    s0ix_mask: u64,
    spi_map: &[SPICtrlrBuses],
) -> Result<(), Error> {
    if set_s5_lazy_wake_mask(s5_mask).is_err() {
        debug!("Error: Set S5 LAZY WAKE mask failed");
    }

    if set_s3_lazy_wake_mask(s3_mask).is_err() {
        debug!("Error: Set S3 LAZY WAKE mask failed");
    }

    if s0ix_mask != 0 && set_s0ix_lazy_wake_mask(s0ix_mask).is_err() {
        debug!("Error: Set S0iX LAZY WAKE mask failed");
    }

    Ok(())
}

pub fn get_mask(type_: EcHostEventMaskType, spi_map: &[SPICtrlrMapBuses]) -> Result<u64, Error> {
    let mut value = 0;

    if is_uhepi_supported(spi_map)? {
        uhepi_cmd(type_, EcHostEventAction::Get, &mut value)?;
    } else {
        assert!((type_ as usize) < EVENT_MAP.len());
        handle_non_uhepi_cmd(
            EVENT_MAP[type_ as usize].get_cmd,
            EcHostEventAction::Get,
            &mut value,
        )?;
    }

    Ok(value)
}

pub fn clear_mask(
    type_: EcHostEventMaskType,
    mut mask: u64,
    spi_map: &[SPICtrlrMapBuses],
) -> Result<(), Error> {
    if is_uhepi_supported(spi_map)? {
        uhepi_cmd(type_, EcHostEventAction::Clear, &mut mask, spi_map);
    }

    assert!((type_ as usize) < EVENT_MAP.len());

    handle_non_uhepi_cmd(
        EVENT_MAP[type_ as usize].clear_cmd,
        EcHostEventAction::Clear,
        &mut mask,
        spi_map,
    )
}

pub fn get_events_b(spi_map: &[SPICtrlrMapBuses]) -> Result<u64, Error> {
    get_mask(EcHostEventMaskType::B, spi_map)
}

pub fn clear_events_b(spi_map: &[SPICtrlrMapBuses]) -> Result<u64, Error> {
    debug!("Chrome EC: clear events_b mask to 0x{:16x}", mask);
    clear_mask(EcHostEventMaskType::B, mask, spi_map)
}

pub fn log_events(mask: u64, spi_map: &[SPICtrlrMapBuses]) -> Result<(), Error> {
    if cfg!(ELOG) {
        return Ok(());
    }

    let events = get_events_b(spi_map)? & mask;

    for i in 1..size_of::<u64>() * 8 {
        if (ec_host_event_mask(i) & events) != 0 {
            elog_add_event_byte(ELOG_TYPE_EC_EVENT, i);
        }
    }

    clear_events_b(events, spi_map)
}
