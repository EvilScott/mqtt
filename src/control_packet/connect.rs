use crate::control_packet::{ControlPacket, FixedHeader, Payload, VariableHeader};

pub(crate) struct Connect {
    fixed_header: FixedHeader,
    variable_header: VariableHeader,
    payload: Payload,
}

impl ControlPacket for Connect {
    fn get_fixed_header(&self) -> &FixedHeader { &self.fixed_header }
    fn get_variable_header(&self) -> &VariableHeader { &self.variable_header }
    fn get_payload(&self) -> &Payload { &self.payload }
    fn from_bytes(bytes: Vec<u8>) -> Connect {
        //TODO parse incoming bytes
        let fixed_header = FixedHeader {
            packet_type_value: 1,
            dup: false,
            qos: 1,
            retain: false
        };
        let variable_header = VariableHeader { keep_alive: 0 };
        let payload = Payload {};
        Connect { fixed_header, variable_header, payload }
    }
}
