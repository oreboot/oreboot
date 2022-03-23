use crate::{infer_type, Entry, FdtIterator, FdtReader, Type, MAX_NAME_SIZE};
use heapless::consts::*;
use heapless::{String, Vec};
use oreboot_drivers::{
    wrappers::{Memory, SectionReader},
    Driver, Result,
};

#[derive(Default, Debug, Clone)]
pub struct Area {
    // Unsure about U512 as a default size. Too big?
    pub description: String<U512>,
    pub compatible: String<U512>,
    // If not specified, it will be automatically computed based on previous areas (if this is
    // first area, we start with 0).
    pub offset: Option<u32>,
    pub size: u32,
    pub file: Option<String<U512>>,
}

pub fn get_kernel_area(dtfs_base: usize, dtfs_size: usize) -> Area {
    let dtb = SectionReader::new(&Memory {}, dtfs_base, dtfs_size);
    let areas = read_areas(&dtb).unwrap();
    let mut last_area_end = 0;
    let mut area_iter = areas.into_iter();

    // Look for the first rompayload area and check for overlapping areas on the way
    // The layoutflash tool sorts offsets while building and errors out if there are overlapping areas.
    // If there are no offsets defined in the fdt the areas are just put one after another
    // the sum of sizes of area 0 to n-1 defines the offset of area n.
    // This code assumes that layoutflash made sure the fdt is sorted in that way.
    // The offset property for an area is set for cur_area here for the first time.
    let area_opt = loop {
        let mut cur_area = match area_iter.next() {
            Some(a) => a,
            None => break None,
        };
        let offset = cur_area.offset.unwrap_or(last_area_end);

        if offset < last_area_end {
            panic!("Malformed dtb: offset lies within previous area.\n");
        }

        if cur_area.compatible.eq("ore-rompayload") {
            cur_area.offset = Some(last_area_end);
            break Some(cur_area);
        }
        last_area_end = offset + cur_area.size;
    };

    // Load payload using kernel_area.
    area_opt.expect("Did not find node of type 'ore-rompayload' while looping through dtfs.\n")
}

// MAX_NAME_SIZE is 64 atm. Thus v shouldn't be able to grow beyond that.
pub fn read_all(d: &dyn Driver) -> Vec<u8, U64> {
    let mut v = Vec::new();
    v.resize(MAX_NAME_SIZE, 0)
        .expect("Tried resizing beyond v's size");
    // Safe to unwrap because SliceReader does not return an error.
    // as_mut_slice() is not implemented on heapless::Vec. However:
    // "Equivalent to &mut s[..].": https://doc.rust-lang.org/std/vec/struct.Vec.html#method.as_mut_slice
    let size = d.pread(&mut v, 0).unwrap();
    v.truncate(size);
    v
}

pub fn read_area_node<D: Driver>(iter: &mut FdtIterator<D>) -> Result<Area> {
    let mut area = Area {
        ..Default::default()
    };
    while let Some(item) = iter.next()? {
        match item {
            Entry::StartNode { name: _ } => {
                iter.skip_node()?;
            }
            Entry::EndNode => return Ok(area),
            Entry::Property { name, value } => {
                let data = read_all(&value);
                match (name, infer_type(&data[..])) {
                    ("description", Type::String(x)) => area.description = String::from(x),
                    ("compatible", Type::String(x)) => area.compatible = String::from(x),
                    ("offset", Type::U32(x)) => area.offset = Some(x),
                    ("size", Type::U32(x)) => area.size = x,
                    ("file", Type::String(x)) => area.file = Some(String::from(x)),
                    (_, _) => {}
                }
            }
        }
    }
    Ok(area)
}

pub fn read_areas(driver: &impl Driver) -> Result<Vec<Area, U64>> {
    let mut areas = Vec::new();
    let reader = FdtReader::new(driver).unwrap();
    let mut iter = reader.walk();
    while let Some(item) = iter.next().unwrap() {
        match item {
            Entry::StartNode { name } => {
                if name.starts_with("area@") {
                    areas
                        .push(read_area_node(&mut iter).unwrap())
                        .expect("Unable to push last Area into results vec");
                }
            }
            Entry::EndNode => continue,
            Entry::Property { name: _, value: _ } => continue,
        }
    }
    Ok(areas)
}
