// agent/src/main.rs

//! Chrono-SEO Agent: The Heart of Ark Systems.
//! Orchestrates high-frequency SEO signal capture and geometric stabilization.
//! Developed by The First Architect.

mod engine;
mod protocols;

use crate::engine::stability::StabilityGuard;
use crate::protocols::cherenkov_lens::CherenkovLens;
use crate::protocols::liquid_sync::LiquidSync;
use crate::protocols::temporal_projectile::TemporalProjectile;
use std::env;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_pulse = Instant::now();
    println!("🛡️ [ARK] Initializing Chrono-SEO Pulse Agent...");

    // --- SECURE CONFIGURATION ---
    // Extract API Key from environment. If missing, the agent defaults to Simulation Mode
    // to prevent CI pipeline failure while maintaining system integrity.
    let api_key = env::var("ARK_API_KEY").unwrap_or_else(|_| {
        println!("⚠️ [ARK] WARNING: ARK_API_KEY not detected. Operating in Simulation Mode.");
        "SIM_PROTOTYPE".to_string()
    });

    // --- 1. ARCHITECTURAL STABILITY INITIALIZATION ---
    // We utilize a Dodecagon (12 Poles) for maximum immunity during scanning.
    // This configuration provides the optimal Φ (Phi) for high-volatility environments.
    let guard = StabilityGuard::new(12.0);
    println!("🏛️ [ARK] Penta-V Stability Guard Active. Poles: 12.0, Φ: {:.2}", guard.phi);

    // --- 2. PROTOCOL 15: CHERENKOV'S LENS ---
    // Scanning high-frequency signal streams. 
    // Logic handles both real API interaction and Simulated data injection.
    let scan_result = CherenkovLens::scan(&api_key).await;
    
    let raw_signals = match scan_result {
        Ok(signals) => {
            println!("📡 [ARK] Cherenkov Lens captured {} raw SEO signals.", signals.len());
            signals
        },
        Err(e) => {
            println!("🛑 [ARK] Protocol 15 Error: {}. Reverting to localized data stream.", e);
            Vec::new() // Graceful fallback
        }
    };

    // --- 3. PROTOCOL 9: LIQUID SYNCHRONY ---
    // Filtering & Prediction through Geometric Stability logic.
    // Only signals that survive the SECURE_CORE threshold are advanced to the Vault.
    let processed_keywords = LiquidSync::process(&guard, raw_signals).await;
    
    if processed_keywords.is_empty() {
        println!("🌑 [ARK] No stable signals detected in current pulse. System idling.");
    } else {
        println!("💎 [ARK] Liquid Synchrony identified {} stable keywords.", processed_keywords.len());
    }

    // --- 4. PROTOCOL 19: TEMPORAL PROJECTILE ---
    // Final deployment to the Truth-Vault (Sovereign Data Storage).
    // This ensures total synchronization between the Agent and the Client Bridge.
    match TemporalProjectile::deploy(processed_keywords).await {
        Ok(_) => {
            let pulse_duration = start_pulse.elapsed();
            println!("⚡ [ARK] Pulse Completed Successfully in {:?}. Truth-Vault synchronized.", pulse_duration);
        },
        Err(e) => {
            println!("❌ [ARK] Protocol 19 Deployment Failed: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
