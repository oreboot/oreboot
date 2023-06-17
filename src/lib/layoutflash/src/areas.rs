use core::option::Option;
use core::result::Result;
use core::result::Result::Err;
use core::result::Result::Ok;
use fdt::node::FdtNode;

pub
struct Areas<'a> {
	nodes: dyn Iterator<Item = FdtNode<'a, 'a>>,
}

pub fn areas(fdt: fdt::Fdt) -> Option<Areas> {
        if let Some(i) = fdt.find_all_nodes("/flash-info/areas"){
	return Areas {
	nodes: i,
	}
	}
None
}
impl Iterator for Areas<'_> {
  fn next(&mut self) -> Option<Area> {
	let child = self.nodes.next();
            let mut a: Area = Area {
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
        }
	a
}


// NOTE: we don't use u32. At the rate that SPI flash is expanding, we're going to see
// 5B addressing soon I bet. The size limitation should be a function of the destination,
// not this program. This problem should just stupidly arrange things.
#[derive(Clone,Debug, PartialEq)]
pub struct Area<'a> {
    pub name: &'a str,
    pub offset: Option<usize>,
    pub size: usize,
    pub file: Option<&'a str>,
}

pub fn find_fdt<'a>(data: &'a [u8]) -> Result<fdt::Fdt<'a>, fdt::FdtError> {
    // The informal standard is that the fdt must be on a 0x1000
    // boundary. It is a fine line between too coarse a boundary
    // and falling into an false positive.
    // yuck. Make a better iterator.
    for pos in (0..data.len() - 0x1000).step_by(0x1000) {
        match fdt::Fdt::new(&data[pos..]) {
            Err(_) => {}
            Ok(fdt) => {
                return Ok(fdt);
            }
        };
    }

    Err(fdt::FdtError::BadMagic)
}

// create_areas: create the areas from the fdt. This is unnecessarily messy,
// as we want to use this same code in std and no_std.
pub fn create_areas<'a>(fdt: &'a fdt::Fdt<'a>, areas: &'a mut [Area<'a>]) -> &'a mut [Area<'a>] {
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
