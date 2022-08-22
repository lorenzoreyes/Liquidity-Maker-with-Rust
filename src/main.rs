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

mod binance_response;
use binance_response::DepthStreamWrapper;
mod bitstamp_response;
use bitstamp_response::Response;
mod socket;
use socket::{bitstamp_socket,binance_url};

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
            _ => continue,//{ panic!("Error getting text");}
        };
        
        //Parse them into proper structs
        let bits: Result<Response, serde_json::Error> = serde_json::from_str(msg_bitstamp.to_text().unwrap());
        let bins: DepthStreamWrapper = serde_json::from_str(&msg_binance).expect("Can't parse");
        
        match bits {
            Result::Ok(ref _x) => _x,
            Result::Err(_x) => continue,//println!("Bitstamp Error"),
        };
 
         for i in 0..10 {
             let bits = bits.as_ref().ok().unwrap(); // for readability
                    // pair-match
                    if bits.channel.as_str().to_uppercase().replace("ORDER_BOOK_","") == bins.stream.as_str().to_uppercase().replace("T@DEPTH10@100MS","") {
                        if i == 0 {
                            println!("\nPair: {},\tExchange: Bitstamp,\t Time {} UTC",
                                 bits.channel.as_str().to_uppercase().replace("ORDER_BOOK_",""),
                                 Utc::now().format("%c %p"));
                            println!("Pair: {},\tExchange: BINANCE,\t Time {} UTC\n",
                                 bins.stream.as_str().to_uppercase().replace("T@DEPTH10@100MS",""),
                                 Utc::now().format("%c %p"));
                            println!("BINANCE  Fair$: {:04.04}\tSpread$: {:.5},\tSpread%: {:.5}",
                             (bins.data.asks[i].price - bins.data.bids[i].price) + bins.data.bids[i].price,
                             (bins.data.asks[i].price - bins.data.bids[i].price),
                             (bins.data.asks[i].price - bins.data.bids[i].price) / bins.data.asks[i].price
                             );

                            println!("BITSTAMP Fair$: {:04.04}\tSpread$: {:.5},\tSpread%: {:.5}\n",
                             (bits.data.asks[i].price - bits.data.bids[i].price) + bits.data.bids[i].price,
                             (bits.data.asks[i].price - bits.data.bids[i].price),
                             (bits.data.asks[i].price - bits.data.bids[i].price) / bits.data.asks[i].price
                                 );
                        }
                        println!("Bitstamp #{}\tBid: ${:04.04}, BQuant: {:04.2},\tAsk: ${:04.04}, AQuant: {:04.2}",
                             i + 1,
                             bits.data.bids[i].price,
                             bits.data.bids[i].size,
                             bits.data.asks[i].price,
                             bits.data.asks[i].size,
                             );

                        println!("Binance  #{}\tBid: ${:04.04}, BQuant: {:04.2},\tAsk: ${:04.04}, AQuant: {:04.2}",
                                i + 1,
                                bins.data.bids[i].price,
                                bins.data.bids[i].size,
                                bins.data.asks[i].price,
                                bins.data.asks[i].size
                                )
                    }


            }


        }

}
