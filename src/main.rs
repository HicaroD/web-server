use std::{
    net::TcpListener,
    io::Read,
};

const BUFFER_SIZE: usize = 2048;

fn main() {
    let address = "0.0.0.0:7000";

    let tcp_socket = TcpListener::bind(address.to_string()).unwrap();

    for session in tcp_socket.incoming() {
        let mut session = session.unwrap();
        let mut buffer = [0;BUFFER_SIZE];

        session.read(&mut buffer).unwrap();
        let request = String::from_utf8_lossy(&buffer);
        println!("{}", request);
    }
}
