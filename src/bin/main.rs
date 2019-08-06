extern crate dotenv;

use dotenv::dotenv;
use std::env;
use tcp_http_server::server::tcp_http_server::TcpHttpServer;

fn main() {
    dotenv().ok();
    let addr = env::var("URL").expect("You must provide a URL var in the .env");
    let server = TcpHttpServer::new(addr);
    server.start();
}
