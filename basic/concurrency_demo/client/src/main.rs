use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    let get = b"GET /42 HTTP/1.1\r\n";
    stream.write(get).unwrap();
    stream.read(&mut [0; 1024]).unwrap();
    stream.flush().unwrap();
}
