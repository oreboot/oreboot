use fdt::node::FdtNode;

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

pub fn find_fdt(data: &[u8]) -> Result<fdt::Fdt<'_>, fdt::FdtError> {
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

#[test]
fn iterator() {
    let data = include_bytes!("testdata/test.dtb");
    let fdt = fdt::Fdt::new(data).unwrap();
    let it = &mut fdt.find_all_nodes("/flash-info/areas");
    let areas = FdtIterator::new(it);
    let count = areas.map(|e| e.children().count()).sum::<usize>();
    let expected = 8;
    assert_eq!(count, expected, "Expected {expected} areas, got {count}");
}
