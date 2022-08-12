use serde::Deserialize;
use std::fs;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::mem::replace;
use serde_json::{json,from_reader};
use tungstenite::{connect, Message, WebSocket};
use tungstenite::stream::MaybeTlsStream;
use std::net::TcpStream;
use url::Url;

#[derive(Debug,Deserialize)]
pub struct Data {
    pairs: Vec<String>,
}

static BINANCE_WS_API: &str = "wss://stream.binance.com:9443/stream?streams=";

pub fn binance_url() -> WebSocket<MaybeTlsStream<TcpStream>> { 

    let file = File::open("src/config2.json").unwrap();
    let reader = BufReader::new(file);
    let json: Data = serde_json::from_reader(reader).expect("Cant read");
    
    let x = json.pairs.iter();
    let z: String = x.fold(String::new(), | a, b | a + b + "@depth10@100ms/");
    let binance_url = String::from(BINANCE_WS_API) + &z[..&z.len()-1];
    
    let (mut socket_binance, _response) = connect(Url::parse(&binance_url).unwrap()).expect("Can't connect");

    return socket_binance
}

pub fn bitstamp_socket() -> WebSocket<MaybeTlsStream<TcpStream>> { 
    let file = File::open("src/config2.json").unwrap();
    let reader = BufReader::new(file);
    let mut json: Data = serde_json::from_reader(reader).expect("Cant read");

    for i in json.pairs.iter_mut() {
        *i = i.as_str().replace("usdt","usd");
        *i = String::from("order_book_") + i;
    }
    // Start connection
    let (mut socket_bitstamp, _response) =
        connect(Url::parse("wss://ws.bitstamp.net").unwrap()).expect("Can't connect");

       // Subscribe iteratively each pair
    for i in json.pairs {
        socket_bitstamp.write_message(Message::Text(json!({"event": "bts:subscribe","data": {"channel":  &i.as_str() }}).to_string(),).into(),).expect("Error sending message");
    }
    return socket_bitstamp
}


struct User {
    username: String,
    name: String,
    lastname: String,
    age: i8,
    email: String,
}

/*fn main() {
    let user = User {
        username: String::from("Ikki"),
        name: String::from("Lorenzo"),
        lastname: String::from("Reyes"),
        age: 28,
        email: String::from("lreyes@udesa.edu.ar"),
    };

    println!("User at Database is {},\nName: {},\nLastname: {},\nAge: {},\nEmail: {}",
             user.username,
             user.name,
             user.lastname,
             user.age,
             user.email);
    println!("{}", binance_url());
    println!("{:?}", bitstamp_socket());
}*/
