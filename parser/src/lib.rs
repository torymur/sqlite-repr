//! Experimentation around sqlite internal format parsing, based on https://www.sqlite.org/fileformat2.html
#![feature(str_from_utf16_endian)]

pub mod cell;
pub mod header;
pub mod page;
pub mod reader;
pub mod record;
pub mod varint;

pub use cell::Cell;
pub use header::{DBHeader, TextEncoding};
pub use page::{CellPointer, Page, PageHeader, PageHeaderType, Unallocated, CELL_PTR_SIZE};
pub use reader::{Reader, DB_HEADER_SIZE};
pub use record::{Record, RecordType, RecordValue};
pub use varint::Varint;

pub type StdError = Box<dyn std::error::Error + Sync + Send + 'static>;
pub type Result<T, E = StdError> = std::result::Result<T, E>;

#[macro_export]
macro_rules! slc {
    ($buf:ident, $offset:expr, $len:expr) => {
        $buf[$offset..($offset + $len)]
    };
    ($buf:ident, $offset:expr, $len:expr, $t:ty) => {
        <$t>::from_be_bytes(slc!($buf, $offset, $len).try_into()?)
    };
}
