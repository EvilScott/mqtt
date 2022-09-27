use crate::common::{Bytes, ParseError, Parseable, Serializable, UTF8String};

pub(crate) struct Payload {
    values: Vec<UTF8String>, //TODO support other types (via trait?)
}

impl Payload {
    pub(crate) fn new(values: Vec<UTF8String>) -> Self {
        Payload { values }
    }

    pub(crate) fn from_bytes(bytes: Bytes) -> Result<Self, ParseError> {
        let byte_slice = &bytes[..];
        let (client_id, _) = byte_slice.parse_utf8_string()?;
        let values = vec![client_id];
        Ok(Payload { values })
    }

    pub(crate) fn as_bytes(&self) -> Bytes {
        self.values.iter().flat_map(|v| v.as_bytes()).collect()
    }

    pub(crate) fn values(&self) -> &[UTF8String] {
        &self.values
    }

    pub(crate) fn len(&self) -> u32 {
        self.as_bytes().len() as u32
    }
}
