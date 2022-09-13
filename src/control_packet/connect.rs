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
    fn new(client_id: String) -> Connect {
        let fixed_header = FixedHeader::new(1,false,0, false);
        let variable_header = VariableHeader::new(0);
        let values = vec![PayloadValue::EncodedString(client_id)];
        let payload = Payload::new(values);
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
