//! UI related traits, data transformations and descriptons to simplify
//! rendering of parsed structures.

pub mod header;
pub mod index;
pub mod pages;
pub mod state;
pub mod viewer;

use core::fmt;
use std::rc::Rc;

use parser::header::TextEncoding;
use parser::page::PageHeaderType;

pub trait BtreePage: std::fmt::Debug {
    fn label(&self) -> String;
    fn desc(&self) -> &'static str;
    fn parts(&self) -> Vec<Rc<dyn Part>>;
    fn page_size(&self) -> u64;
}

pub trait Part: std::fmt::Debug {
    fn label(&self) -> &'static str;
    fn desc(&self) -> &'static str;
    fn fields(&self) -> Vec<Field>;
    fn color(&self) -> &'static str;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub desc: &'static str,
    pub offset: usize,
    pub size: usize,
    pub value: Value,
}

impl Field {
    pub fn to_hex(&self) -> String {
        let pretty_hex = |bytes: &[u8]| -> String {
            bytes
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<String>>()
                .join(" ")
        };
        match &self.value {
            Value::U8(v) => pretty_hex(&v.to_be_bytes()),
            Value::U16(v) => pretty_hex(&v.to_be_bytes()),
            Value::U32(v) => pretty_hex(&v.to_be_bytes()),
            Value::Text(v) => pretty_hex(v.as_bytes()),
            Value::Bool(v) => pretty_hex(&v.to_be_bytes()),
            Value::PageSize(v) => match v {
                65536 => pretty_hex(&1_u16.to_be_bytes()),
                _ => pretty_hex(&(*v as u16).to_be_bytes()),
            },
            Value::Array(v) => pretty_hex(v),
            Value::Encoding(v) => pretty_hex(&v.to_be_bytes()),
            Value::Version(v) => pretty_hex(&v.to_be_bytes()),
            Value::PageType(v) => pretty_hex(&v.to_be_bytes()),
            Value::CellStartOffset(v) => match v {
                65536 => pretty_hex(&0_u16.to_be_bytes()),
                _ => pretty_hex(&(*v as u16).to_be_bytes()),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    U8(u8),
    U16(u16),
    U32(u32),
    Text(Rc<String>),
    Bool(u32),
    PageSize(u64),
    Array(Box<[u8]>),
    Encoding(TextEncoding),
    Version(u32),
    PageType(PageHeaderType),
    CellStartOffset(u32),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::U8(v) => write!(f, "{v}"),
            Self::U16(v) => write!(f, "{v}"),
            Self::U32(v) => write!(f, "{v}"),
            Self::Text(v) => write!(f, "{:?}", v),
            Self::Bool(v) => write!(f, "{:?}", *v != 0),
            Self::PageSize(v) => write!(f, "{v}"),
            Self::Array(v) => write!(f, "{:?}", *v),
            Self::Encoding(v) => write!(f, "{v}"),
            Self::Version(v) => {
                // SQLite version is in the format "X.Y.Z", where:
                // - X is the major version number (always 3 for SQLite3)
                // - Y is the minor version Number
                // - Z is the release number.
                // The SQLITE_Version_NUMBER C preprocessor macro resolves to
                // an integer with the value: X*1000000 + Y*1000 + Z

                let z = v % 1000;
                let y = (v / 1000) % 1000;
                let x = v / 1000000;
                write!(f, "{x}.{y}.{z}")
            }
            Self::PageType(v) => write!(f, "{v}"),
            Self::CellStartOffset(v) => write!(f, "{v}"),
        }
    }
}

impl Field {
    pub fn new(desc: &'static str, offset: usize, size: usize, value: Value) -> Self {
        Self {
            desc,
            offset,
            size,
            value,
        }
    }
}
