use http_server::Server;

fn main() {
    let server = Server::new("127.0.0.1:5000".to_string());
    server.run();
}
