use crate::common::{decode_utf8_string, encode_utf8_string};
use crate::payload::PayloadValue::EncodedString;

pub(crate) enum PayloadValue {
    EncodedString(String),
    Plain(String),
}

impl PayloadValue {
    pub(crate) fn as_bytes(&self) -> Vec<u8> {
        match self {
            PayloadValue::EncodedString(str) => encode_utf8_string(str),
            PayloadValue::Plain(str) => Vec::from(str.as_bytes()),
        }
    }

    pub(crate) fn value(&self) -> &String {
        match self {
            PayloadValue::EncodedString(str) => str,
            PayloadValue::Plain(str) => str,
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

    pub(crate) fn from_bytes(bytes: Vec<u8>) -> Self {
        let (client_id, _) = decode_utf8_string(bytes);
        let values = vec![EncodedString(client_id)];
        Payload { values }
    }

    pub(crate) fn as_bytes(&self) -> Vec<u8> {
        self.values.iter().flat_map(|v| v.as_bytes()).collect()
    }

    pub(crate) fn len(&self) -> u32 {
        self.as_bytes().len() as u32
    }

    pub(crate) fn payload_values(&self) -> &Vec<PayloadValue> {
        &self.values
    }
}
