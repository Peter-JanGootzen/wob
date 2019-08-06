use crate::models::http_request::HttpRequest;
use crate::models::http_response::HttpResponse;
use crate::server::threadpool::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::net::ToSocketAddrs;

pub struct TcpHttpServer {
    tcp_listener: TcpListener,
    threadpool: ThreadPool,
}

impl TcpHttpServer {
    pub fn new<A: ToSocketAddrs>(addr: A) -> TcpHttpServer {
        TcpHttpServer {
            tcp_listener: TcpListener::bind(addr).unwrap(),
            threadpool: ThreadPool::new(3),
        }
    }
    pub fn start(&self) {
        for stream in self.tcp_listener.incoming() {
            match stream {
                Ok(stream) => self
                    .threadpool
                    .execute(|| self.handle_connection(stream)),
                Err(err) => println!("{:?}", err),
            }
        }
    }
    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();

        match HttpRequest::from_buffer(&buffer) {
            Ok(req) => {
                let response = self.handle_request(req);

                stream.write(response.to_string().as_bytes()).unwrap();
                stream.flush().unwrap();
            }
            Err(_) => {
                println!("A TCP packet came in that was not able to be parsed as a Http packet")
            }
        }
    }
    fn handle_request(&self, req: HttpRequest) -> HttpResponse {
        if req.method == "GET" && req.http_version == "HTTP/1.1" {
            match fs::read_to_string(req.get_html_file_path()) {
                Ok(content) => HttpResponse {
                    Http_version: "HTTP/1.1",
                    status_code: "200 OK",
                    content,
                },
                Err(_) => HttpResponse {
                    Http_version: "HTTP/1.1",
                    status_code: "200 OK",
                    content: fs::read_to_string("static/404.html").unwrap(),
                },
            }
        } else {
            HttpResponse {
                Http_version: "HTTP/1.1",
                status_code: "400 BAD REQUEST",
                content: String::from("Invalid Http version or method"),
            }
        }
    }
}
