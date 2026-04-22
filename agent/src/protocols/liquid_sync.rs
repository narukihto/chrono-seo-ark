// agent/src/protocols/liquid_sync.rs

//! Protocol 9: Liquid Synchrony.
//! Optimized to allow high-momentum trends while maintaining architectural integrity.

use crate::engine::stability::StabilityGuard;
use crate::protocols::SeoSignal;

pub struct LiquidSync;

impl LiquidSync {
    pub async fn process(guard: &StabilityGuard, raw_signals: Vec<SeoSignal>) -> Vec<SeoSignal> {
        let mut synchronized_vault = Vec::new();

        for mut signal in raw_signals {
            // 1. Mandatory Purge: Ensure no 'Quantum' noise leaks through at this stage.
            if signal.keyword.to_lowercase().contains("quantum") {
                continue; // Skip legacy noise immediately
            }

            // 2. Geometric Impact Calculation
            let impact = guard.calculate_impact(signal.momentum);

            // 3. Adaptive Validation: 
            // We allow a slightly higher impact threshold (0.15 instead of 0.05) 
            // for fresh trends to ensure the Vault isn't just full of old data.
            if impact < 0.15 { 
                signal.stability_score = Some(impact);
                synchronized_vault.push(signal);
            }
        }

        // 4. Balanced Sorting: 
        // We now sort by Momentum (Descending) to prioritize what's actually trending,
        // rather than just what's "quiet" and "stable".
        synchronized_vault.sort_by(|a, b| {
            b.momentum.partial_cmp(&a.momentum).unwrap_or(std::cmp::Ordering::Equal)
        });

        // Limit to top 10 high-integrity signals
        synchronized_vault.truncate(10);

        synchronized_vault
    }
}
