/// Create BTree.
use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct BTreeNode {
    pub page: Page,
    pub page_num: usize,
    pub children: Option<Vec<BTreeNode>>,
    pub overflow: Option<Vec<OverflowNode>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OverflowNode {
    pub page: OverflowPage,
    pub page_num: usize,
}

impl BTreeNode {
    pub fn new(page_num: usize, reader: &Reader) -> Result<Self, StdError> {
        let page = reader.get_btree_page(page_num)?;
        let mut children = vec![];
        let mut overflow = vec![];

        let mut extend_overflow = |cell_overflow: &Option<CellOverflow>| {
            if let Some(o) = cell_overflow {
                let res = Self::follow_overflow(vec![], o.units.to_vec(), o.page as usize, reader);
                if let Ok(res) = res {
                    let mut page_nums =
                        res.iter().map(|o| o.next_page as usize).collect::<Vec<_>>();

                    // Transform list of 'next_page' numbers into page numbers.
                    // Last next page number is 0 to mark the end of the linked list.
                    page_nums.insert(0, o.page as usize);
                    page_nums.pop();

                    let overflow_list = res
                        .into_iter()
                        .zip(page_nums.into_iter())
                        .map(|(o, n)| OverflowNode {
                            page: o,
                            page_num: n,
                        })
                        .collect::<Vec<OverflowNode>>();
                    overflow.extend(overflow_list);
                }
            }
        };

        for outer_cell in page.cells.iter() {
            match outer_cell {
                Cell::TableInterior(cell) => {
                    children.push(BTreeNode::new(cell.left_page_number as usize, reader)?);
                }
                Cell::TableLeaf(cell) => {
                    extend_overflow(&cell.overflow);
                }
                Cell::IndexInterior(cell) => {
                    children.push(BTreeNode::new(cell.left_page_number as usize, reader)?);
                    extend_overflow(&cell.overflow);
                }
                Cell::IndexLeaf(cell) => {
                    extend_overflow(&cell.overflow);
                }
            };
        }
        if page.page_header.page_type.is_interior() {
            // Don't forget the right-most pointer, which is in the page header.
            // If it's interior page, then page_num is Some by design.
            children.push(BTreeNode::new(
                page.page_header.page_num.unwrap() as usize,
                reader,
            )?);
        };

        Ok(Self {
            page,
            page_num,
            children: (!children.is_empty()).then_some(children),
            overflow: (!overflow.is_empty()).then_some(overflow),
        })
    }

    fn follow_overflow(
        mut opages: Vec<OverflowPage>,
        overflow_units: Vec<OverflowUnit>,
        next_page: usize,
        reader: &Reader,
    ) -> Result<Vec<OverflowPage>, StdError> {
        let opage = reader.get_overflow_page(overflow_units, next_page)?;
        let units = opage.overflow_units.to_vec();
        let next_page = opage.next_page;
        opages.push(opage);
        match next_page {
            0 => Ok(opages),
            n => Self::follow_overflow(opages, units, n as usize, reader),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BTree {
    pub ttype: String,
    pub name: String,
    pub root: BTreeNode,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Schema {
    Type = 0,
    Name = 1,
    TableName = 2,
    RootPage = 3,
    SQL = 4,
}

impl BTree {
    pub fn new(cell: &TableLeafCell, reader: &Reader) -> Result<Self, StdError> {
        match &cell.overflow {
            Some(overflow) => {
                let payload = Self::follow_overflow(
                    cell.payload.values.to_vec(),
                    overflow.units.to_vec(),
                    overflow.page as usize,
                    reader,
                )?;
                Self::parse_tree(&payload, reader)
            }
            None => Self::parse_tree(&cell.payload.values, reader),
        }
    }

    fn follow_overflow(
        mut payload: Vec<RecordValue>,
        overflow_units: Vec<OverflowUnit>,
        next_page: usize,
        reader: &Reader,
    ) -> Result<Vec<RecordValue>, StdError> {
        // We need to merge last of previous with the first of overflow value and
        // add values in between to payload.
        /*
         *  Btree page       Overflow page 1                   Overflow page 2
         *  +-----------+    +----------------------------+    +----------+
         *  |  field1   | -> |  field1 | field2 | field3  | -> |  field3  |
         *  +-----------+    +----------------------------+    +----------+
         *     ↓                ↓        |        ↓               ↓
         *     +----------------+        ↓        +---------------+
         *            merge          as it is           merge
         */
        let opage = reader.get_overflow_page(overflow_units, next_page)?;

        let mut overflow = opage.data.to_vec();
        let last_payload = payload.remove(payload.len() - 1);
        let first_overflow = overflow.remove(0);
        match last_payload.merge(first_overflow.value) {
            Some(value) => payload.push(value),
            None => unreachable!("Attempt to merge the unexpected Record types."),
        };
        payload.extend(overflow.into_iter().map(|v| v.value));

        match opage.next_page {
            0 => Ok(payload),
            n => Self::follow_overflow(payload, opage.overflow_units, n as usize, reader),
        }
    }

    fn parse_tree(values: &[RecordValue], reader: &Reader) -> Result<Self, StdError> {
        let tname = match &values[Schema::Name as usize].value {
            RecordType::Text(v) => v.as_ref().map_or("", |vv| vv),
            _ => unreachable!("Unknown type for table schema name."),
        };
        let ttype = match &values[Schema::Type as usize].value {
            RecordType::Text(v) => v.as_ref().map_or("", |vv| vv),
            _ => unreachable!("Unknown type for table schema type."),
        };
        let tpage = match values[Schema::RootPage as usize].value {
            RecordType::I8(v) => v as usize,
            RecordType::I16(v) => v as usize,
            RecordType::I24(v) | RecordType::I32(v) => v as usize,
            RecordType::I48(v) | RecordType::I64(v) => v as usize,
            _ => unreachable!("Unknown type for table schema root page."),
        };
        Ok(Self {
            ttype: ttype.to_string(),
            name: tname.to_string(),
            root: BTreeNode::new(tpage, reader)?,
        })
    }
}
