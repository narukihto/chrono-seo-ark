// agent/src/main.rs

//! Chrono-SEO Agent: The Heart of Ark Systems.
//! Orchestrates high-frequency SEO signal capture and geometric stabilization.

mod engine;
mod protocols;

use crate::engine::stability::StabilityGuard;
use crate::protocols::cherenkov_lens::CherenkovLens;
use crate::protocols::liquid_sync::LiquidSync;
use crate::protocols::temporal_projectile::TemporalProjectile;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_pulse = std::time::Instant::now();
    println!("🛡️ [ARK] Initializing Chrono-SEO Pulse Agent...");

    // 1. Initialize Penta-V Stability Guard
    // We utilize a Dodecagon (12 Poles) for maximum immunity during scanning.
    let guard = StabilityGuard::new(12.0);
    println!("🏛️ [ARK] Penta-V Stability Guard Active. Poles: 12.0, Φ: {:.2}", guard.phi);

    // 2. Protocol 15: Cherenkov's Lens (Scanning high-frequency signal streams)
    let raw_signals = CherenkovLens::scan().await?;
    println!("📡 [ARK] Cherenkov Lens captured {} raw SEO signals.", raw_signals.len());

    // 3. Protocol 9: Liquid Synchrony (Filtering & Prediction through Geometric Stability)
    let processed_keywords = LiquidSync::process(&guard, raw_signals).await;
    println!("💎 [ARK] Liquid Synchrony identified {} stable keywords.", processed_keywords.len());

    // 4. Protocol 19: Temporal Projectile (Deployment to Truth-Vault)
    TemporalProjectile::deploy(processed_keywords).await?;

    let pulse_duration = start_pulse.elapsed();
    println!("⚡ [ARK] Pulse Completed Successfully in {:?}. Truth-Vault synchronized.", pulse_duration);

    Ok(())
}
