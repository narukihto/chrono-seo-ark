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

    // Using the Google Trends engine via SerpApi
    // Note: We focus on 'trending_queries' to capture immediate momentum.
    let url = format!(
        "https://serpapi.com/search.json?engine=google_trends&api_key={}&data_type=trending_queries",
        api_key
    );

    let response = client.get(url).send().await?;
    
    if !response.status().is_success() {
        return Err(format!("SerpApi Error: Status {}", response.status()).into());
    }

    let json: Value = response.json().await?;
    let mut signals = Vec::new();

    // Parsing SerpApi's specific structure for trending queries
    if let Some(trends) = json["interest_over_time"]["trending_queries"].as_array() {
        for trend in trends.iter().take(8) { // Capture top 8 high-momentum signals
            if let Some(query) = trend["query"].as_str() {
                // In Google Trends, 'Breakout' signals are assigned a baseline high-momentum of 88.0.
                signals.push(SeoSignal::new(query.to_string(), 88.0));
            }
        }
    }

    // Fallback: If 'trending_queries' is empty, check for 'related_queries'
    if signals.is_empty() {
        if let Some(related) = json["related_queries"]["rising"].as_array() {
            for item in related.iter().take(5) {
                if let Some(query) = item["query"].as_str() {
                    signals.push(SeoSignal::new(query.to_string(), 82.0));
                }
            }
        }
    }

    println!("✅ [SECTOR: TRENDS] Captured {} breakout photons.", signals.len());
    Ok(signals)
}
