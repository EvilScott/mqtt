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
}
