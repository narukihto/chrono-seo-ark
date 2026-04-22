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
    /// This is the final impact point of the Chrono-SEO Pulse.
    /// 
    /// # Arguments
    /// * `signals` - The synchronized and filtered SEO signals from Protocol 9.
    pub async fn deploy(signals: Vec<SeoSignal>) -> Result<(), Box<dyn Error>> {
        // Path to the sovereign data storage (Relative to agent execution point)
        let vault_dir = "../vault";
        let vault_file = "../vault/truth-vault.json";

        // 1. Vault Resilience: Ensure the directory exists before attempting deployment.
        // This prevents the 'No such file or directory' error during fresh CI pulses.
        if !Path::new(vault_dir).exists() {
            fs::create_dir_all(vault_dir)?;
            println!("📁 [PROTOCOL 19] Vault directory initialized.");
        }

        // 2. Pulse Packet Assembly: Metadata injection for temporal tracking.
        // Matches the '1.0.0-ARK' specification seen in the Truth-Vault logs.
        let pulse_packet = json!({
            "pulse_timestamp": Utc::now().to_rfc3339(),
            "protocol_version": "1.0.0-ARK",
            "signals_count": signals.len(),
            "data": signals
        });

        // 3. Serialization: Transform to pretty-printed JSON for auditability and manual review.
        let payload = serde_json::to_string_pretty(&pulse_packet)?;

        // 4. Atomic Pulse Deployment: Instantaneous file creation and stream writing.
        // Using File::create ensures the previous pulse is overwritten by the latest Truth.
        let mut file = File::create(vault_file)?;
        file.write_all(payload.as_bytes())?;

        println!("🚀 [PROTOCOL 19] Temporal Impact Complete. {} signals projected to Vault.", signals.len());
        
        Ok(())
    }
}
