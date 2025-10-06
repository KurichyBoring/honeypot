use serde::Deserialize;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct WalletState {
    pub balance: f64,
    pub address: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct SendForm {
    pub to: String,
    pub amount: String,
}

#[derive(Clone)]
pub struct AppState {
    pub wallet: Arc<Mutex<WalletState>>,
    pub bot_token: String,
    pub chat_id: String,
}