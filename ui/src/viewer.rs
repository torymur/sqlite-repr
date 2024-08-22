//! Database UI Viewer.

use std::collections::HashMap;
use std::include_bytes;
use std::rc::Rc;

use parser::Reader;

use crate::pages::PageView;

#[derive(Debug)]
pub struct Viewer {
    pub included_db: HashMap<&'static str, &'static [u8]>,
    pub pages: Vec<Rc<PageView>>,
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// Preloaded examples of databases to start UI with something
pub const SIMPLE_DB_BYTES: &[u8] = include_bytes!("../included/simple");
pub const BIG_PAGE_DB_BYTES: &[u8] = include_bytes!("../included/big_page");
pub const TWO_TABLES_DB_BYTES: &[u8] = include_bytes!("../included/two_tables");
pub const SIMPLE_DB: &str = "Simple";
pub const BIG_PAGE_DB: &str = "Big page";
pub const TWO_TABLES_DB: &str = "Two tables";

impl Viewer {
    pub fn new_from_included(name: &str) -> Result<Self> {
        let included_db = HashMap::from([
            (SIMPLE_DB, SIMPLE_DB_BYTES),
            (BIG_PAGE_DB, BIG_PAGE_DB_BYTES),
            (TWO_TABLES_DB, TWO_TABLES_DB_BYTES),
        ]);

        let bytes = included_db.get(name).ok_or("This db is not included.")?;
        let reader = Reader::new(bytes)?;
        let mut pages = vec![];
        for n in 1..reader.pages_total() + 1 {
            // TODO: handle Err here via ui error message
            let page = reader.get_page(n)?;
            let any_page = Rc::new(PageView::new(reader.db_header.clone(), page, n));
            pages.push(any_page);
        }

        Ok(Self { included_db, pages })
    }

    pub fn included_dbnames(&self) -> Vec<String> {
        self.included_db.keys().map(|k| k.to_string()).collect()
    }

    pub fn first_page(&self) -> Rc<PageView> {
        // Having at least one part is guaranteed by `new_from_...` construct
        self.pages[0].clone()
    }
}
