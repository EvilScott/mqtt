use std::io::BufReader;
use std::net::{TcpListener, TcpStream};

struct Connection {
    client_id: String,
    subscriptions: Vec<String>,
}

pub struct Server {
    connections: Vec<Connection>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            connections: Vec::new(),
        }
    }

    pub fn listen(&mut self) {
        let listener = TcpListener::bind("0.0.0.0:1883").unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.handle_connection(stream);
        }
    }

    fn handle_connection(&mut self, mut stream: TcpStream) {
        //TODO give client 30s to send CONNECT otherwise close
        //TODO if anything but a CONNECT comes first then close
        let _buf_reader = BufReader::new(&mut stream);
        //TODO handle CONNECT packet then store the connection
    }

    pub fn shutdown(&self) {
        //TODO close connections
        //TODO close listen threads
    }
}
