use std::{
    net::TcpListener,
    io::{Read, Write},
    collections::HashMap,
};

const BUFFER_SIZE: usize = 2048;

struct HttpResponse {
    method: u16,
    uri: String,
    version: String,
    headers: HashMap<String, String>,
    body: String,
}

fn main() -> std::io::Result<()> {
    let address = "0.0.0.0:7000";

    let tcp_socket = TcpListener::bind(address.to_string())?;

    for session in tcp_socket.incoming() {
        let mut session = session?;
        let mut buffer = [0;BUFFER_SIZE];

        session.read(&mut buffer)?;
        println!("RAW DATA FROM REQUEST:\n{:?}", buffer);
        let request = String::from_utf8_lossy(&buffer);
        println!("DATA FROM REQUEST CONVERTED TO UTF8:\n{}", request);

        let response = "HTTP/1.1 200 OK\r\nContent-Length: 2\r\nContent-Type: text/plain\r\n\r\nOK";
        session.write(response.as_bytes())?;
        session.flush()?;
    }
    Ok(())
}
