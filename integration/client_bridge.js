/**
 * Ark Systems: Client Bridge (The Neural Link)
 * Version: 1.1.0-PURIFIED
 * Purpose: Connects sovereign websites to the Chrono-SEO Truth-Vault.
 * Optimized for full matrix visibility and international character support.
 */

const ARK_CONFIG = {
    // The public URL to your raw truth-vault.json on GitHub
    VAULT_URL: "https://raw.githubusercontent.com/narukihto/chrono-seo/main/vault/truth-vault.json",
    REFRESH_INTERVAL: 360000, // 6 minutes in ms
    // Dynamic Threshold: Now we show ALL but highlight the elite
    ELITE_MOMENTUM_STAMP: 90.0, 
    TARGET_ID: "ark-seo-pulse" // The DOM element where keywords are injected
};

class ArkBridge {
    constructor() {
        this.vaultData = null;
        this.init();
    }

    async init() {
        console.log("🛡️ [ARK] Neural Link Initializing... Protocol: 1.1.0-PURIFIED");
        await this.pulse();
        // Set heartbeat to sync with the 6-minute GitHub Pulse
        setInterval(() => this.pulse(), ARK_CONFIG.REFRESH_INTERVAL);
    }

    async pulse() {
        try {
            // Using cache: "no-store" to ensure we get the latest 'Temporal Projectile' deployment
            const response = await fetch(ARK_CONFIG.VAULT_URL, { cache: "no-store" });
            if (!response.ok) throw new Error("Vault Connection Interrupted");
            
            this.vaultData = await response.json();

            // Verify Protocol Integrity
            if (this.vaultData.protocol_version !== "1.1.0-PURIFIED") {
                console.warn(`⚠️ [ARK] Legacy Data Detected: ${this.vaultData.protocol_version}`);
            }

            this.inject();
            console.log(`⚡ [ARK] Pulse Synchronized. Photons Captured: ${this.vaultData.signals_count}`);
        } catch (err) {
            console.error("⚠️ [ARK] Pulse Failed:", err.message);
        }
    }

    inject() {
        const container = document.getElementById(ARK_CONFIG.TARGET_ID);
        if (!container || !this.vaultData || !this.vaultData.data) return;

        // NEW LOGIC: Full Visibility. We display all signals but sort by momentum.
        const allSignals = this.vaultData.data;

        // Map signals to visual tags with dynamic priority scaling
        container.innerHTML = allSignals.map(signal => {
            const isElite = signal.momentum >= ARK_CONFIG.ELITE_MOMENTUM_STAMP;
            const priorityClass = isElite ? 'ark-tag-elite' : 'ark-tag-stable';
            
            // Clean keyword for hash-tag display while preserving original for SEO
            const displayHash = signal.keyword.replace(/\s+/g, '');

            return `
                <span class="ark-tag ${priorityClass}" 
                      data-momentum="${signal.momentum}" 
                      style="opacity: ${Math.max(signal.momentum / 100, 0.5)}"
                      title="Sector Precision: ${signal.momentum}%">
                    #${displayHash}
                </span>
            `;
        }).join(' ');

        // Update meta tags for SEO bots with the full purified spectrum
        this.updateMeta(allSignals);
    }

    updateMeta(signals) {
        // We inject the top-performing keywords into the site's head for crawling
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
