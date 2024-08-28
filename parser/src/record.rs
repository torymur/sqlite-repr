/// Cell payload is always in the "record format". The record format defines a sequence
/// of values corresponding to columns in a table or index.
///
/// The record format specifies the number of columns, the datatype of each column, and
/// the content of each column.
/// A record contains a header and a body, in that order.
use crate::{StdError, TextEncoding, Varint};

#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub header: RecordHeader,
    pub values: Vec<RecordValue>,
}

impl TryFrom<(TextEncoding, &[u8])> for Record {
    type Error = StdError;

    fn try_from(value: (TextEncoding, &[u8])) -> Result<Self, Self::Error> {
        let (text_encoding, buf) = value;
        let header = RecordHeader::try_from(buf)?;

        let mut values = vec![];
        let mut offset: usize = header.size.value as usize;
        for datatype in &header.datatypes {
            let value = RecordValue::new(datatype.value, text_encoding, &buf[offset..])?;
            offset += value.bytes.as_ref().map_or(0, |b| b.len());
            values.push(value);
        }
        Ok(Self { header, values })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecordHeader {
    pub size: Varint,
    pub datatypes: Vec<Varint>,
}

impl TryFrom<&[u8]> for RecordHeader {
    type Error = StdError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let size = Varint::new(buf);
        let datatype_size = size.value as usize - size.bytes.len() + 1;
        let datatype_buf = &buf[size.bytes.len()..datatype_size];

        let mut offset = 0;
        let mut datatypes = vec![];
        while offset < datatype_buf.len() {
            let datatype = Varint::new(&datatype_buf[offset..]);
            offset += datatype.bytes.len();
            datatypes.push(datatype);
        }
        Ok(Self { size, datatypes })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecordValue {
    pub value: RecordType,
    pub bytes: Option<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RecordType {
    Null,
    I8(i8),
    I16(i16),
    I24(i32),
    I32(i32),
    I48(i64),
    I64(i64),
    F64(f64),
    Zero(i8),
    One(i8),
    Ten,
    Eleven,
    Blob(Option<Vec<u8>>),
    Text(Option<String>),
}

impl RecordValue {
    pub fn new(code: i64, text_encoding: TextEncoding, buf: &[u8]) -> Result<Self, StdError> {
        match code {
            0 => Ok(Self {
                value: RecordType::Null,
                bytes: None,
            }),
            1 => {
                let bytes = &buf[..1];
                let value = RecordType::I8(i8::from_be_bytes(bytes.try_into()?));
                Ok(Self {
                    bytes: Some(bytes.to_vec()),
                    value,
                })
            }
            2 => {
                let bytes = &buf[..2];
                let value = RecordType::I16(i16::from_be_bytes(bytes.try_into()?));
                Ok(Self {
                    bytes: Some(bytes.to_vec()),
                    value,
                })
            }
            3 => {
                let mut bytes: [u8; 4] = [0; 4];
                let bytes_ref = &mut bytes.as_mut_slice()[1..];
                bytes_ref.copy_from_slice(&buf[..3]);
                let value = RecordType::I24(i32::from_be_bytes(bytes));
                Ok(Self {
                    bytes: Some(buf[..3].to_vec()),
                    value,
                })
            }
            4 => {
                let bytes = &buf[..4];
                let value = RecordType::I32(i32::from_be_bytes(bytes.try_into()?));
                Ok(Self {
                    bytes: Some(bytes.to_vec()),
                    value,
                })
            }
            5 => {
                let mut bytes: [u8; 8] = [0; 8];
                let bytes_ref = &mut bytes.as_mut_slice()[2..];
                bytes_ref.copy_from_slice(&buf[..6]);
                let value = RecordType::I48(i64::from_be_bytes(bytes));
                Ok(Self {
                    bytes: Some(buf[..6].to_vec()),
                    value,
                })
            }
            6 => {
                let bytes = &buf[..8];
                let value = RecordType::I64(i64::from_be_bytes(bytes.try_into()?));
                Ok(Self {
                    bytes: Some(bytes.to_vec()),
                    value,
                })
            }
            7 => {
                let bytes = &buf[..8];
                let value = RecordType::F64(f64::from_be_bytes(bytes.try_into()?));
                Ok(Self {
                    bytes: Some(bytes.to_vec()),
                    value,
                })
            }
            8 => Ok(Self {
                value: RecordType::Zero(0_i8),
                bytes: None,
            }),
            9 => Ok(Self {
                value: RecordType::One(1_i8),
                bytes: None,
            }),
            10 => Ok(Self {
                value: RecordType::Ten,
                bytes: None,
            }),
            11 => Ok(Self {
                value: RecordType::Eleven,
                bytes: None,
            }),
            n if n >= 12 && n % 2 == 0 => {
                let size = ((n - 12) / 2) as usize;
                if size > 0 {
                    let bytes = buf[..size].to_vec();
                    let value = RecordType::Blob(Some(bytes.clone()));
                    Ok(Self {
                        bytes: Some(bytes),
                        value,
                    })
                } else {
                    let value = RecordType::Blob(None);
                    Ok(Self { bytes: None, value })
                }
            }
            n if n >= 13 && n % 2 != 0 => {
                let size = ((n - 13) / 2) as usize;
                if size > 0 {
                    let bytes = &buf[..size].to_vec();
                    let value = match text_encoding {
                        TextEncoding::UTF8 => {
                            RecordType::Text(Some(std::str::from_utf8(bytes)?.to_string()))
                        }
                        TextEncoding::UTF16le => {
                            RecordType::Text(Some(String::from_utf16le(bytes)?))
                        }
                        TextEncoding::UTF16be => {
                            RecordType::Text(Some(String::from_utf16be(bytes)?))
                        }
                    };
                    Ok(Self {
                        bytes: Some(bytes.clone()),
                        value,
                    })
                } else {
                    let value = RecordType::Text(None);
                    Ok(Self { bytes: None, value })
                }
            }
            _ => unreachable!("Record Value of unknown serial type."),
        }
    }
}
