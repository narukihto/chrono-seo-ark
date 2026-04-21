// agent/src/protocols/liquid_sync.rs

//! Protocol 9: Liquid Synchrony.
//! Filters raw SEO signals using Penta-V Geometric Stability to predict
//! high-impact micro-trends while eliminating noise and spam.

use crate::engine::stability::StabilityGuard;
use crate::protocols::SeoSignal;

pub struct LiquidSync;

impl LiquidSync {
    /// Processes raw signals by calculating their geometric impact.
    /// Only signals that maintain the SECURE_CORE integrity are synchronized.
    /// 
    /// # Arguments
    /// * `guard` - The Penta-V StabilityGuard instance.
    /// * `raw_signals` - A vector of SeoSignals captured from high-frequency streams.
    pub async fn process(guard: &StabilityGuard, raw_signals: Vec<SeoSignal>) -> Vec<SeoSignal> {
        let mut synchronized_vault = Vec::new();

        for mut signal in raw_signals {
            // Calculate the impact of this signal (Deficit) on the system.
            // Protocol 9 Logic: Higher momentum requires higher Geometric Immunity (Φ).
            let impact = guard.calculate_impact(signal.momentum);

            // Validation: Only allow signals that do not compromise the SECURE_CORE (0.05).
            if guard.is_stable(impact) {
                signal.stability_score = Some(impact);
                synchronized_vault.push(signal);
            }
            
            // Note: If impact is too high, the signal is 'decayed' and discarded 
            // as noise or potential SEO manipulation (Spam).
        }

        // Sort by stability score (Lower impact = Higher stability/quality)
        synchronized_vault.sort_by(|a, b| {
            a.stability_score.partial_cmp(&b.stability_score).unwrap_or(std::cmp::Ordering::Equal)
        });

        synchronized_vault
    }
}
