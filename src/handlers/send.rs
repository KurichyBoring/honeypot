use axum::{extract::{State, Form}, response::Redirect};
use crate::models::{AppState, SendForm};
use tracing::info;

pub async fn send_eth(State(state): State<AppState>, Form(form): Form<SendForm>) -> Redirect {
    if let Ok(amount) = form.amount.parse::<f64>() {
        let mut wallet = state.wallet.lock().unwrap();
        if amount > 0.0 && amount <= wallet.balance {
            if form.to.len() == 42 && form.to.starts_with("0x") {
                wallet.balance -= amount;
                info!("Transaction: {} ETH sent to {}", amount, form.to);
            }
        }
    }
    Redirect::to("/")
}