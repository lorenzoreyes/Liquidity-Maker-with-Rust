use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Instant;
use tungstenite::{connect, Message};
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
struct Msg {
    channel: String,
    event: String,
}

// for test change the name of the function to main 
fn main() {
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
        break
    }
}

