use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_json::{json,from_reader};
use tungstenite::{connect, Message};
use chrono::{DateTime,Utc};
use chrono::prelude::*;
use url::Url;
use std::thread;
use futures::future;
use std::fs::File;

//mod configurable;
//use configurable::binance_url;
mod binance;
use binance::binance_streams;
mod binance_response;
use binance_response::DepthStreamWrapper;
mod bitstamp_response;
use bitstamp_response::Response;


/* NOTE premature approach
 * I will retrieve binance streams
 * request bitstamp messages
 * generate socket_bitstamps for each exchange here
 * first wanna generate socket_bitstamps here in order if it crash it crash at this file
 * will use threads to spawn them
 */

// Pass data from endpoint to a common struct with gathering
// Exchange, ticket retrieved, bids, asks and calculate the spread
/*
struct orderBook {
    exchange: String,
    ticket: String,
    bids: Vec<price,size>,
    asks: Vec<price,size>,
    spread: (asks - bids) / asks,
    time: Utc::now().format("%c %r"),
}

fn print_book(book: &orderbook) {
    clearscreen>>clear().expect("Error clearing");

    println!("Exchange:\t\t{}", book.exchange);
    println!("Symbol:\t{}.\tSpread%:\t{}",book.ticket,book.spread);
    println!("Time:\t{} UTC\n\n", book.time);
    println!("\tBids\t\tAsks");

    // Dynamic vectors iterators to store && update data
    let mut i = 0;
    for bid in bids.len() {
        if book.asks.len() > i {
            println!("Bid$: {:05.4}, BidQ: {:05.4}, Ask$: {:05.4}, AskQ: {:05.4}\n",
                     book.bids.price,
                     book.bids.size,
                     book.asks.price,
                     book.asks,size
                     );
        } else { // if update comes from the other side
            println!("Bid$: {:05.4}, BidQ: {:05.4}",
                     book.bid.price,
                     book.bid.size
                     );
        }
        i + 1;
        if i > 5 { // as you will only get 5 top bid && asks
            break;
        }
    }
}
*/

fn main() {
    //let binance = String::from(binance_url());

    let binance_url = binance_streams("binance_url".to_string());
    
    let (mut socket_binance, response) = connect(Url::parse(&binance_url).unwrap()).expect("Can't connect.");

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
    let (mut socket_bitstamp, _response) =
        connect(Url::parse("wss://ws.bitstamp.net").unwrap()).expect("Can't connect");

    // Subscribe to Live Trades channel for BTC/USD
    socket_bitstamp.write_message(Message::Text(json!({"event": "bts:subscribe","data": {"channel":  &ticket.as_str() }}).to_string(),).into(),).expect("Error sending message");
    socket_bitstamp.write_message(Message::Text(json!({"event": "bts:subscribe","data": {"channel":  &ticket2.as_str() }}).to_string(),).into(),).expect("Error sending message");
    socket_bitstamp.write_message(Message::Text(json!({"event": "bts:subscribe","data": {"channel" : &ticket3.as_str() }}).to_string(),).into(),).expect("Error sending message");
    socket_bitstamp.write_message(Message::Text(json!({"event": "bts:subscribe","data": {"channel":  &ticket4.as_str() }}).to_string(),).into(),).expect("Error sending message");
    socket_bitstamp.write_message(Message::Text(json!({"event": "bts:subscribe","data": {"channel" : &ticket5.as_str() }}).to_string(),).into(),).expect("Error sending message");
 
    loop {
        // Read message from socket_bitstamp
        let msg_bitstamp = socket_bitstamp.read_message().expect("Error reading message");
        
        // Read mesasge from socket_binance
        let msg_binance = socket_binance.read_message().expect("Cannot read message");
        let msg_binance = match msg_binance {
            tungstenite::Message::Text(s) => s,
            _ => { panic!("Error getting text");}
        };
        
        //Parse them into proper structs
        let bits: Result<Response, serde_json::Error> = serde_json::from_str(msg_bitstamp.to_text().unwrap());
        let bins: DepthStreamWrapper = serde_json::from_str(&msg_binance).expect("Can't parse");
        
        match bits {
            Result::Ok(ref _x) => 
                for i in 0..1 { //parsed.data.asks[0..50] {
                    let bits = bits.as_ref().ok().unwrap(); // for readability
                    println!("\nExchange: Bitstamp,\t Time {} UTC",Utc::now().format("%c %p"));
                    println!("Pair: {},\t\t\tSpread%: {:02.6},\nBid$: {:05.4}, BidQ: {:05.4}, Ask$: {:05.4}, AskQ: {:05.4}\nMicroTimestamp: {:#?}",
                             bits.channel.as_str().to_uppercase().replace("ORDER_BOOK_",""),
                             (bits.data.asks[i].price - bits.data.bids[i].price) / bits.data.asks[i].price,
                             bits.data.bids[i].price,
                             bits.data.bids[i].size,
                             bits.data.asks[i].price, 
                             bits.data.asks[i].size,
                             NaiveDateTime::from_timestamp(bits.data.timestamp.as_str().parse::<i64>().unwrap(),0),
                             );
                    println!("\nExchange: Binance,\t\tTime: {}",Utc::now().format("%c %p"));
                    println!("Pair: {},\t\t\tSpread%: {:05.4},\nBid$: {:05.4}, BidQ: {:05.4}, Ask$: {:05.4}, AskQ: {:05.4}\n",
                                bins.stream.as_str().to_uppercase().replace("@DEPTH10@100MS",""),
                                (bins.data.asks[i].price - bins.data.bids[i].price) / bins.data.asks[i].price,
                                bins.data.bids[i].price,
                                bins.data.bids[i].size,
                                bins.data.asks[i].price, 
                                bins.data.asks[i].size
                                )
 
                },
            Result::Err(_x) => println!("Error"),
            }
        }
    
}
