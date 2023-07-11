use std::collections::HashMap;

use handlebars::Handlebars;
use server::{
    request::{Method, Route},
    response::HttpResponse,
    HttpServer,
};

mod server;

fn main() {
    let routes: &[Route] = &[Route {
        method: Method::Get,
        uri: "/",
        handler: &home_page,
    }];
    HttpServer::new("127.0.0.1:7878", Some(routes)).run();
}

fn home_page() -> HttpResponse {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".hbs", "templates/")
        .unwrap();

    let mut data: HashMap<&str, String> = HashMap::new();
    data.insert("data", "world".to_string());

    HttpResponse::Ok().body(handlebars.render("index", &data).unwrap())
}
