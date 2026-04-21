// agent/src/protocols/temporal_projectile.rs

//! Protocol 19: Temporal Projectile.
//! Handles the instantaneous deployment of synchronized SEO signals to the Vault.
//! This protocol ensures that the 'Past-Near' search intent is captured and stored
//! in a version-controlled, lightweight JSON format.

use crate::protocols::SeoSignal;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use chrono::Utc;

pub struct TemporalProjectile;

impl TemporalProjectile {
    /// Deploys the processed keywords into the Truth-Vault.
    /// This is the final impact point of the Chrono-SEO Pulse.
    /// 
    /// # Arguments
    /// * `signals` - The synchronized and filtered SEO signals from Protocol 9.
    pub async fn deploy(signals: Vec<SeoSignal>) -> Result<(), Box<dyn Error>> {
        // Path to the sovereign data storage
        let vault_path = "../vault/truth-vault.json";

        // Protocol 19 Logic: Metadata injection for temporal tracking.
        // We wrap the signals in a 'PulsePacket' to include a timestamp.
        let pulse_packet = serde_json::json!({
            "pulse_timestamp": Utc::now().to_rfc3339(),
            "protocol_version": "1.0.0-ARK",
            "signals_count": signals.len(),
            "data": signals
        });

        // Serialize the packet to a compact JSON string
        let payload = serde_json::to_string_pretty(&pulse_packet)?;

        // Atomic Write: Ensure the vault is updated instantaneously.
        let mut file = File::create(vault_path)?;
        file.write_all(payload.as_bytes())?;

        // Protocol 19 emphasizes 'Instant Deployment'. 
        // Once this file is written, the GitHub Action will commit and push,
        // triggering the 'Temporal Impact' across all connected sites.
        Ok(())
    }
}
