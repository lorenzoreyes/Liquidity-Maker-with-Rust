use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Instant;
use tungstenite::{connect, Message};
use url::Url;

pub mod binance;
use binance::binance_streams;
pub mod bitstamp;
use bitstamp::bitstamp_streams;


//#[tokio::main]
fn main() {
    binance_streams();
    println!("{}",b);
    // Connect to Bitstamp.net
    let (mut socket, _response) =
        connect(Url::parse("wss://ws.bitstamp.net").unwrap()).expect("Can't connect");

    // Subscribe to Live Trades channel for BTC/USD
    socket.write_message(Message::Text(json!({"event": "bts:subscribe","data": {"channel": "live_orders_btcusd"}}).to_string(),).into(),).expect("Error sending message");
    socket.write_message(Message::Text(json!({"event": "bts:subscribe","data": {"channel": "live_orders_ethusd"}}).to_string(),).into(),).expect("Error sending message");
    socket.write_message(Message::Text(json!({"event": "bts:subscribe","data": {"channel": "live_orders_adausd"}}).to_string(),).into(),).expect("Error sending message");
    socket.write_message(Message::Text(json!({"event": "bts:subscribe","data": {"channel": "live_orders_yfiiusd"}}).to_string(),).into(),).expect("Error sending message");
    socket.write_message(Message::Text(json!({"event": "bts:subscribe","data": {"channel": "live_orders_dogeusd"}}).to_string(),).into(),).expect("Error sending message");
    // Spin loop
    loop {
        // Read message from socket
        let msg = socket.read_message().expect("Error reading message");

        // Deserialize message
        let result: Result<Msg, serde_json::Error> = serde_json::from_str(msg.to_text().unwrap());

        println!("{:?}",&result);
    }
}

