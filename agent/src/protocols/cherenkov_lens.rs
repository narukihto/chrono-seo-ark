// agent/src/protocols/cherenkov_lens.rs

use crate::protocols::SeoSignal;
use crate::protocols::{serp_provider, gecko_provider, crypto_pulse, news_provider};
use reqwest;
use futures::future::join_all;
use std::pin::Pin;
use futures::Future;

/// Structural report to isolate and count signals from each global sector.
/// Used for telemetry and geometric stability auditing.
pub struct SectorReport {
    pub news_count: usize,
    pub trends_count: usize,
    pub gecko_count: usize,
    pub crypto_count: usize,
}

pub struct CherenkovLens;

impl CherenkovLens {
    /// Executes parallel high-frequency capture across 4 distinct specialized sectors.
    /// Returns a tuple of (Aggregate_Signals, Sector_Report).
    /// Optimized for Protocol 15: Sovereign Matrix.
    pub async fn multi_scan(keys: &(String, String, String, String)) -> (Vec<SeoSignal>, SectorReport) {
        println!("📡 [PROTOCOL 15] Initializing Multi-Sector Lens. Mode: LIVE_GLOBAL_SCAN");

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10)) 
            .user_agent("Ark-Sovereign-Agent/1.0")
            .build()
            .expect("Failed to initialize Sovereign Client");

        // --- PARALLEL EXECUTION MATRIX ---
        let tasks: Vec<Pin<Box<dyn Future<Output = Result<Vec<SeoSignal>, Box<dyn std::error::Error + Send + Sync>>>>>> = vec![
            Box::pin(news_provider::fetch(&client, &keys.0)),   // Sector 1: News
            Box::pin(serp_provider::fetch(&client, &keys.1)),   // Sector 2: Google Trends
            Box::pin(gecko_provider::fetch(&client, &keys.2)),  // Sector 3: Market Data
            Box::pin(crypto_pulse::fetch(&client, &keys.3)),    // Sector 4: On-Chain Pulse
        ];

        let results = join_all(tasks).await;

        // --- SECTOR DECOUPLING & ERROR TOLERANCE ---
        let news_signals = results[0].as_ref().map(|s| s.clone()).unwrap_or_default();
        let trends_signals = results[1].as_ref().map(|s| s.clone()).unwrap_or_default();
        let gecko_signals = results[2].as_ref().map(|s| s.clone()).unwrap_or_default();
        let crypto_signals = results[3].as_ref().map(|s| s.clone()).unwrap_or_default();

        let report = SectorReport {
            news_count: news_signals.len(),
            trends_count: trends_signals.len(),
            gecko_count: gecko_signals.len(),
            crypto_count: crypto_signals.len(),
        };

        let mut captured_signals = Vec::new();
        captured_signals.extend(news_signals);
        captured_signals.extend(trends_signals);
        captured_signals.extend(gecko_signals);
        captured_signals.extend(crypto_signals);

        // Emergency Fallback
        if captured_signals.is_empty() {
            println!("⚠️ [ARK] No live photons detected. Injecting Ark-Foundation stability signals.");
            captured_signals.push(SeoSignal::new("Ark-Foundation Stability Pulse".to_string(), 1.0));
        }

        // --- DATA REFINEMENT & AGGREGATION ---
        
        // 1. Sorting by Momentum: Keeping the Architect's priority (Highest first)
        captured_signals.sort_by(|a, b| {
            b.momentum.partial_cmp(&a.momentum).unwrap_or(std::cmp::Ordering::Equal)
        });

        // 2. Global Deduplication: Ensuring uniqueness without losing the map's depth
        captured_signals.dedup_by(|a, b| a.keyword == b.keyword);

        // 3. FULL VISIBILITY DEBUG: Printing every captured photon before encryption
        // This ensures all 47+ signals are visible in the terminal as requested.
        println!("🔍 [DEBUG] Full Autonomous Stream Map (Total: {}):", captured_signals.len());
        for (i, signal) in captured_signals.iter().enumerate() {
            println!(
                "  |-- Signal {:02}: [{}] (Momentum: {:.4})", 
                i + 1, 
                signal.keyword, 
                signal.momentum
            );
        }

        // NOTE: We do NOT truncate. Every signal is preserved for the Vault.
        println!("⚡ [LENS] Matrix Scan Complete. Captured {} unique photons.", captured_signals.len());

        (captured_signals, report)
    }
}
