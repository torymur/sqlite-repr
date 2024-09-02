use crate::{DBHeader, OverflowPage, OverflowUnit, Page, Result, StdError};
use std::rc::Rc;

pub const DB_HEADER_SIZE: usize = 100;

#[derive(Debug)]
pub struct Reader {
    pub bytes: &'static [u8],
    pub db_header: Rc<DBHeader>,
}

impl Reader {
    pub fn new(bytes: &'static [u8]) -> Result<Self, StdError> {
        if bytes.len() < DB_HEADER_SIZE {
            return Err(Self::incomplete(
                "read",
                "database header",
                DB_HEADER_SIZE,
                bytes.len(),
            ));
        }

        let mut bheader = [0; DB_HEADER_SIZE];
        bheader.clone_from_slice(&bytes[..DB_HEADER_SIZE]);
        let db_header = Rc::new(DBHeader::try_from(&bheader)?);

        Ok(Self { bytes, db_header })
    }

    /// Get fully parsed Btree Page.
    pub fn get_btree_page(&self, page_num: usize) -> Result<Page> {
        let buf = self.page_slice(page_num)?;
        let page = Page::try_from((self.db_header.clone(), page_num, buf.as_slice()))?;
        Ok(page)
    }

    /// Get fully parsed Overflow Page.
    pub fn get_overflow_page(
        &self,
        overflow: Vec<OverflowUnit>,
        page_num: usize,
    ) -> Result<OverflowPage> {
        let buf = self.page_slice(page_num)?;
        let page =
            OverflowPage::try_from((self.db_header.text_encoding, overflow, buf.as_slice()))?;
        Ok(page)
    }

    /// Get an actual number of total pages per database file.
    pub fn pages_total(&self) -> usize {
        // Based on docs descriptions, db_size is valid only if:
        // - it's not zero
        // - AND file_change_counter == version_valid_for_number
        //
        // Otherwise, decision is made by looking at the actual db size.

        if self.db_header.db_size != 0
            && self.db_header.file_change_counter == self.db_header.version_valid_for_number
        {
            self.db_header.db_size as usize
        } else {
            self.bytes.len() / self.db_header.page_size as usize
        }
    }

    fn page_slice(&self, page_num: usize) -> Result<Vec<u8>, StdError> {
        self.validate_page_bounds(page_num)?;
        let page_offset = self.page_offset(page_num);
        let page_size = self.db_header.page_size as usize;
        let mut b_page = vec![0; page_size];
        b_page.clone_from_slice(&self.bytes[page_offset..page_offset + page_size]);
        Ok(b_page)
    }

    fn validate_page_bounds(&self, page_num: usize) -> Result<()> {
        let pages_total = self.pages_total();
        // SQLite pages are started from 1
        if page_num > pages_total || page_num == 0 {
            return Err(format!("Out of bounds page access: {}/{}", page_num, pages_total).into());
        }

        let page_end = self.page_offset(page_num) + self.db_header.page_size as usize;
        if self.bytes.len() < page_end {
            return Err(Self::incomplete("read", "page", page_end, self.bytes.len()));
        }
        Ok(())
    }

    fn page_offset(&self, page_num: usize) -> usize {
        // "Index perspective" helps simplify math of pointers to interior pages
        //((page_num - 1) * self.db_header.page_size as usize).max(DB_HEADER_SIZE)
        (page_num - 1) * self.db_header.page_size as usize
    }

    fn incomplete(op: &str, what: &str, expected: usize, got: usize) -> StdError {
        format!(
            "Incomplete {} of {}, expected to read {} bytes, got: {}",
            what, op, expected, got
        )
        .into()
    }
}
