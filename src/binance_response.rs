use serde::de;
use serde::{Deserialize, Deserializer};
use rust_decimal::Decimal;

#[derive(Debug, Deserialize,Clone)]
pub struct OfferData {
    pub price: Decimal,
    pub size: Decimal,
}

#[derive(Debug, Deserialize,Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepthStreamData {
    pub last_update_id: usize,
//    pub bids: Vec<(Decimal,Decimal)>,
//    pub asks: Vec<(Decimal,Decimal)>,
    pub bids: Vec<OfferData>,
    pub asks: Vec<OfferData>,
}


#[derive(Debug, Deserialize,Clone)]
pub struct DepthStreamWrapper {
    pub stream: String,
    pub data: DepthStreamData,
}
