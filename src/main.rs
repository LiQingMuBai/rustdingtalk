#![deny(warnings)]
#![allow(non_snake_case)]

extern crate reqwest;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use job_scheduler::{Job, JobScheduler};
use std::time::Duration;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    _min: f32,
    _symbol: String,
}

use reqwest::header::CONTENT_TYPE;

#[derive(Debug, Serialize, Deserialize)]
struct Constituent {
    symbol: String,
    original_price: f32,
    weight: f32,
    usd_price: f32,
    exchange: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    last: String,
    instrument_id: String,
    timestamp: String,
    constituents: Vec<Constituent>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Infomation {
    code: u8,
    data: Data,
    msg: String,
    detailMsg: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Text {
    content: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct At {
    isAtAll: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    msgtype: String,
    text: Text,
    at: At,
}

fn send_ding_talk(_price: f32) {
    let new_text = Text { content: _price };
    let new_at = At { isAtAll: true };

    let new_post = Post {
        msgtype: "text".into(),
        text: new_text,
        at: new_at,
    };
    let _value = reqwest::Client::new()
        .post("url address")
        .header(CONTENT_TYPE, "application/json")
        .json(&new_post)
        .send()
        .expect("Sorry,The Chinese government had banned the website.");
}

fn main() {
    let _args = Cli::from_args();
    let mut sched = JobScheduler::new();
    sched.add(Job::new("1/30 * * * * *".parse().unwrap(), || {
        println!("I get executed every 30 seconds!");
        let mut _url = String::new();
        _url.push_str("https://www.okex.com/api/index/v3/");
        _url.push_str(&_args._symbol);
        _url.push_str("/constituents");

        let response = reqwest::get(&_url).unwrap().text().unwrap();
        let infomation: Infomation = serde_json::from_str(&response).unwrap();
        println!("Response is {:?}", infomation);
        let _value = infomation.data.constituents.last().unwrap();
        if _value.usd_price > _args._min {
            println!("original_price is {}", _value.usd_price);
            send_ding_talk(_value.usd_price);
        }
    }));

    loop {
        sched.tick();
        std::thread::sleep(Duration::from_millis(500));
    }
}
