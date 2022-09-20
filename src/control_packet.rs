use crate::common::{Byte, Bytes, ParseError};
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
    fn fixed_header_bytes(&self) -> Bytes {
        self.get_fixed_header().as_bytes()
    }

    fn get_variable_header(&self) -> &VariableHeader;
    fn variable_header_bytes(&self) -> Bytes {
        self.get_variable_header().as_bytes()
    }

    fn get_payload(&self) -> &Payload;
    fn payload_bytes(&self) -> Bytes {
        self.get_payload().as_bytes()
    }

    fn from_bytes(bytes: &[Byte]) -> Result<Self, ParseError>
    where
        Self: Sized;
    fn as_bytes(&self) -> Bytes {
        let mut bytes = Vec::new();
        bytes.append(&mut self.fixed_header_bytes());
        bytes.append(&mut self.variable_header_bytes());
        bytes.append(&mut self.payload_bytes());
        bytes
    }
}

pub(crate) fn parse_packet_bytes(bytes: &[Byte]) -> Result<Box<dyn ControlPacket>, &'static str> {
    let first_byte = bytes[0];
    match first_byte >> 4 {
        1 => Ok(Box::new(Connect::from_bytes(bytes).unwrap())),
        //TODO other types here
        _ => Err("unknown package type"),
    }
}
