//! BTree Page exploration
use crate::{slc, CellPointer, CELL_PTR_SIZE};

const PAGE_HEADER_SIZE: usize = 12;
const PAGE_RIGHT_PTR_SIZE: usize = 4;

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
    type Error = String;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            2 => Ok(PageHeaderType::InteriorIndex),
            5 => Ok(PageHeaderType::InteriorTable),
            10 => Ok(PageHeaderType::LeafIndex),
            13 => Ok(PageHeaderType::LeafTable),
            _ => Err(format!("Unexpected btree page type: {}", byte)),
        }
    }
}

impl std::fmt::Display for PageHeaderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InteriorIndex => write!(f, "Interior Index Page"),
            Self::InteriorTable => write!(f, "Interior Table Page"),
            Self::LeafIndex => write!(f, "Leaf Index Page"),
            Self::LeafTable => write!(f, "Leaf Table Page"),
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
        Self {
            page_type,
            free_block_offset,
            cell_num,
            cell_start_offset,
            fragmented_free_bytes,
            page_num,
        }
    }
}

impl TryFrom<&[u8]> for PageHeader {
    type Error = Box<dyn std::error::Error>;

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
pub struct Page {
    pub page_header: PageHeader,
    pub cell_pointer: CellPointer,
    pub cell_pointer_offset: usize,
}

impl Page {
    pub fn new(
        page_header: PageHeader,
        cell_pointer: CellPointer,
        cell_pointer_offset: usize,
    ) -> Self {
        Self {
            page_header,
            cell_pointer,
            cell_pointer_offset,
        }
    }
}

impl TryFrom<&Vec<u8>> for Page {
    type Error = Box<dyn std::error::Error>;

    fn try_from(buf: &Vec<u8>) -> Result<Self, Self::Error> {
        let page_header = PageHeader::try_from(&buf[0..PAGE_HEADER_SIZE])?;

        // For leaf pages page header is actually 8, not 12 bytes
        let ptr_offset = if page_header.page_type.is_interior() {
            PAGE_HEADER_SIZE
        } else {
            PAGE_HEADER_SIZE - PAGE_RIGHT_PTR_SIZE
        };
        let ptrs_size = page_header.cell_num as usize * CELL_PTR_SIZE;
        let cell_pointer = CellPointer::try_from(&buf[ptr_offset..ptr_offset + ptrs_size])?;

        Ok(Page::new(page_header, cell_pointer, ptr_offset))
    }
}
