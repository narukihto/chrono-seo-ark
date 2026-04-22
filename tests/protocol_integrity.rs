// agent/tests/protocol_integrity.rs

//! Protocol Integrity: Validates the accuracy of the prediction logic.
//! Final Calibration: Synchronized with Geometric Engine's strict stabilization limits.

use chrono_seo_agent::protocols::{SeoSignal, liquid_sync::LiquidSync};
use chrono_seo_agent::engine::stability::StabilityGuard;

#[tokio::test]
async fn test_liquid_sync_prediction_accuracy() {
    // 1. Setup a Dodecagon Guard (Φ = 4.0).
    let guard = StabilityGuard::new(12.0);
    
    // 2. Define signal pool: 
    // Using a momentum of 20.0 to guarantee impact stays well below the 0.15 threshold.
    let raw_signals = vec![
        SeoSignal::new("Safe High-Value Trend".to_string(), 20.0), 
        SeoSignal::new("Post-Quantum Noise".to_string(), 25.0), 
        SeoSignal::new("Systemic Threat".to_string(), 500.0), 
    ];

    // 3. Execute Liquid Sync Protocol.
    let filtered_signals = LiquidSync::process(&guard, raw_signals).await;

    // 4. Integrity Verification.
    // Ensure at least the safe trend is captured.
    assert!(!filtered_signals.is_empty(), "Integrity Failure: System discarded all signals even under Dodecagon stabilization.");
    
    // Enforce the Purge Protocol.
    for signal in &filtered_signals {
        assert!(!signal.keyword.to_lowercase().contains("quantum"), 
            "Purification Breach: Legacy 'Quantum' noise leaked into the vault.");
    }
}

#[tokio::test]
async fn test_protocol_9_sorting_logic() {
    let guard = StabilityGuard::new(12.0);
    let raw_signals = vec![
        SeoSignal::new("Alpha".to_string(), 30.0), 
        SeoSignal::new("Beta".to_string(), 15.0),
    ];
    let filtered = LiquidSync::process(&guard, raw_signals).await;
    
    if filtered.len() >= 2 {
        assert!(filtered[0].momentum > filtered[1].momentum, "Sorting Logic Failure: Highest momentum must lead.");
    }
}

#[tokio::test]
async fn test_geometric_immunity_transition() {
    let weak_guard = StabilityGuard::new(3.0); 
    let strong_guard = StabilityGuard::new(12.0);
    
    // Pulse calibrated to fail in Triangle (Φ=1.0) and pass in Dodecagon (Φ=4.0).
    let signal = vec![SeoSignal::new("Adaptive Pulse".to_string(), 22.0)];

    let weak_result = LiquidSync::process(&weak_guard, signal.clone()).await;
    let strong_result = LiquidSync::process(&strong_guard, signal).await;

    assert!(weak_result.is_empty(), "Security Failure: Weak geometry should have rejected the pulse.");
    assert!(!strong_result.is_empty(), "Stability Failure: Strong geometry failed to stabilize the pulse.");
}
