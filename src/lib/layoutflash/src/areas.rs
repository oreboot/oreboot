use fdt::{node::FdtNode, Fdt, FdtError};

pub struct FdtIterator<'a, 'b> {
    iter: &'a mut dyn Iterator<Item = FdtNode<'b, 'b>>,
}

impl<'a, 'b> FdtIterator<'a, 'b> {
    pub fn new(iter: &'a mut dyn Iterator<Item = FdtNode<'b, 'b>>) -> FdtIterator<'a, 'b> {
        FdtIterator { iter }
    }
}

impl<'a, 'b> Iterator for FdtIterator<'a, 'b> {
    type Item = FdtNode<'a, 'b>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

// NOTE: we don't use u32. At the rate that SPI flash is expanding, we're going to see
// 5B addressing soon I bet. The size limitation should be a function of the destination,
// not this program. This problem should just stupidly arrange things.
#[derive(Clone, Debug, PartialEq)]
pub struct Area<'a> {
    pub name: &'a str,
    pub stage: Option<&'a str>,
    pub offset: Option<usize>,
    pub size: usize,
    pub file: Option<&'a str>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AreaError {
    NotFound,
    NoSize,
}

pub fn get_opt_usize(node: &FdtNode, prop: &str) -> Option<usize> {
    node.property(prop).map_or_else(|| None, |e| e.as_usize())
}

impl<'a, 'b> Area<'a> {
    pub fn from_node(node: FdtNode<'a, 'b>) -> Result<Self, AreaError> {
        let Some(size) = get_opt_usize(&node, "size") else {
            return Err(AreaError::NoSize);
        };

        Ok(Area {
            name: node.name,
            stage: node.property("stage").map_or_else(|| None, |e| e.as_str()),
            file: node.property("file").map_or_else(|| None, |e| e.as_str()),
            offset: get_opt_usize(&node, "offset"),
            size,
        })
    }
}

pub const AREAS_PATH: &str = "/flash-info/areas";

pub fn get_stage<'a>(fdt: &'a Fdt, stage: &'a str) -> Result<Area<'a>, AreaError> {
    let mut offset = 0;
    for node in fdt.find_all_nodes(AREAS_PATH) {
        for c in node.children() {
            if let Some(s) = c.property("stage") {
                if s.as_str() == Some(stage) {
                    match Area::from_node(c) {
                        Ok(mut a) => {
                            if a.offset.is_none() {
                                a.offset = Some(offset);
                            }
                            return Ok(a);
                        }
                        Err(e) => return Err(e),
                    }
                }
            }
            if let Some(o) = c.property("offset") {
                offset = o.as_usize().unwrap();
            }
            offset += c.property("size").unwrap().as_usize().unwrap();
        }
    }
    Err(AreaError::NotFound)
}

pub fn find_fdt(data: &[u8]) -> Result<fdt::Fdt<'_>, FdtError> {
    // The informal standard is that the fdt must be on a 0x1000
    // boundary. It is a fine line between too coarse a boundary
    // and falling into an false positive.
    // yuck. Make a better iterator.
    for pos in (0..data.len() - 0x1000).step_by(0x1000) {
        match Fdt::new(&data[pos..]) {
            Err(_) => {}
            Ok(fdt) => {
                return Ok(fdt);
            }
        };
    }

    Err(FdtError::BadMagic)
}

#[cfg(test)]
pub const TEST_DTB: &[u8] = include_bytes!("testdata/test.dtb");

#[cfg(test)]
pub const IMAGE_FIXTURE: &[u8] = include_bytes!("testdata/test.out");

#[test]
fn iterator() {
    let data = include_bytes!("testdata/test.dtb");
    let fdt = fdt::Fdt::new(data).unwrap();
    let it = &mut fdt.find_all_nodes("/flash-info/areas");
    let areas = FdtIterator::new(it);
    let count = areas.map(|e| e.children().count()).sum::<usize>();
    let expected = 4;
    assert_eq!(count, expected, "Expected {expected} areas, got {count}");
}

#[test]
fn find_existing_stage() {
    let fdt = Fdt::new(TEST_DTB).unwrap();
    let stage = get_stage(&fdt, "main");
    let expected = Area {
        name: "area@2",
        stage: Some("main"),
        size: 0x100000,
        offset: Some(0x100000),
        file: None,
    };
    assert_eq!(stage, Ok(expected));
}

#[test]
fn error_on_non_existent_stage() {
    let fdt = Fdt::new(TEST_DTB).unwrap();
    let stage = get_stage(&fdt, "nostage");
    assert_eq!(stage, Err(AreaError::NotFound));
}

#[test]
fn fdt_from_slice() {
    let fdt = find_fdt(IMAGE_FIXTURE);
    assert!(fdt.is_ok());
}
