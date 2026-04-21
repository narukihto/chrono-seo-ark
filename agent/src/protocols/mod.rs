// agent/src/protocols/mod.rs

//! Ark Protocols Module: The implementation of high-speed data capture and deployment.
//! 
//! Protocol 9: Liquid Synchrony (Predictive Analysis)
//! Protocol 15: Cherenkov's Lens (High-Frequency Scanning)
//! Protocol 19: Temporal Projectile (Deployment to Vault)

pub mod liquid_sync;
pub mod cherenkov_lens;
pub mod temporal_projectile;

use serde::{Serialize, Deserialize};

/// The fundamental SEO signal structure used across all Ark protocols.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SeoSignal {
    /// The captured keyword or trend signature.
    pub keyword: String,
    
    /// The momentum factor (Deficit) derived from frequency and search velocity.
    pub momentum: f64,
    
    /// The final stability impact calculated by the Penta-V Engine.
    pub stability_score: Option<f64>,
}

impl SeoSignal {
    /// Creates a new raw signal captured by the Cherenkov Lens.
    pub fn new(keyword: String, momentum: f64) -> Self {
        Self {
            keyword,
            momentum,
            stability_score: None,
        }
    }
}
