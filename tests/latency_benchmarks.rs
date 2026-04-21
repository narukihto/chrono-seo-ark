// agent/tests/latency_benchmarks.rs

//! Latency Benchmarks: Performance verification for the Ark System.
//! Ensures that signal processing stays within the microsecond/millisecond thresholds
//! required for real-time SEO dominance.

use chrono_seo_agent::engine::stability::StabilityGuard;
use chrono_seo_agent::engine::geometry::GeometricCalculator;
use std::time::{Instant, Duration};

#[test]
fn test_geometric_calculation_latency() {
    // Target: Sub-microsecond execution for mathematical operations.
    let poles = 12.0; // Dodecagon configuration
    let momentum = 85.5;
    
    let start = Instant::now();
    
    // Core Penta-V math cycle
    // Using '_' prefix to silence unused variable warnings while maintaining the bench logic
    let immunity = GeometricCalculator::calculate_immunity(poles);
    let _impact = GeometricCalculator::calculate_impact(momentum, immunity);
    
    let duration = start.elapsed();
    
    println!("⏱️ [BENCH] Geometric Math Cycle: {:?}", duration);
    
    // Enforcement: Math should be near-instant (less than 100 microseconds)
    assert!(duration < Duration::from_micros(100), "Geometric calculation is too slow!");
}

#[test]
fn test_mass_signal_filtering_latency() {
    // Target: Processing 10,000 signals in under 5ms.
    // This simulates an extreme 'Cherenkov Flash' where thousands of trends are captured.
    
    let guard = StabilityGuard::new(12.0);
    let mut mock_signals = Vec::with_capacity(10_000);
    
    for i in 0..10_000 {
        mock_signals.push(i as f64);
    }

    let start = Instant::now();

    // Simulate the Liquid Sync loop
    let mut stable_count = 0;
    for momentum in mock_signals {
        let impact = guard.calculate_impact(momentum);
        if guard.is_stable(impact) {
            stable_count += 1;
        }
    }

    let duration = start.elapsed();
    
    println!("⏱️ [BENCH] 10,000 Signal Filter Duration: {:?}", duration);
    println!("🛡️ [BENCH] Stable Signals Found: {}", stable_count);

    // Enforcement: Even with 10k signals, we must stay under 5ms to keep the Pulse efficient.
    assert!(duration < Duration::from_millis(5), "Filtering latency exceeds Ark limits!");
}

#[test]
fn test_secure_core_resilience_under_load() {
    // Target: Ensuring the Guard never falters even when bombarded with 'Infinite' momentum.
    let guard = StabilityGuard::new(12.0);
    
    let extreme_momentum = f64::MAX;
    let impact = guard.calculate_impact(extreme_momentum);
    
    let start = Instant::now();
    let result = guard.is_stable(impact);
    let duration = start.elapsed();

    // The Guard must reject extreme momentum instantly.
    assert!(!result, "Security Breach: Extreme momentum was classified as stable!");
    assert!(duration < Duration::from_micros(50), "Security check latency too high!");
}
