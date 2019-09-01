extern crate dotenv;

use dotenv::dotenv;
use std::env;
use tcp_http_server::server::tcp_http_server::TcpHttpServer;
use tcp_http_server::models::middleware::Middleware;

fn main() {
    dotenv().ok();
    let addr = env::var("URL").expect("You must provide a URL var in the .env");
    let server = TcpHttpServer::new(addr, 3, Middleware::new(
        |_req, res| {
            res.content.push_str(&String::from("1"));
        },
        Some(Middleware::new(
            |_req, res| {
                res.content.push_str(&String::from("2"));
            },
            None
        ))
    ));
    server.start();
}
