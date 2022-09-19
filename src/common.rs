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
            (UTF8StringPair(key_1, val_1), UTF8StringPair(key_2, val_2)) => key_1 == key_2 && val_1 == val_2,
            (BinaryData(val_1), BinaryData(val_2)) => val_1 == val_2,
            _ => false
        }
    }
}

pub(crate) type Byte = u8;
pub(crate) type Bytes = Vec<Byte>;

pub(crate) trait Parseable {
    fn parse_byte(&mut self) -> DataType;
    fn parse_two_byte_int(&mut self) -> DataType;
    fn parse_four_byte_int(&mut self) -> DataType;
    fn parse_variable_byte_int(&mut self) -> DataType;
    fn parse_utf8_string(&mut self) -> DataType;
    fn parse_utf8_string_pair(&mut self) -> DataType;
    fn parse_binary_data(&mut self) -> DataType;
}

impl Parseable for Bytes {
    fn parse_byte(&mut self) -> DataType {
        let byte = DataType::Byte(self[0]);
        *self = Vec::from(&self[1..]);
        byte
    }

    fn parse_two_byte_int(&mut self) -> DataType {
        let bytes: [u8; 2] = [self[0] as u8, self[1] as u8];
        let val = u16::from_be_bytes(bytes);
        let two_byte_int = DataType::TwoByteInt(val);
        *self = Vec::from(&self[2..]);
        two_byte_int
    }

    fn parse_four_byte_int(&mut self) -> DataType {
        let bytes: [u8; 4] = [self[0] as u8, self[1] as u8, self[2] as u8, self[3] as u8];
        let val = u32::from_be_bytes(bytes);
        let four_byte_int = DataType::FourByteInt(val);
        *self = Vec::from(&self[4..]);
        four_byte_int
    }

    fn parse_variable_byte_int(&mut self) -> DataType {
        let (val, len) = decode_variable_length_int(self.clone());
        let variable_byte_int = DataType::VariableByteInt(val);
        *self = Vec::from(&self[len..]);
        variable_byte_int
    }

    fn parse_utf8_string(&mut self) -> DataType {
        let (string, leftover) = decode_utf8_string(self.clone());
        let utf8_string = DataType::UTF8String(string);
        *self = leftover;
        utf8_string
    }

    fn parse_utf8_string_pair(&mut self) -> DataType {
        let (key, key_leftover) = decode_utf8_string(self.clone());
        let (val, leftover) = decode_utf8_string(key_leftover);
        let utf8_string_pair = DataType::UTF8StringPair(key, val);
        *self = leftover;
        utf8_string_pair
    }

    fn parse_binary_data(&mut self) -> DataType {
        let len = u16::from_be_bytes([self[0] as u8, self[1] as u8]) as usize;
        let bytes = Vec::from(&self[2..2+len]);
        let binary_data = DataType::BinaryData(bytes);
        *self = Vec::from(&self[2+len..]);
        binary_data
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
            },
            BinaryData(data) => {
                let mut bytes = Vec::from((data.len() as u16).to_be_bytes());
                bytes.append(&mut data.clone());
                bytes
            },
        }
    }
}

pub(crate) fn encode_variable_length_int(mut int: u32) -> Bytes {
    let mut bytes: Bytes = vec![];
    loop {
        let mut byte: u8 = (int % 128) as u8;
        int = int / 128;
        if int > 0 { byte = byte | 128; }
        bytes.push(byte);
        if int == 0 { return bytes; }
    }
}

pub(crate) fn decode_variable_length_int(bytes: Bytes) -> (u32, usize) {
    let mut multiplier: u32 = 1;
    let mut value: u32 = 0;
    for (idx, byte) in bytes.iter().enumerate() {
        let byte_val = (byte & 127) as u32;
        value = byte_val * multiplier + value;
        multiplier = multiplier * 128;
        if byte & 128 == 0 { return (value, idx+1); }
    }
    panic!("malformed variable length int");
}

pub(crate) fn encode_utf8_string(string: &str) -> Bytes {
    let mut bytes: Bytes = Vec::from((string.len() as u16).to_be_bytes());
    bytes.append(&mut Vec::from(string.as_bytes()));
    bytes
}

pub(crate) fn decode_utf8_string(bytes: Bytes) -> (String, Bytes) {
    let len: usize = (bytes[0] as usize * 256) + bytes[1] as usize;
    let string = String::from_utf8(Vec::from(&bytes[2..2+len])).unwrap();
    (string, Vec::from(&bytes[2+len..]))
}

#[cfg(test)]
mod tests {
    use super::{Parseable, DataType, Bytes,
                encode_variable_length_int, decode_variable_length_int,
                encode_utf8_string, decode_utf8_string};

    #[test]
    fn test_parse_byte() {
        let mut bytes: Bytes = vec![1, 2, 3];
        let byte = bytes.parse_byte();
        assert_eq!(byte, DataType::Byte(1));
        assert_eq!(bytes, vec![2,3]);
    }

