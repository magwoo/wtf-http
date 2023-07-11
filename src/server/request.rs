use crate::server::response::Status;
use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

use super::response::HttpResponse;

#[derive(PartialEq)]
pub enum Method {
    Get,
    Post,
}

pub struct Route {
    pub method: Method,
    pub uri: &'static str,
    pub handler: &'static dyn Fn() -> HttpResponse,
}

pub struct RequestInfo {
    pub method: Method,
    pub uri: String,
}

pub fn deserialize_request(stream: &mut TcpStream) -> Result<RequestInfo, Status> {
    let buf_reader = BufReader::new(stream);
    let request_line = match buf_reader.lines().next() {
        Some(result) => match result {
            Ok(request_line) => {
                if request_line.len() > 100 {
                    return Err(Status::NotFound);
                } else {
                    request_line
                }
            }
            Err(err) => return Err(Status::NotFound),
        },
        None => return Err(Status::NotFound),
    };

    let request_args: Vec<&str> = request_line.split(' ').collect();
    if request_args.len() < 2 {
        return Err(Status::NotFound);
    }

    let method = match request_args[0] {
        "GET" => Method::Get,
        "POST" => Method::Post,
        &_ => return Err(Status::NotFound),
    };
    let uri = request_args[1].to_string();

    Ok(RequestInfo { method, uri })
}
