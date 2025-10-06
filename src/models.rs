use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct WalletState {
    pub balances: HashMap<String, f64>, // "eth" => 0.27, "bnb" => 2.10, ...
    pub addresses: HashMap<String, String>, // "eth" => "0x...", "sol" => "F3B8..."
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
    pub network: String, // "eth", "bnb", "matic", "sol", "arb", "op", "avax", "base", "sui", "apt"
}

#[derive(Clone)]
pub struct AppState {
    pub wallet: Arc<Mutex<WalletState>>,
    pub bot_token: String,
    pub chat_id: String,
}