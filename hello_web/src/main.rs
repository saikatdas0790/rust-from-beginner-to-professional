use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }
}

fn handle_client(mut stream: TcpStream) {
    stream.read(&mut [0; 1024]).unwrap();
    stream
        .write(b"HTTP/1.1 200 OK\r\n\r\nHello, world!")
        .unwrap();
}
