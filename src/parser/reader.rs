use crate::parser::DBHeader;
use crate::ui::header::Parts;
use std::rc::Rc;

#[derive(Debug)]
pub struct Reader {
    bytes: &'static [u8],
    pub parts: Vec<Rc<dyn Parts>>,
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Reader {
    pub fn new(bytes: &'static [u8]) -> Result<Self> {
        let mut i = Self {
            bytes,
            parts: vec![].into(),
        };
        i.available_parts();
        Ok(i)
    }

    fn available_parts(&mut self) {
        let mut parts = vec![];
        if let Ok(header) = self.get_header() {
            let part: Rc<dyn Parts> = Rc::new(header);
            parts.push(part);
        }
        self.parts = parts.clone();
    }

    pub fn get_header(&self) -> Result<DBHeader> {
        let mut header = [0; 100];
        header.clone_from_slice(&self.bytes[..100]);
        Ok(DBHeader::try_from(&header)?)
    }
}
