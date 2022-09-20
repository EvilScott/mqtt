use crate::common::{decode_variable_length_int, encode_utf8_string, Byte, Bytes, ParseError};

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

    pub(crate) fn from_bytes(bytes: &[Byte]) -> Result<(Self, &[Byte]), ParseError> {
        let (prop_len, prop_len_len) = decode_variable_length_int(&bytes[11..15])?;
        let payload_start_idx = 11 + prop_len_len + prop_len as usize;
        let variable_header = VariableHeader { keep_alive: 0 }; //TODO pull from bytes
        Ok((variable_header, &bytes[payload_start_idx..]))
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
