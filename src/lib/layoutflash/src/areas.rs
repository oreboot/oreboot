use core::option::Option;
use core::result::Result::{self, Err, Ok};
pub use fdt::{node::FdtNode, Fdt, FdtError};

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
    pub offset: Option<usize>,
    pub size: usize,
    pub file: Option<&'a str>,
}

pub fn find_fdt(data: &[u8]) -> Result<Fdt, FdtError> {
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

// create_areas: create the areas from the fdt. This is unnecessarily messy,
// as we want to use this same code in std and no_std.
pub fn create_areas<'a>(fdt: &'a Fdt<'a>, areas: &'a mut [Area<'a>]) -> &'a mut [Area<'a>] {
    // Assemble the bits of the fdt we care about into Areas.

    let mut i = 0;
    for node in fdt.find_all_nodes("/flash-info/areas") {
        for child in node.children() {
            let mut a: Area<'a> = Area {
                name: child.name,
                offset: None,
                size: 0,
                file: None,
            };
            for p in child.properties() {
                // There can be all kinds of properties in a node.
                // we only care about file, size, and offset.
                // Not that we remove any, just that those relate
                // to data we put in the image.

                match p.name {
                    "file" => {
                        a.file = Some(p.as_str().expect("MISSING NAME"));
                    }
                    "offset" => {
                        a.offset = Some(p.as_usize().unwrap());
                    }
                    "size" => {
                        a.size = p.as_usize().unwrap();
                    }
                    _ => {}
                }
            }
            areas[i] = a;
            i += 1;
        }
    }

    areas
}

#[test]
fn read_create() {
    static DATA: &'static [u8] = include_bytes!("testdata/test.dtb");
    let fdt = Fdt::new(&DATA).unwrap();
    let it = &mut fdt.find_all_nodes("/flash-info/areas");
    let a = FdtIterator::new(it);
    let mut i = 0;
    for aa in a {
        for c in aa.children() {
            i += 1;
            for _p in c.properties() {}
        }
    }
    if i != 8 {
        panic!("Supposed to have 8 areas, but found {i}");
    }
}
//}
