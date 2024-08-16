use crate::{DBHeader, PageHeader};

const DB_HEADER_SIZE: usize = 100;
const PAGE_HEADER_SIZE: usize = 12;

#[derive(Debug)]
pub struct Reader {
    pub bytes: &'static [u8],
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Reader {
    pub fn new(bytes: &'static [u8]) -> Result<Self> {
        Ok(Self { bytes })
    }

    /// Get db header, located in the first 100 bytes of the root page
    pub fn get_db_header(&self) -> Result<DBHeader> {
        if self.bytes.len() < DB_HEADER_SIZE {
            return Err(Self::incomplete(
                "read",
                "database header",
                DB_HEADER_SIZE,
                self.bytes.len(),
            ));
        }
        let mut bheader = [0; DB_HEADER_SIZE];
        bheader.clone_from_slice(&self.bytes[..DB_HEADER_SIZE]);
        DBHeader::try_from(&bheader)
    }

    /// Get page header of a given page
    pub fn get_page_header(&self, page_num: usize) -> Result<PageHeader> {
        let header = self.get_db_header()?;
        let pages_total = self.pages_total(&header);
        if page_num > pages_total.into() {
            return Err(format!("Out of bounds page access: {}/{}", page_num, pages_total).into());
        }
        // first page header should be read with offset of 100 (db header)
        let offset = (Self::page_num(page_num)? * header.page_size as usize).max(DB_HEADER_SIZE);

        if self.bytes.len() < offset + PAGE_HEADER_SIZE {
            return Err(Self::incomplete(
                "read",
                "page header",
                offset + PAGE_HEADER_SIZE,
                self.bytes.len(),
            ));
        }

        let mut bheader = [0; PAGE_HEADER_SIZE];
        bheader.clone_from_slice(&self.bytes[offset..offset + PAGE_HEADER_SIZE]);
        PageHeader::try_from(&bheader)
    }

    pub fn pages_total(&self, header: &DBHeader) -> usize {
        // Based on docs descriptions, db_size is valid only if:
        // - it's not zero
        // - AND file_change_counter == version_valid_for_number
        //
        // Otherwise, decision is made by looking at the actual db size.

        if header.db_size != 0 && header.file_change_counter == header.version_valid_for_number {
            header.db_size as usize
        } else {
            self.bytes.len() / header.page_size as usize
        }
    }

    fn page_num(page_num: usize) -> Result<usize> {
        // SQLite pages are started from 1
        // Helps simplify math for reading pointers of interior pages
        match page_num {
            0 => Err("SQLite pages start from 1".into()),
            v => Ok(v - 1),
        }
    }

    fn incomplete(op: &str, what: &str, expected: usize, got: usize) -> Box<dyn std::error::Error> {
        format!(
            "Incomplete {} of {}, expected to read {} bytes, got: {}",
            what, op, expected, got
        )
        .into()
    }
}
