#[cfg(feature = "12a")]
pub(crate) use jh71xx_pac::jh7110_vf2_12a_pac as pac;
#[cfg(feature = "13b")]
pub(crate) use jh71xx_pac::jh7110_vf2_13b_pac as pac;

pub use pac::*;

// SAFETY: this function is called during init, when only a single thread on a single core is
// running, ensuring exclusive access.
//
// The reference must be dropped before calling again.
pub(crate) fn syscrg_reg<'r>() -> &'r pac::syscrg::RegisterBlock {
    unsafe { &*pac::SYSCRG::ptr() }
}

// SAFETY: this function is called during init, when only a single thread on a single core is
// running, ensuring exclusive access.
//
// The reference must be dropped before calling again.
pub(crate) fn aoncrg_reg<'r>() -> &'r pac::aoncrg::RegisterBlock {
    unsafe { &*pac::AONCRG::ptr() }
}

// SAFETY: this function is called during init, when only a single thread on a single core is
// running, ensuring exclusive access.
//
// The reference must be dropped before calling again.
pub(crate) fn sys_syscon_reg<'r>() -> &'r pac::sys_syscon::RegisterBlock {
    unsafe { &*pac::SYS_SYSCON::ptr() }
}

// SAFETY: this function is called during init, when only a single thread on a single core is
// running, ensuring exclusive access.
//
// The reference must be dropped before calling again.
pub(crate) fn uart0_reg<'r>() -> &'r pac::uart0::RegisterBlock {
    unsafe { &*pac::UART0::ptr() }
}

// SAFETY: this function is called during init, when only a single thread on a single core is
// running, ensuring exclusive access.
//
// The reference must be dropped before calling again.
pub(crate) fn clint_reg<'r>() -> &'r pac::clint::RegisterBlock {
    unsafe { &*pac::CLINT::ptr() }
}

// SAFETY: this function is called during init, when only a single thread on a single core is
// running, ensuring exclusive access.
//
// The reference must be dropped before calling again.
pub(crate) fn sys_pinctrl_reg<'r>() -> &'r pac::sys_pinctrl::RegisterBlock {
    unsafe { &*pac::SYS_PINCTRL::ptr() }
}
