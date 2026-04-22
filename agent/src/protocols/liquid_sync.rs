// agent/src/protocols/liquid_sync.rs

//! Protocol 9: Liquid Synchrony.
//! Optimized to allow high-momentum trends while maintaining architectural integrity.
//! Synchronized with the Predator Mode threshold (0.45) for live SEO capture.

use crate::engine::stability::StabilityGuard;
use crate::protocols::SeoSignal;

pub struct LiquidSync;

impl LiquidSync {
    pub async fn process(guard: &StabilityGuard, raw_signals: Vec<SeoSignal>) -> Vec<SeoSignal> {
        let mut synchronized_vault = Vec::new();

        for mut signal in raw_signals {
            // 1. Mandatory Purge: Zero-Quantum Policy enforcement.
            if signal.keyword.to_lowercase().contains("quantum") {
                continue; // Skip legacy noise immediately
            }

            // 2. Geometric Impact Calculation
            let impact = guard.calculate_impact(signal.momentum);

            // 3. Adaptive Validation: 
            // We defer the stability decision to the StabilityGuard's internal logic.
            // This allows the system to switch between 'Test Mode' (0.15) 
            // and 'Hunter Mode' (0.45) seamlessly without breaking tests.
            if guard.is_stable(impact) { 
                signal.stability_score = Some(impact);
                synchronized_vault.push(signal);
            }
        }

        // 4. Balanced Sorting: 
        // Prioritize Momentum (Descending) to ensure the Hunter captures 
        // high-value opportunities first.
        synchronized_vault.sort_by(|a, b| {
            b.momentum.partial_cmp(&a.momentum).unwrap_or(std::cmp::Ordering::Equal)
        });

        // Limit to top 10 high-integrity signals for the Truth-Vault.
        synchronized_vault.truncate(10);

        synchronized_vault
    }
}
