use crate::control_packet::ControlPacket;

pub(crate) struct Connect {
    client_id: String
}

impl ControlPacket for Connect {
    fn packet_type_value(&self) -> u8 { 1 }
    fn variable_header_flags(&self) -> u8 { 0b0000_0010 }
    fn payload_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::from((self.client_id.len() as u16).to_be_bytes());
        bytes.append(&mut Vec::from(self.client_id.as_bytes()));
        bytes
    }
}
