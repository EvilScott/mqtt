pub(crate) struct FixedHeader {
    packet_type_value: u8,
    dup: bool,
    qos: u8,
    retain: bool,
}

impl FixedHeader {
    fn flags_byte(&self) -> u8 { 0b0000_0000 } //TODO calculate from flags
    fn remaining_length(&self) -> Vec<u8> { vec![0] } //TODO implement variable length int

    pub(crate) fn new(packet_type_value: u8, dup: bool, qos: u8, retain: bool) -> FixedHeader {
        FixedHeader { packet_type_value, dup, qos, retain }
    }

    pub(crate) fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![(self.packet_type_value << 4) + self.flags_byte()];
        bytes.append(&mut self.remaining_length());
        bytes
    }
}
