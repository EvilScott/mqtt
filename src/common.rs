use std::fmt;

pub(crate) type Byte = u8;
pub(crate) type Bytes = Vec<Byte>;

#[derive(Debug, PartialEq)]
pub(crate) struct TwoByteInt(u16);
#[derive(Debug, PartialEq)]
pub(crate) struct FourByteInt(u32);
#[derive(Debug, PartialEq)]
pub(crate) struct VariableByteInt(u32);
#[derive(Debug, PartialEq)]
pub(crate) struct UTF8String(String);
#[derive(Debug, PartialEq)]
pub(crate) struct UTF8StringPair(String, String);
#[derive(Debug, PartialEq)]
pub(crate) struct BinaryData(Bytes);

#[derive(Debug, Clone)]
pub(crate) struct ParseError(&'static str);

impl ParseError {
    pub(crate) fn new(message: &'static str) -> Self {
        ParseError(message)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error: {}", self.0)
    }
}

pub(crate) trait Parseable {
    fn parse_byte(&self) -> Result<(Byte, &[Byte]), ParseError>;
    fn parse_two_byte_int(&self) -> Result<(TwoByteInt, &[Byte]), ParseError>;
    fn parse_four_byte_int(&self) -> Result<(FourByteInt, &[Byte]), ParseError>;
    fn parse_variable_byte_int(&self) -> Result<(VariableByteInt, &[Byte]), ParseError>;
    fn parse_utf8_string(&self) -> Result<(UTF8String, &[Byte]), ParseError>;
    fn parse_utf8_string_pair(&self) -> Result<(UTF8StringPair, &[Byte]), ParseError>;
    fn parse_binary_data(&self) -> Result<(BinaryData, &[Byte]), ParseError>;
}

impl Parseable for &[Byte] {
    fn parse_byte(&self) -> Result<(Byte, &[Byte]), ParseError> {
        Ok((self[0], &self[1..]))
    }

    fn parse_two_byte_int(&self) -> Result<(TwoByteInt, &[Byte]), ParseError> {
        let bytes: [u8; 2] = [self[0] as u8, self[1] as u8];
        let val = u16::from_be_bytes(bytes);
        Ok((TwoByteInt(val), &self[2..]))
    }

    fn parse_four_byte_int(&self) -> Result<(FourByteInt, &[Byte]), ParseError> {
        let bytes: [u8; 4] = [self[0] as u8, self[1] as u8, self[2] as u8, self[3] as u8];
        let val = u32::from_be_bytes(bytes);
        Ok((FourByteInt(val), &self[4..]))
    }

    fn parse_variable_byte_int(&self) -> Result<(VariableByteInt, &[Byte]), ParseError> {
        let (val, len) = decode_variable_length_int(&self.clone())?;
        Ok((VariableByteInt(val), &self[len..]))
    }

    fn parse_utf8_string(&self) -> Result<(UTF8String, &[Byte]), ParseError> {
        let (string, leftover) = decode_utf8_string(&self)?;
        Ok((UTF8String(string), leftover))
    }

    fn parse_utf8_string_pair(&self) -> Result<(UTF8StringPair, &[Byte]), ParseError> {
        let (key, key_leftover) = decode_utf8_string(&self)?;
        let (val, leftover) = decode_utf8_string(key_leftover)?;
        Ok((UTF8StringPair(key, val), leftover))
    }

    fn parse_binary_data(&self) -> Result<(BinaryData, &[Byte]), ParseError> {
        let len = u16::from_be_bytes([self[0] as u8, self[1] as u8]) as usize;
        let bytes = Vec::from(&self[2..2 + len]);
        Ok((BinaryData(bytes), &self[2 + len..]))
    }
}

pub(crate) trait Serializable {
    fn as_bytes(&self) -> Bytes;
}

impl Serializable for Byte {
    fn as_bytes(&self) -> Bytes {
        Vec::from(self.to_be_bytes())
    }
}

impl TwoByteInt {
    pub(crate) fn new(val: u16) -> Self {
        TwoByteInt(val)
    }

    pub(crate) fn value(&self) -> u16 {
        self.0
    }
}

impl Serializable for TwoByteInt {
    fn as_bytes(&self) -> Bytes {
        Vec::from(self.0.to_be_bytes())
    }
}

impl FourByteInt {
    pub(crate) fn new(val: u32) -> Self {
        FourByteInt(val)
    }

    pub(crate) fn value(&self) -> u32 {
        self.0
    }
}

impl Serializable for FourByteInt {
    fn as_bytes(&self) -> Bytes {
        Vec::from(self.0.to_be_bytes())
    }
}

impl VariableByteInt {
    pub(crate) fn new(val: u32) -> Self {
        VariableByteInt(val)
    }

    pub(crate) fn value(&self) -> u32 {
        self.0
    }
}

impl Serializable for VariableByteInt {
    fn as_bytes(&self) -> Bytes {
        encode_variable_length_int(self.0.clone())
    }
}

impl UTF8String {
    pub(crate) fn new(val: &str) -> Self {
        UTF8String(val.to_string())
    }

    pub(crate) fn value(&self) -> &str {
        &self.0
    }
}

impl Serializable for UTF8String {
    fn as_bytes(&self) -> Bytes {
        encode_utf8_string(&self.0)
    }
}

impl UTF8StringPair {
    pub(crate) fn new(key: &str, val: &str) -> Self {
        UTF8StringPair(key.to_string(), val.to_string())
    }

    pub(crate) fn value(&self) -> (&str, &str) {
        (&self.0, &self.1)
    }
}

impl Serializable for UTF8StringPair {
    fn as_bytes(&self) -> Bytes {
        let mut bytes = encode_utf8_string(&self.0);
        bytes.append(&mut encode_utf8_string(&self.1));
        bytes
    }
}

impl BinaryData {
    pub(crate) fn new(val: Bytes) -> Self {
        BinaryData(val)
    }

    pub(crate) fn value(&self) -> &[Byte] {
        &self.0
    }
}

impl Serializable for BinaryData {
    fn as_bytes(&self) -> Bytes {
        let mut bytes = Vec::from((self.0.len() as u16).to_be_bytes());
        bytes.append(&mut self.0.clone());
        bytes
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

pub(crate) fn decode_variable_length_int(bytes: &[Byte]) -> Result<(u32, usize), ParseError> {
    let mut multiplier: u32 = 1;
    let mut value: u32 = 0;
    for (idx, byte) in bytes.iter().enumerate() {
        let byte_val = (byte & 127) as u32;
        value = byte_val * multiplier + value;
        multiplier = multiplier * 128;
        if byte & 128 == 0 {
            return Ok((value, idx + 1));
        }
    }
    Err(ParseError::new("malformed variable length int"))
}

pub(crate) fn encode_utf8_string(string: &str) -> Bytes {
    let mut bytes: Bytes = Vec::from((string.len() as u16).to_be_bytes());
    bytes.append(&mut Vec::from(string.as_bytes()));
    bytes
}

pub(crate) fn decode_utf8_string(bytes: &[Byte]) -> Result<(String, &[Byte]), ParseError> {
    let len: usize = (bytes[0] as usize * 256) + bytes[1] as usize;
    let string = String::from_utf8(Vec::from(&bytes[2..2 + len])).unwrap(); //TODO check for error here
    Ok((string, &bytes[2 + len..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_byte() {
        let bytes: &[Byte] = &[1, 2, 3];
        let (byte, leftover) = bytes.parse_byte().unwrap();
        assert_eq!(byte, 1);
        assert_eq!(leftover, vec![2, 3]);
    }

    #[test]
    fn test_parse_two_byte_int() {
        let bytes: &[Byte] = &[1, 1, 2, 3];
        let (two_byte_int, leftover) = bytes.parse_two_byte_int().unwrap();
        assert_eq!(two_byte_int, TwoByteInt::new(257));
        assert_eq!(leftover, vec![2, 3]);
    }

    #[test]
    fn test_parse_four_byte_int() {
        let bytes: &[Byte] = &[1, 1, 1, 1, 2, 3];
        let (four_byte_int, leftover) = bytes.parse_four_byte_int().unwrap();
        assert_eq!(four_byte_int, FourByteInt::new(16_843_009));
        assert_eq!(leftover, vec![2, 3]);
    }

    #[test]
    fn test_parse_variable_byte_int() {
        let bytes: &[Byte] = &[encode_variable_length_int(578).as_slice(), &[2, 3]].concat();
        let (variable_length_int, leftover) = bytes.parse_variable_byte_int().unwrap();
        assert_eq!(variable_length_int, VariableByteInt::new(578));
        assert_eq!(leftover, vec![2, 3]);
    }

    #[test]
    fn test_parse_utf8_string() {
        let bytes: &[Byte] = &[encode_utf8_string("foobar").as_slice(), &[2, 3]].concat();
        let (utf8_string, leftover) = bytes.parse_utf8_string().unwrap();
        assert_eq!(utf8_string, UTF8String::new("foobar"));
        assert_eq!(leftover, vec![2, 3]);
    }

    #[test]
    fn test_parse_utf8_string_pair() {
        let encoded_1: Bytes = encode_utf8_string("foo");
        let encoded_2: Bytes = encode_utf8_string("bar");
        let data: Bytes = [encoded_1, encoded_2, vec![2, 3]].concat();
        let bytes: &[Byte] = data.as_slice();
        let (utf8_string_pair, leftover) = bytes.parse_utf8_string_pair().unwrap();
        assert_eq!(utf8_string_pair, UTF8StringPair::new("foo", "bar"));
        assert_eq!(leftover, vec![2, 3]);
    }

    #[test]
    fn test_parse_binary_data() {
        let bytes: &[Byte] = &[0, 4, 1, 1, 1, 1, 2, 3];
        let (binary_data, leftover) = bytes.parse_binary_data().unwrap();
        assert_eq!(binary_data, BinaryData::new(vec![1, 1, 1, 1]));
        assert_eq!(leftover, vec![2, 3]);
    }

    #[test]
    fn test_parse_byte_sequence() {
        let byte_sequence: &[Byte] = &[1, 0, 2, 0, 0, 0, 3, 0, 3, 102, 111, 111];
        let (byte, leftover_1) = byte_sequence.parse_byte().unwrap();
        let (two_byte_int, leftover_2) = leftover_1.parse_two_byte_int().unwrap();
        let (four_byte_int, leftover_3) = leftover_2.parse_four_byte_int().unwrap();
        let (utf8_string, _) = leftover_3.parse_utf8_string().unwrap();
        assert_eq!(byte, 1);
        assert_eq!(two_byte_int, TwoByteInt::new(2));
        assert_eq!(four_byte_int, FourByteInt::new(3));
        assert_eq!(utf8_string, UTF8String::new("foo"));
    }

    #[test]
    fn test_as_bytes() {
        let two_byte_int_bytes = TwoByteInt(512).as_bytes();
        assert_eq!(two_byte_int_bytes, vec![2, 0]);
        let four_byte_int_bytes = FourByteInt(4_000_000).as_bytes();
        assert_eq!(four_byte_int_bytes, vec![0, 61, 9, 0]);
        let utf8_string_bytes = UTF8String("foo".to_string()).as_bytes();
        assert_eq!(utf8_string_bytes, vec![0, 3, 102, 111, 111]);
        let utf8_string_pair_bytes =
            UTF8StringPair("foo".to_string(), "bar".to_string()).as_bytes();
        assert_eq!(
            utf8_string_pair_bytes,
            vec![0, 3, 102, 111, 111, 0, 3, 98, 97, 114]
        );
        let binary_data_bytes = BinaryData(vec![1, 2, 3, 4, 5]).as_bytes();
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
        let actual: (u32, usize) = decode_variable_length_int(bytes).unwrap();
        let expected: (u32, usize) = (128, 2);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_encode_decode_variable_length_int() {
        let int: u32 = 20_668;
        let encoded: Bytes = encode_variable_length_int(int);
        let bytes: &[Byte] = encoded.as_slice();
        let (decoded, byte_num): (u32, usize) = decode_variable_length_int(bytes).unwrap();
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
        let (string, leftover) = decode_utf8_string(bytes).unwrap();
        assert_eq!(string, "foo");
        assert_eq!(leftover, vec![1, 2, 3]);
    }

    #[test]
    fn test_encode_decode_utf8_string() {
        let string = "foobar";
        let encoded: Bytes = encode_utf8_string(string);
        let bytes: &[Byte] = encoded.as_slice();
        let (decoded, _) = decode_utf8_string(bytes).unwrap();
        assert_eq!(decoded, string);
    }
}
