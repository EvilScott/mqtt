use crate::common::Byte;
use crate::control_packet::ControlPacket;
use crate::fixed_header::FixedHeader;
use crate::payload::{Payload, PayloadValue};
use crate::variable_header::VariableHeader;

pub(crate) struct Connect {
    fixed_header: FixedHeader,
    variable_header: VariableHeader,
    payload: Payload,
}

impl Connect {
    pub(crate) fn new(client_id: String) -> Connect {
        let values = vec![PayloadValue::EncodedString(client_id)];
        let payload = Payload::new(values);

        let keep_alive = 0;
        let variable_header = VariableHeader::new(keep_alive);

        let packet_type_value = 1;
        let dup = false;
        let qos = 0;
        let retain = false;
        let remaining_length: u32 = variable_header.len() + payload.len();
        let fixed_header = FixedHeader::new(packet_type_value, dup, qos, retain, remaining_length);

        Connect {
            fixed_header,
            variable_header,
            payload,
        }
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
    fn from_bytes(bytes: &[Byte]) -> Connect {
        let (fixed_header, variable_header_bytes) = FixedHeader::from_bytes(bytes);
        let (variable_header, payload_bytes) = VariableHeader::from_bytes(variable_header_bytes);
        let payload = Payload::from_bytes(payload_bytes);
        Connect {
            fixed_header,
            variable_header,
            payload,
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::{Connect, ControlPacket};

    //TODO FIXME
    // #[test]
    // fn test_connect_as_bytes_from_bytes() {
    //     let bytes = Connect::new(String::from("foo")).as_bytes();
    //     let packet = Connect::from_bytes(bytes);
    //     assert_eq!(packet.payload.payload_values()[0].value(), "foo");
    // }
}
