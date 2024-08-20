use std::rc::Rc;

use parser::cell::{CellPointer, CELL_PTR_SIZE};
use parser::header::DBHeader;
use parser::page::{Page, PageHeader};
use parser::reader::DB_HEADER_SIZE;

use crate::header::DBHeaderPart;
use crate::{BtreePage, Field, Part, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct RootPage {
    id: usize,
    db_header: Rc<DBHeader>,
    page: Rc<Page>,
}

impl RootPage {
    pub fn new(db_header: Rc<DBHeader>, page: Page) -> Self {
        Self {
            db_header: db_header.clone(),
            page: Rc::new(page),
            id: 1,
        }
    }
}

impl BtreePage for RootPage {
    fn id(&self) -> usize {
        self.id
    }

    fn label(&self) -> String {
        format!("Root {}", self.page.page_header.page_type)
    }

    fn desc(&self) -> &'static str {
        "The 100-byte database file header is found only on Page 1, meaning that root page has 100 fewer bytes of storage space available. It's always a table b-tree page: interior or leaf. Page 1 is the root page of a table b-tree, that holds a special table named 'sqlite_schema'. This b-tree is known as the 'schema table' since it stores the complete database schema."
    }

    fn parts(&self) -> Vec<Rc<dyn Part>> {
        vec![
            Rc::new(DBHeaderPart {
                header: self.db_header.clone(),
            }),
            Rc::new(PageHeaderPart::new(self.page.page_header.clone(), true)),
            Rc::new(CellPointerPart::new(
                self.page.cell_pointer.clone(),
                self.page.cell_pointer_offset,
                true,
            )),
        ]
    }

    fn page_size(&self) -> u64 {
        self.db_header.page_size
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnyPage {
    id: usize,
    db_header: Rc<DBHeader>,
    page: Rc<Page>,
}

impl AnyPage {
    pub fn new(db_header: Rc<DBHeader>, page: Page, id: usize) -> Self {
        Self {
            db_header: db_header.clone(),
            page: Rc::new(page),
            id,
        }
    }
}

impl BtreePage for AnyPage {
    fn id(&self) -> usize {
        self.id
    }

    fn label(&self) -> String {
        format!("{}", self.page.page_header.page_type)
    }

    fn desc(&self) -> &'static str {
        "A b-tree page is either an interior page or a leaf page. A b-tree page is either a table b-tree page or an index b-tree page. All pages within each complete b-tree are of the same type: either table or index. A leaf page contains keys and in the case of a table b-tree each key has associated data. An interior page contains K keys together with K+1 pointers to child b-tree pages. A'pointer' in an interior b-tree page is just the 32-bit unsigned integer page number of the child page."
    }

    fn parts(&self) -> Vec<Rc<dyn Part>> {
        vec![
            Rc::new(PageHeaderPart::new(self.page.page_header.clone(), false)),
            Rc::new(CellPointerPart::new(
                self.page.cell_pointer.clone(),
                self.page.cell_pointer_offset,
                false,
            )),
        ]
    }

    fn page_size(&self) -> u64 {
        self.db_header.page_size
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PageHeaderPart {
    pub header: Rc<PageHeader>,
    pub offset: usize,
}

impl PageHeaderPart {
    pub fn new(header: PageHeader, root: bool) -> Self {
        Self {
            header: Rc::new(header),
            offset: if root { DB_HEADER_SIZE } else { 0 },
        }
    }
}

impl Part for PageHeaderPart {
    fn label(&self) -> &'static str {
        "B-tree Page Header"
    }

    fn desc(&self) -> &'static str {
        "The b-tree page header is 8 bytes in size for leaf pages and 12 bytes for interior pages. All multibyte values in the page header are big-endian.The cell pointer array of a b-tree page immediately follows the b-tree page header."
    }

    fn color(&self) -> &'static str {
        "green"
    }

    fn fields(&self) -> Vec<Field> {
        let mut fields = vec![
            Field::new(
                "B-tree page type. 2 (0x02) means the page is an interior index b-tree page, 5 (0x05): interior table b-tree page, 10 (0x0a): leaf index b-tree page, 13 (0x0d): leaf table b-tree page. Any other value for the b-tree page type is an error.",
                self.offset,
                1,
                Value::PageType(self.header.page_type)
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
            ),
            Field::new(
                "Number of cells on the page. A page might contain no cells, which is only possible for a root page of a table that contains no rows. SQLite strives to place cells as far toward the end of the b-tree page as it can, in order to leave space for future growth of the cell pointer array.",
                self.offset + 3,
                2,
                Value::U16(self.header.cell_num)
            ),
            Field::new(
                "Start of the cell content area. A zero value for this integer is interpreted as 65536. SQLite strives to place cells as far toward the end of the b-tree page as it can, in order to leave space for future growth of the cell pointer array. If a page contains no cells, then the offset to the cell content area will equal the page size minus the bytes of reserved space.",
                self.offset + 5,
                2,
                Value::CellStartOffset(self.header.cell_start_offset)
            ),
            Field::new(
                "The number of fragmented free bytes within the cell content area. If there is an isolated group of 1, 2, or 3 unused bytes within the cell content area, those bytes comprise a fragment. The total number of bytes in all fragments is stored in the fifth field of the b-tree page header. In a well-formed b-tree page, the total number of bytes in fragments may not exceed 60. The total amount of free space on a b-tree page consists of the size of the unallocated region plus the total size of all freeblocks plus the number of fragmented free bytes. SQLite may from time to time reorganize a b-tree page so that there are no freeblocks or fragment bytes, all unused bytes are contained in the unallocated space region, and all cells are packed tightly at the end of the page. This is called 'defragmenting' the b-tree page.",
                self.offset + 7,
                1,
                Value::U8(self.header.fragmented_free_bytes)
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
    pub fn new(cell_ptrs: CellPointer, offset: usize, root: bool) -> Self {
        Self {
            cell_ptrs: Rc::new(cell_ptrs),
            offset: if root {
                offset + DB_HEADER_SIZE
            } else {
                offset
            },
        }
    }
}

impl Part for CellPointerPart {
    fn label(&self) -> &'static str {
        "Cell pointer array"
    }

    fn desc(&self) -> &'static str {
        "The cell pointer array of a b-tree page immediately follows the b-tree page header. Let K be the number of cells on the btree. The cell pointer array consists of K 2-byte integer offsets to the cell contents. The cell pointers are arranged in key order with left-most cell (the cell with the smallest key) first and the right-most cell (the cell with the largest key) last."
    }

    fn color(&self) -> &'static str {
        "orange"
    }

    fn fields(&self) -> Vec<Field> {
        let mut offset = self.offset;
        self.cell_ptrs.array.iter().map(|ptr| {
            let field = Field::new(
                "2-byte integer offsets to the cell contents. Cell content is stored in the cell content region of the b-tree page. SQLite strives to place cells as far toward the end of the b-tree page as it can, in order to leave space for future growth of the cell pointer array. If a page contains no cells (which is only possible for a root page of a table that contains no rows) then the offset to the cell content area will equal the page size minus the bytes of reserved space. If the database uses a 65536-byte page size and the reserved space is zero (the usual value for reserved space) then the cell content offset of an empty page wants to be 65536. However, that integer is too large to be stored in a 2-byte unsigned integer, so a value of 0 is used in its place.",
                offset,
                CELL_PTR_SIZE,
                Value::CellStartOffset(*ptr)
            );
            offset += CELL_PTR_SIZE;
            field
        }).collect::<Vec<Field>>()
    }
}
