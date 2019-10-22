pub struct HttpResponse {
    pub http_version: String,
    pub status_code: String,
    pub content: String,
}

impl HttpResponse {
    pub fn new() -> Self {
        Self {
            http_version: String::new(),
            status_code: String::new(),
            content: String::new(),
        }
    }
    pub fn to_string(&self) -> String {
        let mut response_string = String::from("");
        response_string.push_str(&self.http_version);
        response_string.push_str(" ");
        response_string.push_str(&self.status_code);
        response_string.push_str("\r\n\r\n");
        response_string.push_str(&self.content);
        response_string
    }
}
