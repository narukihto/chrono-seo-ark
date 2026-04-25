// agent/src/protocols/serp_provider.rs

//! SECTOR: GOOGLE TRENDS (SERPAPI)
//! Capture high-velocity search queries before they saturate the global market.
//! Feed these breakout photons into the Penta-V Geometric Stability engine.

use crate::protocols::SeoSignal;
use reqwest;
use serde_json::Value;

/// Fetches real-time trending queries from Google Trends via SerpApi.
/// Optimized for Protocol 15: Sovereign Matrix scan.
pub async fn fetch(
    client: &reqwest::Client, 
    api_key: &str
) -> Result<Vec<SeoSignal>, Box<dyn std::error::Error + Send + Sync>> {
    
    if api_key.is_empty() || api_key == "SIM_MODE" {
        return Ok(vec![]);
    }

    println!("🔍 [SECTOR: TRENDS] Probing Google Trends for breakout signals...");

    // Critical: Using 'google_trends_trending_now' engine for live high-velocity data.
    let url = format!(
        "https://serpapi.com{}",
        api_key
    );

    let response = client.get(url).send().await?;
    
    if !response.status().is_success() {
        return Err(format!("SerpApi Error: Status {}", response.status()).into());
    }

    let json: Value = response.json().await?;
    let mut signals = Vec::new();

    // Primary Data Path: Extracting from 'trending_searches'.
    if let Some(trends) = json["trending_searches"].as_array() {
        for trend in trends.iter().take(10) { 
            if let Some(query) = trend["query"].as_str() {
                // Assigning 88.0 momentum to captured breakout signals.
                signals.push(SeoSignal::new(query.to_string(), 88.0));
            }
        }
    }

    // Fallback Matrix: Check 'daily_searches' if real-time data is unavailable.
    if signals.is_empty() {
        if let Some(daily) = json["daily_searches"].as_array() {
            for day in daily.iter().take(1) {
                if let Some(queries) = day["searches"].as_array() {
                    for item in queries.iter().take(5) {
                        if let Some(query) = item["query"].as_str() {
                            signals.push(SeoSignal::new(query.to_string(), 82.0));
                        }
                    }
                }
            }
        }
    }

    println!("✅ [SECTOR: TRENDS] Captured {} breakout photons.", signals.len());
    Ok(signals)
}
