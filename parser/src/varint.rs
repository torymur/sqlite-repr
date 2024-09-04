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
///
/// Implementation in C for reference:
/// https://github.com/sqlite/sqlite/blob/master/tool/varint.c
#[derive(Debug, Clone, PartialEq)]
pub struct Varint {
    pub value: i64,
    pub bytes: Vec<u8>,
}

impl Varint {
    pub fn new(buf: &[u8]) -> Varint {
        let mut value = 0_i64;
        let mut bytes: Vec<u8> = vec![];

        // To check if higher order bit is set => varint byte, 0x80: 10000000
        let varint_mask = 0x80;
        // To drop higher order bit, 0x7F: 01111111
        let drop_msb_mask = 0x7F;

        for (n, &byte) in buf.iter().enumerate() {
            bytes.push(byte);

            if n == 8 {
                // All 8 bits of the ninth byte are used to reconstruct
                value = (value << 8) | byte as i64;
                // Varint could not be longer than 9 bytes
                return Varint { value, bytes };
            };

            value = (value << 7) | (byte & drop_msb_mask) as i64;
            if (byte & varint_mask) == 0 {
                // Then this byte is the last one to belong to varint
                return Varint { value, bytes };
            }
        }
        Varint { value, bytes }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_varint() {
        let res = Varint::new(&[0x88, 0x43]);
        assert_eq!((res.value, res.bytes), (0x443, vec![0x88, 0x43]));

        let res = Varint::new(&[0x04, 0x88, 0x43]);
        assert_eq!((res.value, res.bytes), (0x4, vec![0x04]));

        let res = Varint::new(&[0x88; 10]);
        assert_eq!((res.value, res.bytes), (1161999626690365576, vec![0x88; 9]));
    }
}
