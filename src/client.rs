use crate::control_packet::ControlPacket;
use std::io::Write;
use std::net::TcpStream;

pub struct Client {
    client_id: String,
    stream: TcpStream,
    subscriptions: Vec<String>,
}

impl Client {
    pub fn new(client_id: String, host: &str) -> Self {
        let stream = TcpStream::connect(format!("{host}:1883")).unwrap();
        Client {
            client_id,
            stream,
            subscriptions: Vec::new(),
        }
    }

    fn send(&mut self, packet: &impl ControlPacket) {
        self.stream.write_all(packet.as_bytes().as_slice()).unwrap();
    }

    pub fn connect(&self) {
        //TODO send CONNECT and wait for CONNACK
    }

    pub fn publish(&self, _topic: &str, _payload: &str, _qos: u8, _retain: bool) {
        //TODO send PUBLISH and wait for PUBACK
        //TODO handle PUBLISH with DUP if no PUBACK comes
    }

    pub fn subscribe(&self, _topic: &str) {
        //TODO send SUBSCRIBE and wait for SUBACK
        //TODO spin off thread for each subscription handling incoming PUBLISH/outgoing PUBACK
        //TODO eventually take callback here but for now just echo
    }

    pub fn unsubscribe(&self, _topic: &str) {
        //TODO send UNSUBSCRIBE and wait for UNSUBACK
        //TODO kill subscription thread
    }

    pub fn disconnect(&self) {
        //TODO send DISCONNECT to server
        //TODO join any SUBSCRIBE threads
        //TODO close stream
    }
}
