extern crate actix_web;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use actix_web::{http, server, App, Path, Responder};
use serde::Deserialize;
use std::fmt::Write;

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct PageInfoType {
    #[allow(dead_code)]
    totalResults: u8,
    #[allow(dead_code)]
    resultsPerPage: u8
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct StatisticsType {
    viewCount: String,

    #[allow(dead_code)]
    commentCount: String,
    subscriberCount: String,

    #[allow(dead_code)]
    hiddenSubscriberCount: bool,
    videoCount: String
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct ItemType {
    #[allow(dead_code)]
    kind: String,

    #[allow(dead_code)]
    etag: String,
    id: String,
    statistics: StatisticsType
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct ChannelResponseType {
    #[allow(dead_code)]
    kind: String,

    #[allow(dead_code)]
    etag: String,

    #[allow(dead_code)]
    nextPageToken: String,

    #[allow(dead_code)]
    pageInfo: PageInfoType,
    items: Vec<ItemType>
}

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
    println!("{} {}", info.0, (info.1).len());

    let url: String =
        format!("https://www.googleapis.com/youtube/v3/channels?part=statistics&key={}&id={}",
                info.0, info.1);
    let url: &str = url.as_ref();

    let resp: String = reqwest::get(url).unwrap().text().unwrap();
    let json: ChannelResponseType = serde_json::from_str(resp.as_ref()).unwrap();
    let mut builder: String = String::new();

    for item in json.items {
        write!(&mut builder,
               "subscribers{{channel=\"{}\"}} {}\n",
               item.id,
               item.statistics.subscriberCount).unwrap();

        write!(&mut builder,
               "views{{channel=\"{}\"}} {}\n",
               item.id,
               item.statistics.viewCount).unwrap();

        write!(&mut builder,
               "videos{{channel=\"{}\"}} {}\n",
               item.id,
               item.statistics.videoCount).unwrap();
    }

    builder
}

fn main() {
    let addr: String = get_addr();
    let path: &str = "/{key}/{ids}";

    server::new(move ||
        App::new().route(path, http::Method::GET, f))
        .keep_alive(None)
        .workers(4)
        .bind(addr).unwrap()
        .run();
}