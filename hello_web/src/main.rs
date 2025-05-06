use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    TcpListener::bind("127.0.0.1:3000")
        .unwrap()
        .incoming()
        .for_each(|stream| handle_client(stream.unwrap()));
}

fn handle_client(mut stream: TcpStream) {
    stream
        .read(&mut [0; 1024])
        .and_then(|_| stream.write(b"HTTP/1.1 200 OK\r\n\r\nHello, world!"))
        .unwrap();
}
