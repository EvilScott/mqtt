use crate::control_packet::encode_utf8_string;

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
}

pub(crate) struct Payload {
    values: Vec<PayloadValue>,
}

impl Payload {
    pub(crate) fn new(values: Vec<PayloadValue>) -> Payload {
        Payload { values }
    }

    pub(crate) fn as_bytes(&self) -> Vec<u8> {
        vec![0] //TODO calculate length and values from struct
    }
}
