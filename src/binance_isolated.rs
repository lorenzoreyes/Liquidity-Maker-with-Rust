use tungstenite::connect;
use url::Url;
use std::io;
use std::fs::File;
use serde_json::from_reader; 
use serde_json::Value;
mod binance_response;
use binance_response::DepthStreamWrapper;
use chrono::Utc;

static BINANCE_WS_API: &str = "wss://stream.binance.com:9443";

pub fn main() {
    let file = File::open("src/config.json").expect("Not JSON format");
    let json: serde_json::Value = from_reader(file).expect("file should be a proper json");
    let ticket = json.get("ticket").expect("File should be a currency key");
    let ticket2 = json.get("ticket2").expect("File should be a currency key");
    let ticket3 = json.get("ticket3").expect("File should be a currency key");
    let ticket4 = json.get("ticket4").expect("File should be a currency key");
    let ticket5 = json.get("ticket5").expect("File should be a currency key");
    println!("ticket1: {}, ticket2: {}, ticket3: {}, ticket4: {}, ticket5: {}",&ticket.as_str().unwrap(),&ticket2.as_str().unwrap(),&ticket3.as_str().unwrap(),ticket4.as_str().unwrap(),ticket5.as_str().unwrap());
    
    let binance_url = format!("{}/stream?streams={}@depth10@100ms/{}@depth10@100ms/{}@depth10@100ms/{}@depth10@100ms/{}@depth10@100ms", BINANCE_WS_API,&ticket.as_str().unwrap(),&ticket2.as_str().unwrap(),&ticket3.as_str().unwrap(),ticket4.as_str().unwrap(),ticket5.as_str().unwrap());

    
    println!("\n\n\nYou requested the following stream:\n{:#?}\n\n\n", binance_url);

    
    let (mut socket, response) = connect(Url::parse(&binance_url).unwrap()).expect("Can't connect.");

    // Loop to connect and parse messages
    loop {
        let msg = socket.read_message().expect("Error reading message");
        let msg = match msg {
            tungstenite::Message::Text(s) => s,
            _ => { panic!("Error getting text");}
        };

        let parsed: DepthStreamWrapper = serde_json::from_str(&msg).expect("Can't parse");

        for i in 0..5 { //parsed.data.asks.len() {
 
            if i == 0 { 
                println!("\n\nPair: {},\tExchange: Binance,\tTime: {}\n",
                         parsed.stream.as_str().to_uppercase().replace("@DEPTH10@100MS",""), 
                         Utc::now().format("%a %b %e %T %Y"));
 
                println!("Spread$:\t{:08.2}.\t\tSpread%:\t{:08.2}",
                     (parsed.data.asks[i].price - parsed.data.bids[i].price),
                     (parsed.data.asks[i].price - parsed.data.bids[i].price) / parsed.data.asks[i].price,
                     );}

            println!("Bid$:{:08.8}, BidQ: {:08.2}\tAsk$:{:08.8} AskQ: {:08.2}",
                     parsed.data.bids[i].price,
                     parsed.data.bids[i].size,
                     parsed.data.asks[i].price, 
                     parsed.data.asks[i].size
                );
        }
    }
}
