use reqwest;
use serde_json;

pub async fn send_to_telegram(
    text: &str,
    bot_token: &str,
    chat_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    // ⚠️ Исправлен URL: убраны лишние пробелы!
    let url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);
    let params = serde_json::json!({
        "chat_id": chat_id,
        "text": text,
        "parse_mode": "HTML"
    });

    let res = client.post(&url).json(&params).send().await?;
    if !res.status().is_success() {
        let body = res.text().await?;
        return Err(format!("Telegram API error: {}", body).into());
    }
    Ok(())
}