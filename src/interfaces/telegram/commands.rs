// src/interfaces/telegram/commands.rs

use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Поддерживаемые команды")]
pub enum Command {
    #[command(description = "Справочное сообщение")]
    Start,
    #[command(description = "Курс на валюту (Пример запроса /price BTCUSDT)")]
    Price(String),
    #[command(description = "Получаем динамику за 24 часа по валюте (Пример запроса /info BTCUSDT)")]
    Info(String),
    #[command(description = "Добавить адрес для отслеживания кита (Пример: /add_whale 0x...)")]
    AddWhale(String),
}
