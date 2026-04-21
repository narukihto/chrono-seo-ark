// agent/src/engine/mod.rs

//! Penta-V Engine: The architectural foundation for system resilience.
//! 
//! This module defines the core constants and state structures required
//! for geometric load balancing and stability enforcement.

pub mod geometry;
pub mod stability;

/// The absolute threshold for system integrity.
/// If current_stability drops to or below this value, the Guard triggers 
/// immediate protection protocols.
pub const SECURE_CORE: f64 = 0.05;

/// The base stability value for a pristine system state.
pub const CORE_BASE: f64 = 1.0;

/// The decay constant used to simulate the dissipation of stressors (SEO Momentum).
pub const DECAY_COEFFICIENT: f64 = 0.02;

/// Represents the internal state of the Penta-V Engine.
#[derive(Debug, Clone)]
pub struct KernelState {
    /// The current stability level, ranging from SECURE_CORE to CORE_BASE.
    pub current_stability: f64,
    
    /// The number of geometric poles currently active ($N$).
    pub active_poles: f64,
}

impl KernelState {
    /// Initializes the engine state at maximum stability.
    /// 
    /// # Arguments
    /// * `poles` - The initial geometric configuration (e.g., 12.0 for Dodecagon).
    pub fn new(poles: f64) -> Self {
        Self {
            current_stability: CORE_BASE,
            active_poles: poles,
        }
    }
}
