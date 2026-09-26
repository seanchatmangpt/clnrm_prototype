//! Benchmarks for the PR #17 dev-dependency contract surface: lockfile admission
//! parse cost and the deterministic port allocator hardened in the same change.
//! Numbers and regression bounds are recorded in
//! docs/bench/dev-deps-contract-v26.9.26.json; the committed bound is also
//! enforced by `guard_latency_regression_bound` in tests/dev_deps_contract.rs.

use clnrm::determinism::DeterministicPortAllocator;
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn lock_parse(c: &mut Criterion) {
    let text = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.lock"))
        .expect("Cargo.lock readable");
    c.bench_function("cargo_lock_toml_parse", |b| {
        b.iter(|| {
            let t: toml::Table = toml::from_str(black_box(&text)).expect("parses");
            black_box(t.get("package").and_then(|p| p.as_array()).map(|a| a.len()))
        })
    });
}

fn port_alloc(c: &mut Criterion) {
    c.bench_function("port_alloc_release_1000", |b| {
        b.iter(|| {
            let mut a = DeterministicPortAllocator::new(black_box(20000));
            for _ in 0..1000 {
                let p = a.allocate_port().expect("port");
                black_box(p);
            }
            for p in 20000u16..21000 {
                a.release_port(p).expect("release");
            }
        })
    });
}

criterion_group!(benches, lock_parse, port_alloc);
criterion_main!(benches);
