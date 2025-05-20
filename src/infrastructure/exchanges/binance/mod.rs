// src/infrastructure/exchanges/binance/mod.rs

pub mod client; // Объявляем модуль client.rs
pub mod types;  // Объявляем модуль types.rs

// Реэкспортируем основные функции и структуры для удобства использования
pub use client::{get_binance_price, get_24hr_ticker_info};
pub use types::{BinancePrice, BinanceTicker24hr};
