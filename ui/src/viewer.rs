//! Database UI Viewer.

use std::collections::BTreeMap;
use std::include_bytes;
use std::rc::Rc;

use parser::{Cell, OverflowPage, Reader, StdError};

use crate::overflow_pages::OverflowPageElement;
use crate::pages::BtreePageElement;
use crate::PageView;

#[derive(Debug)]
pub struct Viewer {
    pub included_db: BTreeMap<&'static str, &'static [u8]>,
    pub pages: Vec<Rc<dyn PageView>>,
}

pub type Result<T, E = StdError> = std::result::Result<T, E>;

// Preloaded examples of databases to start UI with something
pub const SIMPLE_DB_BYTES: &[u8] = include_bytes!("../included/simple");
pub const BIG_PAGE_DB_BYTES: &[u8] = include_bytes!("../included/big_page");
pub const TWO_TABLES_DB_BYTES: &[u8] = include_bytes!("../included/two_tables");
pub const OVERFLOW_PAGE_DB_BYTES: &[u8] = include_bytes!("../included/overflow_page");
pub const INTERIOR_TABLE_DB_BYTES: &[u8] = include_bytes!("../included/interior_table");
pub const SIMPLE_DB: &str = "Simple";
pub const BIG_PAGE_DB: &str = "Big page";
pub const TWO_TABLES_DB: &str = "Two tables";
pub const OVERFLOW_PAGE_DB: &str = "Overflow pages";
pub const INTERIOR_TABLE_DB: &str = "Interior Table";

impl Viewer {
    pub fn new_from_included(name: &str) -> Result<Self, StdError> {
        let included_db = BTreeMap::from([
            (SIMPLE_DB, SIMPLE_DB_BYTES),
            (BIG_PAGE_DB, BIG_PAGE_DB_BYTES),
            (TWO_TABLES_DB, TWO_TABLES_DB_BYTES),
            (OVERFLOW_PAGE_DB, OVERFLOW_PAGE_DB_BYTES),
            (INTERIOR_TABLE_DB, INTERIOR_TABLE_DB_BYTES),
        ]);

        let bytes = included_db.get(name).ok_or("This db is not included.")?;
        let reader = Reader::new(bytes)?;
        let size = reader.db_header.page_size as usize;
        let mut pages_map: BTreeMap<usize, Rc<dyn PageView>> = BTreeMap::new();
        for n in 1..reader.pages_total() + 1 {
            if pages_map.contains_key(&n) {
                // Already filled by overflow pages.
                continue;
            };

            // TODO: handle Err here via ui error message
            let page = match reader.get_btree_page(n) {
                Ok(page) => page,
                Err(_) => continue,
            };
            // Check for overflow information in each cell of the page.
            for cell in &page.cells {
                let cell = match cell {
                    Cell::TableInterior(_) => continue, // the only one without overflow
                    Cell::TableLeaf(c) => c,
                };
                match &cell.overflow {
                    None => continue,
                    Some(overflow) => {
                        let opage = reader
                            .get_overflow_page(overflow.units.to_vec(), overflow.page as usize)?;
                        Self::load_overflow_page(
                            opage,
                            overflow.page as usize,
                            &mut pages_map,
                            &reader,
                        )?;
                    }
                };
            }
            pages_map.insert(n, Rc::new(BtreePageElement::new(page, size)));
        }

        let pages: Vec<Rc<dyn PageView>> = pages_map.into_values().collect();
        Ok(Self { included_db, pages })
    }

    pub fn included_dbnames(&self) -> Vec<String> {
        self.included_db.keys().map(|k| k.to_string()).collect()
    }

    pub fn get_page(&self, id: u32) -> Rc<dyn PageView> {
        self.pages
            .get(id as usize - 1)
            .expect("Page is outside of Viewer range.")
            .clone()
    }

    fn load_overflow_page(
        page: OverflowPage,
        page_num: usize,
        pages: &mut BTreeMap<usize, Rc<dyn PageView>>,
        reader: &Reader,
    ) -> Result<(), StdError> {
        let page_size = reader.db_header.page_size as usize;
        pages.insert(
            page_num,
            Rc::new(OverflowPageElement::new(page.clone(), page_size, page_num)),
        );
        match page.next_page {
            0 => Ok(()),
            page_num => {
                let page_num = page_num as usize;
                let next_page = reader.get_overflow_page(page.overflow_units, page_num)?;
                Self::load_overflow_page(next_page, page_num, pages, reader)
            }
        }
    }
}
