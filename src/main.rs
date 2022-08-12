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
mod binance_response;
use binance_response::DepthStreamWrapper;
mod bitstamp_response;
use bitstamp_response::Response;
mod socket;
use socket::{bitstamp_socket,binance_url};

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
    bids: Depth,
    asks: Depth,
    spread: (asks - bids) / asks,
    time: Utc::now().format("%c %r"),
}

struct Depth {
    price: Vec<Decimal>,
    size: Vec<Decimal>,
}

// make orderbook for each pair requested

fn print_book(book: &orderbook) {
    clearscreen::clear().expect("Error clearing");

    println!("Exchange:\t\t{}", book.exchange);
    println!("Symbol:\t{}.\tSpread%:\t{}",book.ticket,book.spread);
    println!("Time:\t{} UTC\n\n", book.time);
    println!("\tBids\t\tAsks");

    // Dynamic vectors iterators to store && update data
    let mut i = 0;
    for bid in bids.len() {
        if book.asks.len() > i {
            println!("Bid$: {:05.4}, BidQ: {:05.4},̣\t\t Ask$: {:05.4}, AskQ: {:05.4}\n",
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
    let mut socket_binance = binance_url();

    let mut socket_bitstamp = bitstamp_socket();

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
                for i in 0..5 { //parsed.data.asks[0..50] {
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
