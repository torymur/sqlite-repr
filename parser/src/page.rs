/// BTree Page exploration
use crate::{
    cell::{TableInteriorCell, TableLeafCell},
    slc, Cell, DBHeader, Result, StdError, DB_HEADER_SIZE,
};
use std::rc::Rc;

const PAGE_HEADER_SIZE: usize = 12;
const PAGE_RIGHT_PTR_SIZE: usize = 4;
pub const CELL_PTR_SIZE: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PageHeaderType {
    InteriorIndex = 2,
    LeafIndex = 10,
    InteriorTable = 5,
    LeafTable = 13,
}

impl PageHeaderType {
    pub fn is_interior(&self) -> bool {
        matches!(self, Self::InteriorIndex | Self::InteriorTable)
    }
}

impl TryFrom<u8> for PageHeaderType {
    type Error = StdError;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            2 => Ok(PageHeaderType::InteriorIndex),
            5 => Ok(PageHeaderType::InteriorTable),
            10 => Ok(PageHeaderType::LeafIndex),
            13 => Ok(PageHeaderType::LeafTable),
            _ => Err(format!("Unexpected btree page type: {}", byte))?,
        }
    }
}

impl std::fmt::Display for PageHeaderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InteriorIndex => write!(f, "Interior Index"),
            Self::InteriorTable => write!(f, "Interior Table"),
            Self::LeafIndex => write!(f, "Leaf Index"),
            Self::LeafTable => write!(f, "Leaf Table"),
        }
    }
}

impl PageHeaderType {
    pub fn to_be_bytes(&self) -> [u8; 1] {
        match self {
            Self::InteriorIndex => 2_u8.to_be_bytes(),
            Self::InteriorTable => 5_u8.to_be_bytes(),
            Self::LeafIndex => 10_u8.to_be_bytes(),
            Self::LeafTable => 13_u8.to_be_bytes(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PageHeader {
    /// B-tree page type
    /// offset: 0, size: 1
    pub page_type: PageHeaderType,
    /// first free block on the page
    /// offset: 1, size: 2,
    pub free_block_offset: Option<u16>,
    /// number of cells on the page
    /// offset: 3, size: 2
    pub cell_num: u16,
    /// start of cell content area
    /// offset: 5, size: 2
    pub cell_start_offset: u32,
    /// number of fragmented free bytes within cell content area
    /// offset: 7, size: 1
    pub fragmented_free_bytes: u8,
    /// right most pointer, value exists only in the header of interior b-tree pages
    /// offset: 8, size: 4
    pub page_num: Option<u32>,
    /// size of page header
    pub size: usize,
}

impl PageHeader {
    pub fn new(
        page_type: PageHeaderType,
        free_block_offset: Option<u16>,
        cell_num: u16,
        cell_start_offset: u32,
        fragmented_free_bytes: u8,
        page_num: Option<u32>,
    ) -> Self {
        // For leaf pages page header is actually 8, not 12 bytes
        let size = if page_type.is_interior() {
            PAGE_HEADER_SIZE
        } else {
            PAGE_HEADER_SIZE - PAGE_RIGHT_PTR_SIZE
        };
        Self {
            page_type,
            free_block_offset,
            cell_num,
            cell_start_offset,
            fragmented_free_bytes,
            page_num,
            size,
        }
    }
}

impl TryFrom<&[u8]> for PageHeader {
    type Error = StdError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let page_type = PageHeaderType::try_from(slc!(buf, 0, 1, u8))?;
        Ok(PageHeader::new(
            page_type,
            // free_block_offset
            slc!(buf, 1, 2, u16).checked_sub(1).map(|x| x + 1),
            // cell_num
            slc!(buf, 3, 2, u16),
            // cell_start_offset
            slc!(buf, 5, 2, u16)
                .checked_sub(1)
                .map_or(65536, |x| (x + 1) as u32),
            // fragmented_free_bytes
            slc!(buf, 7, 1, u8),
            // page_num ptr (only if page type is interior node)
            page_type.is_interior().then_some(slc!(buf, 8, 4, u32)),
        ))
    }
}

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
    type Error = StdError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let mut array = vec![];
        for n in 0..buf.len() / CELL_PTR_SIZE {
            let offset = slc!(buf, n * CELL_PTR_SIZE, CELL_PTR_SIZE, u16)
                .checked_sub(1)
                .map_or(65536, |x| (x + 1) as u32);
            array.push(offset)
        }
        Ok(CellPointer::new(array))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Page {
    pub id: usize,
    pub db_header: Rc<DBHeader>,
    pub page_header: PageHeader,
    pub cell_pointer: CellPointer,
    pub unallocated: Vec<u8>,
    pub cells: Vec<Cell>,
}

impl Page {
    pub fn new(
        id: usize,
        db_header: Rc<DBHeader>,
        page_header: PageHeader,
        cell_pointer: CellPointer,
        unallocated: Vec<u8>,
        cells: Vec<Cell>,
    ) -> Self {
        Self {
            id,
            db_header,
            page_header,
            cell_pointer,
            unallocated,
            cells,
        }
    }
}

impl TryFrom<(Rc<DBHeader>, usize, &[u8])> for Page {
    type Error = StdError;

    fn try_from(value: (Rc<DBHeader>, usize, &[u8])) -> Result<Self, Self::Error> {
        let (db_header, page_num, buf) = value;

        // -- Create page header.
        let mut offset = match page_num {
            1 => DB_HEADER_SIZE,
            _ => 0,
        };
        let page_header = PageHeader::try_from(&buf[offset..offset + PAGE_HEADER_SIZE])?;
        offset += page_header.size;

        // -- Create cell pointer array.
        let ptrs_size = page_header.cell_num as usize * CELL_PTR_SIZE;
        let cell_pointer = CellPointer::try_from(&buf[offset..offset + ptrs_size])?;
        offset += ptrs_size;

        // -- Make an unallocated space.
        let unallocated_size = page_header.cell_start_offset as usize - offset;
        let unallocated = buf[offset..offset + unallocated_size]
            .iter()
            .map(|b| u8::from_be_bytes([*b; 1]))
            .collect::<Vec<u8>>();

        // -- Parse cells.
        let mut cells: Vec<Cell> = vec![];
        for ptr in &cell_pointer.array {
            let cell = match page_header.page_type {
                PageHeaderType::LeafTable => {
                    let params = (
                        db_header.text_encoding,
                        db_header.page_size,
                        db_header.reserved_page_space,
                        &buf[*ptr as usize..],
                    );
                    Cell::TableLeaf(TableLeafCell::try_from(params)?)
                }
                PageHeaderType::InteriorTable => {
                    Cell::TableInterior(TableInteriorCell::try_from(&buf[*ptr as usize..])?)
                }
                _ => unreachable!("Cell isn't yet implemented for this type."),
            };
            cells.push(cell)
        }

        Ok(Page::new(
            page_num,
            db_header,
            page_header,
            cell_pointer,
            unallocated,
            cells,
        ))
    }
}
