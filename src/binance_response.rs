use serde::de;
use serde::{Deserialize, Deserializer};
use rust_decimal::Decimal;

#[derive(Debug, Deserialize)]
pub struct OfferData {
    pub price: Decimal,
    pub size: Decimal,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepthStreamData {
    pub last_update_id: usize,
    pub bids: Vec<OfferData>,
    pub asks: Vec<OfferData>,
}


#[derive(Debug, Deserialize)]
pub struct DepthStreamWrapper {
    pub stream: String,
    pub data: DepthStreamData,
}
