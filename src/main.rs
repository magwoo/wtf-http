use handlebars::Handlebars;
use serde::Serialize;
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

#[derive(Serialize)]
struct IndexPage {
    data: String,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        println!("connected");
        handle_connection(stream.unwrap());
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let mut handlebars = Handlebars::new();
    let _request_line = buf_reader.lines().next().unwrap().unwrap();

    handlebars
        .register_template_file("index", "templates/index.hbs")
        .unwrap();

    let data = IndexPage {
        data: "hello".to_string(),
    };

    let content = handlebars.render("index", &data).unwrap();

    let response = format!("HTTP/1.1 200 OK\n\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}
