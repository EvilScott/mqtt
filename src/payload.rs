use crate::common::{Byte, Bytes, decode_utf8_string, encode_utf8_string};
use crate::payload::PayloadValue::EncodedString;

pub(crate) enum PayloadValue {
    EncodedString(String),
    Plain(String),
}

impl PayloadValue {
    pub(crate) fn as_bytes(&self) -> Bytes {
        use PayloadValue::*;
        match self {
            EncodedString(str) => encode_utf8_string(str),
            Plain(str) => Vec::from(str.as_bytes()),
        }
    }

    pub(crate) fn value(&self) -> &String {
        use PayloadValue::*;
        match self {
            EncodedString(str) => str,
            Plain(str) => str,
        }
    }
}

pub(crate) struct Payload {
    values: Vec<PayloadValue>,
}

impl Payload {
    pub(crate) fn new(values: Vec<PayloadValue>) -> Self {
        Payload { values }
    }

    pub(crate) fn from_bytes(bytes: &[Byte]) -> Self {
        let (client_id, _) = decode_utf8_string(bytes);
        let values = vec![EncodedString(client_id)];
        Payload { values }
    }

    pub(crate) fn as_bytes(&self) -> Bytes {
        self.values.iter().flat_map(|v| v.as_bytes()).collect()
    }

    pub(crate) fn len(&self) -> u32 {
        self.as_bytes().len() as u32
    }

    pub(crate) fn payload_values(&self) -> &Vec<PayloadValue> {
        &self.values
    }
}
