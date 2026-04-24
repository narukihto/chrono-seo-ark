/**
 * ARK-SYSTEMS: Neural Dashboard Controller
 * Version: 1.1.0-PURIFIED
 * Component: UI State Manager & Data Binder
 * * Technical Changes:
 * 1. Fixed Neural Bridge path to fetch local 'truth-vault.json'.
 * 2. Optimized Tier-filtering for 46+ Photon signals.
 * 3. Enhanced numerical interpolation for real-time telemetry.
 */

class ArkDashboard {
    constructor() {
        this.vaultData = null;
        this.isMonitoring = false;
        this.init();
    }

    /**
     * Initializes the dashboard controller and establishes the telemetry loop.
     */
    async init() {
        console.log("🖥️ [ARK-UI] Dashboard Operational. Establishing Neural Link...");
        await this.synchronizeWithVault();
        this.startMonitoring();
        this.setupKeybindings();
    }

    /**
     * Critical Sync: Fetches the injected truth-vault from the local environment.
     * This fixes the 0-signal issue by locating the mirrored JSON.
     */
    async synchronizeWithVault() {
        try {
            // Path adjusted for local deployment (Mirrored during GH Action pulse)
            const response = await fetch('./truth-vault.json');
            if (!response.ok) throw new Error("Vault Mirror Not Found");
            
            this.vaultData = await response.json();
            console.log(`✅ [ARK-UI] Synchronized: ${this.vaultData.data.length} Photons Captured.`);
            this.updateSectorAnalytics();
        } catch (error) {
            console.error("⚠️ [ARK-UI] Neural Link Failure:", error);
            // Fallback: Attempt to fetch from secondary temporal bridge if local fails
        }
    }

    /**
     * Executes the monitoring heartbeat.
     */
    startMonitoring() {
        this.isMonitoring = true;
        setInterval(async () => {
            await this.synchronizeWithVault();
            this.triggerVisualFeedback();
        }, 60000); // Pulse sync every 60s to reduce DOM thrashing
    }

    /**
     * Performs sector decoupling and frequency analysis on the signal stream.
     */
    updateSectorAnalytics() {
        if (!this.vaultData || !this.vaultData.data) return;

        const signals = this.vaultData.data;
        const totalCount = signals.length;

        // Purified Filtering (Aligning with Protocol 1.1.0)
        // Tier 1 (Market): > 90% | Tier 2 (News): 85-89%
        const marketSignals = signals.filter(s => s.momentum >= 90);
        const newsSignals = signals.filter(s => s.momentum >= 85 && s.momentum < 90);

        this.applyNumericalInterpolation("count-total", totalCount);
        this.applyNumericalInterpolation("count-news", newsSignals.length);
        this.applyNumericalInterpolation("count-market", marketSignals.length);

        this.renderSignalList(signals);
    }

    /**
     * Dynamically populates the UI with signal photons.
     */
    renderSignalList(signals) {
        const container = document.getElementById('signal-grid');
        if (!container) return;

        container.innerHTML = signals.map(s => `
            <div class="ark-tag" data-momentum="${s.momentum}">
                <span class="photon-key">${s.keyword}</span>
                <span class="photon-momentum">${s.momentum.toFixed(2)}%</span>
            </div>
        `).join('');
    }

    /**
     * Implements linear interpolation for UI counter transitions.
     */
    applyNumericalInterpolation(id, targetValue) {
        const element = document.getElementById(id);
        if (!element) return;
        
        const currentValue = parseInt(element.innerText) || 0;
        if (currentValue === targetValue) return;

        let current = currentValue;
        const range = targetValue - currentValue;
        const step = range > 0 ? 1 : -1;
        
        const counterInterval = setInterval(() => {
            current += step;
            element.innerText = current;
            if (current == targetValue) clearInterval(counterInterval);
        }, 50);
    }

    /**
     * Triggers high-momentum glow shaders on elite photons.
     */
    triggerVisualFeedback() {
        const activeTags = document.querySelectorAll('.ark-tag');
        activeTags.forEach(tag => {
            const momentumPower = parseFloat(tag.getAttribute('data-momentum'));
            if (momentumPower > 91 && Math.random() > 0.7) {
                tag.classList.add('neural-fire');
                setTimeout(() => tag.classList.remove('neural-fire'), 1000);
            }
        });
    }

    setupKeybindings() {
        document.addEventListener('keydown', (e) => {
            if (e.code === 'KeyR') {
                console.warn("🔄 [ARK] Manual Recalibration Triggered.");
                this.synchronizeWithVault();
            }
        });
    }
}

// Global initialization
window.addEventListener('load', () => {
    window.arkDashboard = new ArkDashboard();
});
