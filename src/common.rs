#[derive(Debug)]
pub(crate) enum DataType {
    Byte(u8),
    TwoByteInt(u16),
    FourByteInt(u32),
    VariableByteInt(u32),
    UTF8String(String),
    UTF8StringPair(String, String),
    BinaryData(Bytes),
}

impl PartialEq for DataType {
    fn eq(&self, other: &Self) -> bool {
        use DataType::*;
        match (self, other) {
            (Byte(val_1), Byte(val_2)) => val_1 == val_2,
            (TwoByteInt(val_1), TwoByteInt(val_2)) => val_1 == val_2,
            (FourByteInt(val_1), FourByteInt(val_2)) => val_1 == val_2,
            (VariableByteInt(val_1), VariableByteInt(val_2)) => val_1 == val_2,
            (UTF8String(val_1), UTF8String(val_2)) => val_1 == val_2,
            (UTF8StringPair(key_1, val_1), UTF8StringPair(key_2, val_2)) => {
                key_1 == key_2 && val_1 == val_2
            }
            (BinaryData(val_1), BinaryData(val_2)) => val_1 == val_2,
            _ => false,
        }
    }
}

pub(crate) type Byte = u8;
pub(crate) type Bytes = Vec<Byte>;

pub(crate) trait Parseable {
    fn parse_byte(&self) -> (DataType, &[Byte]);
    fn parse_two_byte_int(&self) -> (DataType, &[Byte]);
    fn parse_four_byte_int(&self) -> (DataType, &[Byte]);
    fn parse_variable_byte_int(&self) -> (DataType, &[Byte]);
    fn parse_utf8_string(&self) -> (DataType, &[Byte]);
    fn parse_utf8_string_pair(&self) -> (DataType, &[Byte]);
    fn parse_binary_data(&self) -> (DataType, &[Byte]);
}

impl Parseable for &[Byte] {
    fn parse_byte(&self) -> (DataType, &[Byte]) {
        (DataType::Byte(self[0]), &self[1..])
    }

    fn parse_two_byte_int(&self) -> (DataType, &[Byte]) {
        let bytes: [u8; 2] = [self[0] as u8, self[1] as u8];
        let val = u16::from_be_bytes(bytes);
        (DataType::TwoByteInt(val), &self[2..])
    }

    fn parse_four_byte_int(&self) -> (DataType, &[Byte]) {
        let bytes: [u8; 4] = [self[0] as u8, self[1] as u8, self[2] as u8, self[3] as u8];
        let val = u32::from_be_bytes(bytes);
        (DataType::FourByteInt(val), &self[4..])
    }

    fn parse_variable_byte_int(&self) -> (DataType, &[Byte]) {
        let (val, len) = decode_variable_length_int(&self.clone());
        (DataType::VariableByteInt(val), &self[len..])
    }

    fn parse_utf8_string(&self) -> (DataType, &[Byte]) {
        let (string, leftover) = decode_utf8_string(&self);
        (DataType::UTF8String(string), leftover)
    }

    fn parse_utf8_string_pair(&self) -> (DataType, &[Byte]) {
        let (key, key_leftover) = decode_utf8_string(&self);
        let (val, leftover) = decode_utf8_string(key_leftover);
        (DataType::UTF8StringPair(key, val), leftover)
    }

    fn parse_binary_data(&self) -> (DataType, &[Byte]) {
        let len = u16::from_be_bytes([self[0] as u8, self[1] as u8]) as usize;
        let bytes = Vec::from(&self[2..2 + len]);
        (DataType::BinaryData(bytes), &self[2 + len..])
    }
}

impl DataType {
    pub(crate) fn as_bytes(&self) -> Bytes {
        use DataType::*;
        match self {
            Byte(val) => Vec::from(val.to_be_bytes()),
            TwoByteInt(val) => Vec::from(val.to_be_bytes()),
            FourByteInt(val) => Vec::from(val.to_be_bytes()),
            VariableByteInt(val) => encode_variable_length_int(val.clone()),
            UTF8String(val) => encode_utf8_string(val),
            UTF8StringPair(key, val) => {
                let mut bytes = encode_utf8_string(key);
                bytes.append(&mut encode_utf8_string(val));
                bytes
            }
            BinaryData(data) => {
                let mut bytes = Vec::from((data.len() as u16).to_be_bytes());
                bytes.append(&mut data.clone());
                bytes
            }
        }
    }
}

