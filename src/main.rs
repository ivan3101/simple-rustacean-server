mod server;
mod http;

use server::Server;
use http::Request;

fn main() {
    let server = Server::new(String::from("127.0.0.1:8000"));
    server.run();
}