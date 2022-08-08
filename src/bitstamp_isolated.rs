use derivative::Derivative;
use serde::{de, Deserialize, Serialize, Deserializer};
use serde_json::{json,from_reader};
use std::fs::File;
use tungstenite::{connect, Message};
use url::Url;
use chrono::{DateTime,Utc};
use chrono::prelude::*;
use dateparser::datetime::Parse;
mod bitstamp_response;
use bitstamp_response::Response;


// for test change the name of the function to main 
fn main() {
    let file = File::open("src/config.json").expect("Not JSON format");
    let json: serde_json::Value = from_reader(file).expect("file should be a proper json");
    let ticket  = json.get("ticket").clone().expect("cannot read");
    let ticket: String = String::from("order_book_") + ticket.as_str().unwrap();
    let ticket2 = json.get("ticket2").clone().expect("cannot read");
    let ticket2: String = String::from("order_book_") + ticket2.as_str().unwrap();
    let ticket3 = json.get("ticket3").clone().expect("cannot read");
    let ticket3: String = String::from("order_book_") + ticket3.as_str().unwrap();
    let ticket4 = json.get("ticket4").clone().expect("cannot read");
    let ticket4: String = String::from("order_book_") + ticket4.as_str().unwrap();
    let ticket5 = json.get("ticket5").clone().expect("cannot read");
    let ticket5: String = String::from("order_book_") + ticket5.as_str().unwrap();
    let ticket = ticket.as_str().replace("usdt","usd");
    let ticket2 = ticket2.as_str().replace("usdt","usd");
    let ticket3 = ticket3.as_str().replace("usdt","usd");
    let ticket4 = ticket4.as_str().replace("usdt","usd");
    let ticket5 = ticket5.as_str().replace("usdt","usd");
    println!("Tickets are: {}, {}, {}, {}, {}",
        &ticket.as_str(),
        &ticket2.as_str(),
        &ticket3.as_str(),
        &ticket4.as_str(),
        &ticket5.as_str());
    // Connect to Bitstamp.net
    let (mut socket, _response) =
        connect(Url::parse("wss://ws.bitstamp.net").unwrap()).expect("Can't connect");

    // Subscribe to Live Trades channel for BTC/USD
    socket.write_message(Message::Text(json!({"event": "bts:subscribe","data": {"channel":  &ticket.as_str() }}).to_string(),).into(),).expect("Error sending message");
    socket.write_message(Message::Text(json!({"event": "bts:subscribe","data": {"channel":  &ticket2.as_str() }}).to_string(),).into(),).expect("Error sending message");
    socket.write_message(Message::Text(json!({"event": "bts:subscribe","data": {"channel" : &ticket3.as_str() }}).to_string(),).into(),).expect("Error sending message");
    socket.write_message(Message::Text(json!({"event": "bts:subscribe","data": {"channel":  &ticket4.as_str() }}).to_string(),).into(),).expect("Error sending message");
    socket.write_message(Message::Text(json!({"event": "bts:subscribe","data": {"channel" : &ticket5.as_str() }}).to_string(),).into(),).expect("Error sending message");
    // Spin loop
    loop {
        // Read message from socket
        let msg = socket.read_message().expect("Error reading message");
        let parsed: Result<Response, serde_json::Error> = serde_json::from_str(msg.to_text().unwrap());
        match parsed {
            Result::Ok(ref _x) => 
                for i in 0..1 { //parsed.data.asks[0..50] {
                    let parsed = parsed.as_ref().ok().unwrap(); // for readability
                    println!("Exchange: Bitstamp,\t Time {} UTC",Utc::now().format("%c %p"));
                    println!("Pair: {:?},\tSpread%: {:?},\nBid$: {:?}, BidQ: {:?}, Ask$: {:?}, AskQ: {:?}\nMicroTimestamp: {:#?}",
                             parsed.channel.as_str().to_uppercase().replace("ORDER_BOOK_",""),
                             (parsed.data.asks[i].price - parsed.data.bids[i].price) / parsed.data.asks[i].price,
                             parsed.data.bids[i].price,
                             parsed.data.bids[i].size,
                             parsed.data.asks[i].price, 
                             parsed.data.asks[i].size,
                             NaiveDateTime::from_timestamp(parsed.data.timestamp.as_str().parse::<i64>().unwrap(),0)
                    )},
            Result::Err(_x) => println!("Error"),
        }
    }
}

