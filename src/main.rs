// src/main.rs

mod config;
mod infrastructure;
extern crate dotenv;

use infrastructure::exchanges::binance::{
    get_binance_price, get_24hr_ticker_info, BinancePrice, BinanceTicker24hr,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    println!("Переменные окружения загружены");

    println!("Запускаем CryptoForge");

    let symbol = "BTCUSDT";

   
    match get_binance_price(symbol).await {
        Ok(price_data) => {
            println!(
                "
            Symbol: {}
            Price: {}
            ",
                price_data.symbol, price_data.price
            );
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    match get_24hr_ticker_info(symbol).await {
        Ok(ticker_data) => {
            print!("
        --- Динамика за 24 часа ---
        Symbol: {}
        Last Price: {}
        24h Change: {} ({:.2}%)
        24h High: {}
        24h Low: {}
        24h Volume: {}",
            ticker_data.symbol,
                ticker_data.last_price,
                ticker_data.price_change,
                ticker_data
                    .price_change_percent
                    .parse::<f64>()
                    .unwrap_or(0.0), // Конвертируем в f64 для форматирования
                ticker_data.high_price,
                ticker_data.low_price,
                ticker_data.volume
            );
        }
        Err(e) => {
            eprint!("Ошибка при получении ticker info{}", e);
        }
    }

    println!("CryptoForge завершил работу");
}
