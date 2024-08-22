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
        match &self.value {
            Value::U8(v) => Self::pretty_hex(&v.to_be_bytes()),
            Value::U16(v) => Self::pretty_hex(&v.to_be_bytes()),
            Value::U32(v) => Self::pretty_hex(&v.to_be_bytes()),
            Value::Text(v) => Self::pretty_hex(v.as_bytes()),
            Value::Bool(v) => Self::pretty_hex(&v.to_be_bytes()),
            Value::PageSize(v) => match v {
                65536 => Self::pretty_hex(&1_u16.to_be_bytes()),
                _ => Self::pretty_hex(&(*v as u16).to_be_bytes()),
            },
            Value::Array(v) => Self::pretty_hex(v),
            Value::Encoding(v) => Self::pretty_hex(&v.to_be_bytes()),
            Value::Version(v) => Self::pretty_hex(&v.to_be_bytes()),
            Value::PageType(v) => Self::pretty_hex(&v.to_be_bytes()),
            Value::CellStartOffset(v) => match v {
                65536 => Self::pretty_hex(&0_u16.to_be_bytes()),
                _ => Self::pretty_hex(&(*v as u16).to_be_bytes()),
            },
            Value::Unallocated(v) => Self::pretty_hex(v),
        }
    }

    pub fn trim_hex(&self, limit: usize) -> String {
        match &self.value {
            Value::Unallocated(v) => format!("{} ...", Self::pretty_hex(&v[..limit])),
            _ => self.to_hex(),
        }
    }

    pub fn trim_str(&self, limit: usize) -> String {
        match &self.value {
            Value::Unallocated(v) => format!("{:?} ...", &v[..limit]),
            v => format!("{v}"),
        }
    }

    fn pretty_hex(bytes: &[u8]) -> String {
        bytes
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<String>>()
            .join(" ")
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
    Unallocated(Box<[u8]>),
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
            Self::Unallocated(v) => write!(f, "{:?}", *v),
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
