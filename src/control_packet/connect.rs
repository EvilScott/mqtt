use crate::common::{Byte, ParseError, UTF8String};
use crate::control_packet::ControlPacket;
use crate::fixed_header::FixedHeader;
use crate::payload::Payload;
use crate::variable_header::VariableHeader;

pub(crate) struct Connect {
    fixed_header: FixedHeader,
    variable_header: VariableHeader,
    payload: Payload,
}

impl Connect {
    pub(crate) fn new(client_id: &str) -> Connect {
        let values: Vec<UTF8String> = vec![UTF8String::new(client_id)];
        let payload = Payload::new(values);

        let keep_alive = 0;
        let variable_header = VariableHeader::new(keep_alive);

        let packet_type_value = 1;
        let remaining_length: u32 = variable_header.len() + payload.len();
        let fixed_header = FixedHeader::new(packet_type_value, remaining_length);

        Connect {
            fixed_header,
            variable_header,
            payload,
        }
    }

    pub(crate) fn client_id(&self) -> &str {
        &self.payload.values()[0].value()
    }
}

impl ControlPacket for Connect {
    fn get_fixed_header(&self) -> &FixedHeader {
        &self.fixed_header
    }
    fn get_variable_header(&self) -> &VariableHeader {
        &self.variable_header
    }
    fn get_payload(&self) -> &Payload {
        &self.payload
    }
    fn from_bytes(bytes: &[Byte]) -> Result<Self, ParseError> {
        let byte_vec = Vec::from(bytes);
        let (fixed_header, variable_header_bytes) = FixedHeader::from_bytes(byte_vec)?;
        let (variable_header, payload_bytes) = VariableHeader::from_bytes(variable_header_bytes)?;
        let payload = Payload::from_bytes(payload_bytes)?;

        Ok(Connect {
            fixed_header,
            variable_header,
            payload,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Connect, ControlPacket};

    #[test]
    fn test_client_id() {
        let packet = Connect::new("foobar");
        assert_eq!(packet.client_id(), "foobar");
    }

    #[test]
    fn test_connect_as_bytes_from_bytes() {
        let bytes = Connect::new("foo").as_bytes();
        let byte_slice = bytes.as_slice();
        let packet = Connect::from_bytes(byte_slice).unwrap();
        assert_eq!(packet.client_id(), "foo");
    }
}
