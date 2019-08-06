use std::str;

pub struct HTTPRequest<'a> {
    pub uri: &'a str,
    pub method: &'a str,
    pub http_version: &'a str,
}

pub struct HTTPRequestParseError;

impl<'a> HTTPRequest<'a> {
    pub fn from_buffer(buf: &'a [u8]) -> Result<HTTPRequest<'a>, HTTPRequestParseError> {
        let request_full = str::from_utf8(&buf).map_err(|_| HTTPRequestParseError)?;
        let request_full_lines: Vec<&str> = request_full.lines().collect();
        let request: Vec<&str> = request_full_lines[0].split(' ').collect();
        let (request_method, request_uri, request_httpversion) =
            (request[0], request[1], request[2]);

        Ok(HTTPRequest {
            uri: request_uri,
            method: request_method,
            http_version: request_httpversion,
        })
    }

    pub fn get_html_file_path(&self) -> String {
        let mut file_path = String::from("static/");
        if self.uri.starts_with('/') {
            if self.uri == "/" {
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
