/**
 * ARK-SYSTEMS: Neural Dashboard Controller
 * Version: 1.1.0-PURIFIED
 * Component: UI State Manager & Data Binder
 * * Technical Purpose: 
 * 1. Orchestrates real-time synchronization between the Neural Bridge and DOM.
 * 2. Manages sector-specific data aggregation based on momentum heuristics.
 * 3. Implements high-frequency visual telemetry updates.
 */

class ArkDashboard {
    constructor() {
        /** @type {ArkBridge} Reference to the global link instance */
        this.bridge = window.arkBridge;
        this.isMonitoring = false;
        this.init();
    }

    /**
     * Initializes the dashboard controller and establishes the telemetry loop.
     */
    init() {
        console.log("🖥️ [ARK-UI] Dashboard Controller Operational. Awaiting Bridge Sync...");
        this.startMonitoring();
        this.setupKeybindings();
    }

    /**
     * Executes the monitoring heartbeat to track Truth-Vault state changes.
     */
    startMonitoring() {
        this.isMonitoring = true;
        
        // Asynchronous polling interval to decouple UI rendering from network latency
        setInterval(() => {
            if (this.bridge && this.bridge.vaultData) {
                this.updateSectorAnalytics();
                this.triggerVisualFeedback();
            }
        }, 2000); 
    }

    /**
     * Performs sector decoupling and frequency analysis on the signal stream.
     * Maps raw momentum bands to architectural sectors (News vs. Market).
     */
    updateSectorAnalytics() {
        const signals = this.bridge.vaultData.data || [];
        const totalCount = signals.length;

        // Heuristic Filter: Segmenting signals by their Momentum Influence Tiers
        // Tier 1 (Market/Arbitrage): ~92.0 | Tier 2 (Geopolitical News): ~88.0
        const newsCount = signals.filter(s => s.momentum >= 87 && s.momentum <= 89).length;
        const marketCount = totalCount - newsCount;

        this.applyNumericalInterpolation("count-total", totalCount);
        this.applyNumericalInterpolation("count-news", newsCount);
        this.applyNumericalInterpolation("count-market", marketCount);
    }

    /**
     * Implements linear interpolation for UI counter transitions.
     * Prevents abrupt state jumps for a smoother UX flow.
     */
    applyNumericalInterpolation(id, targetValue) {
        const element = document.getElementById(id);
        if (!element) return;
        
        const currentValue = parseInt(element.innerText) || 0;
        if (currentValue === targetValue) return;

        let current = currentValue;
        const range = targetValue - currentValue;
        const step = range > 0 ? 1 : -1;
        const duration = 800; // Total transition ms
        const frameRate = Math.abs(Math.floor(duration / range));

        const counterInterval = setInterval(() => {
            current += step;
            element.innerText = current;
            if (current == targetValue) {
                clearInterval(counterInterval);
            }
        }, frameRate || 40);
    }

    /**
     * Triggers high-momentum glow shaders on specific DOM elements.
     * Used to highlight elite photons (Threshold: >90% Influence).
     */
    triggerVisualFeedback() {
        const activeTags = document.querySelectorAll('.ark-tag');
        activeTags.forEach(tag => {
            const momentumPower = parseFloat(tag.getAttribute('data-momentum'));
            
            // Randomly simulate neural firing for high-influence tags
            if (momentumPower > 90 && Math.random() > 0.85) {
                tag.style.boxShadow = `0 0 20px var(--ark-neon)`;
                tag.style.transition = "box-shadow 0.3s ease";
                
                setTimeout(() => {
                    tag.style.boxShadow = 'none';
                }, 400);
            }
        });
    }

    /**
     * Registers architect-level system overrides.
     */
    setupKeybindings() {
        // Manual Force-Pulse [Key: R] for instant Truth-Vault recalibration
        document.addEventListener('keydown', (e) => {
            if (e.code === 'KeyR') {
                console.warn("🔄 [ARK] Architect Override: Manual Pulse Injection Initiated.");
                this.bridge.pulse();
            }
        });
    }
}

// Instantiate the controller once the runtime environment is ready
window.addEventListener('load', () => {
    // 500ms delay to ensure Neural Bridge handshaking is complete
    setTimeout(() => {
        window.arkDashboard = new ArkDashboard();
    }, 500);
});
