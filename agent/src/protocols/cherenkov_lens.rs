// agent/src/protocols/cherenkov_lens.rs

//! Protocol 15: Cherenkov's Lens (Live Global Projection).
//! High-frequency multi-sector capture mechanism designed to detect emerging data 'glow'
//! across the actual web by replacing static simulation buffers with live API streams.

use crate::protocols::{serp_provider, gecko_provider, crypto_pulse, news_provider, SeoSignal};
use reqwest;
use std::pin::Pin;
use futures::future::{join_all, BoxFuture, FutureExt};

/// Structural report to isolate and count signals from each global sector.
pub struct SectorReport {
    pub news_count: usize,
    pub trends_count: usize,
    pub gecko_count: usize,
    pub crypto_count: usize,
}

pub struct CherenkovLens;

/// Shared error type alias for thread-safe concurrent execution.
type ArkResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

impl CherenkovLens {
    /// Executes a parallel high-frequency scan across 4 distinct specialized sectors.
    /// Uses BoxFuture with Send + Sync bounds to satisfy thread-safety for Tokio.
    pub async fn multi_scan(keys: &(String, String, String, String)) -> (Vec<SeoSignal>, SectorReport) {
        println!("📡 [PROTOCOL 15] Initializing Multi-Sector Lens. Mode: LIVE_GLOBAL_SCAN");

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(12)) // Balanced for 4 concurrent global streams
            .user_agent("Ark-Sovereign-Agent/1.0")
            .build()
            .expect("Failed to initialize Sovereign Client");

        // --- PARALLEL EXECUTION MATRIX ---
        // Unifying all providers into a single vector of thread-safe Boxed Futures.
        let mut tasks: Vec<BoxFuture<'_, ArkResult<Vec<SeoSignal>>>> = Vec::new();

        tasks.push(news_provider::fetch(&client, &keys.0).boxed());   // ARK_API_KEY
        tasks.push(serp_provider::fetch(&client, &keys.1).boxed());   // SERP_API_KEY
        tasks.push(gecko_provider::fetch(&client, &keys.2).boxed());  // COINGECKO_API_KEY
        tasks.push(crypto_pulse::fetch(&client, &keys.3).boxed());    // FREE_CRYPTO_KEY

        let results = join_all(tasks).await;

        // --- SECTOR DECOUPLING & ERROR TOLERANCE ---
        // Safely unwrapping results. If a sector fails, it returns an empty stream.
        let news_signals = results[0].as_ref().map(|s| s.clone()).unwrap_or_default().unwrap_or_default();
        let trends_signals = results[1].as_ref().map(|s| s.clone()).unwrap_or_default().unwrap_or_default();
        let gecko_signals = results[2].as_ref().map(|s| s.clone()).unwrap_or_default().unwrap_or_default();
        let crypto_signals = results[3].as_ref().map(|s| s.clone()).unwrap_or_default().unwrap_or_default();

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

        // Data Sanitization: Standard Ark deduplication
        captured_signals.sort_by(|a, b| a.keyword.cmp(&b.keyword));
        captured_signals.dedup_by(|a, b| a.keyword == b.keyword);

        println!("⚡ [LENS] Matrix Scan Complete. Captured {} unique photons across 4 sectors.", captured_signals.len());

        (captured_signals, report)
    }
}
