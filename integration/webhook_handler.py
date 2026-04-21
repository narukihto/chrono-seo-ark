# integration/webhook_handler.py

import os
import json
import requests
from flask import Flask, request, jsonify

app = Flask(__name__)

# --- Configuration ---
# In a production Ark, these would be environment variables.
DISCORD_WEBHOOK_URL = os.getenv("ARK_DISCORD_WEBHOOK")
STABILITY_ALERT_THRESHOLD = 0.10  # Notify if a keyword is exceptionally stable

@app.route('/ark-pulse-notify', methods=['POST'])
def handle_pulse_notification():
    """
    Receives a webhook trigger after a successful GitHub Action pulse.
    """
    try:
        data = request.json
        pulse_ts = data.get("pulse_timestamp", "Unknown Time")
        signals = data.get("data", [])
        
        # Identify "High-Integrity" signals for the alert
        elite_signals = [s for s in signals if s['stability_score'] < STABILITY_ALERT_THRESHOLD]
        
        if elite_signals and DISCORD_WEBHOOK_URL:
            send_discord_alert(pulse_ts, elite_signals)
            
        return jsonify({"status": "Pulse Logged", "elite_captured": len(elite_signals)}), 200
    
    except Exception as e:
        print(f"⚠️ [ARK] Webhook Error: {e}")
        return jsonify({"status": "Error", "message": str(e)}), 500

def send_discord_alert(timestamp, signals):
    """
    Fires a formatted notification to the Architect's command center.
    """
    keywords_list = "\n".join([f"💎 **{s['keyword']}** (Score: {s['stability_score']})" for s in signals])
    
    payload = {
        "username": "Ark Systems Observer",
        "embeds": [{
            "title": "⚡ Chrono-SEO Pulse Synchronized",
            "description": f"The Truth-Vault has been updated with high-integrity signals.\n\n{keywords_list}",
            "color": 0x00ffcc,
            "footer": {"text": f"Pulse Time: {timestamp} | Protocol 19 Active"}
        }]
    }
    
    requests.post(DISCORD_WEBHOOK_URL, json=payload)

if __name__ == "__main__":
    # The handler runs on a lightweight listener (e.g., a Railway or Heroku instance)
    app.run(host='0.0.0.0', port=int(os.environ.get("PORT", 5000)))
