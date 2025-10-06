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

    let app_state = models::AppState {
        wallet: std::sync::Arc::new(std::sync::Mutex::new(models::WalletState {
            balance: 4.27,
            address: "0x742d35Cc6634C0532925a3b844Bc454e4438f412".to_string(),
        })),
        bot_token,
        chat_id,
    };

    let app = Router::new()
        .route("/", get(handlers::index::index))
        .route("/send", post(handlers::send::send_eth))
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