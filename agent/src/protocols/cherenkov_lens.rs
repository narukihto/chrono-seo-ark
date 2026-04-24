// agent/src/protocols/cherenkov_lens.rs

use crate::protocols::SeoSignal;
use crate::protocols::{serp_provider, gecko_provider, crypto_pulse, news_provider}; // Providers
use reqwest;
use futures::future::join_all;
use std::pin::Pin;
use futures::Future;

/// Structural report to isolate and count signals from each global sector.
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
    pub async fn multi_scan(keys: &(String, String, String, String)) -> (Vec<SeoSignal>, SectorReport) {
        println!("📡 [PROTOCOL 15] Initializing Multi-Sector Lens. Mode: LIVE_GLOBAL_SCAN");

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10)) // Precision timing for parallel load
            .user_agent("Ark-Sovereign-Agent/1.0")
            .build()
            .expect("Failed to initialize Sovereign Client");

        // --- PARALLEL EXECUTION MATRIX ---
        // FIX: Using Pin<Box<dyn Future>> to unify disparate Provider types into a single Vector.
        // This resolves the 'mismatched types' error in the Rust compiler.
        let tasks: Vec<Pin<Box<dyn Future<Output = Result<Vec<SeoSignal>, Box<dyn std::error::Error + Send + Sync>>>>>> = vec![
            Box::pin(news_provider::fetch(&client, &keys.0)),   // ARK_API_KEY (Sector 1)
            Box::pin(serp_provider::fetch(&client, &keys.1)),   // SERP_API_KEY (Sector 2)
            Box::pin(gecko_provider::fetch(&client, &keys.2)),  // COINGECKO_API_KEY (Sector 3)
            Box::pin(crypto_pulse::fetch(&client, &keys.3)),    // FREE_CRYPTO_KEY (Sector 4)
        ];

        let results = join_all(tasks).await;

        // --- SECTOR DECOUPLING & ERROR TOLERANCE ---
        // Safely unwrapping results. If a sector fails, it returns an empty stream to maintain pulse stability.
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

        // Aggregating all captured photons into the primary stream
        let mut captured_signals = Vec::new();
        captured_signals.extend(news_signals);
        captured_signals.extend(trends_signals);
        captured_signals.extend(gecko_signals);
        captured_signals.extend(crypto_signals);

        // Emergency Fallback: Ensure system never idles due to network shadows
        if captured_signals.is_empty() {
            println!("⚠️ [ARK] No live photons detected. Injecting Ark-Foundation stability signals.");
            captured_signals.push(SeoSignal::new("Ark-Foundation Stability Pulse".to_string(), 1.0));
        }

        // Data Sanitization: Global deduplication based on keyword signature
        captured_signals.sort_by(|a, b| a.keyword.cmp(&b.keyword));
        captured_signals.dedup_by(|a, b| a.keyword == b.keyword);

        println!("⚡ [LENS] Matrix Scan Complete. Captured {} unique photons.", captured_signals.len());

        (captured_signals, report)
    }
}
