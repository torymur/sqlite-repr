//! Experimentation around sqlite internal format parsing, based on https://www.sqlite.org/fileformat2.html

pub mod header;
pub mod page;
pub mod reader;

pub use header::DBHeader;
pub use page::PageHeader;
pub use reader::Reader;

#[macro_export]
macro_rules! slc {
    ($buf:ident, $offset:expr, $len:expr) => {
        $buf[$offset..($offset + $len)]
    };
    ($buf:ident, $offset:expr, $len:expr, $t:ty) => {
        <$t>::from_be_bytes(slc!($buf, $offset, $len).try_into()?)
    };
}
