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
use rust_decimal::Decimal;

mod binance_response;
use binance_response::DepthStreamWrapper;
mod bitstamp_response;
use bitstamp_response::Response;
mod socket;
use socket::{bitstamp_socket,binance_url};
use crate::binance_response::OfferData;

// Pass data from endpoint to a common struct with gathering
// Exchange, ticket retrieved, bids, asks and calculate the spread

#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct Book {
    pub exchange: String,
    pub ticket: String,
    pub bids: Vec<Depth>,
    pub asks: Vec<Depth>,
}

#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct Depth {
    pub price: Decimal,
    pub size: Decimal,
}

// make orderBook.for each pair requested

fn print_Book(orderBook: &Book) {
    clearscreen::clear().expect("Error clearing");

    println!("Exchange:\t\t{}", orderBook.exchange);
    println!("Symbol:\t{}.", orderBook.ticket);
    println!("\tBids\t\tAsks");

    // Dynamic vectors iterators to store && update data
    let mut i = 0;
    for bid in orderBook.bids {
        if orderBook.asks.len() > i { 
        if i == 0 { println!("Spread$: {:08.4},\tPercent%: {:08.4}", orderBook.asks[0].price - bid[0].price, (orderBook.asks[0].price - bid[0].price) / orderBook.asks[0].price); }

        println!("Bid$: {:05.4}, BidQ: {:05.4},̣\t\t Ask$: {:05.4}, AskQ: {:05.4}\n",
                     bid[i].price, //Book.bids.price,
                     bid[i].size,  //Book.bids.size,
                     orderBook.asks[i].price,
                     orderBook.asks[i].size
                     );
        
        } else { // if update comes from the other side
        
            println!("Bid$: {:05.4}, BidQ: {:05.4}",
                       bid.price,
                       bid.size
                     );
        
        }
        i + 1;
        if i > 5 { // as you will only get 5 top bid && asks
            break;
        }
    }
}

/*impl From<OfferData> for Depth {
    fn from(price, size) -> Depth {
        Depth { price, size }
    }
}*/


// WARNING BITSTAMP PART COMMENTED TO TEST BINANCE
fn main() {
    let mut socket_binance = binance_url();

    //let mut socket_bitstamp = bitstamp_socket();

    loop {
        // Read message from socket_bitstamp
        //let msg_bitstamp = socket_bitstamp.read_message().expect("Error reading message");
        
        // Read mesasge from socket_binance
        let msg_binance = socket_binance.read_message().expect("Cannot read message");
        let msg_binance = match msg_binance {
            tungstenite::Message::Text(s) => s,
            _ => continue,//lets ignore it for now { panic!("Error getting text");}
        };
        
        //Parse them into proper structs
        //let bits: Result<Response, serde_json::Error> = serde_json::from_str(msg_bitstamp.to_text().unwrap());
        let bins: DepthStreamWrapper = serde_json::from_str(&msg_binance).expect("Can't parse");
        
        /*let bits = match bits {
            Result::Ok(ref _x) => _x,
            Result::Err(_x) => continue, //println!("Error"),
        };*/

         // Innit common structs
       
        for i in 0..10 {
            let mut bin_bids = bins.data.bids[i].into_iter()
                .map(|i| (i))
                .collect::<(Decimal,Decimal)>();
            let mut bin_asks = bins.data.asks.into_iter()[i].clone();
            let mut bins_Book = Book {
                exchange: String::from("Binance"),
                ticket: bins.stream.as_str().to_uppercase().replace("@DEPTH10@100MS",""),
                bids: vec!(bin_bids[i]),   
                asks: vec!(bin_asks[i]),
            };

            print_Book(&bins_Book);
        };
        
        // call order-Book.&& print'em
        //print_Book(&bits_Book);
        //print_Book(&bins_Book);        
        
        }
    
}
