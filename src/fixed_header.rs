use crate::common::{decode_variable_length_int, encode_variable_length_int, Byte, Bytes};

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
        if self.retain {
            flags += 1
        }
        if self.qos == 1 {
            flags += 2
        }
        if self.qos == 2 {
            flags += 4
        }
        if self.dup {
            flags += 8
        }
        flags
    }

    fn remaining_length_bytes(&self) -> Bytes {
        encode_variable_length_int(self.remaining_length)
    }

    pub(crate) fn new(
        packet_type_value: u8,
        dup: bool,
        qos: u8,
        retain: bool,
        remaining_length: u32,
    ) -> Self {
        FixedHeader {
            packet_type_value,
            dup,
            qos,
            retain,
            remaining_length,
        }
    }

    pub(crate) fn from_bytes(bytes: &[Byte]) -> (Self, &[Byte]) {
        let (rem_len, rem_len_len): (u32, usize) = decode_variable_length_int(&bytes[1..5]);
        let fixed_header = FixedHeader {
            packet_type_value: bytes[0] >> 4,
            dup: false,
            qos: 1,
            retain: false,
            remaining_length: rem_len,
        };
        (fixed_header, &bytes[1 + rem_len_len..])
    }

    pub(crate) fn as_bytes(&self) -> Bytes {
        let mut bytes: Bytes = vec![(self.packet_type_value << 4) + self.flags_byte()];
        bytes.append(&mut self.remaining_length_bytes());
        bytes
    }

    pub(crate) fn len(&self) -> u32 {
        self.as_bytes().len() as u32
    }
}
