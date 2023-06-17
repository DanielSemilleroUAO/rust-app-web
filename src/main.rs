use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let address = "127.0.0.1:8000";
    // iniciar el servidor
    let listener = TcpListener::bind(address).unwrap();
    println!("Servidor activo en {}", address);

    // escuchar por conexiones
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Stream recibido");
        handle_connection(stream);
    }
}

// manajear conexiones
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Stream recibido");
    println!("{}", String::from_utf8_lossy(&buffer));

    let get = b"GET / HTTP/1.1";
    if buffer.starts_with(get) {
        // responder al cliente
        send_to_client(stream);
    } else {
        send_not_found(stream);
    }
}

fn send_not_found(mut stream: TcpStream) {
    let content = fs::read_to_string("404.html").unwrap();
    let response = buildResponse(content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn send_to_client(mut stream: TcpStream) {
    let content = fs::read_to_string("index.html").unwrap();
    let response = buildResponse(content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn buildResponse(content: String) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Length {}\r\n\r\n{}",
        content.len(),
        content
    )
}
