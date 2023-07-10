use handlebars::Handlebars;
use std::{
    collections::BTreeMap,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

mod server;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".hbs", "templates/")
        .unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, &handlebars);
    }
}

fn handle_connection(mut stream: TcpStream, handlebars: &Handlebars) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let mut data: BTreeMap<&str, String> = BTreeMap::new();

    let (status_line, content) = if request_line == "GET / HTTP/1.1" {
        data.insert("data", "world".to_string());
        (
            "HTTP/1.1 200 OK",
            handlebars.render("index", &data).unwrap(),
        )
    } else {
        (
            "HTTP/1.1 404 NOT FOUND",
            handlebars.render("404", &data).unwrap(),
        )
    };

    let response = format!("{status_line}\n\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}
