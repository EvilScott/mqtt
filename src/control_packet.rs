use crate::control_packet::connect::Connect;
use crate::fixed_header::FixedHeader;
use crate::payload::Payload;
use crate::variable_header::VariableHeader;

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

pub(crate) fn encode_utf8_string(string: &str) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::from(string.len().to_be_bytes());
    bytes.append(&mut Vec::from(string.as_bytes()));
    bytes
}
