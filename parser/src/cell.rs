/// Within an interior b-tree page, each key and the pointer to its immediate left
/// are combined into a structure called a "cell". The right-most pointer is held separately.
///
/// A leaf b-tree page has no pointers, but it still uses the cell structure to hold
/// keys for index b-trees or keys and content for table b-trees.
/// Data is also contained in the cell.
use std::rc::Rc;

use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
    TableLeaf(TableLeafCell),
    TableInterior(TableInteriorCell),
    IndexLeaf(IndexLeafCell),
    IndexInterior(IndexInteriorCell),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CellOverflow {
    pub page: u32,
    pub units: Vec<OverflowUnit>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableLeafCell {
    pub payload_varint: Varint,
    pub rowid_varint: Varint,
    pub payload: Record,
    pub overflow: Option<CellOverflow>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableInteriorCell {
    pub left_page_number: u32,
    pub rowid_varint: Varint,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndexLeafCell {
    pub payload_varint: Varint,
    pub payload: Record,
    pub overflow: Option<CellOverflow>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndexInteriorCell {
    pub left_page_number: u32,
    pub payload_varint: Varint,
    pub payload: Record,
    pub overflow: Option<CellOverflow>,
}

impl Cell {
    pub fn new(
        page_type: PageHeaderType,
        db_header: Rc<DBHeader>,
        buf: &[u8],
    ) -> Result<Self, StdError> {
        match page_type {
            PageHeaderType::LeafTable => {
                let payload_varint = Varint::new(buf);
                let mut offset = payload_varint.bytes.len();

                let rowid_varint = Varint::new(&buf[offset..]);
                offset += rowid_varint.bytes.len();

                let max_payload = |u| u - 35;
                let (payload, overflow) =
                    Self::parse_payload(db_header, &max_payload, &payload_varint, buf, offset)?;

                Ok(Cell::TableLeaf(TableLeafCell {
                    payload_varint,
                    rowid_varint,
                    payload,
                    overflow,
                }))
            }
            PageHeaderType::InteriorTable => Ok(Cell::TableInterior(TableInteriorCell {
                left_page_number: slc!(buf, 0, 4, u32),
                rowid_varint: Varint::new(&buf[4..]),
            })),
            PageHeaderType::LeafIndex => {
                let payload_varint = Varint::new(buf);
                let offset = payload_varint.bytes.len();

                let max_payload = |u| ((u - 12) * 64 / 255) - 23;
                let (payload, overflow) =
                    Self::parse_payload(db_header, &max_payload, &payload_varint, buf, offset)?;

                Ok(Cell::IndexLeaf(IndexLeafCell {
                    payload_varint,
                    payload,
                    overflow,
                }))
            }
            PageHeaderType::InteriorIndex => {
                let left_page_number = slc!(buf, 0, 4, u32);
                let mut offset = 4;

                let payload_varint = Varint::new(&buf[offset..]);
                offset += payload_varint.bytes.len();

                let max_payload = |u| ((u - 12) * 64 / 255) - 23;
                let (payload, overflow) =
                    Self::parse_payload(db_header, &max_payload, &payload_varint, buf, offset)?;

                Ok(Cell::IndexInterior(IndexInteriorCell {
                    left_page_number,
                    payload_varint,
                    payload,
                    overflow,
                }))
            }
        }
    }

    fn parse_payload(
        db_header: Rc<DBHeader>,
        max_payload: &dyn Fn(u64) -> u64,
        payload_varint: &Varint,
        buf: &[u8],
        offset: usize,
    ) -> Result<(Record, Option<CellOverflow>), StdError> {
        let text_encoding = db_header.text_encoding;
        let page_size = db_header.page_size;
        let reserved_size = db_header.reserved_page_space;

        // -- Do the math to check for overflow.
        // Let:
        // - u: usable size of a database page,
        // - p: payload size,
        // - x: maximum amount of payload that can be stored directly on the page
        //      without spilling onto the overflow page,
        // - m: minimum amount of payload that must be stored on the btree page
        //      before spilling is allowed,
        //
        // u = page size - reserved space
        // - x = u - 35 (for leaf table page) OR
        // - x = ((u-12)*64/255)-23 (for interior/leaf index pages)
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
        let x = max_payload(u);
        let p = payload_varint.value as u64;
        let (overflow_page, payload_size, overflow_size) = if p <= x {
            (0, p as usize, 0_usize)
        } else {
            let m = ((u - 12) * 32 / 255) - 23;
            let k = m + ((p - m) % (u - 4));
            if k <= x {
                (
                    slc!(buf, offset + k as usize, 4, u32),
                    k as usize,
                    (p - k) as usize,
                )
            } else {
                (
                    slc!(buf, offset + m as usize, 4, u32),
                    m as usize,
                    (p - m) as usize,
                )
            }
        };

        // -- Parse cell payload.
        let from_buf = (text_encoding, &buf[offset..offset + payload_size]);
        let payload = Record::try_from(from_buf)?;

        // -- Overflow check.
        if overflow_size == 0 {
            return Ok((payload, None));
        }
        // If there is an overflow in one column, the rest of the columns after the
        // spilled one will be on the overflow pages as well, following it.
        let mut overflow_units = vec![];
        for (n, datatype) in payload.header.datatypes.iter().enumerate() {
            let code = datatype.value;
            let specified_size = RecordCode::size(code);
            let bytes_left = if n < payload.values.len() {
                // Detect overflow comparing sizes.
                let column = &payload.values[n];
                let column_size = column.bytes.as_ref().map_or(0, |b| b.len());
                if column_size == specified_size {
                    // No overflow for this column.
                    continue;
                }
                specified_size - column_size
            } else {
                // Means previous column was spilled, thus this one too.
                specified_size
            };
            overflow_units.push(OverflowUnit {
                overflow_type: code,
                bytes_left,
            });
        }
        let overflow = Some(CellOverflow {
            page: overflow_page,
            units: overflow_units,
        });
        Ok((payload, overflow))
    }
}
