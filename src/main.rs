// src/main.rs

mod config;
mod infrastructure;
mod interfaces;
extern crate dotenv;

use teloxide::types::ChatId; // для уведомлений
use teloxide::prelude::*;

use actix_web::{web, App, HttpServer, Responder, HttpResponse, post};

use log::{info, error};
use pretty_env_logger;

use std::env; 
use std::sync::Arc; // Безопасная передача Bot между потоками
use std::str::FromStr; // Парсинг строки в число
use futures::TryFutureExt; 

#[post("/webhook")]
async fn webhook_receiver(
    info: web::Json<serde_json::Value>,
    bot: web::Data<Arc<Bot>>, // Получаем Bot безопасно 
    alert_chat_id: web::Data<ChatId>, //Получаем id для уведомлений
) -> impl Responder {
    info!("Получен вебхук! Содержимое: {:?}", info);

    // Пример отправки
    if let Err(e) = bot.send_message(**alert_chat_id, "Новый вебхук").await{
        error!("Не отправилось в Телеграмм: {}", e)
    }

    HttpResponse::Ok().body("Вебхук успешно получен!")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> { // Так как у нас Динамическая диспетчеризацияна нужен Бокс
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    info!("Переменое окружени загруженно");

    // Работа с переменным окружением
    let teloxide_token = env::var("TELOXIDE_TOKEN")
        .expect("Переменная окружения TELOXIDE_TOKEN должна быть установлена.");

    let chat_id_str = env::var("CHAT_ID_FOR_ALERTS")
        .expect("Не удалось распарсить CHAT_ID_FOR_ALERTS в число. Убедитесь, что это корректный ID.");

// src/main.rs:49
    let chat_id_val: i64 = chat_id_str.parse()
        .expect("Не удалось распарсить CHAT_ID_FOR_ALERTS в число. Убедитесь, что это корректный ID.");
    let chat_id = ChatId(chat_id_val); // Создаем ChatId из распарсенного i64
    //
    // Arc ссылки
    let alert_chat_id_arc = Arc::new(chat_id);

    
    info!("Запускаем CryptoForge Telegram Bot и Webhook Server...");

    // Инициализация Teloxide бота
    let teloxide_bot = Bot::new(teloxide_token);
    let teloxide_bot_arc = Arc::new(teloxide_bot.clone());

    let dispatcher_handle = tokio::spawn(interfaces::telegram::dispatcher::run_bot(teloxide_bot.clone()));

    // Actix сервер
    let server_handle = HttpServer::new(move || { // Захватываем наши ссылки arc
        App::new()
            .app_data(web::Data::new(Arc::clone(&teloxide_bot_arc)))
            .app_data(web::Data::new(Arc::clone(&alert_chat_id_arc)))
            .service(webhook_receiver) // Пепедаем наш хук 
    })
    .bind("0.0.0.0:8080")?
    .run();

    info!("Webhook server running on http://0.0.0.0:8080");

// 6. Ожидание завершения работы обоих сервисов
    tokio::try_join!(
        dispatcher_handle.map_err(|e| Box::new(e) as Box<dyn std::error::Error>), // Преобразуем ошибку в Box<dyn Error>
        server_handle.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)    // Преобразуем ошибку в Box<dyn Error>
)?; // <--- Не забудьте `?` в конце!

info!("CryptoForge Telegram Bot и Webhook Server завершили работу.");

Ok(())
}    
