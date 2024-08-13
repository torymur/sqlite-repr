//! Database UI Viewer.

use std::collections::HashMap;
use std::include_bytes;
use std::rc::Rc;

use crate::header::Parts;
use parser::Reader;

#[derive(Debug)]
pub struct Viewer {
    pub included_db: HashMap<&'static str, &'static [u8]>,
    pub parts: Vec<Rc<dyn Parts>>,
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// Preloaded examples of databases to start UI with something
pub const SIMPLE_DB_BYTES: &[u8] = include_bytes!("../included/simple");
pub const BIG_PAGE_DB_BYTES: &[u8] = include_bytes!("../included/big_page");
pub const SIMPLE_DB: &str = "Simple";
pub const BIG_PAGE_DB: &str = "Big page";

impl Viewer {
    pub fn new_from_included(name: &str) -> Result<Self> {
        let included_db = HashMap::from([
            (SIMPLE_DB, SIMPLE_DB_BYTES),
            (BIG_PAGE_DB, BIG_PAGE_DB_BYTES),
        ]);

        let bytes = included_db.get(name).ok_or("This db is not included.")?;
        let reader = Reader::new(bytes)?;

        let header: Rc<dyn Parts> = reader.header.clone();
        let parts = vec![header];

        Ok(Self { included_db, parts })
    }

    pub fn included_dbnames(&self) -> Vec<String> {
        self.included_db.keys().map(|k| k.to_string()).collect()
    }

    pub fn first_part(&self) -> Rc<dyn Parts> {
        // Having at least one part is guaranteed by `new_from_...` construct
        self.parts[0].clone()
    }
}
