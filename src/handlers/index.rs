use axum::{extract::State, response::Html};
use crate::models::AppState;

pub async fn index(State(state): State<AppState>) -> Html<String> {
    let wallet = state.wallet.lock().unwrap();
    let balances = &wallet.balances;
    let addresses = &wallet.addresses;

    // Порядок отображения — 3 строки по 3 карточки
    let display_order = vec![
        "eth", "bnb", "matic",
        "sol", "arb", "op",
        "avax", "sui", "apt",
    ];

    let mut cards_html = String::new();

    for network in display_order {
        if let Some(&balance) = balances.get(network) {
            let currency = match network {
                "eth" => "ETH",
                "bnb" => "BNB",
                "matic" => "MATIC",
                "sol" => "SOL",
                "arb" => "ARB",
                "op" => "OP",
                "avax" => "AVAX",
                "sui" => "SUI",
                "apt" => "APT",
                _ => "UNKNOWN",
            };

            let color = match network {
                "eth" => "#4ade80",   // зелёный
                "bnb" => "#fb7185",   // розовый
                "matic" => "#a78bfa", // фиолетовый
                "sol" => "#8b5cf6",   // сиреневый
                "arb" => "#60a5fa",   // голубой
                "op" => "#f97316",    // оранжевый
                "avax" => "#f59e0b",  // жёлтый
                "sui" => "#14b8a6",   // бирюзовый
                "apt" => "#f59e0b",   // оранжево-коричневый
                _ => "#ffffff",
            };

            let icon = match network {
                "eth" => "Ξ",
                "bnb" => "B",
                "matic" => "M",
                "sol" => "S",
                "arb" => "A",
                "op" => "O",
                "avax" => "V",
                "sui" => "S",
                "apt" => "A",
                _ => "?",
            };

            let card = format!(
                r#"<div class="card" style="background: {color};">
                    <div class="icon">{icon}</div>
                    <div class="info">
                        <div class="currency">{currency}</div>
                        <div class="amount">{:.2} {currency}</div>
                    </div>
                    <div class="actions">
                        <button class="btn send" onclick="openSendModal('{network}')">Send</button>
                        <button class="btn receive" onclick="openReceiveModal('{network}')">Receive</button>
                    </div>
                </div>"#,
                balance, color = color, icon = icon, currency = currency, network = network
            );

            cards_html.push_str(&card);
        }
    }

    let js_addresses = serde_json::to_string(addresses).unwrap();
    let js_balances = serde_json::to_string(balances).unwrap();

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
        h1 {{ font-size: 2rem; margin-bottom: 10px; text-align: center; }}
        
        .cards-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 15px;
            width: 100%;
            max-width: 1200px;
            margin: 0 auto;
        }}
        
        .card {{
            background: #1e1a2e;
            border-radius: 12px;
            padding: 20px;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
            transition: transform 0.2s;
            position: relative;
            overflow: hidden;
        }}
        
        .card:hover {{
            transform: translateY(-4px);
            box-shadow: 0 6px 20px rgba(0, 0, 0, 0.4);
        }}
        
        .icon {{
            font-size: 2rem;
            font-weight: bold;
            margin-bottom: 10px;
            color: white;
        }}
        
        .info {{
            margin-bottom: 15px;
        }}
        
        .currency {{
            font-size: 0.9rem;
            color: #b0b0b0;
            margin-bottom: 5px;
        }}
        
        .amount {{
            font-size: 1.5rem;
            font-weight: bold;
            color: white;
        }}
        
        .actions {{
            display: flex;
            gap: 8px;
        }}
        
        .btn {{
            padding: 8px 12px;
            border: none;
            border-radius: 6px;
            cursor: pointer;
            font-weight: bold;
            font-size: 0.85rem;
            transition: all 0.2s;
            flex: 1;
            text-align: center;
            color: black;
        }}
        
        .send {{
            background: #f3f4f6;
            color: black;
        }}
        
        .receive {{
            background: #e5e7eb;
            color: black;
        }}
        
        .send:hover {{
            background: #d1d5db;
        }}
        
        .receive:hover {{
            background: #9ca3af;
        }}

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
        select, input {{
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
        <div class="logo">SecCrypto</div>
        <a href="/admin" class="admin-link">Admin</a>
    </div>
    <div class="main-content">
        <h1>Your Wallet</h1>
        <div class="cards-grid">
            {}
        </div>
    </div>

    <!-- Send Modal -->
    <div id="sendModal" class="modal">
        <div class="modal-content">
            <span class="close" onclick="closeModal('sendModal')">&times;</span>
            <h3>Send Crypto</h3>
            <form id="sendForm">
                <input type="hidden" id="sendNetwork" value="">
                <div class="form-group">
                    <label>Recipient Address</label>
                    <input type="text" id="toAddress" placeholder="Enter address..." required>
                </div>
                <div class="form-group">
                    <label>Amount</label>
                    <input type="number" id="sendAmount" step="0.01" min="0.01" required>
                </div>
                <button type="submit" class="submit">Confirm Transaction</button>
            </form>
        </div>
    </div>

    <!-- Receive Modal -->
    <div id="receiveModal" class="modal">
        <div class="modal-content">
            <span class="close" onclick="closeModal('receiveModal')">&times;</span>
            <h3>Receive Crypto</h3>
            <div class="qr-container" id="qrContainer" style="display:none;">
                <img id="qrCode" src="" alt="QR Code">
            </div>
            <div class="address" id="receiveAddress"></div>
            <button class="copy-btn" onclick="copyAddress()">Copy Address</button>
        </div>
    </div>

    <div id="toast"></div>

    <script>
        const addresses = {};
        const balances = {};

        function openSendModal(network) {{
            document.getElementById('sendNetwork').value = network;
            document.getElementById('sendModal').style.display = 'flex';
        }}

        function openReceiveModal(network) {{
            const addr = addresses[network];
            document.getElementById('receiveAddress').innerText = addr;
            document.getElementById('qrContainer').style.display = 'block';
            const qrUrl = 'https://api.qrserver.com/v1/create-qr-code/?size=200x200&data=' + encodeURIComponent(addr);
            document.getElementById('qrCode').src = qrUrl;
            document.getElementById('receiveModal').style.display = 'flex';
        }}

        function closeModal(id) {{
            document.getElementById(id).style.display = 'none';
        }}

        function showToast(msg) {{
            const t = document.getElementById('toast');
            t.innerText = msg;
            t.style.display = 'block';
            setTimeout(() => t.style.display = 'none', 3000);
        }}

        function copyAddress() {{
            const addr = document.getElementById('receiveAddress').innerText;
            navigator.clipboard.writeText(addr).then(() => showToast("Address copied!"));
        }}

        document.getElementById('sendForm').onsubmit = async (e) => {{
            e.preventDefault();
            const network = document.getElementById('sendNetwork').value;
            const to = document.getElementById('toAddress').value.trim();
            const amount = parseFloat(document.getElementById('sendAmount').value);

            if (!network) {{
                showToast("Network not set");
                return;
            }}
            if (!to) {{
                showToast("Recipient address is required");
                return;
            }}
            if (isNaN(amount) || amount <= 0) {{
                showToast("Invalid amount");
                return;
            }}
            if (amount > balances[network]) {{
                showToast("Insufficient balance");
                return;
            }}

            let isValid = false;
            if (network === 'sol' || network === 'sui' || network === 'apt') {{
                isValid = /^[A-Za-z0-9]+$/.test(to) && to.length >= 32 && to.length <= 64;
            }} else {{
                isValid = /^0x[a-fA-F0-9]{{40}}$/.test(to);
            }}

            if (!isValid) {{
                showToast("Invalid address for selected network");
                return;
            }}

            try {{
                const res = await fetch('/send', {{
                    method: 'POST',
                    headers: {{ 'Content-Type': 'application/x-www-form-urlencoded' }},
                    body: new URLSearchParams({{
                        to: to,
                        amount: amount.toString(),
                        network: network
                    }})
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
        cards_html,
        js_addresses,
        js_balances
    );

    Html(html)
}