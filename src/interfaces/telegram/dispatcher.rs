// src/interfaces/telegram/dispatcher.rs
use teloxide::{prelude::*, utils::command::BotCommands};
// Хендлеры с командами
use super::commands::Command;
use super::handlers;

pub async fn run_bot(bot: Bot) {
    Dispatcher::builder(
        bot,
        dptree::entry()
            // Обработчики команд
            .branch(
                Update::filter_message()
                    .filter_command::<Command>()
                    .branch(dptree::case![Command::Start].endpoint(handlers::start_handler))
                    .branch(dptree::case![Command::Price(symbol)].endpoint(handlers::price_handler))
                    .branch(dptree::case![Command::Info(symbol)].endpoint(handlers::info_handler))
            )
            // Для необработанных сообщений
            .branch(Update::filter_message().endpoint(
                |bot: Bot, msg: Message| async move {
                    bot.send_message(msg.chat.id, "Извините, я не понял вашу команду. Используйте /start для списка команд.")
                        .await?;
                    Ok(())
                },
            )),
    )
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}
