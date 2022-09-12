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
    fn packet_type_value(&self) -> u8;
    fn remaining_length(&self) -> Vec<u8> { vec![0] } //TODO implement variable length int
    fn fixed_header_flags(&self) -> u8 { 0b0000_0000 }
    fn fixed_header_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![(self.packet_type_value() << 4) + self.fixed_header_flags()];
        bytes.append(&mut self.remaining_length());
        bytes
    }

    fn protocol_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![0, 4]; // protocol name msb/lsb
        bytes.append(&mut Vec::from("MQTT".as_bytes()));  // protocol name
        bytes.push(5); // protocol version
        bytes
    }
    fn variable_header_flags(&self) -> u8 { 0b0000_0000 }
    fn keep_alive_bytes(&self) -> Vec<u8> { vec![0, 0] }
    fn property_length(&self) -> Vec<u8> { vec![0] } //TODO implement variable length int
    fn property_bytes(&self) -> Vec<u8> { vec![] }
    fn variable_header_bytes(&self) -> Vec<u8> {
        let mut bytes = self.protocol_bytes();
        bytes.push(self.variable_header_flags());
        bytes.append(&mut self.keep_alive_bytes());
        bytes
    }

    fn payload_bytes(&self) -> Vec<u8> { vec![] } // defaults to empty

    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.append(&mut self.fixed_header_bytes());
        bytes.append(&mut self.variable_header_bytes());
        bytes.append(&mut self.payload_bytes());
        bytes
    }
}
