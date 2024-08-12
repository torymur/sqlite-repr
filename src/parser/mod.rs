//! Experimentation around sqlite internal format parsing, based on https://www.sqlite.org/fileformat2.html

pub mod header;
pub mod reader;

pub use header::DBHeader;
pub use reader::Reader;
