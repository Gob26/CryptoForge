// src/main.rs

mod config;
mod infrastructure;
// Объявляем модуль interfaces, внутри которого будет telegram
mod interfaces;
extern crate dotenv;

use teloxide::prelude::*;
use log::info;
use pretty_env_logger;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    info!("Переменные окружения загружены.");
    info!("Запускаем CryptoForge Telegram Bot...");

    let bot = Bot::from_env();

    // Запускаем наш диспетчер бота из модуля interfaces::telegram::dispatcher
    interfaces::telegram::dispatcher::run_bot(bot).await;

    info!("CryptoForge Telegram Bot завершил работу.");
}
