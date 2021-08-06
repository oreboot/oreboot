/* SPDX-License-Identifier: GPL-2.0-only */
#![no_std]
#[allow(non_upper_case_globals)]

pub mod units {
    pub const KiB: usize = 1 << 10;
    pub const MiB: usize = 1 << 20;
    pub const GiB: usize = 1 << 30;
}
pub use units::*;

// Baud is baud rate.
// Since it is not exclusive to UARTs we have it here.
pub enum Baud {
    B115200,
}

/// DeviceCtl is the (small!) set of operations that control a device.
/// Code can call a Ctl funtion in a device, or can use the multictl to call a
/// sequence, e.g. dev.multictl(Off, Baud, On) ...
/// Devices are allowed to ignore any such calls.
/// Why do it this way?
/// Classically, in kernels, this is done with a function-per-operation:
/// on(), off(), ...
/// The problem is that then every device needs to implement methods for even those
/// things they do not do, or the code needs to try to figure out which devices
/// implement a given method and whether to call it.
/// Further, if more methods are added over time, all the devices need to be fixed, and
/// that's painful. It was bad enough just adding Ctl and Stat.
/// Finally, with our union driver, not all operations make sense on all devices in the
/// union: a port80 console supported by a PCH will not have an On, Off, or Baud,
/// but a port80 implemented in a BMC or UART certainly will.
/// Hence, while this interface may seem a bit odd at first, it is informed by what
/// we learned in coreboot, plan 9, and various Unix kernels over the years.
pub enum DeviceCtl {
    /// Turn the device on. This is intended to be a catch-all for any power, clock, etc. needed.
    /// For, e.g., DDR, it would include training. It should be idempotent.
    On,
    /// Turn the device off. This is also dependent on the device.
    Off,
    /// Pause the device. This is intended to be less of a heavy operation than Off.
    Pause,
    /// Resume from a Pause.
    Resume,
    /// Enable TTY functions. This is generally all about baud rate.
    /// There's never been anything else to set, but TTY
    /// should allow us in future to expand. Or should we just
    /// have enums for Baud rate and stop it there?
    /// We ALWAYS end up setting 8n1.
    TTY { baud: Baud },
}
