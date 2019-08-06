mod models;

use models::http_request::HTTPRequest;
use models::http_response::HTTPResponse;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(err) => println!("{:?}", err),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    match HTTPRequest::from_buffer(&buffer) {
        Ok(req) => {
            let response = handle_request(req);

            stream.write(response.to_string().as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(_) => println!("A TCP packet came in that was not able to be parsed as a HTTP packet"),
    }
}

fn handle_request(req: HTTPRequest) -> HTTPResponse {
    if req.method == "GET" && req.http_version == "HTTP/1.1" {
        match fs::read_to_string(req.get_html_file_path()) {
            Ok(content) => HTTPResponse {
                http_version: "HTTP/1.1",
                status_code: "200 OK",
                content,
            },
            Err(_) => {
                HTTPResponse {
                    http_version: "HTTP/1.1",
                    status_code: "200 OK",
                    content: fs::read_to_string("static/404.html").unwrap(),
                }
            }
        }
    } else {
        HTTPResponse {
            http_version: "HTTP/1.1",
            status_code: "400 BAD REQUEST",
            content: String::from("Invalid HTTP version or method"),
        }
    }
}
