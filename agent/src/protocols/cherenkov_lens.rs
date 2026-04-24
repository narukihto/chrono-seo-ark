 // agent/src/protocols/cherenkov_lens.rs

//! Protocol 15: Cherenkov's Lens (Live Global Projection).
//! High-frequency capture mechanism designed to detect emerging data 'glow'
//! across the actual web by replacing static simulation buffers with live API streams.

use crate::protocols::SeoSignal;
use reqwest;
use serde_json::Value;
use std::error::Error;
use futures::future::{join_all, BoxFuture, FutureExt}; // Added FutureExt for .boxed()

pub struct CherenkovLens;

impl CherenkovLens {
    /// Executes a parallel scan across live global data nodes.
    /// Uses BoxFuture to unify distinct async function types into a single collection.
    pub async fn scan(api_key: &str) -> Result<Vec<SeoSignal>, Box<dyn Error>> {
        println!("📡 [PROTOCOL 15] Initializing Lens. Mode: LIVE_GLOBAL_SCAN");

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .user_agent("Ark-Sovereign-Agent/1.0")
            .build()?;

        // Unified vector for boxed futures to resolve E0308 type mismatch
        let mut tasks: Vec<BoxFuture<Result<Vec<SeoSignal>, Box<dyn Error>>>> = Vec::new();

        // 1. Source: GitHub API (Cryptographic trends)
        tasks.push(Self::fetch_github_trends(&client).boxed());

        // 2. Source: NewsAPI (Global tech trends)
        if !api_key.starts_with("ARK_SIM") {
            tasks.push(Self::fetch_global_news(&client, api_key).boxed());
        }

        // Execute concurrent requests to maintain signal velocity
        let results = join_all(tasks).await;
        let mut captured_signals = Vec::new();

        for res in results {
            if let Ok(signals) = res {
                captured_signals.extend(signals);
            }
        }

        // Emergency Fallback if all sensors report zero photons
        if captured_signals.is_empty() {
            println!("⚠️ [ARK] No live photons detected. Falling back to emergency buffer.");
            captured_signals.push(SeoSignal::new("Post-Quantum Transition".to_string(), 90.0));
        }

        // Data Sanitization: Sorting and deduplication
        captured_signals.sort_by(|a, b| a.keyword.cmp(&b.keyword));
        captured_signals.dedup_by(|a, b| a.keyword == b.keyword);

        println!("💎 [ARK] Cherenkov Lens captured {} unique live signals.", captured_signals.len());
        Ok(captured_signals)
    }

    /// Fetches trending repositories from GitHub.
    async fn fetch_github_trends(client: &reqwest::Client) -> Result<Vec<SeoSignal>, Box<dyn Error>> {
        let url = "https://api.github.com/search/repositories?q=topic:cryptography&sort=updated";
        let resp = client.get(url).send().await?.json::<Value>().await?;
        
        let mut signals = Vec::new();
        if let Some(items) = resp["items"].as_array() {
            for item in items.iter().take(5) {
                if let Some(name) = item["name"].as_str() {
                    let clean_name = name.replace("-", " ").replace("_", " ");
                    signals.push(SeoSignal::new(clean_name, 85.0));
                }
            }
        }
        Ok(signals)
    }

    /// Captures global trends via NewsAPI.
    async fn fetch_global_news(client: &reqwest::Client, key: &str) -> Result<Vec<SeoSignal>, Box<dyn Error>> {
        let url = format!(
            "https://newsapi.org/v2/everything?q=cryptography+OR+quantum&sortBy=publishedAt&apiKey={}", 
            key
        );
        let resp = client.get(url).send().await?.json::<Value>().await?;

        let mut signals = Vec::new();
        if let Some(articles) = resp["articles"].as_array() {
            for article in articles.iter().take(3) {
                if let Some(title) = article["title"].as_str() {
                    let short_title: String = title.split_whitespace().take(3).collect::<Vec<&str>>().join(" ");
                    signals.push(SeoSignal::new(short_title, 78.0));
                }
            }
        }
        Ok(signals)
    }
}
