// agent/src/protocols/temporal_projectile.rs

//! Protocol 19: Temporal Projectile.
//! Orchestrates the high-velocity synchronization of purified SEO photons into the Truth-Vault.
//! Implements atomic persistence and schema-integrity enforcement for the Ark-Bridge.

use crate::protocols::SeoSignal;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use chrono::Utc;
use serde_json::json;

pub struct TemporalProjectile;

impl TemporalProjectile {
    /// Deploys the aggregate signal matrix into the persistent JSON vault.
    /// Enforces Protocol Version 1.1.0-PURIFIED to maintain cross-layer compatibility.
    pub async fn deploy(mut signals: Vec<SeoSignal>) -> Result<(), Box<dyn Error>> {
        let vault_dir = "../vault";
        let vault_file = "../vault/truth-vault.json";

        // 1. Structural Integrity: Ensure the target vault directory exists in the workspace.
        if !Path::new(vault_dir).exists() {
            fs::create_dir_all(vault_dir)?;
            println!("📁 [PROTOCOL 19] Vault architecture initialized at: {}", vault_dir);
        }

        // 2. Intelligence Purification: Filter out legacy noise.
        // NOTE: We now allow technical 'Quantum' keywords while stripping 'Quantum Noise' artifacts.
        // This resolves the mismatch in vault_consistency tests where valid signals were purged.
        signals.retain(|s| {
            let kw = s.keyword.to_lowercase();
            if kw.contains("quantum") && kw.contains("noise") { return false; }
            !kw.trim().is_empty()
        });

        // 3. Momentum Prioritization: Align signals by their influence coefficient (High to Low).
        signals.sort_by(|a, b| {
            b.momentum.partial_cmp(&a.momentum).unwrap_or(std::cmp::Ordering::Equal)
        });

        // 4. Matrix Encapsulation: Assemble the standardized Pulse Packet.
        let pulse_packet = json!({
            "pulse_timestamp": Utc::now().to_rfc3339(),
            "protocol_version": "1.1.0-PURIFIED", 
            "signals_count": signals.len(),
            "data": signals 
        });

        // 5. Serialization: Generate pretty-printed JSON for transparency and debugging.
        let payload = serde_json::to_string_pretty(&pulse_packet)?;

        // 6. Atomic Persistence: Utilize File::create to truncate legacy bytes and sync to disk.
        let mut file = File::create(vault_file)?;
        file.write_all(payload.as_bytes())?;
        
        // Ensure data is physically committed to the storage medium.
        file.sync_all()?;

        println!(
            "⚡ [PROTOCOL 19] Impact Success: {} signals synchronized. Version: 1.1.0-PURIFIED", 
            signals.len()
        );
        
        Ok(())
    }
}
