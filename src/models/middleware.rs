use crate::models::http_request::HttpRequest;
use crate::models::http_response::HttpResponse;
impl Middleware {
    pub fn new<F>(func: F, next: Option<Middleware>) -> Self
        where F: Fn(&mut HttpRequest, &mut HttpResponse) + Send + Sync+ 'static
    {
        Self {
            func: Box::new(func),
            next: next.map(Box::new)
        }
    }
    pub fn handle_request(&self, mut req: &mut HttpRequest, mut res: &mut HttpResponse)
    {
        (self.func)(&mut req, &mut res);
        if let Some(next) = &self.next {
            next.handle_request(&mut req, &mut res);
        }
    }
}

pub struct Middleware
{
    func: Box<dyn Fn(&mut HttpRequest, &mut HttpResponse) + Send + Sync>,
    next: Option<Box<Middleware>>,
}
