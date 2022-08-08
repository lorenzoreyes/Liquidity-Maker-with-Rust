use serde::de;
use serde::{Serialize, Deserialize, Deserializer};
use rust_decimal::Decimal;

#[derive(Debug, Deserialize)]
pub struct Response {
    pub channel: String,
    pub event: String,
    pub data: Data,
}

#[derive(Debug,Deserialize)]
pub struct Data {
    pub timestamp: String,
    pub microtimestamp: String,
    pub bids: Vec<Orders>,
    pub asks: Vec<Orders>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Orders {
    pub price: Decimal,
    pub size: Decimal,
}



