use server::Server;
use http::request::Request;

fn main() {
    let server = Server::new(String::from("127.0.0.1:8000"));
    server.run();
}

mod server {
    pub struct Server {
        addr: String,
    }

    impl Server {
        pub fn new(addr: String) -> Self {
            Self { addr }
        }

        pub fn run(&self) {
            println!("Server listening on: {}", self.addr)
        }
    }
}

mod http {
    pub mod request {
        use super::http_verbs::HttpVerbs;

        pub struct Request {
            path: String,
            query_string: Option<String>,
            method: HttpVerbs,
        }
    }

    mod http_verbs {
        pub enum HttpVerbs {
            GET,
            DELETE,
            POST,
            PUT
        }
    }
}

