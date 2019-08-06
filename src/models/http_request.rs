use std::str;
use std::thread;
use std::time::Duration;

pub struct HttpRequest<'a> {
    pub uri: &'a str,
    pub method: &'a str,
    pub http_version: &'a str,
}

pub struct HttpRequestParseError;

impl<'a> HttpRequest<'a> {
    pub fn from_buffer(buf: &'a [u8]) -> Result<HttpRequest<'a>, HttpRequestParseError> {
        let request_full = str::from_utf8(&buf).map_err(|_| HttpRequestParseError)?;
        let request_full_lines: Vec<&str> = request_full.lines().collect();
        let request: Vec<&str> = request_full_lines[0].split(' ').collect();
        let (method, uri, http_version) = (request[0], request[1], request[2]);

        Ok(HttpRequest {
            uri,
            method,
            http_version,
        })
    }

    pub fn get_html_file_path(&self) -> String {
        let mut file_path = String::from("static/");
        if self.uri.starts_with('/') {
            if self.uri == "/" {
                file_path.push_str("index.html");
            } else if self.uri == "/sleep" {
                thread::sleep(Duration::from_secs(3));
                file_path.push_str("index.html");
            } else {
                file_path.push_str(&self.uri[1..]);
            }
        } else {
            file_path.push_str("404.html");
        }
        file_path
    }
}
