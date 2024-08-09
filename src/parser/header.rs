use crate::slc;

#[derive(Debug, Clone)]
pub struct DBHeader {
    /// should be 'SQLite format 3\0'
    /// offset: 0, size: 16
    pub header: String,
    /// page size of database, value between 512 and 32768 inclusive
    /// 0x0001 for 65536
    /// offset: 16, size: 2
    pub page_size: u16,
}

impl TryFrom<&[u8; 100]> for DBHeader {
    type Error = Box<dyn std::error::Error>;

    fn try_from(buf: &[u8; 100]) -> Result<Self, Self::Error> {
        Ok(Self::new(
            // header
            std::str::from_utf8(&slc!(buf, 0, 16))?.to_string(),
            // page_size
            slc!(buf, 16, 2, u16),
        ))
    }
}

impl DBHeader {
    pub fn new(header: String, page_size: u16) -> Self {
        Self { header, page_size }
    }
}
