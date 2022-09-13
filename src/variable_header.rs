use crate::control_packet::encode_utf8_string;

pub(crate) struct VariableHeader {
    keep_alive: u16,
    //TODO flags
    //TODO properties
}

impl VariableHeader {
    fn protocol_bytes(&self) -> Vec<u8> {
        let mut bytes = encode_utf8_string("MQTT"); // protocol name
        bytes.push(5); // protocol version
        bytes
    }
    fn flags_byte(&self) -> u8 { 0b0000_0000 }
    fn keep_alive_bytes(&self) -> Vec<u8> { vec![0, 0] } //TODO calculate from flags
    fn property_bytes(&self) -> Vec<u8> { vec![0] } //TODO calculate from properties

    pub(crate) fn new(keep_alive: u16) -> VariableHeader {
        VariableHeader { keep_alive }
    }

    pub(crate) fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = self.property_bytes();
        bytes.push(self.flags_byte());
        bytes.append(&mut self.keep_alive_bytes());
        bytes.append(&mut self.property_bytes());
        bytes
    }
}
