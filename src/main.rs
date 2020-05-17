use log::{info, warn, trace};
use std::io::prelude::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;

use rusty_http::ThreadPool;

mod routing;

use routing::{Response};

#[macro_use(lazy_static)]
extern crate lazy_static;

#[macro_use]
extern crate serde_json;

fn main() {
    simple_logger::init().unwrap();

    let listener: TcpListener = TcpListener::bind("127.0.0.1:5000").unwrap();
    let pool: ThreadPool = ThreadPool::new(num_cpus::get()).unwrap();

    info!("Server started!");
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 8192] = [0; 8192];

    stream.read(&mut buffer).unwrap();

    trace!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    const GET_REQUEST: &'static [u8; 3] = b"GET";

    let response = if buffer.starts_with(GET_REQUEST) {
        route_request(&buffer)
    } else {
        warn!("Unhandled request type!");
        routing::get_404()
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn route_request(buffer: &[u8; 8192]) -> Response {
    let payload: String = String::from_utf8_lossy(&buffer[..]).to_string();
    match payload.split_whitespace().skip(1).next() {
        Some(path) => {
            routing::match_route(path)
        }
        None => {
            routing::get_404()
        }
    }
}
