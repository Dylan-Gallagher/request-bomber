use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let num_requests = 100000;
    for _ in 0..num_requests {
        let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
        stream.write(b"GET /sleep HTTP/1.1").unwrap();
    }
}
