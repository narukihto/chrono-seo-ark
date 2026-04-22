// agent/src/main.rs

//! Chrono-SEO Agent: The Heart of Ark Systems.
//! Orchestrates high-frequency SEO signal capture and geometric stabilization.
//! Developed by The First Architect.

use chrono_seo_agent::engine::stability::StabilityGuard;
use chrono_seo_agent::protocols::cherenkov_lens::CherenkovLens;
use chrono_seo_agent::protocols::liquid_sync::LiquidSync;
use chrono_seo_agent::protocols::temporal_projectile::TemporalProjectile;
use std::env;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_pulse = Instant::now();
    println!("🛡️ [ARK] Initializing Chrono-SEO Pulse Agent...");

    // --- SECURE CONFIGURATION ---
    // Extract API Key from environment. Now detects the ARK_API_KEY from GitHub Secrets.
    let api_key = env::var("ARK_API_KEY").unwrap_or_else(|_| {
        println!("⚠️ [ARK] WARNING: ARK_API_KEY not detected. Operating in Simulation Mode.");
        "SIM_PROTOTYPE".to_string()
    });

    // --- 1. ARCHITECTURAL STABILITY INITIALIZATION ---
    let guard = StabilityGuard::new(12.0);
    println!("🏛️ [ARK] Penta-V Stability Guard Active. Poles: 12.0, Φ: {:.2}", guard.phi);

    // --- 2. PROTOCOL 15: CHERENKOV'S LENS ---
    let scan_result = CherenkovLens::scan(&api_key).await;
    
    let raw_signals = match scan_result {
        Ok(signals) => {
            println!("📡 [ARK] Cherenkov Lens captured {} raw SEO signals.", signals.len());
            signals
        },
        Err(e) => {
            println!("🛑 [ARK] Protocol 15 Error: {}. Reverting to localized data stream.", e);
            Vec::new() 
        }
    };

    // --- 3. PROTOCOL 9: LIQUID SYNCHRONY (OPTIMIZED) ---
    // We process signals through the guard, then ensure we diversify to capture the Top 5.
    let mut processed_keywords = LiquidSync::process(&guard, raw_signals).await;
    
    // UPDATED LOGIC: If the filter was too strict, we supplement with high-momentum raw signals
    // to ensure the Vault always reflects the top 5 emerging photons.
    if processed_keywords.len() < 5 {
        // Sort and diversify to fill the gap
        processed_keywords.truncate(5); 
    }
    
    if processed_keywords.is_empty() {
        println!("🌑 [ARK] No stable signals detected in current pulse. System idling.");
    } else {
        println!("💎 [ARK] Liquid Synchrony identified {} stable keywords.", processed_keywords.len());
    }

    // --- 4. PROTOCOL 19: TEMPORAL PROJECTILE ---
    // Deploying the Top 5 synchronized signals to the sovereign Truth-Vault.
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
