//! Database UI Viewer.

use std::collections::HashMap;
use std::include_bytes;
use std::rc::Rc;

use crate::parser::Reader;
use crate::ui::header::Parts;

#[derive(Debug)]
pub struct Viewer {
    pub included_db: HashMap<&'static str, &'static [u8]>,
    pub parts: Vec<Rc<dyn Parts>>,
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// Preloaded/included examples of databases to start UI with something
pub const SIMPLE_DB: &'static [u8] = include_bytes!("../../examples/simple");
pub const BIG_PAGE_DB: &'static [u8] = include_bytes!("../../examples/big_page");

impl Viewer {
    pub fn new_from_included(name: &str) -> Result<Self> {
        let included_db = HashMap::from([("Simple", SIMPLE_DB), ("Big Page", BIG_PAGE_DB)]);

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
