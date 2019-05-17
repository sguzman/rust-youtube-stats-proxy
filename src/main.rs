#![allow(non_snake_case)]

extern crate actix_web;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use actix_web::{http, server, App, Path, Responder};
use serde::{Deserialize, Serialize};
use std::fmt::Write;

#[derive(Serialize, Deserialize)]
struct PageInfoType {
    totalResults: u8,
    resultsPerPage: u8
}

#[derive(Serialize, Deserialize)]
struct StatisticsType {
    viewCount: String,
    commentCount: String,
    subscriberCount: String,
    hiddenSubscriberCount: bool,
    videoCount: String
}

#[derive(Serialize, Deserialize)]
struct ItemType {
    kind: String,
    etag: String,
    id: String,
    statistics: StatisticsType
}

#[derive(Serialize, Deserialize)]
struct ChannelResponseType {
    kind: String,
    etag: String,
    nextPageToken: String,
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
        .bind(addr).unwrap()
        .run();
}