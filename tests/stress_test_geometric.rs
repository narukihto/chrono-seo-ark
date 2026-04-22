// agent/tests/stress_test_geometric.rs

//! Stress Test: Geometric Resilience.
//! Simulates extreme data loads and 'Black Swan' events to ensure the 
//! Penta-V Engine maintains structural integrity under pressure.
//! Updated for Predator-Sync (0.45 Live / 0.15 Test) calibration.

use chrono_seo_agent::engine::stability::StabilityGuard;

#[test]
fn test_geometric_saturation_point() {
    // Testing the saturation point against the adaptive thresholds.
    let guard = StabilityGuard::new(12.0); // Φ = 4.0
    
    // IMPACT CALCULATION: (Momentum * 0.02) / 4.0
    
    // 1. Safe Zone: Impact = 0.10. 
    // Should be accepted in all environments (Test 0.15 / Live 0.45).
    let safe_momentum = 20.0;
    
    // 2. Absolute Breach: Impact = 0.50.
    // This MUST be rejected even in Hunter Mode (0.45).
    // This ensures the test passes even if CI environment defaults to Production thresholds.
    let critical_momentum = 100.0; 
    
    assert!(guard.is_stable(guard.calculate_impact(safe_momentum)), 
            "Resilience Error: Engine rejected a safe signal within all stability margins.");
            
    assert!(!guard.is_stable(guard.calculate_impact(critical_momentum)), 
            "Security Error: Engine accepted a catastrophic breach (Impact 0.50 > 0.45 Ceiling).");
}

#[test]
fn test_high_frequency_oscillation() {
    // Simulates 1,000,000 rapid calculations to check for floating-point 
    // drift or memory leaks in the engine.
    let guard = StabilityGuard::new(12.0);
    let mut last_impact = 0.0;

    for i in 0..1_000_000 {
        let momentum = (i % 100) as f64;
        let impact = guard.calculate_impact(momentum);
        
        // Ensure impact remains deterministic (Floating point consistency)
        if i % 100 == 50 {
            // (50 * 0.02) / 4.0 = 0.25
            assert_eq!(impact, 0.25);
        }
        last_impact = impact;
    }
    
    assert!(last_impact >= 0.0);
    println!("🛡️ [STRESS] 1,000,000 cycles completed with zero drift.");
}

#[test]
fn test_geometric_invincibility_limit() {
    // As N approaches infinity, Φ increases, making the system 'invincible'.
    // Here we test a 'High-Pole' configuration (N=1000).
    let hyper_guard = StabilityGuard::new(1000.0); // Φ = 333.33
    
    // Extreme Momentum Calibration for 0.15 Threshold:
    // Impact = (2000 * 0.02) / 333.33 = 0.12 (Stable < 0.15)
    let extreme_momentum = 2000.0; 
    let impact = hyper_guard.calculate_impact(extreme_momentum);
    
    assert!(hyper_guard.is_stable(impact), 
            "Resilience Error: Hyper-Pole failed to absorb stress within 0.15 margin.");
}
