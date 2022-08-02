use serde::de;
use serde::{Deserialize, Deserializer};

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

#[derive(Debug,Deserialize)]
pub struct Orders {
    #[serde(deserialize_with = "de_float_from_str")]
    pub price: f32,
    #[serde(deserialize_with = "de_float_from_str")]
    pub size: f32,
}

pub fn de_float_from_str<'a, D>(deserializer: D) -> Result<f32, D::Error> where
D: Deserializer<'a>,
{
    let str_val = String::deserialize(deserializer)?;
    str_val.parse::<f32>().map_err(de::Error::custom)
}



