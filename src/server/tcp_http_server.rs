use crate::models::http_request::HttpRequest;
use crate::models::http_response::HttpResponse;
use crate::models::middleware::Middleware;
use crate::server::threadpool::ThreadPool;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::sync::Arc;

pub struct TcpHttpServer {
    tcp_listener: TcpListener,
    threadpool: ThreadPool,
    pipeline: Middleware,
}

impl TcpHttpServer {
    pub fn new<A: ToSocketAddrs>(
        addr: A,
        threadpool_size: usize,
        pipeline: Middleware,
    ) -> TcpHttpServer {
        TcpHttpServer {
            tcp_listener: TcpListener::bind(addr).unwrap(),
            threadpool: ThreadPool::new(threadpool_size),
            pipeline,
        }
    }
    pub fn start(self) {
        let self_arc = Arc::new(self);
        for stream in self_arc.tcp_listener.incoming() {
            match stream {
                Ok(stream) => {
                    let cloned_self_arc = Arc::clone(&self_arc);
                    self_arc
                        .threadpool
                        .execute(move || cloned_self_arc.handle_connection(stream))
                }
                Err(err) => println!("{:?}", err),
            }
        }
    }
    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();

        match HttpRequest::from_buffer(&buffer) {
            Ok(mut req) => {
                let mut res = HttpResponse::new();
                self.pipeline.handle_request(&mut req, &mut res);

                stream.write(res.to_string().as_bytes()).unwrap();
                stream.flush().unwrap();
            }
            Err(_) => {
                println!("A TCP packet came in that was not able to be parsed as a Http packet")
            }
        }
    }
}
