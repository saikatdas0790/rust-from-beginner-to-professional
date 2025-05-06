use std::{
    fs,
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
    let buffer = &mut [0; 1024];
    stream.read(buffer).unwrap();

    let response = if buffer.starts_with(b"GET /hello HTTP/1.1\r\n") {
        "HTTP/1.1 200 OK\r\n\r\nHello, world!"
    } else if buffer.starts_with(b"POST /world HTTP/1.1\r\n") {
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"message\": \"World received!\"}"
    } else {
        &if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
            let html_content = fs::read_to_string("./src/index.html").unwrap_or_else(|_| {
                "<html><body><h1>404 Not Found</h1><p>index.html not found</p></body></html>"
                    .to_string()
            });
            format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{html_content}")
        } else {
            "HTTP/1.1 404 NOT FOUND\r\n\r\nEndpoint not found!".to_string()
        }
    };

    stream
        .write(response.as_bytes())
        .and_then(|_| stream.flush())
        .unwrap();

    stream.flush().unwrap();
}