pub(crate) fn encode_variable_length_int(mut int: u32) -> Bytes {
    let mut bytes: Bytes = vec![];
    loop {
        let mut byte: u8 = (int % 128) as u8;
        int = int / 128;
        if int > 0 {
            byte = byte | 128;
        }
        bytes.push(byte);
        if int == 0 {
            return bytes;
        }
    }
}

pub(crate) fn decode_variable_length_int(bytes: &[Byte]) -> (u32, usize) {
    let mut multiplier: u32 = 1;
    let mut value: u32 = 0;
    for (idx, byte) in bytes.iter().enumerate() {
        let byte_val = (byte & 127) as u32;
        value = byte_val * multiplier + value;
        multiplier = multiplier * 128;
        if byte & 128 == 0 {
            return (value, idx + 1);
        }
    }
    panic!("malformed variable length int");
}

pub(crate) fn encode_utf8_string(string: &str) -> Bytes {
    let mut bytes: Bytes = Vec::from((string.len() as u16).to_be_bytes());
    bytes.append(&mut Vec::from(string.as_bytes()));
    bytes
}

pub(crate) fn decode_utf8_string(bytes: &[Byte]) -> (String, &[Byte]) {
    let len: usize = (bytes[0] as usize * 256) + bytes[1] as usize;
    let string = String::from_utf8(Vec::from(&bytes[2..2 + len])).unwrap();
    (string, &bytes[2 + len..])
}

#[cfg(test)]
mod tests {
    use super::{
        decode_utf8_string, decode_variable_length_int, encode_utf8_string,
        encode_variable_length_int, Byte, Bytes, DataType, Parseable,
    };

    #[test]
    fn test_parse_byte() {
        let bytes: &[Byte] = &[1, 2, 3];
        let (byte, leftover) = bytes.parse_byte();
        assert_eq!(byte, DataType::Byte(1));
        assert_eq!(leftover, vec![2, 3]);
    }

    #[test]
    fn test_parse_two_byte_int() {
        let bytes: &[Byte] = &[1, 1, 2, 3];
        let (two_byte_int, leftover) = bytes.parse_two_byte_int();
        assert_eq!(two_byte_int, DataType::TwoByteInt(257));
        assert_eq!(leftover, vec![2, 3]);
    }

    #[test]
    fn test_parse_four_byte_int() {
        let bytes: &[Byte] = &[1, 1, 1, 1, 2, 3];
        let (four_byte_int, leftover) = bytes.parse_four_byte_int();
        assert_eq!(four_byte_int, DataType::FourByteInt(16_843_009));
        assert_eq!(leftover, vec![2, 3]);
    }

    #[test]
    fn test_parse_variable_byte_int() {
        let bytes: &[Byte] = &[encode_variable_length_int(578).as_slice(), &[2, 3]].concat();
        let (variable_length_int, leftover) = bytes.parse_variable_byte_int();
        assert_eq!(variable_length_int, DataType::VariableByteInt(578));
        assert_eq!(leftover, vec![2, 3]);
    }

    #[test]
    fn test_parse_utf8_string() {
        let bytes: &[Byte] = &[encode_utf8_string("foobar").as_slice(), &[2, 3]].concat();
        let (utf8_string, leftover) = bytes.parse_utf8_string();
        assert_eq!(utf8_string, DataType::UTF8String("foobar".to_string()));
        assert_eq!(leftover, vec![2, 3]);
    }

    #[test]
    fn test_parse_utf8_string_pair() {
        let encoded_1: Bytes = encode_utf8_string("foo");
        let encoded_2: Bytes = encode_utf8_string("bar");
        let data: Bytes = [encoded_1, encoded_2, vec![2, 3]].concat();
        let bytes: &[Byte] = data.as_slice();
        let (utf8_string_pair, leftover) = bytes.parse_utf8_string_pair();
        assert_eq!(
            utf8_string_pair,
            DataType::UTF8StringPair("foo".to_string(), "bar".to_string())
        );
        assert_eq!(leftover, vec![2, 3]);
    }

