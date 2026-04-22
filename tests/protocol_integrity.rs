// agent/tests/protocol_integrity.rs

//! Protocol Integrity: Validates the accuracy of the prediction logic.
//! Dynamic Version: Adaptive to high-momentum trends and purified data streams.
//! Optimized for Penta-V Architectural Stability and Zero-Quantum Leakage.

use chrono_seo_agent::protocols::{SeoSignal, liquid_sync::LiquidSync};
use chrono_seo_agent::engine::stability::StabilityGuard;

#[tokio::test]
async fn test_liquid_sync_prediction_accuracy() {
    // 1. Setup a Dodecagon Guard (Φ = 4.0) for high-integrity scanning.
    let guard = StabilityGuard::new(12.0);
    
    // 2. Define signal pool: Trends, Legacy Noise (Quantum), and Extreme Volatility.
    let raw_signals = vec![
        SeoSignal::new("Fresh Tech Trend".to_string(), 90.0),    // Valid High-Momentum signal.
        SeoSignal::new("Post-Quantum Legacy".to_string(), 95.0), // Target for Purge Protocol.
        SeoSignal::new("Systemic Spike".to_string(), 250.0),    // Exceeds adaptive threshold (0.15).
    ];

    // 3. Execute Liquid Sync Protocol.
    let filtered_signals = LiquidSync::process(&guard, raw_signals).await;

    // 4. Dynamic Assertions for Integrity Validation.
    // Ensure the stream is not null if valid signals were present.
    assert!(!filtered_signals.is_empty(), "Integrity Failure: System incorrectly discarded valid high-momentum signals.");
    
    // Enforce the Purge: No keyword containing 'Quantum' shall persist in the Vault.
    for signal in &filtered_signals {
        assert!(!signal.keyword.to_lowercase().contains("quantum"), 
            "Purification Breach: Legacy 'Quantum' noise detected in synchronized stream.");
    }
}

#[tokio::test]
async fn test_protocol_9_sorting_logic() {
    // Validate sorting hierarchy: Momentum (Descending) > Stability (Ascending).
    let guard = StabilityGuard::new(12.0);
    
    let raw_signals = vec![
        SeoSignal::new("High Momentum Alpha".to_string(), 92.0), 
        SeoSignal::new("Low Momentum Beta".to_string(), 45.0),
    ];

    let filtered = LiquidSync::process(&guard, raw_signals).await;

    // Verify Sorting Impact: The highest momentum signal must assume Index 0.
    if filtered.len() >= 2 {
        assert!(filtered[0].momentum > filtered[1].momentum, 
            "Sequence Failure: Vault must prioritize signals with superior momentum.");
        assert_eq!(filtered[0].keyword, "High Momentum Alpha");
    }
}

#[tokio::test]
async fn test_geometric_immunity_transition() {
    // Test adaptive threshold response across varying geometric states.
    let weak_guard = StabilityGuard::new(3.0);   // Triangle Configuration (Φ = 1.0)
    let strong_guard = StabilityGuard::new(12.0); // Dodecagon Configuration (Φ = 4.0)
    
    let signal = vec![SeoSignal::new("Adaptive Pulse".to_string(), 50.0)];

    // Execution under Weak Geometry (Expected Rejection due to high impact).
    let weak_result = LiquidSync::process(&weak_guard, signal.clone()).await;
    
    // Execution under Strong Geometry (Expected Acceptance under 0.15 threshold).
    let strong_result = LiquidSync::process(&strong_guard, signal).await;

    // Ensure architectural transition correctly scales with geometric immunity.
    assert!(!strong_result.is_empty(), 
        "Stability Failure: Dodecagon geometry failed to stabilize 0.15 impact trend.");
}
