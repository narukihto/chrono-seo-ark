// agent/src/main.rs

//! Chrono-SEO Agent: The Heart of Ark Systems.
//! Orchestrates high-frequency SEO signal capture and geometric stabilization.
//! Purge Protocol Active: Eliminating legacy Quantum noise to ensure Vault purity.

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
    // Initialize sovereign API access. System defaults to Simulation Mode if environment variables are restricted.
    let api_key = env::var("ARK_API_KEY").unwrap_or_else(|_| {
        println!("⚠️ [ARK] WARNING: ARK_API_KEY not detected. Operating in Simulation Mode.");
        "SIM_PROTOTYPE".to_string()
    });

    // Detect runtime flags. Enable extended logging if --debug is present in execution arguments.
    let args: Vec<String> = env::args().collect();
    let debug_mode = args.contains(&"--debug".to_string());

    // --- 1. ARCHITECTURAL STABILITY INITIALIZATION ---
    // Deploy Penta-V Stability Guard to maintain geometric integrity across data poles.
    let guard = StabilityGuard::new(12.0);
    println!("🏛️ [ARK] Penta-V Stability Guard Active. Poles: 12.0, Φ: {:.2}", guard.phi);

    // --- 2. PROTOCOL 15: CHERENKOV'S LENS ---
    // Execute high-frequency scan of the global data stream via Cherenkov Lens protocol.
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

    // --- 3. PROTOCOL 9: LIQUID SYNCHRONY & PURGE PROTOCOL ---
    // Synchronize raw signals and execute hard-purge of legacy 'Quantum' noise.
    let mut processed_keywords = LiquidSync::process(&guard, raw_signals).await;
    
    // Purge Protocol: Eliminate signals conflicting with current architectural focus.
    processed_keywords.retain(|s| !s.keyword.to_lowercase().contains("quantum"));
    
    // Sort signals by Momentum to prioritize high-impact data points.
    if processed_keywords.len() > 1 {
        processed_keywords.sort_by(|a, b| b.momentum.partial_cmp(&a.momentum).unwrap_or(std::cmp::Ordering::Equal));
    }

    // Dynamic Injection: The system now operates autonomously without fixed truncation.
    if processed_keywords.is_empty() {
        println!("🌑 [ARK] No stable signals survived the Purge Protocol. System idling.");
    } else {
        println!("💎 [ARK] Liquid Synchrony identified {} clean stable keywords.", processed_keywords.len());
        
        // Debug Telemetry: Reveal the complete purified signal map.
        if debug_mode {
            println!("🔍 [DEBUG] Full Purified Signals Map (Autonomous Stream):");
            for (index, signal) in processed_keywords.iter().enumerate() {
                println!("   |-- Signal {:02}: [{}] (Momentum: {:.4})", index + 1, signal.keyword, signal.momentum);
            }
        }
    }

    // --- 4. PROTOCOL 19: TEMPORAL PROJECTILE ---
    // Final deployment of the purified dataset to the Truth-Vault.
    match TemporalProjectile::deploy(processed_keywords).await {
        Ok(_) => {
            let pulse_duration = start_pulse.elapsed();
            println!("⚡ [ARK] Pulse Completed Successfully in {:?}. Vault is CLEAN.", pulse_duration);
        },
        Err(e) => {
            println!("❌ [ARK] Protocol 19 Deployment Failed: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
