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

struct FixedHeader {
    packet_type_value: u8,
    dup: bool,
    qos: u8,
    retain: bool,
    remaining_length: u8,
}

impl FixedHeader {
    fn flags_byte(&self) -> u8 { 0b0000_0000 } //TODO calculate from flags
    fn remaining_length(&self) -> Vec<u8> { vec![0] } //TODO implement variable length int
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![(self.packet_type_value << 4) + self.flags_byte()];
        bytes.append(&mut self.remaining_length());
        bytes
    }
}

struct VariableHeader {
    keep_alive: u16,
    //TODO flags
    //TODO properties
}

impl VariableHeader {
    fn protocol_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![0, 4]; // protocol name msb/lsb
        bytes.append(&mut Vec::from("MQTT".as_bytes()));  // protocol name
        bytes.push(5); // protocol version
        bytes
    }
    fn flags_byte(&self) -> u8 { 0b0000_0000 }
    fn keep_alive_bytes(&self) -> Vec<u8> { vec![0, 0] } //TODO calculate from flags
    fn property_bytes(&self) -> Vec<u8> { vec![0] } //TODO calculate from properties
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = self.property_bytes();
        bytes.push(self.flags_byte());
        bytes.append(&mut self.keep_alive_bytes());
        bytes.append(&mut self.property_bytes());
        bytes
    }
}

struct Payload {
    //TODO add payload values
}

impl Payload {
    fn as_bytes(&self) -> Vec<u8> {
        vec![0] //TODO calculate length and values from struct
    }
}

pub(crate) trait ControlPacket {
    fn get_fixed_header(&self) -> &FixedHeader;
    fn fixed_header_bytes(&self) -> Vec<u8> { self.get_fixed_header().as_bytes() }

    fn get_variable_header(&self) -> &VariableHeader;
    fn variable_header_bytes(&self) -> Vec<u8> { self.get_variable_header().as_bytes() }

    fn get_payload(&self) -> &Payload;
    fn payload_bytes(&self) -> Vec<u8> { self.get_payload().as_bytes() }

    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.append(&mut self.fixed_header_bytes());
        bytes.append(&mut self.variable_header_bytes());
        bytes.append(&mut self.payload_bytes());
        bytes
    }
}

// pub(crate) fn parse_packet_bytes(bytes: Vec<u8>) -> &impl ControlPacket {
//     //TODO
// }
