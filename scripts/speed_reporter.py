# scripts/speed_reporter.py

import json
from datetime import datetime
import os

VAULT_PATH = "vault/truth-vault.json"
REPORT_PATH = "reports/performance_log.md"

def generate_report():
    """
    Analyzes the latest pulse and appends a performance summary to the logs.
    Includes safety checks for None types and dynamic status indicators.
    """
    if not os.path.exists(VAULT_PATH):
        print("❌ [ARK] Vault not found. Cannot generate report.")
        return

    try:
        with open(VAULT_PATH, 'r') as f:
            vault_data = json.load(f)
    except json.JSONDecodeError:
        print("❌ [ARK] Vault contains corrupted JSON.")
        return

    timestamp = vault_data.get("pulse_timestamp", "N/A")
    signals = vault_data.get("data", [])
    count = vault_data.get("signals_count", 0)
    version = vault_data.get("protocol_version", "1.0.0-ARK")

    # 🛠️ Robust Stability Calculation (Handles NoneType and Empty lists)
    # We filter out any scores that are None or missing
    valid_scores = [
        s.get('stability_score') for s in signals 
        if s.get('stability_score') is not None
    ]
    
    if valid_scores:
        avg_stability = sum(valid_scores) / len(valid_scores)
    else:
        avg_stability = 0.0

    # 🚦 Dynamic Status Logic
    # In Ark Logic, lower stability score = higher system stability
    status = "🟢 SECURE" if avg_stability < 0.95 else "🟡 WARNING"
    if count == 0: status = "⚪ IDLE"

    # Formatting the markdown report entry
    # Using code blocks for timestamp to ensure alignment in GitHub UI
    report_entry = (
        f"| `{timestamp}` | `{version}` | **{count}** | `{avg_stability:.4f}` | {status} |\n"
    )

    # Ensure reports directory exists
    os.makedirs("reports", exist_ok=True)
    
    # Initialize log file with clean headers if it doesn't exist
    if not os.path.exists(REPORT_PATH):
        with open(REPORT_PATH, 'w', encoding='utf-8') as f:
            f.write("# 🛡️ Ark Pulse Performance Logs\n")
            f.write("> **System Sovereignty:** Protocol 19 | Temporal Projection Active\n\n")
            f.write("| Timestamp | Version | Signals | Avg Stability | Status |\n")
            f.write("| :--- | :--- | :--- | :--- | :--- |\n")

    # Append the new pulse data
    with open(REPORT_PATH, 'a', encoding='utf-8') as f:
        f.write(report_entry)

    print(f"📊 [ARK] Performance report generated for pulse: {timestamp}")

if __name__ == "__main__":
    generate_report()
