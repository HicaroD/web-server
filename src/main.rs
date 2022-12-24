use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpListener,
};

const BUFFER_SIZE: usize = 2048;

#[derive(Debug)]
enum ServerError<'e> {
    Io(std::io::Error),
    ParserError(&'e str),
}

impl<'e> From<std::io::Error> for ServerError<'e> {
    fn from(io_error: std::io::Error) -> Self {
        Self::Io(io_error)
    }
}

#[derive(Debug)]
enum HttpMethod {
    GET,
    POST,
    PUT,
    Other(String),
}

impl HttpMethod {
    fn from_status_code(code: &str) -> Self {
        match code {
            "GET" => Self::GET,
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            c => Self::Other(c.to_string()),
        }
    }
}

#[derive(Debug)]
struct HttpRequest {
    method: HttpMethod,
    uri: String,
    version: String,
    headers: HashMap<String, String>,
    body: String,
}

impl HttpRequest {
    pub fn from_response_string<'http>(response: String) -> Result<Self, ServerError<'http>> {
        // TODO: refactor this parsing
        let response: Vec<&str> = response.splitn(2, "\r\n\r\n").collect();
        if response.len() < 2 {
            return Err(ServerError::ParserError("Response format is invalid"));
        }
        let request_data = response[0].to_string();
        let body = response[1].to_string();

        let request_data: Vec<&str> = request_data.split(" ").collect();
        if request_data.len() < 3 {
            return Err(ServerError::ParserError("Response data format is invalid"));
        }
        let method = HttpMethod::from_status_code(request_data[0]);
        let uri = request_data[1].to_string();
        let version = request_data[2].to_string();

        let header_data: Vec<&str> = response[1].split("\r\n").collect();
        let mut headers = HashMap::<String, String>::new();
        for header in header_data.iter() {
            let key_value: Vec<&str> = header.splitn(2, ":").collect();
            if key_value.len() < 2 {
                return Err(ServerError::ParserError("Header format is invalid"));
            }
            headers.insert(key_value[0].to_string(), key_value[1].to_string());
        }

        Ok(Self {
            method,
            uri,
            version,
            headers,
            body,
        })
    }
}

fn main() -> Result<(), ServerError<'static>> {
    let address = "0.0.0.0:7000";

    let tcp_socket = TcpListener::bind(address.to_string())?;

    for session in tcp_socket.incoming() {
        let mut session = session?;
        let mut buffer = [0; BUFFER_SIZE];

        session.read(&mut buffer)?;
        println!("RAW DATA FROM REQUEST:\n{:?}", buffer);
        let request = String::from_utf8_lossy(&buffer);
        println!("DATA FROM REQUEST CONVERTED TO UTF8:\n{}", request);
        let request = HttpRequest::from_response_string(request.to_string())?;
        println!("PARSED REQUEST: {:#?}", request);

        let response = "HTTP/1.1 200 OK\r\nContent-Length: 2\r\nContent-Type: text/plain\r\n\r\nOK";
        session.write(response.as_bytes())?;
        session.flush()?;
    }
    Ok(())
}

mod tests {
    use super::*;

    #[test]
    fn test_http_request_parser() {
        // TODO(hícaro)
        assert!(true);
    }
}
