# integration/webhook_handler.py

import os
import json
import requests
from flask import Flask, request, jsonify

/**
 * ARK-SYSTEMS: Webhook Notification Handler
 * Version: 1.1.0-PURIFIED
 * Component: Neural Observer / Alert Gateway
 * Technical Purpose: 
 * 1. Consumes post-deployment telemetry from GitHub Action pulses.
 * 2. Filters high-velocity signals based on Momentum Influence Tiers.
 * 3. Dispatches formatted cryptographic status reports to Discord.
 */

app = Flask(__name__)

# --- Configuration Environment ---
# Ensure ARK_DISCORD_WEBHOOK is set in your deployment secrets
DISCORD_WEBHOOK_URL = os.getenv("ARK_DISCORD_WEBHOOK")

# Alert Threshold: Only trigger notifications for Elite Photons (Momentum >= 90%)
MOMENTUM_ALERT_THRESHOLD = 90.0  

@app.route('/ark-pulse-notify', methods=['POST'])
def handle_pulse_notification():
    """
    Inbound gateway for pulse telemetry.
    Decouples Truth-Vault updates from administrative alerting.
    """
    try:
        data = request.json
        pulse_ts = data.get("pulse_timestamp", "Unknown Time")
        signals = data.get("data", [])
        version = data.get("protocol_version", "v1.1.0-PURIFIED")
        
        # Heuristic Analysis: Identifying High-Momentum 'Elite' Photons
        elite_signals = [s for s in signals if s.get('momentum', 0) >= MOMENTUM_ALERT_THRESHOLD]
        
        # Dispatch alert if elite photons are detected in the current stream
        if elite_signals and DISCORD_WEBHOOK_URL:
            send_discord_alert(pulse_ts, elite_signals, version)
            
        return jsonify({
            "status": "Pulse Logged", 
            "version": version,
            "elite_captured": len(elite_signals),
            "state": "SYNCHRONIZED"
        }), 200
    
    except Exception as e:
        print(f"⚠️ [ARK-SYSTEMS] Webhook Execution Error: {e}")
        return jsonify({"status": "Error", "message": str(e)}), 500

def send_discord_alert(timestamp, signals, version):
    """
    Constructs and fires a high-integrity Discord embed report.
    Optimized for the Architect's mobile/desktop command center.
    """
    # Formatting high-influence signals with momentum precision
    keywords_list = "\n".join([
        f"🔥 **{s['keyword']}** (Influence: {s['momentum']:.2f}%)" 
        for s in signals
    ])
    
    payload = {
        "username": "Ark Systems Observer",
        "avatar_url": "https://raw.githubusercontent.com/narukihto/chrono-seo/main/interface/assets/logo.png",
        "embeds": [{
            "title": "🏛️ Sovereign Pulse Synchronized",
            "description": (
                f"**Vault Architecture:** `{version}`\n"
                f"The Truth-Vault has been successfully updated with purified signals.\n\n"
                f"{keywords_list}"
            ),
            "color": 0x00FFCC,  // Matrix Neon Signature
            "fields": [
                {"name": "Detection Result", "value": f"{len(signals)} Elite Photons", "inline": True},
                {"name": "System Integrity", "value": "🟢 PURIFIED", "inline": True}
            ],
            "footer": {
                "text": f"Pulse Time: {timestamp} | Protocol 19: Temporal Projectile Active"
            }
        }]
    }
    
    # Executing the HTTP POST request to the Discord API Gateway
    requests.post(DISCORD_WEBHOOK_URL, json=payload)

if __name__ == "__main__":
    # Lightweight production-ready listener entry point
    print("🚀 [ARK-SYSTEMS] Observer Node Active. Monitoring Signal Matrix...")
    app.run(host='0.0.0.0', port=int(os.environ.get("PORT", 5000)))
