// src/main.rs

mod config;
mod infrastructure;
extern crate dotenv;
// use std::env; // Удален, так как не используется

// Импортируем только get_binance_price, т.к. BinancePrice не используется напрямую для аннотации типа
use infrastructure::exchanges::binance::get_binance_price; 

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    println!("Переменные окружения загружены"); // Используем println!

    println!("Запускаем CryptoForge"); // Используем println!

    let symbol = "BTCUSDT";

    // ЭТОТ БЛОК match ОЧЕНЬ ВАЖЕН! ОН ДОЛЖЕН БЫТЬ ТОЧНО ТАКИМ: Ok(price_data)
    match get_binance_price(symbol).await {
        Ok(price_data) => { // <-- ВОТ ЗДЕСЬ ИСПРАВЛЕНИЕ: ТОЛЬКО `Ok(price_data)`
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
    println!("CryptoForge завершил работу"); // Используем println!
}
