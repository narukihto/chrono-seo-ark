// agent/src/protocols/serp_provider.rs

//! SECTOR: GOOGLE TRENDS (SERPAPI)
//! Capture high-velocity search queries before they saturate the global market
//! and feed them into the Penta-V Geometric Stability engine.

use crate::protocols::SeoSignal;
use reqwest;
use serde_json::Value;

/// Fetches real-time trending queries from Google Trends via SerpApi.
/// Updated with Send + Sync bounds for high-frequency parallel execution.
pub async fn fetch(
    client: &reqwest::Client, 
    api_key: &str
) -> Result<Vec<SeoSignal>, Box<dyn std::error::Error + Send + Sync>> {
    
    if api_key.is_empty() || api_key == "SIM_MODE" {
        return Ok(vec![]);
    }

    println!("🔍 [SECTOR: TRENDS] Probing Google Trends for breakout signals...");

    // Critical Fix: Added the missing /search.json and engine parameters.
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

    // Mapping 'trending_searches' from SerpApi payload.
    if let Some(trends) = json["trending_searches"].as_array() {
        for trend in trends.iter().take(8) { 
            if let Some(query) = trend["query"].as_str() {
                // Assigning baseline high-momentum (88.0) to breakout signals.
                signals.push(SeoSignal::new(query.to_string(), 88.0));
            }
        }
    }

    // Fallback: Accessing 'daily_searches' if immediate trending data is empty.
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
