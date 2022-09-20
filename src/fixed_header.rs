use crate::common::{Byte, Bytes, DataType, ParseError, Parseable};

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
        DataType::VariableByteInt(self.remaining_length).as_bytes()
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
        let packet_type_value: u8;
        let dup = false;
        let qos: u8 = 1;
        let retain = false;
        let remaining_length: u32;

        let fb_leftover: &[Byte];
        if let (DataType::Byte(val), lo) = bytes.parse_byte()? {
            packet_type_value = val >> 4;
            //TODO handle dup/qos/retain bits
            fb_leftover = lo;
        } else {
            return Err(ParseError::new("malformed fixed header"));
        };

        let leftover: &[Byte];
        if let (DataType::VariableByteInt(val), lo) = fb_leftover.parse_variable_byte_int()? {
            remaining_length = val;
            leftover = lo;
        } else {
            return Err(ParseError::new("malformed fixed header"));
        }

        let fixed_header = FixedHeader {
            packet_type_value,
            dup,
            qos,
            retain,
            remaining_length,
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
