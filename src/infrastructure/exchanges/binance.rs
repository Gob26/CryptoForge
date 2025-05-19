use reqwest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BinancePrice {
    pub symbol: String,
    pub price: String,
}

pub async fn get_binance_price(symbol: &str) -> Result<BinancePrice, reqwest::Error> {
    let url = format!("https://api.binance.com/api/v3/ticker/price?symbol={}", symbol);
    println!("Отправляем GET-запрос к Binance: {}", url);
    
    let response = reqwest::get(url).await?;
    let response = response.error_for_status()?;
    let price_data: BinancePrice = response.json::<BinancePrice>().await?;
    
    Ok(price_data)
}