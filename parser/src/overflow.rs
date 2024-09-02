/// When the size of payload for a cell exceeds a certain threshold,
/// then only the first few bytes of the payload are stored on the b-tree page and
/// the balance is stored in a linked list of content overflow pages.
///
/// The first four bytes of each overflow page are a big-endian integer, which is
/// the page number of the next page in the chain, or zero for the final page in
/// the chain.
///
/// The fifth byte through the last usable byte are used to hold overflow content.
use crate::{slc, RecordValue, StdError, TextEncoding};

#[derive(Debug, Clone, PartialEq)]
pub struct OverflowPage {
    pub overflow_units: Vec<OverflowUnit>,
    pub next_page: u32,
    pub data: Vec<OverflowData>,
    pub unallocated: Option<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OverflowUnit {
    pub bytes_left: usize,
    pub overflow_type: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OverflowData {
    pub bytes: Vec<u8>,
    pub value: RecordValue,
}

impl OverflowPage {
    pub fn new(
        overflow_units: Vec<OverflowUnit>,
        next_page: u32,
        data: Vec<OverflowData>,
        unallocated: Option<Vec<u8>>,
    ) -> Self {
        Self {
            overflow_units,
            next_page,
            data,
            unallocated,
        }
    }
}

impl TryFrom<(TextEncoding, Vec<OverflowUnit>, &[u8])> for OverflowPage {
    type Error = StdError;

    fn try_from(value: (TextEncoding, Vec<OverflowUnit>, &[u8])) -> Result<Self, Self::Error> {
        let (text_encoding, mut overflow_units, buf) = value;

        let next_page_size = 4;
        let next_page = slc!(buf, 0, next_page_size, u32);
        let mut offset = next_page_size;

        // Overflow content goes from the fifth byte to the last usable byte
        // of the page. All overflow units follow each other sequentially.
        let mut data = vec![];
        let mut usable_size = buf.len() - next_page_size;
        while usable_size > 0 && !overflow_units.is_empty() {
            let unit = overflow_units.remove(0);
            let content_size = unit.bytes_left.min(usable_size);
            let bytes = buf[offset..offset + content_size].to_vec();
            let value = RecordValue::new(unit.overflow_type, text_encoding, &bytes)?;
            data.push(OverflowData { bytes, value });

            usable_size -= content_size;
            offset += content_size;

            let bytes_left = unit.bytes_left - content_size;
            if bytes_left > 0 {
                overflow_units.insert(0, OverflowUnit { bytes_left, ..unit });
            }
        }

        let unallocated = if usable_size == 0 {
            None
        } else {
            Some(buf[offset..].to_vec())
        };
        Ok(Self {
            overflow_units: overflow_units.to_vec(),
            next_page,
            data,
            unallocated,
        })
    }
}
