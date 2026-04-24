// agent/src/protocols/gecko_provider.rs

//! SECTOR: MARKET PULSE (COINGECKO)
//! Ingesting real-time market sentiment and trending crypto assets.

use crate::protocols::SeoSignal;
use reqwest;
use serde_json::Value;
use std::error::Error;

/// Captures top trending coins and market-driven keywords from CoinGecko.
pub async fn fetch(client: &reqwest::Client, api_key: &str) -> Result<Vec<SeoSignal>, Box<dyn Error>> {
    if api_key.is_empty() || api_key == "SIM_MODE" {
        // CoinGecko allows some public calls, but we respect the Sovereign Key if provided.
        return Ok(vec![]);
    }

    println!("📊 [SECTOR: MARKET] Scanning CoinGecko trending matrix...");

    // We target the /search/trending endpoint for the highest SEO relevance.
    let url = "https://api.coingecko.com/api/v3/search/trending";
    
    let mut request = client.get(url);
    
    // If a Pro/Demo API key is provided, we inject it into the headers.
    if !api_key.is_empty() {
        request = request.header("x-cg-demo-api-key", api_key);
    }

    let response = request.send().await?;
    
    if !response.status().is_success() {
        return Err(format!("CoinGecko Error: Status {}", response.status()).into());
    }

    let json: Value = response.json().await?;
    let mut signals = Vec::new();

    // Ingesting trending coins
    if let Some(coins) = json["coins"].as_array() {
        for coin in coins.iter().take(7) { // Capture top 7 trending assets
            if let Some(item) = coin["item"].as_object() {
                let name = item.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let symbol = item.get("symbol").and_then(|v| v.as_str()).unwrap_or("");
                
                // Construct a composite signal (Name + Symbol) for maximum SEO coverage
                let keyword = format!("{} {}", name, symbol);
                
                // Market trends are assigned a volatile momentum of 85.0
                signals.push(SeoSignal::new(keyword, 85.0));
            }
        }
    }

    // Ingesting trending NFT/Categories if available
    if let Some(categories) = json["categories"].as_array() {
        for cat in categories.iter().take(3) {
            if let Some(name) = cat["name"].as_str() {
                signals.push(SeoSignal::new(name.to_string(), 80.0));
            }
        }
    }

    println!("✅ [SECTOR: MARKET] Extracted {} asset-driven photons.", signals.len());
    Ok(signals)
}
