use ec::google::chromeec::ec_commands::{ec_host_event_mask, HostEventCode};

pub const MAINBOARD_EC_SCI_EVENTS: u64 = ec_host_event_mask(HostEventCode::LidClosed as u32) |
    ec_host_event_mask(HostEventCode::LidOpen as u32) |
    ec_host_event_mask(HostEventCode::AcConnected as u32) |
    ec_host_event_mask(HostEventCode::AcDisconnected as u32) |
    ec_host_event_mask(HostEventCode::BatteryLow as u32) |
    ec_host_event_mask(HostEventCode::BatteryCritical as u32) |
    ec_host_event_mask(HostEventCode::Battery as u32) |
    ec_host_event_mask(HostEventCode::BatteryStatus as u32) |
    ec_host_event_mask(HostEventCode::ThermalThreshold as u32) |
    ec_host_event_mask(HostEventCode::ThrottleStart as u32) |
    ec_host_event_mask(HostEventCode::ThrottleStop as u32) |
    ec_host_event_mask(HostEventCode::Mkbp as u32) |
    ec_host_event_mask(HostEventCode::PdMcu as u32) |
    ec_host_event_mask(HostEventCode::ModeChange as u32); 

pub const MAINBOARD_EC_SMI_EVENTS: u64 = ec_host_event_mask(HostEventCode::LidClosed as u32);

/// EC can wake from S5 with lid or power button
pub const MAINBOARD_EC_S5_WAKE_EVENTS: u64 = ec_host_event_mask(HostEventCode::LidOpen as u32) |
    ec_host_event_mask(HostEventCode::PowerButton as u32);

/// EC can wake from S3/S0ix with:
/// 1. Lid open
/// 2. Power button
/// 3. AC power supply connected
/// 4. AC power supply disconnected
/// 5. Key press
/// 6. Mode change
pub const MAINBOARD_EC_S3_WAKE_EVENTS: u64 = MAINBOARD_EC_S5_WAKE_EVENTS |
    ec_host_event_mask(HostEventCode::AcConnected as u32) |
    ec_host_event_mask(HostEventCode::AcDisconnected as u32) |
    ec_host_event_mask(HostEventCode::KeyPressed as u32) |
    ec_host_event_mask(HostEventCode::ModeChange as u32);

pub const MAINBOARD_EC_S0IX_WAKE_EVENTS: u64 = MAINBOARD_EC_S3_WAKE_EVENTS;

/// Log EC wake events plus EC shutdown events
pub const MAINBOARD_EC_LOG_EVENTS: u64 = ec_host_event_mask(HostEventCode::ThermalShutdown as u32) |
    ec_host_event_mask(HostEventCode::BatteryShutdown as u32) |
    ec_host_event_mask(HostEventCode::Panic as u32);
