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
        Ok(DBHeader::new(header))
    }
}

pub trait Parts: std::fmt::Debug {
    fn label(&self) -> String;
    fn desc(&self) -> String;
    fn bytes(&self) -> Box<[u8]>;
}

impl Parts for DBHeader {
    fn label(&self) -> String {
        "Database Header".to_string()
    }

    fn desc(&self) -> String {
        "The first 100 bytes of the database file comprise the database file header. All multibyte fields in the database file header are stored with the most significant byte first (big-endian).".to_string()
    }

    fn bytes(&self) -> Box<[u8]> {
        Box::new(self.bytes)
    }
}

#[derive(Debug, Clone)]
pub struct DBHeader {
    bytes: [u8; 100],
}

impl DBHeader {
    pub fn new(bytes: [u8; 100]) -> Self {
        Self { bytes }
    }
}
