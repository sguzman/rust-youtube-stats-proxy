extern crate actix_web;

use std::env;
use actix_web::{http, server, App, Path, Responder};

fn get_addr() -> String {
    let key: &str = "PORT";
    let value = match env::var(key) {
        Ok(val) => val,
        Err(_) => format!("{}","8888")
    };

    let addr: String = format!("localhost:{}", value);
    return addr;
}

fn index(info: Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

fn main() {
    let addr = get_addr();

    server::new(
        || App::new()
            .route("/{id}/{name}", http::Method::GET, index))
        .bind(addr).unwrap()
        .run();
}