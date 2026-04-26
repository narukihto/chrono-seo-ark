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
    let keys = (
        env::var("ARK_API_KEY").unwrap_or_else(|_| "SIM_MODE".to_string()),
        env::var("SERP_API_KEY").unwrap_or_else(|_| "SIM_MODE".to_string()),
        env::var("COINGECKO_API_KEY").unwrap_or_else(|_| "SIM_MODE".to_string()),
        env::var("FREE_CRYPTO_KEY").unwrap_or_else(|_| "SIM_MODE".to_string()),
    );

    let args: Vec<String> = env::args().collect();
    let debug_mode = args.contains(&"--debug".to_string());

    // --- 1. ARCHITECTURAL STABILITY INITIALIZATION ---
    let guard = StabilityGuard::new(12.0);
    println!("🏛️ [ARK] Penta-V Stability Guard Active. Poles: 12.0, Φ: {:.2}", guard.phi);

    // --- 2. PROTOCOL 15: MULTI-SECTOR CHERENKOV SCAN ---
    let (raw_signals, report) = CherenkovLens::multi_scan(&keys).await;

    println!("📡 [ARK] Sector Scan Complete:");
    println!("   |-- News Sector  : {} signals", report.news_count);
    println!("   |-- Trends Sector: {} signals", report.trends_count);
    println!("   |-- Market Sector: {} signals", report.gecko_count);
    println!("   |-- Crypto Sector: {} signals", report.crypto_count);
    println!("🛰️ [ARK] Total Aggregate Signals: {}", raw_signals.len());

    // --- 3. PROTOCOL 9: LIQUID SYNCHRONY & PURGE PROTOCOL ---
    let mut processed_keywords = LiquidSync::process(&guard, raw_signals).await;
    
    processed_keywords.retain(|s| !s.keyword.to_lowercase().contains("quantum"));
    
    if processed_keywords.len() > 1 {
        processed_keywords.sort_by(|a, b| b.momentum.partial_cmp(&a.momentum).unwrap_or(std::cmp::Ordering::Equal));
    }

    if processed_keywords.is_empty() {
        println!("🌑 [ARK] No stable signals survived the Purge Protocol. System idling.");
    } else {
        println!("💎 [ARK] Liquid Synchrony identified {} clean stable keywords.", processed_keywords.len());
        
        // --- GEMINI AI DEPLOYMENT BRIDGE ---
        // Capture the primary momentum signal and project it through Gemini API
        // This occurs BEFORE Protocol 19 encryption to ensure plaintext capture.
        if let Some(primary_signal) = processed_keywords.first() {
            println!("🚀 [ARK] Projecting Primary Signal to Gemini: [{}]", primary_signal.keyword);
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
