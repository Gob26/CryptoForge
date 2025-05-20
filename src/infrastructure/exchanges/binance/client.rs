// src/infrastructure/exchanges/binance/client.rs

use reqwest;
// Импортируем типы из нашего нового модуля types.rs (ВАЖНО! "super::types")
use super::types::{BinancePrice, BinanceTicker24hr};

// Оставляем старую функцию, если она нужна для простого запроса цены
pub async fn get_binance_price(symbol: &str) -> Result<BinancePrice, reqwest::Error> {
    let url = format!("https://api.binance.com/api/v3/ticker/price?symbol={}", symbol);
    println!("Отправляем GET-запрос к Binance: {}", url);

    let response = reqwest::get(&url).await?;
    let response = response.error_for_status()?;
    let price_data: BinancePrice = response.json::<BinancePrice>().await?;
    Ok(price_data)
}

pub async fn get_24hr_ticker_info(symbol: &str) -> Result<BinanceTicker24hr, reqwest::Error>{
    let url = format!("https://api.binance.com/api/v3/ticker/24hr?symbol={}", symbol);
    println!("Отправляем GET-запрос к Бинанс: {}", url);
    
    let response = reqwest::get(&url).await?;
    let response = response.error_for_status()?;
    let ticker_info: BinanceTicker24hr = response.json::<BinanceTicker24hr>().await?;
    Ok(ticker_info)
}
