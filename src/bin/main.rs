extern crate dotenv;

use tcp_http_server::server::tcp_http_server::TcpHttpServer;
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let addr = env::var("URL").expect("You must provide a URL var in the .env");
    let server = TcpHttpServer::new(addr);
    server.start();
}
