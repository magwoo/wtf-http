use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }
}

fn handle_client(stream: TcpStream) {
    println!("{:#?}", stream);
}
