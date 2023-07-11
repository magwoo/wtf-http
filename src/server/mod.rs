#![allow(dead_code, unused)]
use handlebars::Handlebars;
use std::{
    collections::LinkedList,
    fmt::Display,
    io::{BufReader, Write},
    net, process,
};

use crate::server::{
    request::RequestInfo,
    response::{HttpResponse, Status},
};

use self::request::{deserialize_request, Method, Route};

pub mod request;
pub mod response;

pub struct HttpServer {
    pub adress: &'static str,
    handlebars: Handlebars<'static>,
    routes: Option<&'static [Route]>,
}

impl HttpServer {
    pub fn bind(adress: &'static str, routes: Option<&'static [Route]>) -> Self {
        let mut handlebars = Handlebars::new();
        handlebars.register_templates_directory("hbs", "templates/*");

        Self {
            adress,
            handlebars,
            routes,
        }
    }

    pub fn run(self) {
        let listener = match net::TcpListener::bind(self.adress) {
            Ok(listener) => listener,
            Err(err) => {
                println!("Bind error: {err}");
                process::exit(1);
            }
        };

        for stream in listener.incoming() {
            let mut stream = match stream {
                Ok(stream) => stream,
                Err(err) => {
                    println!("Stream error: {err}");
                    continue;
                }
            };
            let request_info = match deserialize_request(&mut stream) {
                Ok(request_info) => request_info,
                Err(status) => {
                    let header = status.generate_header();
                    println!("{}", header);
                    stream.write_all(header.as_bytes());
                    continue;
                }
            };

            match self.routes {
                Some(routes) => match search_route(routes, request_info) {
                    Ok(response) => {
                        let header = response.status.generate_header();
                        let body = response.body.expect("test");
                        stream.write_all(format!("{header}{body}").as_bytes());
                    }
                    Err(status) => {
                        stream.write_all(status.generate_header().as_bytes());
                    }
                },
                None => {
                    stream.write_all(Status::NotFound.generate_header().as_bytes());
                }
            }
        }

        fn search_route(routes: &[Route], req_info: RequestInfo) -> Result<HttpResponse, Status> {
            for route in routes {
                if route.method == req_info.method && route.uri == req_info.uri {
                    println!("{:?}", req_info);
                    return Ok((route.handler)());
                }
            }
            Err(Status::NotFound)
        }
    }
}
