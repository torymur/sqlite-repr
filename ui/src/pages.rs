use std::rc::Rc;

use parser::header::DBHeader;
use parser::page::PageHeader;

use crate::header::DBHeaderPart;
use crate::{Field, Page, Part, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct RootPage {
    db_header: Rc<DBHeader>,
    page_header: Rc<PageHeader>,
}

impl RootPage {
    pub fn new(db_header: DBHeader, page_header: PageHeader) -> Self {
        Self {
            db_header: Rc::new(db_header),
            page_header: Rc::new(page_header),
        }
    }
}

impl Page for RootPage {
    fn label(&self) -> String {
        format!("Root {}", self.page_header.page_type)
    }

    fn desc(&self) -> &'static str {
        "The 100-byte database file header is found only on Page 1, meaning that root page has 100 fewer bytes of storage space available. It's always a table b-tree page: interior or leaf. Page 1 is the root page of a table b-tree, that holds a special table named 'sqlite_schema'. This b-tree is known as the 'schema table' since it stores the complete database schema."
    }

    fn parts(&self) -> Vec<Rc<dyn Part>> {
        vec![
            Rc::new(DBHeaderPart {
                header: self.db_header.clone(),
            }),
            Rc::new(PageHeaderPart {
                header: self.page_header.clone(),
            }),
        ]
    }

    fn page_size(&self) -> u64 {
        self.db_header.page_size
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PageHeaderPart {
    pub header: Rc<PageHeader>,
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
                100,
                1,
                Value::PageType(self.header.page_type)
            ),
            Field::new(
                "Start of the first freeblock on the page or zero if there are no freeblocks.",
                101,
                2,
                {
                    match self.header.free_block_offset {
                        None => Value::U16(0),
                        Some(v) => Value::U16(v),
                    }
                },
            ),
            Field::new(
                "Number of cells on the page.",
                103,
                2,
                Value::U16(self.header.cell_num)
            ),
            Field::new(
                "Start of the cell content area. A zero value for this integer is interpreted as 65536.",
                105,
                2,
                Value::CellStartOffset(self.header.cell_start_offset)
            ),
            Field::new(
                "The number of fragmented free bytes within the cell content area.",
                107,
                1,
                Value::U8(self.header.fragmented_free_bytes)
            ),
        ];
        match self.header.page_num {
            None => fields,
            Some(v) => {
                let page_num = Field::new(
                    "The right-most pointer. This value appears in the header of interior b-tree pages only and is omitted from all other pages.",
                    108,
                    4,
                    Value::U32(v),
                );
                fields.push(page_num);
                fields
            }
        }
    }
}
