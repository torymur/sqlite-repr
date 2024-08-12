//! [Sqlite Database Header]<https://www.sqlite.org/fileformat2.html#the_database_header>
//! Stored in the first 100 bytes of sqlite database file

use crate::slc;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextEncoding {
    UTF8,
    UTF16le,
    UTF16be,
}

impl TryFrom<u32> for TextEncoding {
    type Error = String;

    fn try_from(val: u32) -> Result<Self, Self::Error> {
        match val {
            1 => Ok(Self::UTF8),
            2 => Ok(Self::UTF16le),
            3 => Ok(Self::UTF16be),
            _ => Err(format!("Wrong db encoding value: {}", val)),
        }
    }
}

impl std::fmt::Display for TextEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UTF8 => write!(f, "UTF-8"),
            Self::UTF16le => write!(f, "UTF-16 LE"),
            Self::UTF16be => write!(f, "UTF-16 BE"),
        }
    }
}

impl TextEncoding {
    pub fn to_be_bytes(&self) -> [u8; 4] {
        match self {
            Self::UTF8 => (1 as u32).to_be_bytes(),
            Self::UTF16le => (2 as u32).to_be_bytes(),
            Self::UTF16be => (3 as u32).to_be_bytes(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DBHeader {
    /// should be 'SQLite format 3\0'
    /// offset: 0, size: 16
    pub header: String,
    /// page size of database, value between 512 and 32768 inclusive
    /// 0x0001 for 65536
    /// offset: 16, size: 2
    pub page_size: u16,
    /// 1 for legacy, 2 for WAL
    /// offset: 18, size: 1
    pub write_version: u8,
    /// 1 for legacy, 2 for WAL
    /// offset: 19, size: 1
    pub read_version: u8,
    /// reserved space at the end of each page
    /// offset: 20, size: 1
    pub reserved_page_space: u8,
    /// must be 64
    /// offset: 21, size: 1
    pub max_embedded_payload_fraction: u8,
    /// must be 32
    /// offset: 22, size: 1
    pub min_embedded_payload_fraction: u8,
    /// must be 32
    /// offset: 23, size: 1
    pub leaf_payload_fraction: u8,
    /// file change counter
    /// offset: 24, size: 4
    pub file_change_counter: u32,
    /// size of db in pages
    /// offset: 28, size: 4
    pub db_size: u32,
    /// num of first freelist trunk page
    /// offset: 32, size: 4
    pub first_free_page_num: u32,
    /// total number of freelist pages
    /// offset: 36, size: 4
    pub freelist_total: u32,
    /// schema cookie
    /// offset: 40, size: 4
    pub schema_cookie: u32,
    /// schema format number, supported values are 1, 2, 3 and 4
    /// offset: 44, size: 4
    pub schema_format_num: u32,
    /// default page cache size
    /// offset: 48, size: 4
    pub default_page_cache_size: u32,
    /// page number of largest root b-tree page when in auto-vacuum
    /// or incremental vacuum modes, zero otherwise
    /// offset: 52, size: 4
    pub largest_root: u32,
    /// db text encoding:
    /// UTF-8    - 1
    /// UTF-16le - 2
    /// UTF-16be - 3
    /// offset: 56, size: 4
    pub text_encoding: TextEncoding,
    /// user version, set by user version pragma
    /// offset: 60, size: 4
    pub user_version: u32,
    /// Incremental vacuum mode flag, true if not 0, false otherwize
    /// offset: 64, size: 4
    pub inc_vacuum_mode: u32,
    /// application id, set by pragma application id
    /// offset: 68, size: 4
    pub application_id: u32,
    // reserved, must be zero
    // offset: 72, size: 20
    pub reserved_for_expansion: [u8; 20],
    /// version of sqlite which modified database recently
    /// offset: 92, size: 4
    pub version_valid_for_number: u32,
    /// sqlite version number
    /// offset: 96, size: 4
    pub version: u32,
}

impl TryFrom<&[u8; 100]> for DBHeader {
    type Error = Box<dyn std::error::Error>;

    fn try_from(buf: &[u8; 100]) -> Result<Self, Self::Error> {
        Ok(Self::new(
            // header
            std::str::from_utf8(&slc!(buf, 0, 16))?.to_string(),
            // page_size
            slc!(buf, 16, 2, u16),
            // write_version
            slc!(buf, 18, 1, u8),
            // read_version
            slc!(buf, 19, 1, u8),
            // reserved_page_space
            slc!(buf, 20, 1, u8),
            // max_embedded_payload_fraction
            slc!(buf, 21, 1, u8),
            // min_embedded_payload_fraction
            slc!(buf, 22, 1, u8),
            // leaf_payload_fraction
            slc!(buf, 23, 1, u8),
            // file_change_counter
            slc!(buf, 24, 4, u32),
            // db_size
            slc!(buf, 28, 4, u32),
            // first_free_page_num
            slc!(buf, 32, 4, u32),
            // freelist_total
            slc!(buf, 36, 4, u32),
            // schema_cookie
            slc!(buf, 40, 4, u32),
            // schema_format_num
            slc!(buf, 44, 4, u32),
            // default_page_cache
            slc!(buf, 48, 4, u32),
            // largest_root
            slc!(buf, 52, 4, u32),
            // text_encoding
            slc!(buf, 56, 4, u32).try_into()?,
            // user_version
            slc!(buf, 60, 4, u32),
            // inc_vacuum_mode
            slc!(buf, 64, 4, u32),
            // application_id
            slc!(buf, 68, 4, u32),
            // reserved_for_expansion
            &buf[72..92],
            // version_valid_for_number
            slc!(buf, 92, 4, u32),
            // version
            slc!(buf, 96, 4, u32),
        ))
    }
}

impl DBHeader {
    pub fn new(
        header: String,
        page_size: u16,
        write_version: u8,
        read_version: u8,
        reserved_page_space: u8,
        max_embedded_payload_fraction: u8,
        min_embedded_payload_fraction: u8,
        leaf_payload_fraction: u8,
        file_change_counter: u32,
        db_size: u32,
        first_free_page_num: u32,
        freelist_total: u32,
        schema_cookie: u32,
        schema_format_num: u32,
        default_page_cache_size: u32,
        largest_root: u32,
        text_encoding: TextEncoding,
        user_version: u32,
        inc_vacuum_mode: u32,
        application_id: u32,
        reserved_for_expansion_slice: &[u8],
        version_valid_for_number: u32,
        version: u32,
    ) -> Self {
        let mut reserved_for_expansion: [u8; 20] = [0; 20];
        reserved_for_expansion.copy_from_slice(reserved_for_expansion_slice);
        Self {
            header,
            page_size,
            write_version,
            read_version,
            reserved_page_space,
            max_embedded_payload_fraction,
            min_embedded_payload_fraction,
            leaf_payload_fraction,
            file_change_counter,
            db_size,
            first_free_page_num,
            freelist_total,
            schema_cookie,
            schema_format_num,
            default_page_cache_size,
            largest_root,
            text_encoding,
            user_version,
            inc_vacuum_mode,
            application_id,
            reserved_for_expansion,
            version_valid_for_number,
            version,
        }
    }
}
