// agent/src/protocols/temporal_projectile.rs

//! Protocol 19: Temporal Projectile.
//! Handles the instantaneous deployment of synchronized SEO signals to the Vault.
//! This protocol ensures that the 'Past-Near' search intent is captured and stored
//! in a version-controlled, lightweight JSON format.

use crate::protocols::SeoSignal;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use chrono::Utc;
use serde_json::json;

pub struct TemporalProjectile;

impl TemporalProjectile {
    /// Deploys the processed keywords into the Truth-Vault.
    /// Updated to ensure all captured signals (up to 5) are projected without strict filtering.
    pub async fn deploy(mut signals: Vec<SeoSignal>) -> Result<(), Box<dyn Error>> {
        // Path to the sovereign data storage
        let vault_dir = "../vault";
        let vault_file = "../vault/truth-vault.json";

        // 1. Vault Resilience: Ensure the directory exists
        if !Path::new(vault_dir).exists() {
            fs::create_dir_all(vault_dir)?;
            println!("📁 [PROTOCOL 19] Vault directory initialized.");
        }

        // 2. Diversification Logic: 
        // Sort by momentum descending to ensure the strongest signals are at the top.
        signals.sort_by(|a, b| b.momentum.partial_cmp(&a.momentum).unwrap_or(std::cmp::Ordering::Equal));

        // 3. Pulse Packet Assembly: Metadata injection for temporal tracking.
        let pulse_packet = json!({
            "pulse_timestamp": Utc::now().to_rfc3339(),
            "protocol_version": "1.0.0-ARK",
            "signals_count": signals.len(),
            "data": signals // Now includes all captured signals (e.g., 5)
        });

        // 4. Serialization: Transform to pretty-printed JSON
        let payload = serde_json::to_string_pretty(&pulse_packet)?;

        // 5. Atomic Pulse Deployment: Instantaneous file creation
        let mut file = File::create(vault_file)?;
        file.write_all(payload.as_bytes())?;

        println!("🚀 [PROTOCOL 19] Temporal Impact Complete. {} signals projected to Vault.", signals.len());
        
        Ok(())
    }
}
