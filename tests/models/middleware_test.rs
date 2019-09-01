use tcp_http_server::models::middleware::Middleware;
use tcp_http_server::models::http_request::HttpRequest;
use tcp_http_server::models::http_response::HttpResponse;

#[test]
fn create_pipeline() {
    let mut req = HttpRequest {
        uri: &String::from(""),
        method: &String::from(""),
        http_version: &String::from("")
    };
    let mut res = HttpResponse {
        http_version: &String::from(""),
        status_code: &String::from(""),
        content: String::from("")
    };
    let m1 = Middleware::new(
        |req, res| {
            res.content.push_str(&String::from("1"));
        },
        Some(Middleware::new(
            |req, res| {
                res.content.push_str(&String::from("2"));
            },
            None
        ))
    );
    m1.handle_request(&mut req, &mut res);

    assert_eq!(res.content, String::from("12"));
}

