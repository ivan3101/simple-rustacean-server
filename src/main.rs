fn main() {
    let server = Server::new(String::from("127.0.0.1:8000"));
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self { addr }
    }

    fn run(&self) {
        println!("Server listening on: {}", self.addr)
    }
}

enum HttpVerbs {
    GET,
    DELETE,
    POST,
    PUT
}

struct Request {
    path: String,
    query_string: Option<String>,
    method: HttpVerbs,
}
