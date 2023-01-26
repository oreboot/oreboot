#[cfg(any(
    feature = "apollolake",
    feature = "geminilake",
    feature = "baytrail",
    feature = "braswell",
    feature = "broadwell",
    feature = "cannonlake",
    feature = "elkhartlake",
    feature = "icelake",
    feature = "jasperlake",
    feature = "meteorlake",
    feature = "skylake",
    feature = "tigerlake",
    feature = "xeon_sp",
))]
pub const PM1_CNT: u8 = 0x04;
