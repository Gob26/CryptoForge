// src/interfaces/telegram/handlers.rs рабоч

use teloxide::{prelude::*, utils::command::BotCommands};
// Наша инфраструктура для связи с Binance
use crate::infrastructure::exchanges::binance;
// enum Телеграмма 
use super::commands::Command;
// appstate импортируем
use super::dispatcher::AppState;

// Старт
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
            let message = format!(
                "📈 Текущая цена {}\n💰 Цена: {}",
                price_data.symbol, 
                price_data.price
            );
            bot.send_message(msg.chat.id, message).await?;
        }
        Err(e) => {
            eprintln!("Error getting simple price for {}: {}", symbol, e);
            let error_message = format!(
                "Не удалось получить цену для {}. Ошибка: {}", 
                symbol, e
            );
            bot.send_message(msg.chat.id, error_message).await?;
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
            let percentage = ticker_data.price_change_percent
                .parse::<f64>()
                .unwrap_or(0.0);
            
            let message = format!(
                "📊 Динамика за 24 часа для {}\n💰 Последняя цена: {}\n📈 Изменение за 24ч: {} ({:.2}%)\n⬆️ Макс. за 24ч: {}\n⬇️ Мин. за 24ч: {}\n📦 Объем за 24ч: {}",
                ticker_data.symbol,
                ticker_data.last_price,
                ticker_data.price_change,
                percentage,
                ticker_data.high_price,
                ticker_data.low_price,
                ticker_data.volume
            );
            
            bot.send_message(msg.chat.id, message).await?;
        }
        Err(e) => {
            eprintln!("Error getting 24hr ticker info for {}: {}", symbol, e);
            let error_message = format!(
                "Не удалось получить динамику для {}. Ошибка: {}", 
                symbol, e
            );
            bot.send_message(msg.chat.id, error_message).await?;
        }
    }    
    Ok(())
}

pub async fn add_whale_handler(
    bot: Bot,
    msg: Message,
    state: AppState,
    address: String,
) -> ResponseResult<()> {
    if address.is_empty() {
        bot.send_message(
            msg.chat.id,
            "Укажите адрес кошелька для отслеживания. Пример: /add_whale 0x...",
        )
        .await?;
        return Ok(());
    }

    if !address.starts_with("0x") || address.len() != 42 {
        let error_message = format!(
            "Некорректный формат адреса: {}. Адрес должен начинаться с '0x' и быть длиной 42 символа.",
            address
        );
        bot.send_message(msg.chat.id, error_message).await?;
        return Ok(());
    }

    // Блокируем мьютекс и формируем сообщение без await
    let message = {
        let mut whales = state.tracked_whales.lock().await;
        if whales.contains(&address) {
            format!("Адрес {} уже отслеживается.", address)
        } else {
            whales.insert(address.clone());
            format!("Адрес {} успешно добавлен для отслеживания", address)
        }
    }; // здесь MutexGuard выходит из области, можно безопасно await

    // Теперь отправляем сообщение
    bot.send_message(msg.chat.id, message).await?;
    Ok(())
}



