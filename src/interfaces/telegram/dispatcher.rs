// src/interfaces/telegram/dispatcher.rs рабоч

use teloxide::{prelude::*, utils::command::BotCommands};
use log::info;
use std::sync::Arc;
use std::collections::{HashSet, HashMap};
use tokio::sync::Mutex;
use super::commands::Command;
use super::handlers;


#[derive(Clone)]
pub struct AppState {
    pub tracked_whales: Arc<Mutex<HashSet<String>>>,
    pub thresholds: Arc<Mutex<HashMap<String, f64>>>,
}

pub async fn run_bot(bot: Bot) {
    info!("Запускаем диспетчер тг бота");

    let app_state = AppState {
        tracked_whales: Arc::new(Mutex::new(HashSet::new())),
        thresholds: Arc::new(Mutex::new(HashMap::new())),
    };

    Dispatcher::builder(
        bot.clone(),
        Update::filter_message()
            .branch(
                dptree::entry()
                    .filter_command::<Command>()
                    .branch(dptree::case![Command::Start].endpoint(handlers::start_handler))
                    .branch(dptree::case![Command::Price(symbol)].endpoint(handlers::price_handler))
                    .branch(dptree::case![Command::Info(symbol)].endpoint(handlers::info_handler))
                    .branch(
    dptree::case![Command::AddWhale(address)]
        .map(|msg: Message, bot: Bot, state: AppState, address: String| (bot, msg, state, address))
        .endpoint(|bot, msg, state, address| async move {
            handlers::add_whale_handler(bot, msg, state, address).await
        })
)

            )
            .branch(
                dptree::endpoint(|bot: Bot, msg: Message| async move {
                    bot.send_message(
                        msg.chat.id,
                        "Извините, я не понял вашу команду. Используйте /start для списка команд.",
                    )
                    .await?;
                    Ok(())
                })
            ),
    )
    .dependencies(dptree::deps![app_state])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

