use crate::common::{Byte, Bytes, DataType, ParseError, Parseable};

pub(crate) struct Payload {
    pub(crate) values: Vec<DataType>,
}

impl Payload {
    pub(crate) fn new(values: Vec<DataType>) -> Self {
        Payload { values }
    }

    pub(crate) fn from_bytes(bytes: &[Byte]) -> Result<Self, ParseError> {
        let (client_id, _) = bytes.parse_utf8_string().unwrap(); //TODO handle error
        let values = vec![client_id];
        Ok(Payload { values })
    }

    pub(crate) fn as_bytes(&self) -> Bytes {
        self.values.iter().flat_map(|v| v.as_bytes()).collect()
    }

    pub(crate) fn len(&self) -> u32 {
        self.as_bytes().len() as u32
    }
}
