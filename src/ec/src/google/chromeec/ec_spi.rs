use crate::google::chromeec::{
    crosec_proto::crosec_command_proto,
    ec::{ChromeECCommand, Error},
    ec_commands::HostEventCode,
    ec_i2c::{REQ_BUF, RESP_BUF},
};
use drivers::{
    context::Context,
    spi::spi_generic::{SPICtrlrBuses, SPISlave},
};
use log::error;
use util::timer::{Stopwatch, USECS_PER_SEC};

use spin::rwlock::RwLock;

pub const EC_GOOGLE_CHROMEEC_SPI_WAKEUP_DELAY_US: i32 = 0;
pub const CS_COOLDOWN_US: u64 = 200;
pub const EC_FRAMING_BYTE: u8 = 0xec;

pub static CS_COOLDOWN_SW: RwLock<Stopwatch> = RwLock::new(Stopwatch::new());

//pub const SPI_BUS: u32 = CONFIG_EC_GOOGLE_CHROMEEC_SPI_BUS;
//pub const SPI_CHIP: u32 = CONFIG_EC_GOOGLE_CHROMEEC_SPI_CHIP;
pub const SPI_BUS: u32 = 0x1; // FIXME: default value needs proper kconfig
pub const SPI_CHIP: u32 = 0x0; // FIXME: default value needs proper kconfig

pub fn crosec_spi_io(
    req_size: usize,
    resp_size: usize,
    ctx: &mut dyn Context,
) -> Result<(), Error> {
    const FUNC_NAME: &'static str = "crosec_spi_io";
    if let Some(slave) = ctx.as_any_mut().downcast_mut::<SPISlave>() {
        let out = |slave: &SPISlave| -> Result<(), Error> {
            slave.release_bus().map_err(|e| Error::ECSPIError(e))?;
            (*CS_COOLDOWN_SW.write()).init_usecs_expire(CS_COOLDOWN_US);
            Ok(())
        };

        /* Wait minimum delay between CS assertions. */
        (*CS_COOLDOWN_SW.write()).wait_until_expired();
        slave.claim_bus().map_err(|e| Error::ECSPIError(e))?;

        /* Allow EC to ramp up clock after being awaken.
         * See chrome-os-partner:32223 for more details. */
        // FIXME: uncomment with a udelay implementation
        // udelay(EC_GOOGLE_CHROMEEC_SPI_WAKEUP_DELAY_US);

        if slave
            .xfer(&(*REQ_BUF.read()).data[..req_size], &mut [0u8; 0])
            .is_err()
        {
            error!("{}: Failed to send request.", FUNC_NAME);
            return out(&slave);
        }

        let mut sw = Stopwatch::new();
        sw.init_usecs_expire(USECS_PER_SEC);

        loop {
            let mut byte = [0u8; 1];
            if slave.xfer(&[0u8; 0], &mut byte[..]).is_err() {
                error!("{}: Failed to receive byte", FUNC_NAME);
                return out(&slave);
            }
            if byte[0] == EC_FRAMING_BYTE {
                break;
            }
            if sw.expired() {
                error!("{}: Timeout waiting for framing byte.", FUNC_NAME);
                return out(&slave);
            }
        }

        if slave
            .xfer(&[0u8; 0], &mut (*RESP_BUF.write()).data[..resp_size])
            .is_err()
        {
            error!("{}: Failed to receive a response.", FUNC_NAME);
        }

        out(&slave)
    } else {
        Err(Error::ECFailedContextDowncast)
    }
}

pub fn google_chromeec_command(
    cec_command: &mut ChromeECCommand,
    spi_map: &[SPICtrlrBuses],
) -> Result<(), Error> {
    static DONE: RwLock<bool> = RwLock::new(false);
    static SLAVE: RwLock<SPISlave> = RwLock::new(SPISlave::new());

    if !*DONE.read() {
        (*SLAVE.write())
            .setup(SPI_BUS, SPI_CHIP, spi_map)
            .map_err(|e| Error::ECSPIError(e))?;
        (*CS_COOLDOWN_SW.write()).init();
        (*DONE.write()) = true;
    }

    crosec_command_proto(cec_command, crosec_spi_io, &mut (*SLAVE.write()))?;

    Ok(())
}

pub fn google_chromeec_get_event() -> HostEventCode {
    error!("{}: Not supported.", "google_chromeec_get_event");
    HostEventCode::None
}
