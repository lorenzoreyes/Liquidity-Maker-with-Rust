//mod binance;
//use crate::binance::binance_streams;

use serde::Deserialize;
use std::fs;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;


#[derive(Debug,Deserialize)]
pub struct Data {
    pairs: Vec<String>,
}

static BINANCE_WS_API: &str = "wss://stream.binance.com:9443";

pub fn binance_url() {
    let file = File::open("src/config2.json").unwrap();
    let reader = BufReader::new(file);
    let json: Data = serde_json::from_reader(reader).expect("Cant read");
    
    let x = json.pairs.iter();
    let z: String = x.fold(String::new(), | a, b | a + b + "@depth10@100ms/");
    let binance_url = String::from(BINANCE_WS_API) + &z;
    return binance_url
}

pub bitstamp_socket() {
    let file = File::open("src/config2.json").unwrap();
    let reader = BufReader::new(file);
    let json: Data = serde_json::from_reader(reader).expect("Cant read");
    // replace usdt => usd
    //let x = json.pairs.iter();
    //let pairs = [];
    for i in &json.pairs {
        let i = i.as_str().replace("usdt","usd");
        println!("i is: {}",i);
    }
}
 
