use std::rc::Rc;

use parser::cell::{TableInteriorCell, TableLeafCell};
use parser::*;

use crate::header::DBHeaderPart;
use crate::{Field, PageView, Part, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct BtreePageElement {
    pub id: usize,
    pub page: Rc<Page>,
    pub size: usize,
}

impl BtreePageElement {
    pub fn new(page: Page, size: usize) -> Self {
        Self {
            id: page.id,
            page: Rc::new(page),
            size,
        }
    }
}

impl PageView for BtreePageElement {
    fn id(&self) -> usize {
        self.id
    }

    fn size(&self) -> usize {
        self.size
    }

    fn label(&self) -> String {
        let sign = match self.page.page_header.page_type {
            PageHeaderType::LeafTable => "ꕤ ",
            PageHeaderType::InteriorTable => "☰ ",
            _ => "",
        };
        format!("{} {}", sign, self.page.page_header.page_type)
    }

    fn desc(&self) -> &'static str {
        if self.id == 1 {
            "The 100-byte database file header is found only on Page 1, meaning that root page has 100 fewer bytes of storage space available. It's always a table b-tree page: interior or leaf. Page 1 is the root page of a table b-tree, that holds a special table named 'sqlite_schema'. This b-tree is known as the 'schema table' since it stores the complete database schema."
        } else {
            "A b-tree page is either an interior page or a leaf page. A b-tree page is either a table b-tree page or an index b-tree page. All pages within each complete b-tree are of the same type: either table or index. A leaf page contains keys and in the case of a table b-tree each key has associated data. An interior page contains K keys together with K+1 pointers to child b-tree pages. A'pointer' in an interior b-tree page is just the 32-bit unsigned integer page number of the child page."
        }
    }

    fn parts(&self) -> Vec<Rc<dyn Part>> {
        let mut parts: Vec<Rc<dyn Part>> = vec![
            Rc::new(PageHeaderPart::new(self.page.clone())),
            Rc::new(CellPointerPart::new(self.page.clone())),
            Rc::new(UnallocatedPart::new(self.page.clone())),
        ];

        // Generate CellPart(s).
        let mut cells = self.page.cells.clone();
        cells.reverse();
        let mut offsets = self.page.cell_pointer.array.clone();
        offsets.reverse();
        let mut cell_parts: Vec<Rc<dyn Part>> = vec![];
        for (n, cell) in cells.iter().enumerate() {
            let offset = offsets[n] as usize;
            cell_parts.push(Rc::new(CellPart {
                cell: cell.clone(),
                offset,
                id: n + 1,
            }))
        }
        parts.extend(cell_parts);

        // Consider database header to go first.
        if self.id == 1 {
            parts.insert(
                0,
                Rc::new(DBHeaderPart {
                    header: self.page.db_header.clone(),
                }),
            )
        };
        parts
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PageHeaderPart {
    pub header: Rc<PageHeader>,
    pub offset: usize,
}

impl PageHeaderPart {
    pub fn new(page: Rc<Page>) -> Self {
        let header = Rc::new(page.page_header.clone());
        let offset = if page.id == 1 { DB_HEADER_SIZE } else { 0 };
        Self { header, offset }
    }
}

impl Part for PageHeaderPart {
    fn label(&self) -> String {
        "B-tree Page Header".to_string()
    }

    fn desc(&self) -> &'static str {
        "The b-tree page header is 8 bytes in size for leaf pages and 12 bytes for interior pages. All multibyte values in the page header are big-endian.The cell pointer array of a b-tree page immediately follows the b-tree page header."
    }

    fn color(&self) -> String {
        "green".to_string()
    }

    fn fields(&self) -> Vec<Field> {
        let mut fields = vec![
            Field::new(
                "B-tree page type. 2 (0x02) means the page is an interior index b-tree page, 5 (0x05): interior table b-tree page, 10 (0x0a): leaf index b-tree page, 13 (0x0d): leaf table b-tree page. Any other value for the b-tree page type is an error.",
                self.offset,
                1,
                Value::PageType(self.header.page_type),
                ""
            ),
            Field::new(
                "Start of the first freeblock on the page or zero if there are no freeblocks. A freeblock is a structure used to identify unallocated space within a b-tree page. Freeblocks are organized as a chain. The first 2 bytes of a freeblock are a big-endian integer which is the offset in the b-tree page of the next freeblock in the chain, or zero if the freeblock is the last on the chain. The third and fourth bytes of each freeblock form a big-endian integer which is the size of the freeblock in bytes, including the 4-byte header. Freeblocks are always connected in order of increasing offset. The second field of the b-tree page header is the offset of the first freeblock, or zero if there are no freeblocks on the page. In a well-formed b-tree page, there will always be at least one cell before the first freeblock.A freeblock requires at least 4 bytes of space.",
                self.offset + 1,
                2,
                {
                    match self.header.free_block_offset {
                        None => Value::U16(0),
                        Some(v) => Value::U16(v),
                    }
                },
                ""
            ),
            Field::new(
                "Number of cells on the page. A page might contain no cells, which is only possible for a root page of a table that contains no rows. SQLite strives to place cells as far toward the end of the b-tree page as it can, in order to leave space for future growth of the cell pointer array.",
                self.offset + 3,
                2,
                Value::U16(self.header.cell_num),
                ""
            ),
            Field::new(
                "Start of the cell content area. A zero value for this integer is interpreted as 65536. SQLite strives to place cells as far toward the end of the b-tree page as it can, in order to leave space for future growth of the cell pointer array. If a page contains no cells, then the offset to the cell content area will equal the page size minus the bytes of reserved space.",
                self.offset + 5,
                2,
                Value::CellStartOffset(self.header.cell_start_offset),
                ""
            ),
            Field::new(
                "The number of fragmented free bytes within the cell content area. If there is an isolated group of 1, 2, or 3 unused bytes within the cell content area, those bytes comprise a fragment. The total number of bytes in all fragments is stored in the fifth field of the b-tree page header. In a well-formed b-tree page, the total number of bytes in fragments may not exceed 60. The total amount of free space on a b-tree page consists of the size of the unallocated region plus the total size of all freeblocks plus the number of fragmented free bytes. SQLite may from time to time reorganize a b-tree page so that there are no freeblocks or fragment bytes, all unused bytes are contained in the unallocated space region, and all cells are packed tightly at the end of the page. This is called 'defragmenting' the b-tree page.",
                self.offset + 7,
                1,
                Value::U8(self.header.fragmented_free_bytes),
                ""
            ),
        ];
        match self.header.page_num {
            None => fields,
            Some(v) => {
                let page_num = Field::new(
                    "The right-most pointer. This value appears in the header of interior b-tree pages only and is omitted from all other pages.",
                    self.offset + 8,
                    4,
                    Value::U32(v),
                    ""
                );
                fields.push(page_num);
                fields
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CellPointerPart {
    pub cell_ptrs: Rc<CellPointer>,
    pub offset: usize,
}

impl CellPointerPart {
    pub fn new(page: Rc<Page>) -> Self {
        let cell_ptrs = Rc::new(page.cell_pointer.clone());
        let mut offset = if page.id == 1 { DB_HEADER_SIZE } else { 0 };
        offset += page.page_header.size;
        Self { cell_ptrs, offset }
    }
}

impl Part for CellPointerPart {
    fn label(&self) -> String {
        "Cell pointer array".to_string()
    }

    fn desc(&self) -> &'static str {
        "The cell pointer array of a b-tree page immediately follows the b-tree page header. Let K be the number of cells on the btree. The cell pointer array consists of K 2-byte integer offsets to the cell contents. The cell pointers are arranged in key order with left-most cell (the cell with the smallest key) first and the right-most cell (the cell with the largest key) last."
    }

    fn color(&self) -> String {
        "orange".to_string()
    }

    fn fields(&self) -> Vec<Field> {
        let mut offset = self.offset;
        self.cell_ptrs.array.iter().map(|ptr| {
            let field = Field::new(
                "2-byte integer offsets to the cell contents. Cell content is stored in the cell content region of the b-tree page. SQLite strives to place cells as far toward the end of the b-tree page as it can, in order to leave space for future growth of the cell pointer array. If a page contains no cells (which is only possible for a root page of a table that contains no rows) then the offset to the cell content area will equal the page size minus the bytes of reserved space. If the database uses a 65536-byte page size and the reserved space is zero (the usual value for reserved space) then the cell content offset of an empty page wants to be 65536. However, that integer is too large to be stored in a 2-byte unsigned integer, so a value of 0 is used in its place.",
                offset,
                CELL_PTR_SIZE,
                Value::CellStartOffset(*ptr),
                ""
            );
            offset += CELL_PTR_SIZE;
            field
        }).collect::<Vec<Field>>()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnallocatedPart {
    pub unallocated: Vec<u8>,
    pub offset: usize,
}

impl UnallocatedPart {
    pub fn new(page: Rc<Page>) -> Self {
        let unallocated = page.unallocated.clone();
        let mut offset = if page.id == 1 { DB_HEADER_SIZE } else { 0 };
        offset += page.page_header.size + page.page_header.cell_num as usize * CELL_PTR_SIZE;
        Self {
            unallocated,
            offset,
        }
    }
}

impl Part for UnallocatedPart {
    fn label(&self) -> String {
        "Unallocated space".to_string()
    }

    fn desc(&self) -> &'static str {
        "The area in between the last cell pointer array entry and the beginning of the first cell is the unallocated region. SQLite strives to place cells as far toward the end of the b-tree page as it can, in order to leave space for future growth of the cell pointer array."
    }

    fn color(&self) -> String {
        "green".to_string()
    }

    fn fields(&self) -> Vec<Field> {
        vec![Field::new(
            "The total amount of free space on a b-tree page consists of the size of the unallocated region plus the total size of all freeblocks plus the number of fragmented free bytes. SQLite may from time to time reorganize a b-tree page so that there are no freeblocks or fragment bytes, all unused bytes are contained in the unallocated space region, and all cells are packed tightly at the end of the page. This is called 'defragmenting' the b-tree page.",
            self.offset,
            self.unallocated.len(),
            Value::Unallocated(self.unallocated.clone().into_boxed_slice()),
            ""
        )]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CellPart {
    pub id: usize,
    pub cell: Cell,
    pub offset: usize,
}

impl CellPart {
    fn table_leaf_fields(&self, cell: &TableLeafCell) -> Vec<Field> {
        let rowid_offset = self.offset + cell.payload_varint.bytes.len();
        let cell_header_style = "bg-slate-300";
        let mut fields = vec![
            Field::new(
                "Cell Header. A varint, which is the total number of bytes of payload, including any overflow.",
                self.offset,
                cell.payload_varint.bytes.len(),
                Value::Varint(cell.payload_varint.clone()),
                cell_header_style,
            ),
            Field::new(
                "Cell Header. A varint which is the integer key, a.k.a. 'rowid'.",
                rowid_offset,
                cell.rowid_varint.bytes.len(),
                Value::Varint(cell.rowid_varint.clone()),
                cell_header_style,
            ),
        ];

        let payload = &cell.payload;
        let payload_offset = rowid_offset + cell.rowid_varint.bytes.len();
        let record_header_style = "bg-slate-330";
        fields.push(
            Field::new(
                "Cell Payload: Record Header. First value is varint, which determines total number of bytes in the header, including the size of varint.",
                payload_offset,
                payload.header.size.bytes.len(),
                Value::Varint(payload.header.size.clone()),
                record_header_style,
            )
        );
        let mut offset = payload_offset + payload.header.size.bytes.len();
        for datatype in &payload.header.datatypes {
            fields.push(
                Field::new(
                    "Cell Payload: Record Header. Second value(s) are one or more additional varints, one per column, which determine the datatype of each column ('serial types').",
                    offset,
                    datatype.bytes.len(),
                    Value::Varint(datatype.clone()),
                    record_header_style,
                )
            );
            offset += datatype.bytes.len();
        }

        for record in &payload.values {
            let size = record.bytes.as_ref().map_or(0, |b| b.len());
            let style = if size == 0 {
                "pattern-vertical-lines pattern-white pattern-bg-slate-200 pattern-size-1 pattern-opacity-60 bg-slate-360"
            } else {
                "bg-slate-360"
            };
            fields.push(Field::new(
                "Cell Payload: Record Payload. The values for each column in the record immediately follow the header. For serial types 0, 8, 9, 12, and 13, the value is zero bytes in length. If all columns are of these types then the body section of the record is empty. A record might have fewer values than the number of columns in the corresponding table. This can happen, for example, after an ALTER TABLE ... ADD COLUMN SQL statement has increased the number of columns in the table schema without modifying preexisting rows in the table. Missing values at the end of the record are filled in using the default value for the corresponding columns defined in the table schema.",
                offset,
                size,
                Value::Record(record.clone()),
                style,
            ));
            offset += size;
        }

        if let Some(overflow) = &cell.overflow {
            fields.push(Field::new(
                "Cell Payload: Page Overflow. When the payload of a b-tree cell is too large for the b-tree page, the surplus is spilled onto overflow pages. Overflow pages form a linked list. The first four bytes of each overflow page are a big-endian integer which is the page number of the next page in the chain, or zero for the final page in the chain. The fifth byte through the last usable byte are used to hold overflow content.",
                offset,
                4,
                Value::U32(overflow.page),
                "bg-slate-390",
            ));
        }

        fields
    }

    fn table_interior_fields(&self, cell: &TableInteriorCell) -> Vec<Field> {
        let cell_header_style = "bg-slate-300";
        vec![
            Field::new(
                "Page number of the left child.",
                self.offset,
                4,
                Value::U32(cell.left_page_number),
                cell_header_style,
            ),
            Field::new(
                "A varint which is the integer key, a.k.a. 'rowid'.",
                4,
                cell.rowid_varint.bytes.len(),
                Value::Varint(cell.rowid_varint.clone()),
                cell_header_style,
            ),
        ]
    }
}

impl Part for CellPart {
    fn label(&self) -> String {
        format!("Cell Content {}", self.id)
    }

    fn desc(&self) -> &'static str {
        "The format of a cell depends on which kind of b-tree page the cell appears on. Cell elements like number of bytes of payload and rowid are encoded by a variable-length integer or 'varint', which is a static Huffman encoding of 64-bit twos-complement integers, that uses less space for small positive values."
    }

    fn color(&self) -> String {
        if self.id % 2 == 0 {
            "green".to_string()
        } else {
            "orange".to_string()
        }
    }

    fn fields(&self) -> Vec<Field> {
        match &self.cell {
            Cell::TableLeaf(c) => self.table_leaf_fields(c),
            Cell::TableInterior(c) => self.table_interior_fields(c),
        }
    }
}
