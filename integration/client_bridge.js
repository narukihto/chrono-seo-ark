/**
 * Ark Systems: Client Bridge (The Neural Link)
 * Version: 1.0.0
 * Purpose: Connects sovereign websites to the Chrono-SEO Truth-Vault.
 */

const ARK_CONFIG = {
    // The public URL to your raw truth-vault.json on GitHub
    VAULT_URL: "https://raw.githubusercontent.com/narukihto/chrono-seo/main/vault/truth-vault.json",
    REFRESH_INTERVAL: 360000, // 6 minutes in ms
    STABILITY_THRESHOLD: 0.15, // Only show high-quality keywords
    TARGET_ID: "ark-seo-pulse" // The DOM element where keywords are injected
};

class ArkBridge {
    constructor() {
        this.vaultData = null;
        this.init();
    }

    async init() {
        console.log("🛡️ [ARK] Bridge Initializing...");
        await this.pulse();
        // Set heartbeat to sync with the 6-minute GitHub Pulse
        setInterval(() => this.pulse(), ARK_CONFIG.REFRESH_INTERVAL);
    }

    async pulse() {
        try {
            const response = await fetch(ARK_CONFIG.VAULT_URL, { cache: "no-store" });
            if (!response.ok) throw new Error("Vault Connection Interrupted");
            
            this.vaultData = await response.json();
            this.inject();
            
            console.log(`⚡ [ARK] Pulse Synchronized. Timestamp: ${this.vaultData.pulse_timestamp}`);
        } catch (err) {
            console.error("⚠️ [ARK] Pulse Failed:", err.message);
        }
    }

    inject() {
        const container = document.getElementById(ARK_CONFIG.TARGET_ID);
        if (!container || !this.vaultData) return;

        // Filter: Protocol 9 (Client-Side)
        // We only show keywords that pass our internal stability threshold
        const stableSignals = this.vaultData.data.filter(signal => 
            signal.stability_score <= ARK_CONFIG.STABILITY_THRESHOLD
        );

        // Map signals to metadata or visual tags
        container.innerHTML = stableSignals.map(signal => `
            <span class="ark-tag" 
                  data-momentum="${signal.momentum}" 
                  title="Stability: ${signal.stability_score}">
                #${signal.keyword.replace(/\s+/g, '')}
            </span>
        `).join(' ');

        // Update meta tags for SEO bots
        this.updateMeta(stableSignals);
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

// Activate the Bridge once the DOM is ready
document.addEventListener('DOMContentLoaded', () => {
    window.arkBridge = new ArkBridge();
});
