// agent/src/engine/stability.rs

//! Stability Module: The enforcement layer for SECURE_CORE constraints.
//! 
//! This module provides the StabilityGuard, which uses geometric immunity
//! to protect the kernel from volatile SEO momentum and potential spam.

use super::geometry::GeometricCalculator;
use super::{SECURE_CORE, CORE_BASE};

pub struct StabilityGuard {
    /// The number of geometric poles currently active ($N$).
    #[allow(dead_code)]
    pub poles: f64,
    /// The pre-calculated geometric immunity coefficient (Φ).
    pub phi: f64,
}

impl StabilityGuard {
    /// Initializes a new StabilityGuard with a specific geometric configuration.
    pub fn new(poles: f64) -> Self {
        let phi = GeometricCalculator::calculate_immunity(poles);
        Self { poles, phi }
    }

    /// Calculates the normalized impact of a signal's momentum.
    pub fn calculate_impact(&self, momentum: f64) -> f64 {
        GeometricCalculator::calculate_impact(momentum, self.phi)
    }

    /// Determines if a calculated impact is safe for the SECURE_CORE.
    /// 
    /// Optimized for Protocol 9 Adaptive Thresholds (0.15 Tolerance).
    pub fn is_stable(&self, impact: f64) -> bool {
        // 1. Sanity Check: Prevent negative or non-finite impacts from bypassing the guard.
        if !impact.is_finite() || impact < 0.0 {
            return false;
        }

        // 2. Adaptive Stability Calculation:
        // We ensure that the system allows for high-momentum trends 
        // by verifying that impact does not breach the 0.15 threshold.
        // This is synchronized with the new purified signal architecture.
        let projected_stability = CORE_BASE - impact;

        // 3. Enforcement: 
        // A signal is stable if it maintains a core buffer above 0.85 (Impact < 0.15).
        // This prevents the 'Systemic Failure' seen in the legacy 0.05 constraint.
        impact < 0.15 && projected_stability > SECURE_CORE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_core_enforcement() {
        // Initialize with a Dodecagon (N=12.0, Φ=4.0) for modern trend validation
        let guard = StabilityGuard::new(12.0);
        
        // Scenario A: Fresh High-Momentum Trend
        // Should be accepted under the 0.15 adaptive threshold.
        let safe_impact = 0.10; 
        assert!(guard.is_stable(safe_impact));

        // Scenario B: Extreme Volatility / Systemic Threat
        // Momentum causing impact > 0.15 should still be rejected.
        let threat_impact = 0.20;
        assert!(!guard.is_stable(threat_impact));
    }
}
