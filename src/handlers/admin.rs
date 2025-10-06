use axum::{
    extract::{ConnectInfo, Form, State},
    http::HeaderMap,
    response::{Html, Redirect},
};
use crate::models::{AppState, LoginForm};
use crate::telegram::send_to_telegram;
use tracing::info;
use std::net::SocketAddr;

pub async fn admin_login() -> Html<String> {
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Adm Panel</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            background-color: #0a0810;
            color: #e0e0e0;
            font-family: 'Courier New', monospace;
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            padding: 20px;
        }
        .login-container {
            background-color: #120e1a;
            padding: 30px;
            border-radius: 8px;
            width: 100%;
            max-width: 400px;
            text-align: center;
            box-shadow: 0 0 15px rgba(0, 255, 157, 0.05);
        }
        h2 {
            color: #00ff9d;
            margin-bottom: 20px;
            font-size: 1.4rem;
            font-weight: 500;
        }
        .form-group {
            margin-bottom: 15px;
            text-align: left;
        }
        label {
            display: block;
            margin-bottom: 5px;
            font-size: 0.9rem;
            color: #b0b0b0;
        }
        input[type="text"],
        input[type="password"] {
            width: 100%;
            padding: 10px;
            border: 1px solid #2d2a3b;
            border-radius: 4px;
            background-color: #1a1625;
            color: #ffffff;
            font-size: 1rem;
            font-family: inherit;
        }
        input[type="text"]:focus,
        input[type="password"]:focus {
            border-color: #00ff9d;
            outline: none;
            box-shadow: 0 0 5px rgba(0, 255, 157, 0.2);
        }
        button {
            width: 100%;
            padding: 12px;
            background-color: #ff5a8c;
            color: white;
            border: none;
            border-radius: 4px;
            cursor: pointer;
            font-weight: bold;
            font-size: 1rem;
            font-family: inherit;
            transition: background-color 0.2s;
            margin-top: 20px;
        }
        button:hover {
            background-color: #ff457a;
        }
        .footer {
            margin-top: 20px;
            font-size: 0.8rem;
            color: #666;
        }
    </style>
</head>
<body>
    <div class="login-container">
        <h2>Admin Panel — Login</h2>
        <form method="post">
            <div class="form-group">
                <label for="username">Username:</label>
                <input type="text" id="username" name="username" required>
            </div>
            <div class="form-group">
                <label for="password">Password:</label>
                <input type="password" id="password" name="password" required>
            </div>
            <button type="submit">Login</button>
        </form>
        <div class="footer">
            © 2025 Secure
        </div>
    </div>
</body>
</html>"#
        .to_string();
    Html(html)
}

pub async fn handle_login(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Form(form): Form<LoginForm>,
) -> Redirect {
    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("Unknown");

    let message = format!(
        "🚨 Honeypot Triggered!\n\
        IP: {}\n\
        User-Agent: {}\n\
        Username: {}\n\
        Password: {}\n\
        Time: {}",
        addr,
        user_agent,
        form.username,
        form.password,
        chrono::Utc::now().to_rfc3339()
    );

    info!("Login attempt: {}", message);
    let _ = send_to_telegram(&message, &state.bot_token, &state.chat_id).await;
    Redirect::to("/admin")
}