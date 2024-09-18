/// A database file might contain one or more pages that are not in active use, for example,
/// when information is deleted from the database.
/// Unused pages are stored on the freelist and are reused when additional pages are required.

/// The freelist is organized as a linked list of freelist trunk pages with each trunk page
/// containing page numbers for zero or more freelist leaf pages.

/// A freelist trunk page consists of an array of 4-byte big-endian integers.
/// The size of the array is as many integers as will fit in the usable space of a page.
///
/// The first integer on a freelist trunk page is the page number of the next freelist trunk page
/// in the list or zero if this is the last freelist trunk page.
/// The second integer on a freelist trunk page is the number of leaf page pointers to follow.
/// Call the second integer on a freelist trunk page L. If L >= 0 then integers with array
/// indexes between 2 and L+1 inclusive contain page numbers for freelist leaf pages.

/// Freelist leaf pages contain no information.
/// SQLite avoids reading or writing freelist leaf pages in order to reduce disk I/O.
use crate::{slc, StdError};

#[derive(Debug, Clone, PartialEq)]
pub struct TrunkFreelistPage {
    pub next_page: u32,
    pub leaf_page_amount: u32,
    pub leaf_page_numbers: Option<Vec<u32>>,
    pub unallocated: Option<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LeafFreelistPage {
    pub unallocated: Vec<u8>,
}

impl TryFrom<&[u8]> for TrunkFreelistPage {
    type Error = StdError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let size = 4;
        let mut offset = 0;
        let next_page = slc!(buf, offset, size, u32);
        offset += size;

        let leaf_page_amount = slc!(buf, offset, size, u32);
        offset += size;

        let leaf_page_numbers = if leaf_page_amount > 0 {
            let mut numbers = vec![];
            for _ in 0..leaf_page_amount {
                numbers.push(slc!(buf, offset, size, u32));
                offset += size;
            }
            Some(numbers)
        } else {
            None
        };

        let unallocated = if offset < buf.len() - 1 {
            Some(buf[offset..].to_vec())
        } else {
            None
        };

        Ok(Self {
            next_page,
            leaf_page_amount,
            leaf_page_numbers,
            unallocated,
        })
    }
}

impl TryFrom<&[u8]> for LeafFreelistPage {
    type Error = StdError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self {
            unallocated: buf[..].to_vec(),
        })
    }
}
