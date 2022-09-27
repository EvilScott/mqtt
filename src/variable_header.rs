use crate::common::{Bytes, ParseError, Parseable, Serializable, TwoByteInt, UTF8String};

const PROTOCOL_NAME: &str = "MQTT";
const PROTOCOL_VERSION: u8 = 5;

#[derive(Debug, PartialEq)]
pub(crate) struct VariableHeader {
    keep_alive: u16,
    //TODO flags
    //TODO properties
}

impl VariableHeader {
    fn protocol_bytes(&self) -> Bytes {
        let protocol_name = UTF8String::new(PROTOCOL_NAME);
        let mut bytes = protocol_name.as_bytes();
        bytes.push(PROTOCOL_VERSION);
        bytes
    }
    fn flag_bytes(&self) -> Bytes {
        vec![0b0000_0000]
    }
    fn keep_alive_bytes(&self) -> Bytes {
        TwoByteInt::new(self.keep_alive).as_bytes()
    }
    fn property_bytes(&self) -> Bytes {
        vec![0]
    } //TODO calculate from properties

    pub(crate) fn new(keep_alive: u16) -> Self {
        VariableHeader { keep_alive }
    }

    pub(crate) fn from_bytes(bytes: Bytes) -> Result<(Self, Bytes), ParseError> {
        let byte_slice = &bytes[..];
        let (_protocol_name, pn_leftover) = byte_slice.parse_utf8_string()?;
        let (_protocol_version, pv_leftover) = pn_leftover.parse_byte()?;
        let (_flag_byte, f_leftover) = pv_leftover.parse_byte()?;
        let (keep_alive, ka_leftover) = f_leftover.parse_two_byte_int()?;
        let (prop_len, prop_len_leftover) = ka_leftover.parse_variable_byte_int()?;
        if prop_len.value() > 0 {
            return Err(ParseError::new("TODO handle property length > 0"));
        }
        let variable_header = VariableHeader::new(keep_alive.value());
        Ok((variable_header, Vec::from(prop_len_leftover)))
    }

    pub(crate) fn as_bytes(&self) -> Bytes {
        let mut bytes = self.protocol_bytes();
        bytes.append(&mut self.flag_bytes());
        bytes.append(&mut self.keep_alive_bytes());
        bytes.append(&mut self.property_bytes());
        bytes
    }

    pub(crate) fn len(&self) -> u32 {
        self.as_bytes().len() as u32
    }
}

#[cfg(test)]
mod tests {
    use crate::variable_header::VariableHeader;
    use std::env::var;

    const KEEP_ALIVE: u16 = 3;

    #[test]
    fn test_as_bytes() {
        let variable_header = VariableHeader::new(KEEP_ALIVE);
        let bytes = variable_header.as_bytes();
        assert_eq!(bytes, vec![0, 4, 77, 81, 84, 84, 5, 0, 0, 3, 0]);
    }

    #[test]
    fn test_from_bytes() {
        let bytes = vec![0, 4, 77, 81, 84, 84, 5, 0, 0, 3, 0, 2, 3];
        let (parsed_variable_header, leftover) = VariableHeader::from_bytes(bytes).unwrap();
        let variable_header = VariableHeader::new(KEEP_ALIVE);
        assert_eq!(parsed_variable_header, variable_header);
        assert_eq!(leftover, vec![2, 3]);
    }

    #[test]
    fn test_as_bytes_from_bytes() {
        let variable_header = VariableHeader::new(KEEP_ALIVE);
        let bytes = variable_header.as_bytes();
        let (parsed_variable_header, _leftover) = VariableHeader::from_bytes(bytes).unwrap();
        assert_eq!(parsed_variable_header, variable_header);
    }

    #[test]
    fn test_len() {
        let variable_header = VariableHeader::new(KEEP_ALIVE);
        let len = variable_header.len();
        assert_eq!(len, 11);
    }
}
