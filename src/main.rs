extern crate actix_web;
extern crate reqwest;

use actix_web::{http, server, App, Path, Responder};

fn get_addr() -> String {
    let key: &str = "PORT";
    let value: String = match std::env::var(key) {
        Ok(val) => val,
        Err(_) => format!("{}","8888")
    };

    let addr: String = format!("localhost:{}", value);
    println!("Listening at {}", addr);

    return addr;
}

fn f(info: Path<(String, String)>) -> impl Responder {
    let url: String =
        format!("https://www.googleapis.com/youtube/v3/channels?part=statistics&key={}&id={}",
                info.0, info.1);
    let url: &str = url.as_ref();

    let resp: String = reqwest::get(url).unwrap().text().unwrap();
    format!("{}:{}\n{}", info.1, info.0, resp)
}

fn main() {
    let addr: String = get_addr();
    let path: &str = "/{key}/{ids}";

    server::new(move ||
        App::new().route(path, http::Method::GET, f))
        .bind(addr).unwrap()
        .run();
}