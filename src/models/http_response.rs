pub struct HttpResponse<'a> {
    pub http_version: &'a str,
    pub status_code: &'a str,
    pub content: String,
}

impl<'a> HttpResponse<'a> {
    pub fn to_string(&self) -> String {
        let mut response_string = String::from("");
        response_string.push_str(self.http_version);
        response_string.push_str(" ");
        response_string.push_str(self.status_code);
        response_string.push_str("\r\n\r\n");
        response_string.push_str(&self.content);
        response_string
    }
}
