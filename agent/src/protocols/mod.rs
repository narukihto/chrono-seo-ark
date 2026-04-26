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
    pub keyword: String,
    pub momentum: f64,
    pub stability_score: Option<f64>,
}

impl SeoSignal {
    pub fn new(keyword: String, momentum: f64) -> Self {
        Self {
            keyword,
            momentum,
            stability_score: None,
        }
    }

    /// Integration: Gemini AI Capture & Professional HTML Deployment.
    /// Final Version: Features enhanced text purification and multi-path resilience.
    pub fn deploy_to_gemini(&self) {
        let api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY env var not set");
        let client = Client::new();

        // Using Gemini 1.5 Flash for optimal latency within the 6-minute pulse window.
        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}", api_key);
        
        let prompt = format!(
            "Act as a DeepTech Lead Architect. Write a high-authority, professional SEO report for: '{}'. \
            Focus on technical implications, strategic market impact, and future outlook. \
            Output in plain text with clear paragraph breaks. STRICTLY NO markdown, NO stars, NO bolding.", 
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

        // Safely extract the content or fallback to recalibration message.
        let raw_ai_content = response["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("System is recalibrating content stream. Please wait for the next pulse.");

        // --- CONTENT PURIFICATION PROTOCOL ---
        // Force-remove markdown artifacts (stars, hashes) to maintain HTML integrity.
        let purified_text = raw_ai_content.replace("*", "").replace("#", "");

        // Semantic HTML Formatting: Transform plain text blocks into valid paragraph tags.
        let formatted_content = purified_text
            .split('\n')
            .filter(|line| line.trim().len() > 20) // Filter out noise and fragments.
            .map(|line| format!("<p>{}</p>", line.trim()))
            .collect::<Vec<String>>()
            .join("\n");

        // Path Strategy: Ensuring alignment between local dev and GitHub Actions CI/CD.
        let template_path = "agent/template.html";
        let output_path = "agent/index.html";

        // Deployment Logic: Atomic string replacement for WORD and CONTENT tokens.
        let final_processed_html = match fs::read_to_string(template_path) {
            Ok(template) => {
                template
                    .replace("{{WORD}}", &self.keyword)
                    .replace("{{CONTENT}}", &formatted_content)
            },
            Err(_) => {
                // Fallback mechanism for different execution contexts.
                let local_template = fs::read_to_string("template.html")
                    .expect("❌ [CRITICAL] Architectural Template Missing in all paths.");
                local_template
                    .replace("{{WORD}}", &self.keyword)
                    .replace("{{CONTENT}}", &formatted_content)
            }
        };

        // Persistence Layer: Write to relative agent path, fallback to root for GH-Pages.
        if let Err(_) = fs::write(output_path, &final_processed_html) {
             fs::write("index.html", &final_processed_html).expect("Failed to write SEO Index to root");
        }

        println!("🚀 [PULSE] SEO Intelligence Purified and Deployed: [{}]", self.keyword);
    }
}
