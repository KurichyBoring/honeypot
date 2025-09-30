use axum::{
    extract::{ConnectInfo, Form, State},
    http::HeaderMap,
    response::{Html, Redirect},
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::{
    env,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tracing::info;

#[derive(Debug, Clone)]
struct WalletState {
    balance: f64,
    address: String,
}

#[derive(Deserialize, Debug)]
struct LoginForm {
    username: String,
    password: String,
}

#[derive(Deserialize, Debug)]
struct SendForm {
    to: String,
    amount: String,
}

#[derive(Clone)]
struct AppState {
    wallet: Arc<Mutex<WalletState>>,
    bot_token: String,
    chat_id: String,
}

async fn index(State(state): State<AppState>) -> Html<String> {
    let wallet = state.wallet.lock().unwrap();
    let balance = wallet.balance;
    let address = &wallet.address;

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>SecureCrypto</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{
            background-color: #0f0c1a;
            color: white;
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            min-height: 100vh;
            display: flex;
            flex-direction: column;
        }}
        .header {{
            width: 100%;
            background-color: #120e21;
            padding: 10px 20px;
            display: flex;
            justify-content: space-between;
            align-items: center;
            border-bottom: 1px solid #2d2a3b;
        }}
        .logo {{ color: #00ff9d; font-weight: bold; font-size: 1.2rem; }}
        .admin-link {{
            color: #ffffff;
            text-decoration: none;
            font-size: 0.9rem;
            padding: 5px 10px;
            border-radius: 4px;
            transition: all 0.2s;
        }}
        .admin-link:hover {{
            color: #00ff9d;
            background-color: #1e1a2e;
        }}
        .main-content {{
            flex: 1;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            padding: 20px;
        }}
        h1 {{ font-size: 2rem; margin-bottom: 10px; }}
        .balance {{ font-size: 3rem; color: #00ff9d; font-weight: bold; margin: 20px 0; }}
        .address {{
            background-color: #1e1a2e;
            padding: 10px;
            border-radius: 5px;
            font-size: 0.8rem;
            word-break: break-all;
            max-width: 80%;
            margin: 0 auto 20px;
        }}
        .buttons {{
            display: flex;
            gap: 10px;
            justify-content: center;
        }}
        .btn {{
            padding: 10px 20px;
            border: none;
            border-radius: 5px;
            cursor: pointer;
            font-weight: bold;
            transition: all 0.2s;
        }}
        .send-btn {{ background-color: #ff5a8c; color: white; }}
        .receive-btn {{ background-color: #4a90e2; color: white; }}
        .btn:hover {{ opacity: 0.9; }}

        .modal {{
            position: fixed;
            top: 0; left: 0;
            width: 100%; height: 100%;
            background: rgba(0,0,0,0.8);
            display: none;
            align-items: center;
            justify-content: center;
            z-index: 1000;
        }}
        .modal-content {{
            background: #120e21;
            padding: 20px;
            border-radius: 8px;
            width: 90%;
            max-width: 450px;
            color: white;
        }}
        .modal h3 {{
            margin-bottom: 15px;
            color: #00ff9d;
        }}
        .form-group {{
            margin-bottom: 15px;
            text-align: left;
        }}
        label {{
            display: block;
            margin-bottom: 5px;
            font-size: 0.9rem;
        }}
        input {{
            width: 100%;
            padding: 10px;
            border: 1px solid #2d2a3b;
            border-radius: 4px;
            background: #1e1a2e;
            color: white;
        }}
        button.submit {{
            width: 100%;
            padding: 12px;
            margin-top: 10px;
            border: none;
            border-radius: 4px;
            cursor: pointer;
            font-weight: bold;
            background: #00ff9d;
            color: #0f0c1a;
        }}
        .close {{
            float: right;
            cursor: pointer;
            color: #ff5a8c;
            font-size: 1.5rem;
        }}

        #toast {{
            position: fixed;
            bottom: 20px;
            left: 50%;
            transform: translateX(-50%);
            background: #1e1a2e;
            color: white;
            padding: 12px 20px;
            border-radius: 5px;
            display: none;
            z-index: 2000;
        }}

        .qr-container {{
            text-align: center;
            margin: 15px 0;
        }}
        .qr-container img {{
            width: 180px;
            height: 180px;
            background: white;
            padding: 10px;
            border-radius: 8px;
        }}
        .copy-btn {{
            background: #4a90e2;
            color: white;
            border: none;
            padding: 6px 12px;
            border-radius: 4px;
            cursor: pointer;
            font-size: 0.85rem;
            margin-top: 10px;
        }}
    </style>
</head>
<body>
    <div class="header">
        <div class="logo">🔒 SecureCrypto</div>
        <a href="/admin" class="admin-link">Admin</a>
    </div>
    <div class="main-content">
        <h1>Your Wallet</h1>
        <div class="balance">{:.2} ETH</div>
        <div class="address">{}</div>
        <div class="buttons">
            <button class="btn send-btn" onclick="openSendModal()">Send</button>
            <button class="btn receive-btn" onclick="openReceiveModal()">Receive</button>
        </div>
    </div>

    <!-- Send Modal -->
    <div id="sendModal" class="modal">
        <div class="modal-content">
            <span class="close" onclick="closeModal('sendModal')">&times;</span>
            <h3>Send ETH</h3>
            <form id="sendForm">
                <div class="form-group">
                    <label>Recipient Address</label>
                    <input type="text" id="toAddress" placeholder="0x..." required>
                </div>
                <div class="form-group">
                    <label>Amount (ETH)</label>
                    <input type="number" id="sendAmount" step="0.01" min="0.01" max="{:.2}" required>
                </div>
                <button type="submit" class="submit">Confirm Transaction</button>
            </form>
        </div>
    </div>

    <!-- Receive Modal -->
    <div id="receiveModal" class="modal">
        <div class="modal-content">
            <span class="close" onclick="closeModal('receiveModal')">&times;</span>
            <h3>Receive ETH</h3>
            <p>Your wallet address:</p>
            <div class="address" id="receiveAddress">{}</div>
            <div class="qr-container">
                <img src="https://api.qrserver.com/v1/create-qr-code/?size=200x200&data={}" alt="QR Code">
            </div>
            <button class="copy-btn" onclick="copyAddress()">Copy Address</button>
        </div>
    </div>

    <div id="toast"></div>

    <script>
        const address = "{}";
        const balance = {};

        function openSendModal() {{ document.getElementById('sendModal').style.display = 'flex'; }}
        function openReceiveModal() {{ document.getElementById('receiveModal').style.display = 'flex'; }}
        function closeModal(id) {{ document.getElementById(id).style.display = 'none'; }}

        function showToast(msg) {{
            const t = document.getElementById('toast');
            t.innerText = msg;
            t.style.display = 'block';
            setTimeout(() => t.style.display = 'none', 3000);
        }}

        function copyAddress() {{
            navigator.clipboard.writeText(address).then(() => showToast("Address copied!"));
        }}

        document.getElementById('sendForm').onsubmit = async (e) => {{
            e.preventDefault();
            const to = document.getElementById('toAddress').value.trim();
            const amount = parseFloat(document.getElementById('sendAmount').value);

            if (!/^0x[a-fA-F0-9]{{40}}$/.test(to)) {{
                showToast("Invalid Ethereum address");
                return;
            }}
            if (to.toLowerCase() === address.toLowerCase()) {{
                showToast("Cannot send to yourself");
                return;
            }}
            if (isNaN(amount) || amount <= 0 || amount > balance) {{
                showToast("Invalid amount");
                return;
            }}

            try {{
                const res = await fetch('/send', {{
                    method: 'POST',
                    headers: {{ 'Content-Type': 'application/x-www-form-urlencoded' }},
                    body: "to=" + encodeURIComponent(to) + "&amount=" + encodeURIComponent(amount)
                }});
                if (res.ok) {{
                    showToast("Transaction sent!");
                    setTimeout(() => location.reload(), 1000);
                }} else {{
                    showToast("Failed to send");
                }}
            }} catch (err) {{
                showToast("Network error");
            }}
        }};
    </script>
</body>
</html>"#,
        balance, address, balance, address, address, address, balance
    );

    Html(html)
}

async fn admin_login() -> Html<String> {
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

async fn send_eth(State(state): State<AppState>, Form(form): Form<SendForm>) -> Redirect {
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

async fn handle_login(
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

async fn send_to_telegram(
    text: &str,
    bot_token: &str,
    chat_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
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

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let bot_token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set in .env");
    let chat_id = env::var("TELEGRAM_CHAT_ID").expect("TELEGRAM_CHAT_ID must be set in .env");

    tracing_subscriber::fmt::init();

    let app_state = AppState {
        wallet: Arc::new(Mutex::new(WalletState {
            balance: 4.27,
            address: "0x742d35Cc6634C0532925a3b844Bc454e4438f412".to_string(),
        })),
        bot_token,
        chat_id,
    };

    let app = Router::new()
        .route("/", get(index))
        .route("/send", post(send_eth))
        .route("/admin", get(admin_login).post(handle_login))
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Запущен: http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