    #[test]
    fn test_parse_binary_data() {
        let bytes: &[Byte] = &[0, 4, 1, 1, 1, 1, 2, 3];
        let (binary_data, leftover) = bytes.parse_binary_data();
        assert_eq!(binary_data, DataType::BinaryData(vec![1, 1, 1, 1]));
        assert_eq!(leftover, vec![2, 3]);
    }

    #[test]
    fn test_parse_byte_sequence() {
        let byte_sequence: &[Byte] = &[1, 0, 2, 0, 0, 0, 3, 0, 3, 102, 111, 111];
        let (byte, leftover_1) = byte_sequence.parse_byte();
        let (two_byte_int, leftover_2) = leftover_1.parse_two_byte_int();
        let (four_byte_int, leftover_3) = leftover_2.parse_four_byte_int();
        let (utf8_string, leftover_4) = leftover_3.parse_utf8_string();
        assert_eq!(byte, DataType::Byte(1));
        assert_eq!(two_byte_int, DataType::TwoByteInt(2));
        assert_eq!(four_byte_int, DataType::FourByteInt(3));
        assert_eq!(utf8_string, DataType::UTF8String("foo".to_string()));
    }

    #[test]
    fn test_as_bytes() {
        let byte_bytes = DataType::Byte(9).as_bytes();
        assert_eq!(byte_bytes, vec![9]);
        let two_byte_int_bytes = DataType::TwoByteInt(512).as_bytes();
        assert_eq!(two_byte_int_bytes, vec![2, 0]);
        let four_byte_int_bytes = DataType::FourByteInt(4_000_000).as_bytes();
        assert_eq!(four_byte_int_bytes, vec![0, 61, 9, 0]);
        let utf8_string_bytes = DataType::UTF8String("foo".to_string()).as_bytes();
        assert_eq!(utf8_string_bytes, vec![0, 3, 102, 111, 111]);
        let utf8_string_pair_bytes =
            DataType::UTF8StringPair("foo".to_string(), "bar".to_string()).as_bytes();
        assert_eq!(
            utf8_string_pair_bytes,
            vec![0, 3, 102, 111, 111, 0, 3, 98, 97, 114]
        );
        let binary_data_bytes = DataType::BinaryData(vec![1, 2, 3, 4, 5]).as_bytes();
        assert_eq!(binary_data_bytes, vec![0, 5, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_encode_variable_length_int() {
        let actual: Bytes = encode_variable_length_int(128);
        let expected: Bytes = vec![0x80, 0x01];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_decode_variable_length_int() {
        let bytes: &[Byte] = &[0x80, 0x01, 0xFF, 0x30];
        let actual: (u32, usize) = decode_variable_length_int(bytes);
        let expected: (u32, usize) = (128, 2);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_encode_decode_variable_length_int() {
        let int: u32 = 20_668;
        let encoded: Bytes = encode_variable_length_int(int);
        let bytes: &[Byte] = encoded.as_slice();
        let (decoded, byte_num): (u32, usize) = decode_variable_length_int(bytes);
        assert_eq!(decoded, int);
        assert_eq!(byte_num, 3);
    }

    #[test]
    fn test_encode_utf8_string() {
        let actual: Bytes = encode_utf8_string("foobar");
        let expected: Bytes = vec![0, 6, 102, 111, 111, 98, 97, 114];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_encode_utf8_string_empty() {
        let actual: Bytes = encode_utf8_string("");
        let expected: Bytes = vec![0, 0];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_decode_utf8_string() {
        let bytes: &[Byte] = &[0, 3, 102, 111, 111, 1, 2, 3];
        let (string, leftover) = decode_utf8_string(bytes);
        assert_eq!(string, "foo");
        assert_eq!(leftover, vec![1, 2, 3]);
    }

    #[test]
    fn test_encode_decode_utf8_string() {
        let string = "foobar";
        let encoded: Bytes = encode_utf8_string(string);
        let bytes: &[Byte] = encoded.as_slice();
        let (decoded, _) = decode_utf8_string(bytes);
        assert_eq!(decoded, string);
    }
}
