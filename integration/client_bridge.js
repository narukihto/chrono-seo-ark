/**
 * Ark Systems: Client Bridge (The Neural Link)
 * Version: 1.1.0-PURIFIED
 * Purpose: Connects sovereign websites to the Chrono-SEO Truth-Vault.
 * Optimized for: Neural Dashboard Integration & Global Matrix Visibility.
 */

const ARK_CONFIG = {
    // The public URL to your raw truth-vault.json on GitHub
    VAULT_URL: "https://raw.githubusercontent.com/narukihto/chrono-seo/main/vault/truth-vault.json",
    
    // Interval matching the GitHub Action heartbeat
    REFRESH_INTERVAL: 300000, // 5 minutes
    
    // Momentum threshold for visual highlighting
    ELITE_MOMENTUM_STAMP: 90.0,
    
    // DOM Targets
    TARGET_ID: "ark-seo-pulse",
    STATUS_ID: "status-text",
    TIMER_ID: "last-pulse",
    COUNT_TOTAL_ID: "count-total"
};

class ArkBridge {
    constructor() {
        this.vaultData = null;
        this.init();
    }

    async init() {
        console.log("🛡️ [ARK] Neural Link Initializing... Protocol: 1.1.0-PURIFIED");
        await this.pulse();
        
        // Synchronize heartbeat
        setInterval(() => this.pulse(), ARK_CONFIG.REFRESH_INTERVAL);
    }

    async pulse() {
        try {
            const statusEl = document.getElementById(ARK_CONFIG.STATUS_ID);
            if (statusEl) statusEl.innerText = "SYNCHRONIZING...";

            // Use Cache-Busting to prevent GitHub RAW stale data
            const cacheBuster = `?t=${Date.now()}`;
            const response = await fetch(ARK_CONFIG.VAULT_URL + cacheBuster, { cache: "no-store" });
            
            if (!response.ok) throw new Error("Vault Connection Interrupted");
            
            this.vaultData = await response.json();

            // Verify Protocol Integrity
            if (this.vaultData.protocol_version !== "1.1.0-PURIFIED") {
                console.warn(`⚠️ [ARK] Protocol Mismatch: ${this.vaultData.protocol_version}`);
            }

            this.updateUIStats();
            this.inject();
            
            console.log(`⚡ [ARK] Pulse Synchronized. Photons: ${this.vaultData.signals_count}`);
        } catch (err) {
            console.error("⚠️ [ARK] Pulse Failed:", err.message);
            const statusEl = document.getElementById(ARK_CONFIG.STATUS_ID);
            if (statusEl) statusEl.innerText = "OFFLINE / ERROR";
        }
    }

    updateUIStats() {
        // Update Status & Timestamp
        const statusEl = document.getElementById(ARK_CONFIG.STATUS_ID);
        const timerEl = document.getElementById(ARK_CONFIG.TIMER_ID);
        const countEl = document.getElementById(ARK_CONFIG.COUNT_TOTAL_ID);

        if (statusEl) statusEl.innerText = "SYNCHRONIZED";
        if (timerEl) {
            const time = new Date(this.vaultData.pulse_timestamp).toLocaleTimeString();
            timerEl.innerText = `LAST PULSE: ${time}`;
        }
        if (countEl) countEl.innerText = this.vaultData.signals_count;

        // Note: Sector-specific counts (News/Market) can be added here if 
        // the vault JSON is updated to include individual sector lengths.
    }

    inject() {
        const container = document.getElementById(ARK_CONFIG.TARGET_ID);
        if (!container || !this.vaultData || !this.vaultData.data) return;

        const allSignals = this.vaultData.data;

        // Render Matrix Grid
        container.innerHTML = allSignals.map(signal => {
            const isElite = signal.momentum >= ARK_CONFIG.ELITE_MOMENTUM_STAMP;
            const priorityClass = isElite ? 'ark-tag-elite' : 'ark-tag-stable';
            
            // Generate clean hashtag
            const displayHash = signal.keyword.replace(/\s+/g, '');

            return `
                <div class="ark-tag-wrapper">
                    <span class="ark-tag ${priorityClass}" 
                          data-momentum="${signal.momentum}" 
                          style="opacity: ${Math.max(signal.momentum / 100, 0.4)}"
                          title="Momentum: ${signal.momentum}%">
                        #${displayHash}
                    </span>
                </div>
            `;
        }).join('');

        this.updateMeta(allSignals);
    }

    updateMeta(signals) {
        const keywords = signals.map(s => s.keyword).join(', ');
        let meta = document.querySelector('meta[name="keywords"]');
        if (!meta) {
            meta = document.createElement('meta');
            meta.name = "keywords";
            document.head.appendChild(meta);
        }
        meta.content = keywords;
    }
}

// Global initialization
document.addEventListener('DOMContentLoaded', () => {
    window.arkBridge = new ArkBridge();
});
