// src/interfaces/telegram/handlers.rs
use teloxide::{prelude::*, utils::command::BotCommands};
//Наша инфраструктура для связи с Бинансом
use crate::infrastructure::exchanges::binance;
// enum Телеграмма 
use super::commands::Command;

//Старт
pub async fn start_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}

pub async fn price_handler(bot: Bot, msg: Message, symbol: String) -> ResponseResult<()> {
    if symbol.is_empty() {
        bot.send_message(
            msg.chat.id,
           "Пожалуйста, укажите символ для команды /price. Например: /price BTCUSDT"
        ).await?;
        return Ok(());
    }



    match binance::get_binance_price(&symbol).await {
        Ok(price_data) => {
            bot.send_message(
                msg.chat.id,
                format!("
📈 Текущая цена {}
💰 Цена: {}
",
                price_data.symbol, price_data.price
),
            )
                .await?;
        }
        Err(e) => {
            eprintln!("Error getting simple price for {}: {}", symbol, e);
            bot.send_message(
                msg.chat.id,
                format!("
Не удалось получить цену для {}. Ошибка: {}", symbol, e),
            )
            .await?;
        }
    }    
    Ok(())
}

pub async fn info_handler(bot: Bot, msg: Message, symbol: String) -> ResponseResult<()> {
    if symbol.is_empty() {
        bot.send_message(
            msg.chat.id,
            "Пожалуйста, укажите символ для команды /info. Например: /info BTCUSDT"
        ).await?;
        return Ok(());
    }

    match binance::get_24hr_ticker_info(&symbol).await {
        Ok(ticker_data) => {
            bot.send_message(
                msg.chat.id,
                format!(
        "
📊 Динамика за 24 часа для {}
💰 Последняя цена: {}
📈 Изменение за 24ч: {} ({:.2}%)
⬆️ Макс. за 24ч: {}
⬇️ Мин. за 24ч: {}
📦 Объем за 24ч: {}",
                    ticker_data.symbol,
                    ticker_data.last_price,
                    ticker_data.price_change,
                    ticker_data.price_change_percent.parse::<f64>().unwrap_or(0.0),
                    ticker_data.high_price,
                    ticker_data.low_price,
                    ticker_data.volume
                ),
            )
            .await?;
        }
        Err(e) => {
            eprintln!("Error getting 24hr ticker info for {}: {}", symbol, e);
            bot.send_message(
                msg.chat.id,
                format!("Не удалось получить динамику для {}. Ошибка: {}", symbol, e),
            )
            .await?;
        }
    }    
    Ok(())
}
