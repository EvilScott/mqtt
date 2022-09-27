use crate::common::{encode_utf8_string, Bytes, ParseError, Parseable};

pub(crate) struct VariableHeader {
    keep_alive: u16,
    //TODO flags
    //TODO properties
}

impl VariableHeader {
    fn protocol_bytes(&self) -> Bytes {
        let mut bytes = encode_utf8_string("MQTT"); // protocol name
        bytes.push(5); // protocol version
        bytes
    }
    fn flag_bytes(&self) -> Bytes {
        vec![0b0000_0000]
    }
    fn keep_alive_bytes(&self) -> Bytes {
        vec![0, 0]
    } //TODO calculate from flags
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
