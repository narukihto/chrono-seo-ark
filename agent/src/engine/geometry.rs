// agent/src/engine/geometry.rs

//! Geometric Engine: Calculates the structural immunity of the system.
//! 
//! This module translates the number of poles (N) into a stability coefficient (Φ)
//! based on the principle that N -> ∞ approaches a perfect circle (Absolute Stability).

use super::DECAY_COEFFICIENT;

pub struct GeometricCalculator;

impl GeometricCalculator {
    /// Calculates the Geometric Immunity (Φ) based on the active poles (N).
    /// 
    /// Formula: Φ = N / 3.0
    /// This represents the distribution of stress across the geometric vertices.
    /// 
    /// # Arguments
    /// * `poles` - The number of active poles (e.g., 3.0 for Triangle, 12.0 for Dodecagon).
    #[inline(always)]
    pub fn calculate_immunity(poles: f64) -> f64 {
        // We use a base divisor of 3.0 to establish the Triangle as the 
        // minimum stable geometric structure (Unity).
        if poles < 3.0 { 3.0 / 3.0 } else { poles / 3.0 }
    }

    /// Calculates the impact of a deficit (SEO Momentum) on the system.
    /// 
    /// Formula: Impact = (Deficit * Decay) / Φ
    /// 
    /// # Arguments
    /// * `deficit` - The raw momentum of the captured signal.
    /// * `immunity` - The calculated Φ (Phi) from calculate_immunity.
    #[inline(always)]
    pub fn calculate_impact(deficit: f64, immunity: f64) -> f64 {
        // High immunity (Φ) reduces the final impact, protecting the SECURE_CORE.
        (deficit * DECAY_COEFFICIENT) / immunity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immunity_scaling() {
        let triangle_phi = GeometricCalculator::calculate_immunity(3.0); // Should be 1.0
        let dodecagon_phi = GeometricCalculator::calculate_immunity(12.0); // Should be 4.0
        
        assert_eq!(triangle_phi, 1.0);
        assert_eq!(dodecagon_phi, 4.0);
    }

    #[test]
    fn test_impact_reduction() {
        let immunity = 4.0; // Dodecagon
        let deficit = 100.0;
        let impact = GeometricCalculator::calculate_impact(deficit, immunity);
        
        // (100 * 0.02) / 4 = 2.0 / 4 = 0.5
        assert_eq!(impact, 0.5);
    }
}
