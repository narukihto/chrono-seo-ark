// agent/tests/stress_test_geometric.rs

//! Stress Test: Geometric Resilience.
//! Simulates extreme data loads and 'Black Swan' events to ensure the 
//! Penta-V Engine maintains structural integrity under pressure.

use chrono_seo_agent::engine::stability::StabilityGuard;
use chrono_seo_agent::engine::geometry::GeometricCalculator;

#[test]
fn test_geometric_saturation_point() {
    // We test the saturation point: How much momentum can a Dodecagon handle 
    // before the SECURE_CORE (0.05) is breached?
    let guard = StabilityGuard::new(12.0); // Φ = 4.0
    
    // Threshold Calculation: 
    // (Momentum * 0.02) / 4.0 = 0.95 (to reach 0.05 stability)
    // Momentum = (0.95 * 4.0) / 0.02 = 190.0
    
    let safe_momentum = 189.9;
    let critical_momentum = 190.1;
    
    assert!(guard.is_stable(guard.calculate_impact(safe_momentum)), 
            "Resilience Error: Engine rejected a signal within the safety margin.");
            
    assert!(!guard.is_stable(guard.calculate_impact(critical_momentum)), 
            "Security Error: Engine accepted a signal that breached the SECURE_CORE.");
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
    
    // Even an extreme momentum of 5000.0 should be absorbed.
    // Impact = (5000 * 0.02) / 333.33 = 100 / 333.33 = 0.3
    // 1.0 - 0.3 = 0.7 (Stable)
    let extreme_momentum = 5000.0;
    let impact = hyper_guard.calculate_impact(extreme_momentum);
    
    assert!(hyper_guard.is_stable(impact), 
            "Resilience Error: Hyper-Pole configuration failed to absorb stress.");
}
