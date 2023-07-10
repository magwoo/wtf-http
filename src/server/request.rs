pub enum Method {
    Get,
    Post,
}

pub struct Route {
    pub method: Method,
    pub uri: &'static str,
    pub handler: &'static dyn Fn(),
}
