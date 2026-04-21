// agent/benches/engine_performance.rs

//! Engine Performance: Nanosecond-level profiling.
//! Utilizes Criterion.rs to perform statistical benchmarking of the 
//! Penta-V core calculations and the Stability Guard enforcement.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use chrono_seo_agent::engine::stability::StabilityGuard;
use chrono_seo_agent::engine::geometry::GeometricCalculator;

/// Benchmark the raw geometric immunity calculation (Φ).
fn bench_geometry_phi(c: &mut Criterion) {
    let poles = 12.0; // Dodecagon

    c.bench_function("Penta-V: Φ Calculation (Dodecagon)", |b| {
        b.iter(|| {
            // black_box prevents the compiler from optimizing away the call
            GeometricCalculator::calculate_immunity(black_box(poles))
        })
    });
}

/// Benchmark the full stability check cycle.
fn bench_stability_check(c: &mut Criterion) {
    let guard = StabilityGuard::new(12.0);
    let momentum = 150.5;

    c.bench_function("Penta-V: Stability Check Cycle", |b| {
        b.iter(|| {
            let impact = guard.calculate_impact(black_box(momentum));
            guard.is_stable(black_box(impact))
        })
    });
}

/// Benchmark the impact of increasing poles on performance.
/// (Validating that N->∞ doesn't significantly increase CPU cost).
fn bench_scaling_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("Penta-V: Scaling");
    
    for n in [3.0, 12.0, 100.0, 1000.0].iter() {
        group.bench_with_input(format!("Poles-{}", n), n, |b, &n| {
            b.iter(|| GeometricCalculator::calculate_immunity(black_box(n)))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_geometry_phi, bench_stability_check, bench_scaling_performance);
criterion_main!(benches);
