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

    /// Integration: Gemini AI Capture & Professional HTML Deployment
    pub fn deploy_to_gemini(&self) {
        let api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY env var not set");
        let client = Client::new();

        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}", api_key);
        
        // تحسين الـ Prompt لإنتاج محتوى احترافي مهيكل
        let prompt = format!(
            "Write a high-authority, professional SEO article for the keyword: '{}'. \
            Include an engaging introduction, structured body paragraphs with insights, and a strategic conclusion. \
            Output the content in plain text with clear paragraphs.", 
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

        // استخراج النص وتأمينه
        let raw_ai_content = response["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("System is recalibrating content stream. Please wait for the next pulse.");

        // تحويل النص إلى HTML محترم (تحويل الفقرات)
        let formatted_content = raw_ai_content
            .split('\n')
            .filter(|line| !line.trim().is_empty())
            .map(|line| format!("<p>{}</p>", line))
            .collect::<Vec<String>>()
            .join("\n");

        // --- إدارة المسارات والرفع ---
        let template_path = "agent/template.html";
        let output_path = "agent/index.html";

        match fs::read_to_string(template_path) {
            Ok(template) => {
                let processed_html = template
                    .replace("{{WORD}}", &self.keyword)
                    .replace("{{CONTENT}}", &formatted_content);

                fs::write(output_path, processed_html).expect("Failed to write index.html");
                println!("🚀 [PULSE] High-Impact SEO Index deployed at: {}", output_path);
            },
            Err(_) => {
                if let Ok(template) = fs::read_to_string("template.html") {
                    let processed_html = template
                        .replace("{{WORD}}", &self.keyword)
                        .replace("{{CONTENT}}", &formatted_content);
                    fs::write("index.html", processed_html).expect("Failed to write locally");
                    println!("🚀 [LOCAL] SEO Index updated in current directory.");
                } else {
                    println!("❌ [CRITICAL] Architectural Template Missing.");
                }
            }
        }
    }
}
