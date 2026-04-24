// agent/src/protocols/crypto_pulse.rs

//! SECTOR: REAL-TIME CRYPTO PULSE (FREECRYPTOAPI)
//! Captures high-frequency volatility signals and immediate market entries 
//! to feed the Penta-V stability engine.

use crate::protocols::SeoSignal;
use reqwest;
use serde_json::Value;

/// Fetches instantaneous market signals from FreeCryptoAPI.
/// Updated with Send + Sync bounds for high-frequency parallel execution.
pub async fn fetch(
    client: &reqwest::Client, 
    api_key: &str
) -> Result<Vec<SeoSignal>, Box<dyn std::error::Error + Send + Sync>> {
    
    // Return empty set if in Simulation Mode or key is missing
    if api_key.is_empty() || api_key == "SIM_MODE" {
        return Ok(vec![]);
    }

    println!("⚡ [SECTOR: CRYPTO-PULSE] Detecting high-frequency market entry photons...");

    // Using the /cryptos endpoint to get the latest active assets
    let url = format!("https://freecryptoapi.com/api/v1/cryptos?api_key={}", api_key);

    let response = client.get(url).send().await?;
    
    if !response.status().is_success() {
        return Err(format!("FreeCryptoAPI Error: Status {}", response.status()).into());
    }

    let json: Value = response.json().await?;
    let mut signals = Vec::new();

    // Ingesting the 'data' array which contains the active assets
    if let Some(assets) = json["data"].as_array() {
        for asset in assets.iter().take(15) { // Capture the top 15 high-pulse assets
            if let Some(symbol) = asset["symbol"].as_str() {
                let name = asset["name"].as_str().unwrap_or(symbol);
                
                // Constructing the signal with an emphasis on "News" and "Price" 
                // for broad SEO reach in the finance sector.
                let keyword = format!("{} Price Update", name);
                
                // Crypto Pulse signals are assigned a momentum of 84.0
                signals.push(SeoSignal::new(keyword, 84.0));
                
                // Also capture the pure symbol for technical SEO relevance
                signals.push(SeoSignal::new(format!("{} Crypto Signal", symbol), 81.0));
            }
        }
    }

    println!("✅ [SECTOR: CRYPTO-PULSE] Captured {} high-frequency photons.", signals.len());
    Ok(signals)
}
