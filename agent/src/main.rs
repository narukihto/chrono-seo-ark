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

    // --- SECURE CONFIGURATION MATRIX ---
    // Fetching environmental keys for multi-sector intelligence gathering.
    let keys = (
        env::var("ARK_API_KEY").unwrap_or_else(|_| "SIM_MODE".to_string()),
        env::var("SERP_API_KEY").unwrap_or_else(|_| "SIM_MODE".to_string()),
        env::var("COINGECKO_API_KEY").unwrap_or_else(|_| "SIM_MODE".to_string()),
        env::var("FREE_CRYPTO_KEY").unwrap_or_else(|_| "SIM_MODE".to_string()),
    );

    let args: Vec<String> = env::args().collect();
    let debug_mode = args.contains(&"--debug".to_string());

    // --- 1. ARCHITECTURAL STABILITY INITIALIZATION ---
    // Activating the Penta-V Stability Guard to filter market volatility.
    let guard = StabilityGuard::new(12.0);
    println!("🏛️ [ARK] Penta-V Stability Guard Active. Poles: 12.0, Φ: {:.2}", guard.phi);

    // --- 2. PROTOCOL 15: MULTI-SECTOR CHERENKOV SCAN ---
    // High-frequency scanning across News, Trends, and Crypto sectors.
    let (raw_signals, report) = CherenkovLens::multi_scan(&keys).await;

    println!("📡 [ARK] Sector Scan Complete:");
    println!("    |-- News Sector   : {} signals", report.news_count);
    println!("    |-- Trends Sector: {} signals", report.trends_count);
    println!("    |-- Market Sector: {} signals", report.gecko_count);
    println!("    |-- Crypto Sector: {} signals", report.crypto_count);
    println!("🛰️ [ARK] Total Aggregate Signals: {}", raw_signals.len());

    // --- 3. PROTOCOL 9: LIQUID SYNCHRONY & PURGE PROTOCOL ---
    // Filtering noise and aligning keywords by momentum coefficients.
    let mut processed_keywords = LiquidSync::process(&guard, raw_signals).await;
    
    // Purge logic: Removing deprecated 'quantum' noise artifacts.
    processed_keywords.retain(|s| !s.keyword.to_lowercase().contains("quantum"));
    
    if processed_keywords.len() > 1 {
        processed_keywords.sort_by(|a, b| b.momentum.partial_cmp(&a.momentum).unwrap_or(std::cmp::Ordering::Equal));
    }

    if processed_keywords.is_empty() {
        println!("🌑 [ARK] No stable signals survived the Purge Protocol. System idling.");
    } else {
        println!("💎 [ARK] Liquid Synchrony identified {} clean stable keywords.", processed_keywords.len());
        
        // --- PROTOCOL 19 BRIDGE: GEMINI AI DEPLOYMENT ---
        // Crucial: Execute Gemini projection for the top-tier signal.
        // This ensures the Template-to-Index injection occurs before final vault commit.
        if let Some(primary_signal) = processed_keywords.first() {
            println!("🚀 [ARK] Projecting Primary Signal to Gemini: [{}]", primary_signal.keyword);
            
            // Triggering the deployment method defined in protocols/mod.rs
            primary_signal.deploy_to_gemini();
        }

        if debug_mode {
            println!("🔍 [DEBUG] Full Purified Signals Map (Autonomous Stream):");
            for (index, signal) in processed_keywords.iter().enumerate() {
                println!("     |-- Signal {:02}: [{}] (Momentum: {:.4})", index + 1, signal.keyword, signal.momentum);
            }
        }
    }

    // --- 4. PROTOCOL 19: TEMPORAL PROJECTILE ---
    // Atomic synchronization of the purified signal matrix into the Truth-Vault.
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
