use crate::control_packet::encode_variable_length_int;

pub(crate) struct FixedHeader {
    packet_type_value: u8,
    dup: bool,
    qos: u8,
    retain: bool,
    remaining_length: u32,
}

impl FixedHeader {
    fn flags_byte(&self) -> u8 {
        let mut flags: u8 = 0;
        if self.retain { flags += 1 }
        if self.qos == 1 { flags += 2 }
        if self.qos == 2 { flags += 4 }
        if self.dup { flags += 8 }
        flags
    }

    fn remaining_length(&self) -> Vec<u8> {
        encode_variable_length_int(self.remaining_length)
    }

    pub(crate) fn new(packet_type_value: u8, dup: bool, qos: u8, retain: bool, remaining_length: u32) -> FixedHeader {
        FixedHeader { packet_type_value, dup, qos, retain, remaining_length }
    }

    pub(crate) fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![(self.packet_type_value << 4) + self.flags_byte()];
        bytes.append(&mut self.remaining_length());
        bytes
    }

    pub(crate) fn len(&self) -> u32 {
        self.as_bytes().len() as u32
    }
}
