/// Within an interior b-tree page, each key and the pointer to its immediate left
/// are combined into a structure called a "cell". The right-most pointer is held separately.
///
/// A leaf b-tree page has no pointers, but it still uses the cell structure to hold
/// keys for index b-trees or keys and content for table b-trees.
/// Data is also contained in the cell.
use crate::{Record, StdError, TextEncoding, Varint};

#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub payload_varint: Varint,
    pub rowid_varint: Varint,
    pub payload: Option<Record>,
}

impl TryFrom<(TextEncoding, &[u8])> for Cell {
    type Error = StdError;

    fn try_from(value: (TextEncoding, &[u8])) -> Result<Self, Self::Error> {
        let (text_encoding, buf) = value;
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
        Ok(Cell {
            payload_varint,
            rowid_varint,
            payload,
        })
    }
}
