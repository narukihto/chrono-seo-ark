// agent/src/protocols/news_provider.rs

//! SECTOR: NEWS PULSE (NEWSAPI INTEGRATION)
//! Captures high-impact global events and geopolitical sentiment 
//! to feed the Penta-V stability engine.

use crate::protocols::SeoSignal;
use reqwest;
use serde_json::Value;

/// Fetches trending news topics related to high-volatility sectors.
/// Updated with UTF-8 safe slicing to prevent char-boundary panics.
pub async fn fetch(
    client: &reqwest::Client, 
    api_key: &str
) -> Result<Vec<SeoSignal>, Box<dyn std::error::Error + Send + Sync>> {
    
    // Return empty set if in Simulation Mode or key is missing
    if api_key.is_empty() || api_key == "SIM_MODE" {
        return Ok(vec![]);
    }

    println!("📰 [SECTOR: NEWS] Ingesting global headlines and impact factors...");

    // Target: Global crypto, blockchain, and tech volatility news
    let url = format!(
        "https://newsapi.org/v2/everything?q=crypto+OR+blockchain+OR+bitcoin&sortBy=publishedAt&pageSize=15&apiKey={}",
        api_key
    );

    let response = client.get(url)
        .header("User-Agent", "Ark-Systems-Agent/1.0")
        .send().await?;

    if !response.status().is_success() {
        return Err(format!("NewsAPI Response Error: Status {}", response.status()).into());
    }

    let json: Value = response.json().await?;
    let mut signals = Vec::new();

    if let Some(articles) = json["articles"].as_array() {
        for article in articles {
            if let Some(title) = article["title"].as_str() {
                
                // --- ARCHITECT FIX: UTF-8 Safe Slicing ---
                // We use .chars() to ensure we count characters, not bytes.
                // This prevents 'not a char boundary' panics on international headlines.
                let char_count = title.chars().count();
                let clean_title = if char_count > 65 {
                    let truncated: String = title.chars().take(62).collect();
                    format!("{}...", truncated)
                } else {
                    title.to_string()
                };

                // News signals are high-impact photons (Momentum: 88.0)
                signals.push(SeoSignal::new(clean_title, 88.0));
            }
        }
    }

    println!("✅ [SECTOR: NEWS] Successfully extracted {} headline photons.", signals.len());
    Ok(signals)
}
