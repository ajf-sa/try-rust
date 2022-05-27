use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let response = "HTTP/1.1 200 OK\r\n\r\n<html><body><h1>Hello, World!</h1></body></html>\r\n";
    stream.write(response.as_bytes()).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    stream.flush().unwrap();
}
