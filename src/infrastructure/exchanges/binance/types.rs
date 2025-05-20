// src/infrastructure/exchanges/binance/types.rs

use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct BinancePrice{
	pub symbol: String,
	pub price: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")] //Binance использует camelCase в JSON
pub struct BinanceTicker24hr{
    pub symbol: String,
    pub price_change: String,
    pub price_change_percent: String,
    pub weighted_avg_price: String,
    pub prev_close_price: String,
    pub last_price: String,
    pub last_qty: String,
    pub bid_price: String,
    pub bid_qty: String,
    pub ask_price: String,
    pub ask_qty: String,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub volume: String,
    pub quote_volume: String,
    pub open_time: u64,
    pub close_time: u64,
    pub first_id: u64,
    pub last_id: u64,
    pub count: u64,
}
