// src/main.rs
mod config;
mod infrastructure;
mod interfaces;
extern crate dotenv;
use teloxide::types::ChatId;
use teloxide::prelude::*;
use actix_web::{web, App, HttpServer, Responder, HttpResponse, post};
use log::{info, error};
use pretty_env_logger;
use std::env; 
use std::sync::Arc;
use std::str::FromStr;
use futures::TryFutureExt; 

#[post("/webhook")]
async fn webhook_receiver(
    info: web::Json<serde_json::Value>,
    bot: web::Data<Arc<Bot>>,
    alert_chat_id: web::Data<ChatId>, // Теперь это просто ChatId, не Arc<ChatId>
) -> impl Responder {
    info!("Получен вебхук! Содержимое: {:?}", info);
    
    // Отправляем сообщение в Telegram
    if let Err(e) = bot.send_message(**alert_chat_id, "Новый вебхук получен!").await {
        error!("Не удалось отправить сообщение в Telegram: {}", e);
    }
    
    HttpResponse::Ok().body("Вебхук успешно получен!")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    info!("Переменные окружения загружены");
    
    // Работа с переменными окружения
    let teloxide_token = env::var("TELOXIDE_TOKEN")
        .expect("Переменная окружения TELOXIDE_TOKEN должна быть установлена.");
    
    let chat_id_str = env::var("CHAT_ID_FOR_ALERTS")
        .expect("Переменная окружения CHAT_ID_FOR_ALERTS должна быть установлена.");
    
    let chat_id_val: i64 = chat_id_str.parse()
        .expect("Не удалось распарсить CHAT_ID_FOR_ALERTS в число. Убедитесь, что это корректный ID.");
    
    let chat_id = ChatId(chat_id_val);
    
    info!("Запускаем CryptoForge Telegram Bot и Webhook Server...");
    
    // Инициализация Teloxide бота
    let teloxide_bot = Bot::new(teloxide_token);
    let teloxide_bot_arc = Arc::new(teloxide_bot.clone());
    
    // Запуск диспетчера бота
    let dispatcher_handle = tokio::spawn(interfaces::telegram::dispatcher::run_bot(teloxide_bot.clone()));
    
    // Actix сервер
    let server_handle = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&teloxide_bot_arc)))
            .app_data(web::Data::new(chat_id)) // Передаем ChatId напрямую в web::Data
            .service(webhook_receiver)
    })
    .bind("0.0.0.0:8080")?
    .run();
    
    info!("Webhook server running on http://0.0.0.0:8080");
    
    // Ожидание завершения работы обоих сервисов
    tokio::try_join!(
        dispatcher_handle.map_err(|e| Box::new(e) as Box<dyn std::error::Error>),
        server_handle.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    )?;
    
    info!("CryptoForge Telegram Bot и Webhook Server завершили работу.");
    Ok(())
}
