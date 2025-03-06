use std::{
    io::{prelude::*},
    net::{TcpListener, TcpStream},
    env
};

fn main() {
    let address = env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1:8888".to_string());
    let listener = TcpListener::bind(&address).unwrap();

    println!("Listening on {address}");

    for stream in listener.incoming() {
        println!("Received a connection, handling ...");
        let stream = stream.unwrap();

        handle(stream);
    }
}

fn handle(mut stream: TcpStream) {
    let response = "HTTP/1.1 200 OK\r\n\r\nHello, World!\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}
