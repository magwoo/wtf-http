#![allow(dead_code, unused)]
use handlebars::Handlebars;
use std::{
    fmt::Display,
    io::{BufReader, Write},
    net, process,
};

use crate::server::response::Status;

use self::request::deserialize_request;

pub mod request;
pub mod response;

pub struct HttpServer {
    pub adress: &'static str,
    handlebars: Handlebars<'static>,
    routes: Option<&'static [request::Route]>,
}

impl HttpServer {
    pub fn new(adress: &'static str) -> Self {
        let mut handlebars = Handlebars::new();
        handlebars.register_templates_directory("hbs", "templates/");

        Self {
            adress,
            handlebars,
            routes: None,
        }
    }

    pub fn run(&self) {
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
        }
    }
}
