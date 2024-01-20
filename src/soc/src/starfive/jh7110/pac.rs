use jh71xx_hal::pac;

pub use pac::*;

/// Gets a reference to the SYSCRG register.
///
/// # Safety
///
/// Caller must ensure exclusive access.
///
/// The reference must be dropped before calling again.
pub fn syscrg_reg<'r>() -> &'r pac::syscrg::RegisterBlock {
    unsafe { &*pac::SYSCRG::ptr() }
}

/// Gets a reference to the AONCRG register.
///
/// # Safety
///
/// Caller must ensure exclusive access.
///
/// The reference must be dropped before calling again.
pub fn aoncrg_reg<'r>() -> &'r pac::aoncrg::RegisterBlock {
    unsafe { &*pac::AONCRG::ptr() }
}

/// Gets a reference to the AONCRG register.
///
/// # Safety
///
/// Caller must ensure exclusive access.
///
/// The reference must be dropped before calling again.
pub fn sys_syscon_reg<'r>() -> &'r pac::sys_syscon::RegisterBlock {
    unsafe { &*pac::SYS_SYSCON::ptr() }
}

/// Gets a reference to the UART0 register.
///
/// # Safety
///
/// Caller must ensure exclusive access.
///
/// The reference must be dropped before calling again.
pub fn uart0_reg<'r>() -> &'r pac::uart0::RegisterBlock {
    unsafe { &*pac::UART0::ptr() }
}

/// Gets a reference to the SYS_PINCTRL register.
///
/// # Safety
///
/// Caller must ensure exclusive access.
///
/// The reference must be dropped before calling again.
pub fn sys_pinctrl_reg<'r>() -> &'r pac::sys_pinctrl::RegisterBlock {
    unsafe { &*pac::SYS_PINCTRL::ptr() }
}

/// Gets a reference to the CLINT register.
///
/// # Safety
///
/// Caller must ensure exclusive access.
///
/// The reference must be dropped before calling again.
pub fn clint_reg<'r>() -> &'r pac::clint::RegisterBlock {
    unsafe { &*pac::CLINT::ptr() }
}

/// Gets a reference to the PLIC register.
///
/// # Safety
///
/// Caller must ensure exclusive access.
///
/// The reference must be dropped before calling again.
pub fn plic_reg<'r>() -> &'r pac::plic::RegisterBlock {
    unsafe { &*pac::PLIC::ptr() }
}
