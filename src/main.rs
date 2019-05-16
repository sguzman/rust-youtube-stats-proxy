extern crate actix_web;

use std::env;
use actix_web::{http, server, App, Path, Responder};

fn get_addr() -> String {
    let key: &str = "PORT";
    let value: String = match env::var(key) {
        Ok(val) => val,
        Err(_) => format!("{}","8888")
    };

    let addr: String = format!("localhost:{}", value);
    return addr;
}

fn f(info: Path<(String, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

fn main() {
    let addr: String = get_addr();
    let path: &str = "/{key}/{ids}";

    server::new(move ||
        App::new().route(path, http::Method::GET, f))
        .bind(addr).unwrap()
        .run();
}