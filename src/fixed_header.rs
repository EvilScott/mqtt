use crate::common::{Byte, Bytes, ParseError, Parseable, Serializable, VariableByteInt};

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
        VariableByteInt::new(self.remaining_length).as_bytes()
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

    pub(crate) fn from_bytes(bytes: &[Byte]) -> Result<(Self, &[Byte]), ParseError> {
        let (first_byte, first_byte_leftover) = bytes.parse_byte()?;
        let packet_type_value: u8 = first_byte >> 4;
        let dup = false; //TODO dup/qos/retain bits
        let qos: u8 = 1;
        let retain = false;
        let (remaining_length, leftover) = first_byte_leftover.parse_variable_byte_int()?;
        let fixed_header = FixedHeader {
            packet_type_value,
            dup,
            qos,
            retain,
            remaining_length: remaining_length.value(),
        };

        Ok((fixed_header, leftover))
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

#[cfg(test)]
mod tests {
    use crate::fixed_header::FixedHeader;

    #[test]
    fn test_as_bytes() {
        let packet_type_value = 1;
        let dup = false;
        let qos: u8 = 1;
        let retain = false;
        let remaining_length: u32 = 3;
        let fixed_header = FixedHeader::new(packet_type_value, dup, qos, retain, remaining_length);
        let bytes = fixed_header.as_bytes();
        assert_eq!(bytes, vec![257, 3]);
    }

    #[test]
    fn test_from_bytes() {
        todo!()
    }

    #[test]
    fn test_as_bytes_from_bytes() {
        todo!()
    }

    #[test]
    fn test_len() {
        let packet_type_value = 1;
        let dup = false;
        let qos: u8 = 1;
        let retain = false;
        let remaining_length: u32 = 3;
        let fixed_header = FixedHeader::new(packet_type_value, dup, qos, retain, remaining_length);
        let len = fixed_header.len();
        assert_eq!(len, 2);
    }
}
