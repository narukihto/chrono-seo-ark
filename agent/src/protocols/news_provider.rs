// agent/src/protocols/news_provider.rs

//! SECTOR: NEWS PULSE (NEWSAPI INTEGRATION)
//! Captures high-impact global events and geopolitical sentiment 
//! to feed the Penta-V stability engine.

use crate::protocols::SeoSignal;
use reqwest;
use serde_json::Value;
use std::error::Error;

/// Fetches trending news topics related to high-volatility sectors.
pub async fn fetch(client: &reqwest::Client, api_key: &str) -> Result<Vec<SeoSignal>, Box<dyn Error>> {
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
                // Formatting title to fit SEO keyword constraints
                let clean_title = if title.len() > 65 {
                    format!("{}...", &title[..62])
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
