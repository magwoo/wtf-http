use std::fmt;

pub enum Status {
    Ok,
    NotFound,
    ServerError,
}

impl Status {
    pub fn generate_header(&self) -> &str {
        match *self {
            Self::Ok => "HTTP/1.1 200 OK\n\n",
            Self::NotFound => "HTTP/1.1 404 NOT FOUND\n\n",
            Self::ServerError => "HTTP/1.1 500 SERVER ERROR\n\n",
        }
    }
}

pub struct HttpResponse {
    pub status: Status,
    pub body: Option<String>,
}

#[allow(non_snake_case)]
impl HttpResponse {
    pub fn Ok() -> Self {
        HttpResponse {
            status: Status::Ok,
            body: None,
        }
    }

    pub fn NotFound() -> Self {
        HttpResponse {
            status: Status::NotFound,
            body: None,
        }
    }

    pub fn body(self, body: String) -> Self {
        HttpResponse {
            status: self.status,
            body: Some(body),
        }
    }
}
