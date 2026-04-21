# scripts/speed_reporter.py

import json
from datetime import datetime
import os

VAULT_PATH = "vault/truth-vault.json"
REPORT_PATH = "reports/performance_log.md"

def generate_report():
    """
    Analyzes the latest pulse and appends a performance summary to the logs.
    """
    if not os.path.exists(VAULT_PATH):
        print("❌ [ARK] Vault not found. Cannot generate report.")
        return

    with open(VAULT_PATH, 'r') as f:
        vault_data = json.load(f)

    timestamp = vault_data.get("pulse_timestamp")
    signals = vault_data.get("data", [])
    count = vault_data.get("signals_count", 0)

    # Calculate average stability of the pulse
    if signals:
        avg_stability = sum(s['stability_score'] for s in signals) / len(signals)
    else:
        avg_stability = 0

    # Formatting the markdown report
    report_entry = (
        f"| {timestamp} | {count} | {avg_stability:.4f} | ✅ SUCCESS |\n"
    )

    # Ensure reports directory exists
    os.makedirs("reports", exist_ok=True)
    
    # Initialize log file with headers if it doesn't exist
    if not os.path.exists(REPORT_PATH):
        with open(REPORT_PATH, 'w') as f:
            f.write("# 🛡️ Ark Pulse Performance Logs\n\n")
            f.write("| Timestamp | Signals | Avg Stability | Status |\n")
            f.write("|-----------|---------|---------------|--------|\n")

    with open(REPORT_PATH, 'a') as f:
        f.write(report_entry)

    print(f"📊 [ARK] Performance report generated for pulse: {timestamp}")

if __name__ == "__main__":
    generate_report()
