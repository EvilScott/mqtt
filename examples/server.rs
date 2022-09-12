use mqtt::server::Server;

fn main() {
    let mut server = Server::new();
    server.listen();
    //TODO graceful shutdown
}
