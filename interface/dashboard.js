/**
 * ARK-SYSTEMS: Neural Dashboard Controller
 * Version: 1.1.0-PURIFIED (Final Patch)
 * Component: UI State Manager & Data Binder
 */

class ArkDashboard {
    constructor() {
        this.vaultData = [];
        this.init();
    }

    async init() {
        console.log("🖥️ [ARK-UI] Establishing Neural Link...");
        await this.synchronize();
        
        // Pulse sync every 30s to stay aligned with high-velocity signals
        setInterval(() => this.synchronize(), 30000);
        this.setupKeybindings();
    }

    async synchronize() {
        try {
            const response = await fetch('./truth-vault.json');
            if (!response.ok) throw new Error("Vault Mirror Offline");
            
            const rawData = await response.json();
            
            /** * 🔍 FIX: Normalizing Data Structure
             * The Rust engine output can be { "data": [...] } or just [...]
             */
            this.vaultData = Array.isArray(rawData) ? rawData : (rawData.data || []);
            
            console.log(`✅ [ARK-UI] Pulse Success: ${this.vaultData.length} Photons Captured.`);
            this.render();
        } catch (error) {
            console.error("⚠️ [ARK-UI] Link Failure:", error);
        }
    }

    render() {
        if (!this.vaultData || this.vaultData.length === 0) return;

        const total = this.vaultData.length;
        // Tier 1 (Market): Based on 92.0 momentum in logs
        const marketSignals = this.vaultData.filter(s => s.momentum >= 90);
        // Tier 2 (News): Based on 88.0 momentum in logs
        const newsSignals = this.vaultData.filter(s => s.momentum >= 85 && s.momentum < 90);

        this.updateCounter("count-total", total);
        this.updateCounter("count-market", marketSignals.length);
        this.updateCounter("count-news", newsSignals.length);

        this.renderGrid();
    }

    updateCounter(id, target) {
        const el = document.getElementById(id);
        if (el) el.innerText = target; // Direct update for high-speed accuracy
    }

    renderGrid() {
        const grid = document.getElementById('signal-grid');
        if (!grid) return;

        grid.innerHTML = this.vaultData.map(s => `
            <div class="ark-tag ${s.momentum >= 90 ? 'market-photon' : 'news-photon'}" data-momentum="${s.momentum}">
                <span class="photon-key">${s.keyword || s.title || 'UNKNOWN'}</span>
                <span class="photon-momentum">${s.momentum.toFixed(1)}%</span>
            </div>
        `).join('');
    }

    setupKeybindings() {
        document.addEventListener('keydown', (e) => {
            if (e.code === 'KeyR') {
                console.warn("🔄 [ARK] Manual Recalibration.");
                this.synchronize();
            }
        });
    }
}

window.addEventListener('load', () => {
    new ArkDashboard();
});
