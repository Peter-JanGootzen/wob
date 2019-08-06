use tcp_http_server::server::tcp_http_server::TcpHttpServer;

fn main() {
    let server = TcpHttpServer::new("127.0.0.1:8000");
    server.start();
}
