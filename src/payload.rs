use crate::common::{Bytes, ParseError, Parseable, Serializable, UTF8String};

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use crate::common::{Bytes, UTF8String};
    use crate::payload::Payload;

    const CLIENT_ID: &str = "id1";

    #[test]
    fn test_as_bytes() {
        let values: Vec<UTF8String> = vec![UTF8String::new(CLIENT_ID)];
        let payload = Payload::new(values);
        let bytes = payload.as_bytes();
        assert_eq!(bytes, vec![0, 3, 105, 100, 49]);
    }

    #[test]
    fn test_from_bytes() {
        let values: Vec<UTF8String> = vec![UTF8String::new(CLIENT_ID)];
        let payload = Payload::new(values);
        let bytes: Bytes = vec![0, 3, 105, 100, 49, 2, 3];
        let parsed_payload = Payload::from_bytes(bytes).unwrap();
        assert_eq!(parsed_payload, payload);
    }

    fn test_as_bytes_from_bytes() {
        let values: Vec<UTF8String> = vec![UTF8String::new(CLIENT_ID)];
        let payload = Payload::new(values);
        let bytes = payload.as_bytes();
        let parse_payload = Payload::from_bytes(bytes).unwrap();
        assert_eq!(parse_payload, payload);
    }

    #[test]
    fn test_len() {
        let values: Vec<UTF8String> = vec![UTF8String::new(CLIENT_ID)];
        let payload = Payload::new(values);
        let len = payload.len();
        assert_eq!(len, 5);
    }
}
