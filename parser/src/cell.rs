#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub payload_varint: Varint,
    pub rowid_varint: Varint,
}

impl TryFrom<&[u8]> for Cell {
    type Error = Box<dyn std::error::Error>;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let payload_varint = decode_varint(buf);
        let rowid_varint = decode_varint(&buf[payload_varint.bytes.len()..]);
        Ok(Cell {
            payload_varint,
            rowid_varint,
        })
    }
}

/// A variable-length integer or "varint" is a static Huffman encoding of
/// 64-bit twos-complement integers, big-endian.
///
/// A varint is between 1 and 9 bytes in length. The varint consists
/// of either zero or more bytes which have the high-order bit set followed
/// by a single byte with the high-order bit clear, or nine bytes, whichever
/// is shorter.
///
/// The lower seven bits of each of the first eight bytes and all 8 bits of the
/// ninth byte are used to reconstruct the 64-bit twos-complement integer.
#[derive(Debug, Clone, PartialEq)]
pub struct Varint {
    pub value: i64,
    pub bytes: Vec<u8>,
}

fn decode_varint(buf: &[u8]) -> Varint {
    let mut value = 0_i64;
    let mut bytes: Vec<u8> = vec![];

    // To check if higher order bit is set => varint byte, 0x80: 10000000
    let varint_mask = 0x80;
    // To drop higher order bit, 0x7F: 01111111
    let drop_msb_mask = 0x7F;

    for (n, &byte) in buf.iter().enumerate() {
        match n {
            0 => {
                // Assumption is that first byte doesn't have to have msb = 1
                value = (byte & drop_msb_mask) as i64;
                bytes.push(byte);
            }
            8 => {
                // All 8 bits of the ninth byte are used to reconstruct
                value = (value << 8) | byte as i64;
                bytes.push(byte);
                // Varint could not be longer than 9 bytes
                break;
            }
            _ => {
                if (byte & varint_mask) == 0 {
                    // Then this byte doesn't belong to varint
                    break;
                } else {
                    value = (value << 7) | (byte & drop_msb_mask) as i64;
                    bytes.push(byte);
                }
            }
        }
    }
    Varint { value, bytes }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_varint() {
        let res = decode_varint(&[0x88, 0x43]);
        assert_eq!((res.value, res.bytes), (0x08, vec![0x88]));

        let res = decode_varint(&[0x04, 0x88, 0x43]);
        assert_eq!((res.value, res.bytes), (0x208, vec![0x04, 0x88]));

        let res = decode_varint(&[0x88; 10]);
        assert_eq!((res.value, res.bytes), (1161999626690365576, vec![0x88; 9]));
    }
}
