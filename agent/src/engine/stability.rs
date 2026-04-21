// agent/src/engine/stability.rs

//! Stability Module: The enforcement layer for SECURE_CORE constraints.
//! 
//! This module provides the StabilityGuard, which uses geometric immunity
//! to protect the kernel from volatile SEO momentum and potential spam.

use super::geometry::GeometricCalculator;
use super::{SECURE_CORE, CORE_BASE};

pub struct StabilityGuard {
    /// The number of geometric poles currently active ($N$).
    /// Marked as `allow(dead_code)` to maintain architectural data even when
    /// logic primarily relies on Φ (phi).
    #[allow(dead_code)]
    pub poles: f64,
    /// The pre-calculated geometric immunity coefficient (Φ).
    pub phi: f64,
}

impl StabilityGuard {
    /// Initializes a new StabilityGuard with a specific geometric configuration.
    /// 
    /// # Arguments
    /// * `poles` - The number of poles to activate (e.g., 12.0 for a Dodecagon).
    pub fn new(poles: f64) -> Self {
        let phi = GeometricCalculator::calculate_immunity(poles);
        Self { poles, phi }
    }

    /// Calculates the normalized impact of a signal's momentum.
    /// 
    /// # Arguments
    /// * `momentum` - The raw deficit value from the signal capture.
    pub fn calculate_impact(&self, momentum: f64) -> f64 {
        GeometricCalculator::calculate_impact(momentum, self.phi)
    }

    /// Determines if a calculated impact is safe for the SECURE_CORE.
    /// 
    /// A signal is considered 'stable' if $(CORE\_BASE - impact) > SECURE\_CORE$.
    /// 
    /// # Arguments
    /// * `impact` - The calculated stability loss.
    pub fn is_stable(&self, impact: f64) -> bool {
        // 1. Sanity Check: Prevent negative or non-finite impacts from bypassing the guard.
        if !impact.is_finite() || impact < 0.0 {
            return false;
        }

        // 2. Threshold Check: Calculate the projected stability.
        let projected_stability = CORE_BASE - impact;

        // 3. Enforcement: If stability falls to 0.05 or lower, the signal is rejected.
        projected_stability > SECURE_CORE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_core_enforcement() {
        // Initialize with a Hexagon (N=6.0, Φ=2.0)
        let guard = StabilityGuard::new(6.0);
        
        // Scenario A: Safe Signal
        // Momentum 50.0 -> Impact (50 * 0.02) / 2 = 0.5
        // 1.0 - 0.5 = 0.5 (Which is > 0.05) -> Stable
        assert!(guard.is_stable(guard.calculate_impact(50.0)));

        // Scenario B: Critical Impact (Spam/Volatility)
        // Momentum 100.0 -> Impact (100 * 0.02) / 2 = 1.0
        // 1.0 - 1.0 = 0.0 (Which is <= 0.05) -> Unstable (Rejected)
        assert!(!guard.is_stable(guard.calculate_impact(100.0)));
    }
}
