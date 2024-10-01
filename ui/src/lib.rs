//! UI related traits, data transformations and descriptons to simplify
//! rendering of parsed structures.

pub mod freelist;
pub mod header;
pub mod included_db;
pub mod index;
pub mod overflow_pages;
pub mod pages;
pub mod state;
pub mod viewer;

use core::fmt;
use std::rc::Rc;

use parser::*;

pub trait PageView: std::fmt::Debug {
    fn id(&self) -> usize;
    fn size(&self) -> usize;
    fn label(&self) -> String;
    fn desc(&self) -> &'static str;
    fn parts(&self) -> &[Rc<dyn Part>];
}

pub trait Part: std::fmt::Debug {
    fn label(&self) -> String;
    fn desc(&self) -> &'static str;
    fn fields(&self) -> &[Rc<Field>];
    fn color(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct PageElement {
    pub id: usize,
    pub page: Rc<PageLayout>,
    pub size: usize,
    parts: Vec<Rc<dyn Part>>,
}

pub struct PageElementBuilder {
    pub id: usize,
    pub page: PageLayout,
    pub size: usize,
    #[allow(dead_code)]
    parts: Option<Vec<Rc<dyn Part>>>,
}

#[derive(Debug, Clone)]
pub enum PageLayout {
    Btree(Page),
    Overflow(OverflowPage),
    TrunkFreelist(TrunkFreelistPage),
    LeafFreelist(LeafFreelistPage),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BTreeNodeView {
    pub page_num: usize,
    pub children: Vec<BTreeNodeView>,
    pub overflow: Vec<usize>,
}

impl Default for BTreeNodeView {
    fn default() -> Self {
        Self {
            page_num: 0,
            children: vec![],
            overflow: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BTreeView {
    pub ttype: String,
    pub name: String,
    pub root: BTreeNodeView,
}

impl PageElementBuilder {
    pub fn new(page: PageLayout, size: usize, page_num: usize) -> Self {
        Self {
            id: page_num,
            page,
            size,
            parts: None,
        }
    }

    pub fn build(self) -> PageElement {
        let parts = match &self.page {
            PageLayout::Btree(page) => self.build_btree_parts(page),
            PageLayout::Overflow(page) => self.build_overflow_parts(page),
            PageLayout::TrunkFreelist(page) => self.build_trunk_freelist_parts(page),
            PageLayout::LeafFreelist(page) => self.build_leaf_freelist_parts(page),
        };
        PageElement {
            id: self.id,
            page: Rc::new(self.page),
            size: self.size,
            parts,
        }
    }

    fn build_btree_parts(&self, page: &Page) -> Vec<Rc<dyn Part>> {
        use header::*;
        use pages::*;

        let mut parts: Vec<Rc<dyn Part>> = vec![
            Rc::new(PageHeaderPart::new(page)),
            Rc::new(CellPointerPart::new(page)),
            Rc::new(UnallocatedPart::new(page)),
        ];

        // Generate CellPart(s).
        let mut cells = page.cells.clone();
        cells.reverse();
        let mut offsets = page.cell_pointer.array.clone();
        offsets.reverse();
        let mut cell_parts: Vec<Rc<dyn Part>> = vec![];
        for (n, cell) in cells.iter().enumerate() {
            let offset = offsets[n] as usize;
            cell_parts.push(Rc::new(CellPart::new(cell, offset, n + 1)))
        }
        parts.extend(cell_parts);

        // Consider for database header to go first.
        if self.id == 1 {
            parts.insert(0, Rc::new(DBHeaderPart::new(&page.db_header)))
        };
        parts
    }

    fn build_overflow_parts(&self, page: &OverflowPage) -> Vec<Rc<dyn Part>> {
        use overflow_pages::*;

        let mut parts: Vec<Rc<dyn Part>> = vec![
            Rc::new(NextPagePart::new(page.next_page)),
            Rc::new(DataPart::new(&page.data)),
        ];

        if let Some(unallocated) = &page.unallocated {
            parts.push(Rc::new(UnallocatedOverflowPart::new(unallocated, page)));
        }
        parts
    }

    fn build_trunk_freelist_parts(&self, page: &TrunkFreelistPage) -> Vec<Rc<dyn Part>> {
        use freelist::*;

        let mut parts: Vec<Rc<dyn Part>> = vec![
            Rc::new(NextPagePart::new(page.next_page)),
            Rc::new(LeafPageAmountPart::new(page.leaf_page_amount)),
        ];

        if let Some(leaf_page_numbers) = &page.leaf_page_numbers {
            parts.push(Rc::new(LeafPageNumbersPart::new(leaf_page_numbers)));
        };

        if let Some(unallocated) = &page.unallocated {
            let offset = (page.leaf_page_amount * 4 + 8) as usize;
            parts.push(Rc::new(UnallocatedPart::new(unallocated, offset)));
        }
        parts
    }

    fn build_leaf_freelist_parts(&self, page: &LeafFreelistPage) -> Vec<Rc<dyn Part>> {
        use freelist::*;

        vec![Rc::new(UnallocatedPart::new(&page.unallocated, 0))]
    }
}

impl PageView for PageElement {
    fn id(&self) -> usize {
        self.id
    }

    fn size(&self) -> usize {
        self.size
    }

    fn label(&self) -> String {
        match &*self.page {
            PageLayout::Btree(page) => {
                let sign = match page.page_header.page_type {
                    PageHeaderType::LeafTable => "ꕤ ",
                    PageHeaderType::InteriorTable => "☰ ",
                    PageHeaderType::LeafIndex => "✦ ",
                    PageHeaderType::InteriorIndex => "𝄃𝄃𝄃",
                };
                format!("{} {}", sign, page.page_header.page_type)
            }
            PageLayout::Overflow(_) => "ᨒ  Overflow".to_string(),
            PageLayout::TrunkFreelist(_) => "⩩ Trunk Freelist".to_string(),
            PageLayout::LeafFreelist(_) => "● Leaf Freelist".to_string(),
        }
    }

    fn desc(&self) -> &'static str {
        match &*self.page {
            PageLayout::Btree(_) => {
                if self.id == 1 {
                    "The 100-byte database file header is found only on Page 1, meaning that root page has 100 fewer bytes of storage space available. It's always a table b-tree page: interior or leaf. Page 1 is the root page of a table b-tree, that holds a special table named 'sqlite_schema'. This b-tree is known as the 'schema table' since it stores the complete database schema."
                } else {
                    "A b-tree page is either an interior page or a leaf page. A b-tree page is either a table b-tree page or an index b-tree page. All pages within each complete b-tree are of the same type: either table or index. A leaf page contains keys and in the case of a table b-tree each key has associated data. An interior page contains K keys together with K+1 pointers to child b-tree pages. A'pointer' in an interior b-tree page is just the 32-bit unsigned integer page number of the child page."
                }
            }
            PageLayout::Overflow(_) => "When the size of payload for a cell exceeds a certain threshold, then only the first few bytes of the payload are stored on the b-tree page and the balance is stored in a linked list of content overflow pages.",
            PageLayout::TrunkFreelist(_) => "A database file might contain one or more pages that are not in active use. Unused pages can come about, for example, when information is deleted from the database. Unused pages are stored on the freelist and are reused when additional pages are required. The freelist is organized as a linked list of freelist trunk pages with each trunk page containing page numbers for zero or more freelist leaf pages. The database header also stores the page number of the first freelist trunk page and the number of freelist pages.",
            PageLayout::LeafFreelist(_) => "Freelist leaf pages contain no information. SQLite avoids reading or writing freelist leaf pages in order to reduce disk I/O.",
        }
    }

    fn parts(&self) -> &[Rc<dyn Part>] {
        self.parts.as_slice()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub desc: &'static str,
    pub offset: usize,
    pub size: usize,
    pub value: Value,
    pub style: &'static str,
}

impl Field {
    pub fn to_hex(&self) -> String {
        match &self.value {
            Value::U8(v) => Self::pretty_hex(&v.to_be_bytes()),
            Value::U16(v) => Self::pretty_hex(&v.to_be_bytes()),
            Value::U32(v) => Self::pretty_hex(&v.to_be_bytes()),
            Value::Text(v) => Self::pretty_hex(v.as_bytes()),
            Value::Bool(v) => Self::pretty_hex(&v.to_be_bytes()),
            Value::PageSize(v) => match v {
                65536 => Self::pretty_hex(&1_u16.to_be_bytes()),
                _ => Self::pretty_hex(&(*v as u16).to_be_bytes()),
            },
            Value::Array(v) => Self::pretty_hex(v),
            Value::Encoding(v) => Self::pretty_hex(&v.to_be_bytes()),
            Value::Version(v) => Self::pretty_hex(&v.to_be_bytes()),
            Value::PageType(v) => Self::pretty_hex(&v.to_be_bytes()),
            Value::CellStartOffset(v) => match v {
                65536 => Self::pretty_hex(&0_u16.to_be_bytes()),
                _ => Self::pretty_hex(&(*v as u16).to_be_bytes()),
            },
            Value::Unallocated(v) => Self::pretty_hex(v),
            Value::Varint(v) => Self::pretty_hex(&v.bytes),
            Value::PageNumber(v) => Self::pretty_hex(&v.to_be_bytes()),
            Value::Record(record) => match record.value {
                RecordType::Null
                | RecordType::Zero(_)
                | RecordType::One(_)
                | RecordType::Blob(None)
                | RecordType::Text(None) => "─".to_string(),
                _ => Self::pretty_hex(record.bytes.as_ref().map_or(&[], |b| b)),
            },
        }
    }

    pub fn try_page_number(&self) -> Result<u32, StdError> {
        match &self.value {
            Value::PageNumber(v) if *v != 0 => Ok(*v),
            _ => Err("Page number cannot be made from this Value.".into()),
        }
    }

    pub fn trim_hex(&self, limit: usize) -> String {
        match &self.value {
            Value::Unallocated(v) => {
                if limit.min(v.len()) == limit {
                    format!("{} ...", Self::pretty_hex(&v[..limit]))
                } else {
                    self.to_hex()
                }
            }
            _ => self.to_hex(),
        }
    }

    pub fn trim_str(&self, limit: usize) -> String {
        match &self.value {
            Value::Unallocated(v) => {
                if limit.min(v.len()) == limit {
                    format!("{:?} ...", &v[..limit])
                } else {
                    format!("{:?}", v)
                }
            }
            v => format!("{v}"),
        }
    }

    fn pretty_hex(bytes: &[u8]) -> String {
        bytes
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    U8(u8),
    U16(u16),
    U32(u32),
    Text(Rc<String>),
    Bool(u32),
    PageSize(u64),
    Array(Box<[u8]>),
    Encoding(TextEncoding),
    Version(u32),
    PageType(PageHeaderType),
    CellStartOffset(u32),
    Unallocated(Box<[u8]>),
    Varint(Varint),
    Record(RecordValue),
    PageNumber(u32),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::U8(v) => write!(f, "{v}"),
            Self::U16(v) => write!(f, "{v}"),
            Self::U32(v) => write!(f, "{v}"),
            Self::Text(v) => write!(f, "{:?}", v),
            Self::Bool(v) => write!(f, "{:?}", *v != 0),
            Self::PageSize(v) => write!(f, "{v}"),
            Self::Array(v) => write!(f, "{:?}", *v),
            Self::Encoding(v) => write!(f, "{v}"),
            Self::Version(v) => {
                // SQLite version is in the format "X.Y.Z", where:
                // - X is the major version number (always 3 for SQLite3)
                // - Y is the minor version Number
                // - Z is the release number.
                // The SQLITE_Version_NUMBER C preprocessor macro resolves to
                // an integer with the value: X*1000000 + Y*1000 + Z

                let z = v % 1000;
                let y = (v / 1000) % 1000;
                let x = v / 1000000;
                write!(f, "{x}.{y}.{z}")
            }
            Self::PageType(v) => write!(f, "{v}"),
            Self::CellStartOffset(v) => write!(f, "{v}"),
            Self::Unallocated(v) => write!(f, "{:?}", *v),
            Self::Varint(v) => write!(f, "{}", v.value),
            Self::PageNumber(v) => write!(f, "{v}"),
            Value::Record(record) => match &record.value {
                RecordType::Null => write!(f, "Null"),
                RecordType::Zero(v) | RecordType::One(v) => write!(f, "Integer {v}"),
                RecordType::I8(v) => write!(f, "{v}"),
                RecordType::I16(v) => write!(f, "{v}"),
                RecordType::I24(v) | RecordType::I32(v) => write!(f, "{v}"),
                RecordType::I48(v) | RecordType::I64(v) => write!(f, "{v}"),
                RecordType::F64(v) => write!(f, "{v}"),
                RecordType::Ten | RecordType::Eleven => write!(f, "Internal codes"),
                RecordType::Blob(Some(v)) => write!(f, "Blob {:?}", v),
                RecordType::Text(Some(v)) => write!(f, "{v}"),
                RecordType::Blob(None) => write!(f, "Empty Blob"),
                RecordType::Text(None) => write!(f, "Empty Text"),
            },
        }
    }
}

impl Field {
    pub fn new(
        desc: &'static str,
        offset: usize,
        size: usize,
        value: Value,
        style: &'static str,
    ) -> Self {
        Self {
            desc,
            offset,
            size,
            value,
            style,
        }
    }
}
