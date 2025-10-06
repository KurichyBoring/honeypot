use axum::{extract::State, response::Html};
use crate::models::AppState;

pub async fn index(State(state): State<AppState>) -> Html<String> {
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