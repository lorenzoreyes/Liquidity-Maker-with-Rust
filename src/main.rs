use tungstenite::connect;
use url::Url;
use std::io;

mod models;

static BINANCE_WS_API: &str = "wss://stream.binance.com:9443";

fn main() {
    println!("Provide 1rst pair for the stream\n");
    let mut ticket = String::new();
    io::stdin().read_line(&mut ticket).expect("Failed to read pair");
    // 2nd
    println!("First one was: {}.\n Now provide Second pair for the stream",ticket.to_uppercase());
    let mut ticket2 = String::new();
    io::stdin().read_line(&mut ticket2).expect("Failed to read pair");
    // 3rd
    println!("2nd one: {}.\nNow the 3rd pair for the stream",ticket2);
    let mut ticket3 = String::new();
    io::stdin().read_line(&mut ticket3).expect("Failed to read pair");
    // 4th
    println!("3rd: {}.\nProvide 4th pair for the stream",ticket3);
    let mut ticket4 = String::new();
    io::stdin().read_line(&mut ticket4).expect("Failed to read pair");
    // 5th and last one
    println!("4th one: {}.\nProvide 5th and last pair for the stream",ticket4);
    let mut ticket5 = String::new();
    io::stdin().read_line(&mut ticket5).expect("Failed to read pair");
    println!("{} was the last pair as input",ticket5);
    
    let binance_url = format!("{}/stream?streams={}@depth10@100ms/{}@depth10@100ms/{}@depth10@100ms/{}@depth10@100ms/{}@depth10@100ms", BINANCE_WS_API,ticket,ticket2,ticket3,ticket4,ticket5);
    
    println!("You requested the following stream: {}", binance_url);

    let (mut socket, response) = connect(Url::parse(&binance_url).unwrap()).expect("Can't connect.");

    loop {
        let msg = socket.read_message().expect("Error reading message");
        let msg = match msg {
            tungstenite::Message::Text(s) => s,
            _ => { panic!("Error getting text");}
        };

        let parsed: models::DepthStreamWrapper = serde_json::from_str(&msg).expect("Can't parse");
        // Asks
        for i in 0..parsed.data.asks.len() {
            println!(
                "{}: {}, ask: {}, size: {}",
                parsed.stream, i, parsed.data.asks[i].price, parsed.data.asks[i].size
                );
        }
        // Bids
        for i in 0..parsed.data.bids.len() {
            println!(
                "{}: {}, bid: {}, size: {}",
                parsed.stream, i, parsed.data.bids[i].price, parsed.data.bids[i].size
                );
        }
    }
}
