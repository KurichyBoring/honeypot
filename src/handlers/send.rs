use axum::{extract::{State, Form}, response::Redirect};
use crate::models::{AppState, SendForm};
use tracing::info;

pub async fn send_eth(State(state): State<AppState>, Form(form): Form<SendForm>) -> Redirect {
    let mut wallet = state.wallet.lock().unwrap();

    // Проверяем, есть ли такая сеть
    if !wallet.balances.contains_key(&form.network) {
        info!("Unknown network: {}", form.network);
        return Redirect::to("/");
    }

    // Парсим сумму
    if let Ok(amount) = form.amount.parse::<f64>() {
        if amount > 0.0 && amount <= wallet.balances[&form.network] {
            // Проверка адреса (упрощённая)
            let is_valid_address = match form.network.as_str() {
                "sol" => form.to.len() >= 32 && form.to.len() <= 44 && form.to.chars().all(|c| c.is_ascii_alphanumeric()),
                _ => form.to.starts_with("0x") && form.to.len() == 42,
            };

            if is_valid_address {
                wallet.balances.entry(form.network.clone()).and_modify(|b| *b -= amount);
                info!("Transaction: {} {} sent to {} via {}", amount, form.network.to_uppercase(), form.to, form.network);
            }
        }
    }

    Redirect::to("/")
}