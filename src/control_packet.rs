use crate::fixed_header::FixedHeader;
use crate::variable_header::VariableHeader;
use crate::payload::Payload;
use crate::control_packet::connect::Connect;

pub(crate) mod connect;

pub(crate) enum PacketType {
    CONNECT,
    CONNACK,
    PUBLISH,
    PUBACK,
    PUBREC,
    PUBREL,
    PUBCOMP,
    SUBSCRIBE,
    SUBACK,
    UNSUBSCRIBE,
    UNSUBACK,
    PINGREQ,
    PINGRESP,
    DISCONNECT,
    AUTH,
}

pub(crate) trait ControlPacket {
    fn get_fixed_header(&self) -> &FixedHeader;
    fn fixed_header_bytes(&self) -> Vec<u8> { self.get_fixed_header().as_bytes() }

    fn get_variable_header(&self) -> &VariableHeader;
    fn variable_header_bytes(&self) -> Vec<u8> { self.get_variable_header().as_bytes() }

    fn get_payload(&self) -> &Payload;
    fn payload_bytes(&self) -> Vec<u8> { self.get_payload().as_bytes() }

    fn from_bytes(bytes: Vec<u8>) -> Self where Self: Sized;
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.append(&mut self.fixed_header_bytes());
        bytes.append(&mut self.variable_header_bytes());
        bytes.append(&mut self.payload_bytes());
        bytes
    }
}

pub(crate) fn parse_packet_bytes(bytes: Vec<u8>) -> Box<dyn ControlPacket> {
    let first_byte = bytes[0];
    match first_byte >> 4 {
        1 => Box::new(Connect::from_bytes(bytes)),
        //TODO other types here
        _ => panic!("unknown package type"),
    }
}

#[cfg(test)]
mod tests {
    use super::{encode_variable_length_int, decode_variable_length_int, encode_utf8_string};

    #[test]
    fn test_encode_variable_length_int() {
        let actual: Vec<u8> = encode_variable_length_int(128);
        let expected: Vec<u8> = vec![0x80, 0x01];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_decode_variable_length_int() {
        let bytes: Vec<u8> = vec![0x80, 0x01, 0xFF, 0x30];
        let actual: (u32, usize) = decode_variable_length_int(bytes);
        let expected: (u32, usize) = (128, 2);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_encode_decode_variable_length_int() {
        let int: u32 = 20_668;
        let encoded: Vec<u8> = encode_variable_length_int(int);
        let (decoded, byte_num): (u32, usize) = decode_variable_length_int(encoded);
        assert_eq!(decoded, int);
        assert_eq!(byte_num, 3);
    }

    #[test]
    fn test_encode_utf8_string() {
        let actual: Vec<u8> = encode_utf8_string("foobar");
        let expected: Vec<u8> = vec![0,6,102,111,111,98,97,114];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_encode_utf8_string_empty() {
        let actual: Vec<u8> = encode_utf8_string("");
        let expected: Vec<u8> = vec![0,0];
        assert_eq!(actual, expected);
    }
}

pub(crate) fn encode_variable_length_int(mut int: u32) -> Vec<u8> {
    let mut bytes: Vec<u8> = vec![];
    loop {
        let mut byte: u8 = (int % 128) as u8;
        int = int / 128;
        if int > 0 { byte = byte | 128; }
        bytes.push(byte);
        if int == 0 { return bytes; }
    }
}

pub(crate) fn decode_variable_length_int(bytes: Vec<u8>) -> (u32, usize) {
    let mut multiplier: u32 = 1;
    let mut value: u32 = 0;
    for (idx, byte) in bytes.iter().enumerate() {
        value = value + ((byte & 127) as u32 * multiplier) as u32;
        multiplier = multiplier * 128;
        if byte & 128 == 0 { return (value, idx+1); }
    }
    panic!("malformed variable length int");
}

pub(crate) fn encode_utf8_string(string: &str) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::from((string.len() as u16).to_be_bytes());
    bytes.append(&mut Vec::from(string.as_bytes()));
    bytes
}
