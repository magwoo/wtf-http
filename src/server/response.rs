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
