use crate::slc;
pub const CELL_PTR_SIZE: usize = 2;

#[derive(Debug, Clone, PartialEq)]
pub struct CellPointer {
    /// Let K be the number of the cells on the btree, then
    /// cell array are K*2 bytes integer to the cell contents.
    /// 0 to handle 65536-byte page size w/o cells & no reserved space.
    pub array: Vec<u32>,
}

impl CellPointer {
    pub fn new(array: Vec<u32>) -> Self {
        Self { array }
    }
}

impl TryFrom<&[u8]> for CellPointer {
    type Error = Box<dyn std::error::Error>;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let mut array = vec![];
        for n in 0..buf.len() / CELL_PTR_SIZE {
            let offset = slc!(
                buf,
                n * CELL_PTR_SIZE,
                n * CELL_PTR_SIZE + CELL_PTR_SIZE,
                u16
            )
            .checked_sub(1)
            .map_or(65536, |x| (x + 1) as u32);
            array.push(offset)
        }
        Ok(CellPointer::new(array))
    }
}
