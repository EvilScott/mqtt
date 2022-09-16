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
        let remaining_length: u32 = 0; //TODO calculate this
        let fixed_header = FixedHeader::new(packet_type_value,dup,qos, retain, remaining_length);

        Connect { fixed_header, variable_header, payload }
    }
}

impl ControlPacket for Connect {
    fn get_fixed_header(&self) -> &FixedHeader { &self.fixed_header }
    fn get_variable_header(&self) -> &VariableHeader { &self.variable_header }
    fn get_payload(&self) -> &Payload { &self.payload }
    fn from_bytes(bytes: Vec<u8>) -> Connect {
        //TODO parse incoming bytes
        Connect::new(String::from("foo"))
    }
}
