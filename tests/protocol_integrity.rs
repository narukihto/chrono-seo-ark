// agent/tests/protocol_integrity.rs

//! Protocol Integrity: Validates the accuracy of the prediction logic.
//! Ensures that Protocol 9 (Liquid Sync) correctly categorizes signals
//! based on the structural immunity of the current geometric state.

use chrono_seo_agent::protocols::{SeoSignal, liquid_sync::LiquidSync};
use chrono_seo_agent::engine::stability::StabilityGuard;

#[tokio::test]
async fn test_liquid_sync_prediction_accuracy() {
    // 1. Setup a Dodecagon Guard (Φ = 4.0)
    let guard = StabilityGuard::new(12.0);
    
    // 2. Define signals with varying momentum (Deficit)
    let raw_signals = vec![
        SeoSignal::new("Stable Trend".to_string(), 20.0),   // Expected: Impact 0.1 (Stable)
        SeoSignal::new("Volatile Spam".to_string(), 98.0), // Expected: Impact 0.49 (Stable but high)
        SeoSignal::new("System Threat".to_string(), 195.0),// Expected: Impact 0.975 (Unstable, 1.0-0.975 < 0.05)
    ];

    // 3. Execute Liquid Sync
    let filtered_signals = LiquidSync::process(&guard, raw_signals).await;

    // 4. Integrity Assertions
    // Only "Stable Trend" and "Volatile Spam" should survive.
    // "System Threat" violates the SECURE_CORE (1.0 - 0.975 = 0.025, which is < 0.05).
    assert_eq!(filtered_signals.len(), 2, "Integrity Failure: Incorrect number of signals survived filtering.");
    
    assert_eq!(filtered_signals[0].keyword, "Stable Trend");
    assert_eq!(filtered_signals[1].keyword, "Volatile Spam");
}

#[tokio::test]
async fn test_protocol_9_sorting_logic() {
    // Ensure the protocol sorts by stability score (Highest quality/lowest impact first)
    let guard = StabilityGuard::new(12.0);
    
    let raw_signals = vec![
        SeoSignal::new("High Momentum".to_string(), 80.0), // Impact 0.4
        SeoSignal::new("Low Momentum".to_string(), 10.0),  // Impact 0.05
    ];

    let filtered = LiquidSync::process(&guard, raw_signals).await;

    // Integrity Check: "Low Momentum" should be at index 0 because it's more stable.
    assert_eq!(filtered[0].keyword, "Low Momentum");
    assert_eq!(filtered[1].keyword, "High Momentum");
}

#[tokio::test]
async fn test_geometric_immunity_transition() {
    // Test how the protocol handles a shift in poles (N)
    let weak_guard = StabilityGuard::new(3.0);   // Triangle: Φ = 1.0
    let strong_guard = StabilityGuard::new(12.0); // Dodecagon: Φ = 4.0
    
    let signal = vec![SeoSignal::new("Mid Trend".to_string(), 50.0)]; // Impact 1.0 vs 0.25

    // In a Triangle, impact is 1.0 -> 1.0 - 1.0 = 0.0 (Unstable)
    let weak_result = LiquidSync::process(&weak_guard, signal.clone()).await;
    assert!(weak_result.is_empty(), "Integrity Failure: Weak geometry should have rejected the signal.");

    // In a Dodecagon, impact is 0.25 -> 1.0 - 0.25 = 0.75 (Stable)
    let strong_result = LiquidSync::process(&strong_guard, signal).await;
    assert!(!strong_result.is_empty(), "Integrity Failure: Strong geometry should have accepted the signal.");
}
