// agent/src/protocols/mod.rs

//! Ark Protocols Module: The implementation of high-speed data capture and deployment.
//! 
//! Protocol 9: Liquid Synchrony (Predictive Analysis)
//! Protocol 15: Cherenkov's Lens (High-Frequency Multi-Sector Scanning)
//! Protocol 19: Temporal Projectile (Deployment to Vault)

// --- CORE LOGIC MODULES ---
pub mod liquid_sync;
pub mod cherenkov_lens;
pub mod temporal_projectile;

// --- SECTOR PROVIDER MODULES (MULTI-SCAN ARCHITECTURE) ---
pub mod serp_provider;
pub mod gecko_provider;
pub mod crypto_pulse;
pub mod news_provider;

use serde::{Serialize, Deserialize};
use std::fs;
use reqwest::blocking::Client;

/// The fundamental SEO signal structure used across all Ark protocols.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SeoSignal {
    /// The captured keyword or trend signature.
    pub keyword: String,
    
    /// The momentum factor (Deficit) derived from frequency and search velocity.
    pub momentum: f64,
    
    /// The final stability impact calculated by the Penta-V Engine.
    pub stability_score: Option<f64>,
}

impl SeoSignal {
    /// Creates a new raw signal captured by the Cherenkov Lens.
    pub fn new(keyword: String, momentum: f64) -> Self {
        Self {
            keyword,
            momentum,
            stability_score: None,
        }
    }

    /// Integration: Gemini AI Capture & HTML Deployment
    /// This function triggers the Gemini API call and updates the local index.html template.
    /// Aligned with Ark Protocol 19 for deployment purity.
    pub fn deploy_to_gemini(&self) {
        let api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY env var not set");
        let client = Client::new();

        // Gemini API Request Construction
        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}", api_key);
        
        let prompt = format!(
            "Analyze and write high-impact SEO viral content for the keyword: '{}'. Structure it for market dominance.", 
            self.keyword
        );

        let response = client.post(url)
            .json(&serde_json::json!({
                "contents": [{ "parts": [{ "text": prompt }] }]
            }))
            .send()
            .expect("Failed to connect to Gemini API")
            .json::<serde_json::Value>()
            .expect("Failed to parse Gemini response");

        let ai_content = response["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("Engine failed to retrieve content.");

        // Template Bridge: Read template.html and replace placeholders.
        // Paths are relative to the 'agent' directory execution context.
        match fs::read_to_string("template.html") {
            Ok(template) => {
                let processed_html = template
                    .replace("{{WORD}}", &self.keyword)
                    .replace("{{CONTENT}}", ai_content);

                // Export to index.html in the current agent directory.
                fs::write("index.html", processed_html).expect("Failed to write deployment index.html");
                println!("✅ [DEPLOY] SEO Index generated successfully for: {}", self.keyword);
            },
            Err(_) => {
                println!("⚠️ [WARN] template.html not found in agent root. Skipping HTML deployment.");
            }
        }
    }
}
