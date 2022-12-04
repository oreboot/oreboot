/*
 * Region support.
 *
 * Regions are intended to abstract away the access mechanisms for blocks of
 * data. This could be SPI, eMMC, or a memory region as the backing store.
 * They are accessed through a region_device.  Subregions can be made by
 * chaining together multiple region_devices.
 */

#[repr(C)]
pub struct Region {
    offset: usize,
    size: usize,
}
