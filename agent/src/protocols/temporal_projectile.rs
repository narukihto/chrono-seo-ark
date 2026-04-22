// agent/src/protocols/temporal_projectile.rs

//! Protocol 19: Temporal Projectile.
//! Handles the instantaneous deployment of synchronized SEO signals to the Vault.
//! Enhanced with Purity-Validation to ensure zero legacy noise.

use crate::protocols::SeoSignal;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use chrono::Utc;
use serde_json::json;

pub struct TemporalProjectile;

impl TemporalProjectile {
    /// Deploys the purified and processed signals into the Truth-Vault.
    /// This version enforces a protocol-level version jump to track deployment success.
    pub async fn deploy(mut signals: Vec<SeoSignal>) -> Result<(), Box<dyn Error>> {
        let vault_dir = "../vault";
        let vault_file = "../vault/truth-vault.json";

        // 1. Vault Resilience: Directory verification
        if !Path::new(vault_dir).exists() {
            fs::create_dir_all(vault_dir)?;
            println!("📁 [PROTOCOL 19] Vault directory initialized.");
        }

        // 2. Final Purity Check: Redundant filtering to eliminate any 'Quantum' leakage
        // This ensures that even if main.rs misses it, the Vault remains clean.
        signals.retain(|s| !s.keyword.to_lowercase().contains("quantum"));

        // 3. Momentum Calibration: Sorting by strength
        signals.sort_by(|a, b| b.momentum.partial_cmp(&a.momentum).unwrap_or(std::cmp::Ordering::Equal));

        // 4. Pulse Packet Assembly (Version 1.1.0-PURIFIED)
        // We increment the version to verify that the file on GitHub is actually updating.
        let pulse_packet = json!({
            "pulse_timestamp": Utc::now().to_rfc3339(),
            "protocol_version": "1.1.0-PURIFIED", 
            "signals_count": signals.len(),
            "data": signals 
        });

        // 5. Serialization: Pretty-printed for clear audit logs
        let payload = serde_json::to_string_pretty(&pulse_packet)?;

        // 6. Atomic Overwrite: Direct write to ensure the previous state is purged.
        let mut file = File::create(vault_file)?;
        file.write_all(payload.as_bytes())?;

        println!("⚡ [PROTOCOL 19] Temporal Impact Success: {} signals projected. Version: 1.1.0-PURIFIED", signals.len());
        
        Ok(())
    }
}
