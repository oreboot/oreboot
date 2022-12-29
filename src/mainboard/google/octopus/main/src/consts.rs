#[cfg(not(any(feature = "bobba")))]
pub const DRAM_PART_IN_CBI_BOARD_ID_MIN: i32 = 255;

#[cfg(feature = "bobba")]
pub const DRAM_PART_IN_CBI_BOARD_ID_MIN: i32 = 3;
