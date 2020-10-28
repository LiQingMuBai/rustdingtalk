//#![deny(warnings)]
#![allow(non_snake_case)]

extern crate reqwest;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use job_scheduler::{Job, JobScheduler};
use std::time::Duration;

use structopt::StructOpt;

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

#[derive(StructOpt)]
struct Cli {
//    _min: f32,
//    _symbol: String,

    _key: String,
    _target: String,
}

use reqwest::header::CONTENT_TYPE;



#[derive(Debug, Serialize, Deserialize)]
struct Transaction {
    blockNumber: String,
    timeStamp: String,
    hash: String,
    nonce: String,
    blockHash: String,
    from: String,
    contractAddress: String,
    to: String,
    value: String,
    tokenName: String,
    tokenSymbol: String,
    tokenDecimal: String,
    transactionIndex: String,
    gas: String,
    gasPrice: String,
    gasUsed: String,
    input: String,
    cumulativeGasUsed: String,
    confirmations: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseResult {
    status: String,
    message: String,
    result: Vec<Transaction>,
}


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

lazy_static! {
    static ref  tokenMap: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();

        map
    };
}



fn main() {
    let _args = Cli::from_args();

//    let dai: String = String::from("DAI0xe1685ae343e39b94ac237d8261dff389d9b6e5869315cac20f9e8b8d26ce4744");
//
//    if map.get("DAI0xe1685ae343e39b94ac237d8261dff389d9b6e5869315cac20f9e8b8d26ce4744").is_some() {
//        println!("key exit map");
//    }

    let mut sched = JobScheduler::new();
    sched.add(Job::new("1/60 * * * * *".parse().unwrap(), || {
        println!("I get executed every 30 seconds!");
//        let mut _url = String::new();
//        _url.push_str("https://www.okex.com/api/index/v3/");
//        _url.push_str(&_args._symbol);
//        _url.push_str("/constituents");
//
//        let response = reqwest::get(&_url).unwrap().text().unwrap();
//        let infomation: Infomation = serde_json::from_str(&response).unwrap();
//        println!("Response is {:?}", infomation);
//        let _value = infomation.data.constituents.last().unwrap();
//        if _value.usd_price > _args._min {
//            println!("original_price is {}", _value.usd_price);
//            send_ding_talk(_value.usd_price);
//        }


        let mut _url = String::new();
        _url.push_str("https://api.etherscan.io/api?module=account&action=tokentx&address=");
        _url.push_str(&_args._target);
        _url.push_str( "&startblock=11139510&endblock=999999999&sort=asc&apikey=");
        _url.push_str(&_args._key);
        let response = reqwest::get(&_url).unwrap().text().unwrap();
        let data: ResponseResult = serde_json::from_str(&response).unwrap();

        for token in &data.result {
            println!("Response is {:?}", token.tokenSymbol);
            let mut tokenSymbol = String::from(&token.tokenSymbol);
            tokenSymbol.push_str(&token.blockHash);


//            let key=tokenSymbol.to_string();

            if tokenMap.contains_key(&tokenSymbol.as_str()){
                println!("Hello,I had been here!");
            }else {
                println!("Hello,I am here!");
                tokenMap.insert(tokenSymbol.as_str(), &token.value);
            }
        }
//
//        for (k, v) in &map {
//            println!("{:?} -> {:?}", k, v);
//        }




    }));

    loop {
        sched.tick();
        std::thread::sleep(Duration::from_millis(500));
    }
}
