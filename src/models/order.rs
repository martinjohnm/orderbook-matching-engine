use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Side { Buy, Sell }

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Order {
    pub id: u64,
    pub symbol: String,
    pub side: Side,
    pub price: u64,
    pub quantity: u64,
}