    #[test]
    fn test_parse_two_byte_int() {
        let mut bytes: Bytes = vec![1,1,2,3];
        let two_byte_int = bytes.parse_two_byte_int();
        assert_eq!(two_byte_int, DataType::TwoByteInt(257));
        assert_eq!(bytes, vec![2,3]);
    }

    #[test]
    fn test_parse_four_byte_int() {
        let mut bytes: Bytes = vec![1,1,1,1,2,3];
        let four_byte_int = bytes.parse_four_byte_int();
        assert_eq!(four_byte_int, DataType::FourByteInt(16_843_009));
        assert_eq!(bytes, vec![2,3]);
    }

    #[test]
    fn test_parse_variable_byte_int() {
        let mut bytes = encode_variable_length_int(578);
        bytes.append(&mut vec![2,3]);
        let variable_length_int= bytes.parse_variable_byte_int();
        assert_eq!(variable_length_int, DataType::VariableByteInt(578));
        assert_eq!(bytes, vec![2,3]);
    }

    #[test]
    fn test_parse_utf8_string() {
        let mut bytes = encode_utf8_string("foobar");
        bytes.append(&mut vec![2,3]);
        let utf8_string = bytes.parse_utf8_string();
        assert_eq!(utf8_string, DataType::UTF8String("foobar".to_string()));
        assert_eq!(bytes, vec![2,3]);
    }

    #[test]
    fn test_parse_utf8_string_pair() {
        let mut bytes = encode_utf8_string("foo");
        bytes.append(&mut encode_utf8_string("bar"));
        bytes.append(&mut vec![2,3]);
        let utf8_string_pair = bytes.parse_utf8_string_pair();
        assert_eq!(utf8_string_pair, DataType::UTF8StringPair("foo".to_string(), "bar".to_string()));
        assert_eq!(bytes, vec![2,3]);

    }

    #[test]
    fn test_parse_binary_data() {
        let mut bytes: Bytes = vec![0,4,1,1,1,1,2,3];
        let binary_data = bytes.parse_binary_data();
        assert_eq!(binary_data, DataType::BinaryData(vec![1,1,1,1]));
        assert_eq!(bytes, vec![2,3]);
    }

    #[test]
    fn test_as_bytes() {
        let byte_bytes = DataType::Byte(9).as_bytes();
        assert_eq!(byte_bytes, vec![9]);
        let two_byte_int_bytes = DataType::TwoByteInt(512).as_bytes();
        assert_eq!(two_byte_int_bytes, vec![2,0]);
        let four_byte_int_bytes = DataType::FourByteInt(4_000_000).as_bytes();
        assert_eq!(four_byte_int_bytes, vec![0,61,9,0]);
        let utf8_string_bytes = DataType::UTF8String("foo".to_string()).as_bytes();
        assert_eq!(utf8_string_bytes, vec![0,3,102,111,111]);
        let utf8_string_pair_bytes = DataType::UTF8StringPair("foo".to_string(), "bar".to_string()).as_bytes();
        assert_eq!(utf8_string_pair_bytes, vec![0,3,102,111,111,0,3,98,97,114]);
        let binary_data_bytes = DataType::BinaryData(vec![1,2,3,4,5]).as_bytes();
        assert_eq!(binary_data_bytes, vec![0,5,1,2,3,4,5]);
    }

    #[test]
    fn test_encode_variable_length_int() {
        let actual: Bytes = encode_variable_length_int(128);
        let expected: Bytes = vec![0x80, 0x01];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_decode_variable_length_int() {
        let bytes: Bytes = vec![0x80, 0x01, 0xFF, 0x30];
        let actual: (u32, usize) = decode_variable_length_int(bytes);
        let expected: (u32, usize) = (128, 2);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_encode_decode_variable_length_int() {
        let int: u32 = 20_668;
        let encoded: Bytes = encode_variable_length_int(int);
        let (decoded, byte_num): (u32, usize) = decode_variable_length_int(encoded);
        assert_eq!(decoded, int);
        assert_eq!(byte_num, 3);
    }

    #[test]
    fn test_encode_utf8_string() {
        let actual: Bytes = encode_utf8_string("foobar");
        let expected: Bytes = vec![0,6,102,111,111,98,97,114];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_encode_utf8_string_empty() {
        let actual: Bytes = encode_utf8_string("");
        let expected: Bytes = vec![0,0];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_decode_utf8_string() {
        let bytes: Bytes = vec![0,3,102,111,111,1,2,3];
        let (string, leftover) = decode_utf8_string(bytes);
        assert_eq!(string, "foo");
        assert_eq!(leftover, vec![1,2,3]);
    }

    #[test]
    fn test_encode_decode_utf8_string() {
        let string = "foobar";
        let encoded = encode_utf8_string(string);
        let (decoded, _) = decode_utf8_string(encoded);
        assert_eq!(decoded, string);
    }
}