/// Within an interior b-tree page, each key and the pointer to its immediate left
/// are combined into a structure called a "cell". The right-most pointer is held separately.
///
/// A leaf b-tree page has no pointers, but it still uses the cell structure to hold
/// keys for index b-trees or keys and content for table b-trees.
/// Data is also contained in the cell.
use crate::{slc, Record, StdError, TextEncoding, Varint};

#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub payload_varint: Varint,
    pub rowid_varint: Varint,
    pub payload: Option<Record>,
    pub overflow_page: Option<u32>,
}

impl TryFrom<(TextEncoding, u64, u8, &[u8])> for Cell {
    type Error = StdError;

    fn try_from(value: (TextEncoding, u64, u8, &[u8])) -> Result<Self, Self::Error> {
        let (text_encoding, page_size, reserved_size, buf) = value;
        let payload_varint = Varint::new(buf);
        let mut offset = payload_varint.bytes.len();
        let rowid_varint = Varint::new(&buf[offset..]);
        let payload = if payload_varint.value > 0 {
            offset += rowid_varint.bytes.len();
            let from_buf = (text_encoding, &buf[offset..]);
            Some(Record::try_from(from_buf)?)
        } else {
            None
        };

        // Let:
        // - u: usable size of a database page,
        // - p: payload size,
        // - x: maximum amount of payload that can be stored directly on the page
        //      without spilling onto the overflow page,
        // - m: minimum amount of payload that must be stored on the btree page
        //      before spilling is allowed,
        //
        // u = page size - reserved space
        // x = u - 35
        //
        // if p <= x {
        //      entire payload stored on the btree leaf page
        // } else {
        //      m = ((u-12)*32/255)-23
        //      k = m+((p-m)%(u-4))
        //      if k <= x {
        //          - first k-bytes of p are stored on the btree page,
        //          - p-k bytes are stored on overflow page
        //      } else {
        //          - first m-bytes of p are stored on the btree page,
        //          - p-m bytes are stored on overflow page
        //      }
        // }
        let u = page_size - reserved_size as u64;
        let x = u - 35;
        let p = payload_varint.value as u64;
        let overflow_page = if p <= x {
            None
        } else {
            let m = ((u - 12) * 32 / 255) - 23;
            let k = m + ((p - m) % (u - 4));
            if k <= x {
                offset += k as usize;
                Some(slc!(buf, offset, 4, u32))
            } else {
                offset += m as usize;
                Some(slc!(buf, offset, 4, u32))
            }
        };

        Ok(Cell {
            payload_varint,
            rowid_varint,
            payload,
            overflow_page,
        })
    }
}
