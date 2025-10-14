# Performance Benchmarks - clnrm Testing Framework

**Status**: Production Baseline Established
**Last Updated**: 2025-10-13
**Framework Version**: v0.1.0

## Overview

This document establishes production performance baselines for the clnrm (cleanroom) testing framework. Following the 80/20 principle, it focuses on the 20% of metrics that matter for 80% of production performance.

## Executive Summary

### Performance Targets (80/20 Principle)

Based on README.md claims and production requirements:

| Metric Category | Target | Production Impact |
|-----------------|--------|-------------------|
| **Singleton Containers** | 80% faster than regular containers | Critical |
| **Container Startup** | < 2s | High |
| **Test Execution** | < 100ms (simple tests) | Critical |
| **Memory Usage** | < 100MB (basic operations) | High |
| **Concurrent Throughput** | >= 10 tests/second | High |

### Quick Start

```bash
# Run all benchmarks
cargo bench --bench production_benchmarks

# Run specific category
cargo bench --bench production_benchmarks -- container_lifecycle
cargo bench --bench production_benchmarks -- test_execution
cargo bench --bench production_benchmarks -- memory_usage
cargo bench --bench production_benchmarks -- io_performance
cargo bench --bench production_benchmarks -- async_performance
cargo bench --bench production_benchmarks -- singleton_pattern

# Generate baseline for comparisons
cargo bench --bench production_benchmarks -- --save-baseline main

# Compare against baseline
cargo bench --bench production_benchmarks -- --baseline main
```

## Benchmark Categories

### 1. Container Lifecycle (Critical Path)

**Why it matters**: Container lifecycle performance directly impacts test startup time and overall test suite execution speed.

#### Key Benchmarks

| Benchmark | Target | Measured | Status | Notes |
|-----------|--------|----------|--------|-------|
| `environment_creation` | < 100ms | TBD | ⏳ Pending | 95th percentile |
| `environment_creation_with_config` | < 150ms | TBD | ⏳ Pending | Advanced scenarios |
| `session_initialization` | < 50ms | TBD | ⏳ Pending | Per-session overhead |
| `environment_cleanup` | < 200ms | TBD | ⏳ Pending | Cleanup efficiency |
| `container_registration` | < 10ms | TBD | ⏳ Pending | Singleton pattern |
| `multiple_container_registrations` | Linear scaling | TBD | ⏳ Pending | 1-50 containers |

**Production Impact**: High - These benchmarks establish the foundation for all test execution.

**Optimization Opportunities**:
- Container reuse via singleton pattern (80% faster)
- Lazy initialization for unused resources
- Connection pooling for frequently-used containers

### 2. Test Execution (Critical Path)

**Why it matters**: Test execution performance determines CI/CD pipeline speed and developer feedback loop.

#### Key Benchmarks

| Benchmark | Target | Measured | Status | Notes |
|-----------|--------|----------|--------|-------|
| `simple_test_execution` | < 100ms | TBD | ⏳ Pending | 95th percentile |
| `test_execution_with_metrics` | < 150ms | TBD | ⏳ Pending | Monitoring overhead |
| `sequential_test_throughput` | >= 10 tests/sec | TBD | ⏳ Pending | CI/CD pipeline |
| `concurrent_task_spawning` | < 50ms | TBD | ⏳ Pending | Parallel execution |
| `multiple_concurrent_tasks` | < 500ms (10 tasks) | TBD | ⏳ Pending | Throughput scaling |

**Production Impact**: Critical - These benchmarks determine developer productivity and CI/CD efficiency.

**Optimization Opportunities**:
- Parallel test execution (10-50x speedup)
- Test result caching
- Smart test ordering (fast tests first)

### 3. Memory Usage (Resource Efficiency)

**Why it matters**: Memory efficiency enables running more tests concurrently and reduces infrastructure costs.

#### Key Benchmarks

| Benchmark | Target | Measured | Status | Notes |
|-----------|--------|----------|--------|-------|
| `environment_memory_allocation` | < 100MB | TBD | ⏳ Pending | Basic operations |
| `memory_usage_multiple_containers` | < 500MB (10 containers) | TBD | ⏳ Pending | Scaling behavior |
| `metrics_memory_footprint` | < 10KB per metrics | TBD | ⏳ Pending | Monitoring overhead |

**Production Impact**: High - Memory efficiency directly impacts infrastructure costs.

**Optimization Opportunities**:
- Container memory limits
- Metrics aggregation
- Memory-mapped files for large test data

### 4. I/O Performance (Bottleneck Analysis)

**Why it matters**: I/O operations often become bottlenecks in test execution. Optimizing I/O can yield significant performance improvements.

#### Key Benchmarks

| Benchmark | Target | Measured | Status | Notes |
|-----------|--------|----------|--------|-------|
| `container_registry_read` | < 1ms | TBD | ⏳ Pending | 95th percentile |
| `container_registry_write` | < 5ms | TBD | ⏳ Pending | 95th percentile |
| `metrics_update_operations` | < 5ms | TBD | ⏳ Pending | Update overhead |
| `concurrent_metrics_reads` | < 10ms (100 reads) | TBD | ⏳ Pending | Monitoring dashboards |

**Production Impact**: High - I/O operations are on the critical path for container management.

**Optimization Opportunities**:
- Read-heavy optimization (RwLock)
- Metrics caching
- Batch operations

### 5. Async Performance (Concurrency Overhead)

**Why it matters**: Async overhead determines the efficiency of concurrent test execution and task orchestration.

#### Key Benchmarks

| Benchmark | Target | Measured | Status | Notes |
|-----------|--------|----------|--------|-------|
| `async_task_spawning_overhead` | < 1ms | TBD | ⏳ Pending | Per-spawn cost |
| `async_context_switching` | < 10ms (100 switches) | TBD | ⏳ Pending | Context switching |
| `async_rwlock_contention` | < 50ms (100 ops) | TBD | ⏳ Pending | Lock contention |
| `async_channel_throughput` | >= 100k msgs/sec | TBD | ⏳ Pending | Communication overhead |

**Production Impact**: Critical - Async overhead affects all concurrent operations.

**Optimization Opportunities**:
- Task batching
- Lock-free data structures
- Work stealing schedulers

### 6. Singleton Pattern (80% Performance Claim)

**Why it matters**: The singleton pattern is the cornerstone of clnrm's 80% performance improvement claim. These benchmarks validate that claim.

#### Key Benchmarks

| Benchmark | Target | Measured | Status | Notes |
|-----------|--------|----------|--------|-------|
| `container_startup_baseline` | < 2s | TBD | ⏳ Pending | First container |
| `container_lookup_singleton` | < 5ms (80% faster) | TBD | ⏳ Pending | Subsequent lookups |
| `health_check_operations` | < 50ms | TBD | ⏳ Pending | Container validation |
| `orchestrator_stats_collection` | < 10ms | TBD | ⏳ Pending | Monitoring overhead |

**Production Impact**: Critical - These benchmarks validate the core value proposition.

**Expected Results**:
- First container startup: ~1-2s (baseline)
- Subsequent lookups: ~2-5ms (80% faster)
- Performance multiplier: 10-50x for test suites with container reuse

## Benchmark Methodology

### Test Environment

**Hardware**:
- CPU: 8 cores minimum (for concurrency tests)
- RAM: 16GB minimum (for memory tests)
- Disk: SSD (for I/O tests)
- Network: Low latency (for container operations)

**Software**:
- Rust: Latest stable
- Docker: Latest stable
- Operating System: macOS/Linux
- Criterion: 0.7

### Measurement Approach

1. **Sample Size**: 100 iterations (50 for expensive operations)
2. **Warmup**: 10 iterations
3. **Measurement Time**: 10 seconds per benchmark
4. **Statistical Analysis**: 95th percentile targets
5. **Noise Reduction**: CPU affinity, process priority

### Baseline Establishment

```bash
# Create initial baseline
cargo bench --bench production_benchmarks -- --save-baseline v0.1.0

# Track performance over time
cargo bench --bench production_benchmarks -- --baseline v0.1.0
```

## Performance Regression Testing

### CI/CD Integration

```yaml
# .github/workflows/benchmarks.yml
name: Performance Benchmarks

on:
  push:
    branches: [main, master]
  pull_request:
    branches: [main, master]

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run benchmarks
        run: cargo bench --bench production_benchmarks
      - name: Compare with baseline
        run: cargo bench --bench production_benchmarks -- --baseline main
```

### Performance Gates

**Automated Alerts** (fail CI if exceeded):
- Container startup time: > 2s (100% degradation)
- Test execution: > 200ms (100% degradation)
- Memory usage: > 200MB (100% degradation)
- Throughput: < 5 tests/sec (50% degradation)

## Optimization Priorities (80/20 Principle)

Focus optimization efforts on these high-impact areas:

### Priority 1: Critical Path (80% of impact)
1. **Container lifecycle** - Singleton pattern optimization
2. **Test execution** - Parallel execution and caching
3. **Async overhead** - Task batching and lock reduction

### Priority 2: Resource Efficiency (15% of impact)
4. **Memory usage** - Container limits and metrics aggregation
5. **I/O operations** - Caching and batch operations

### Priority 3: Nice-to-Have (5% of impact)
6. **Monitoring overhead** - Sampling and aggregation
7. **Health checks** - Frequency tuning

## Interpreting Results

### Green Flags (Good Performance)
- ✅ Container startup < 2s
- ✅ Test execution < 100ms
- ✅ Memory usage < 100MB
- ✅ Throughput >= 10 tests/sec
- ✅ Singleton lookup 80% faster than startup

### Yellow Flags (Review Required)
- ⚠️  Container startup 2-4s
- ⚠️  Test execution 100-200ms
- ⚠️  Memory usage 100-200MB
- ⚠️  Throughput 5-10 tests/sec

### Red Flags (Performance Regression)
- 🔴 Container startup > 4s
- 🔴 Test execution > 200ms
- 🔴 Memory usage > 200MB
- 🔴 Throughput < 5 tests/sec

## Performance Monitoring

### Real-Time Monitoring

```rust
// Example: Enable performance monitoring
let config = CleanroomConfig::default();
let env = CleanroomEnvironment::new(config).await?;

// Execute tests
for i in 0..100 {
    env.execute_test(&format!("test_{}", i), || {
        Ok::<(), CleanroomError>(())
    }).await?;
}

// Get performance metrics
let metrics = env.get_metrics().await;
println!("Tests executed: {}", metrics.tests_executed);
println!("Average execution time: {:?}", metrics.average_execution_time);
println!("Peak memory: {} MB", metrics.peak_memory_usage_bytes / 1024 / 1024);
println!("Peak CPU: {:.2}%", metrics.peak_cpu_usage_percent);
```

### Dashboards

**Key Metrics to Track**:
1. Test execution time (p50, p95, p99)
2. Container startup time (p50, p95, p99)
3. Memory usage (peak, average)
4. Throughput (tests/second)
5. Error rate (%)

## Troubleshooting Performance Issues

### Common Issues and Solutions

| Issue | Symptoms | Solution |
|-------|----------|----------|
| **Slow container startup** | > 2s startup | Check Docker daemon, network, disk I/O |
| **High memory usage** | > 100MB basic ops | Enable memory limits, review metrics aggregation |
| **Low throughput** | < 10 tests/sec | Enable parallel execution, review test ordering |
| **Lock contention** | High async overhead | Reduce lock scope, use lock-free structures |
| **I/O bottleneck** | High registry latency | Enable caching, batch operations |

### Performance Profiling

```bash
# CPU profiling
cargo flamegraph --bench production_benchmarks

# Memory profiling
cargo instruments -t Allocations --bench production_benchmarks

# Detailed profiling
cargo bench --bench production_benchmarks -- --profile-time=10
```

## Future Enhancements

### Planned Benchmarks (Future)
- Network I/O performance (Docker API)
- Container image pull time
- Cross-platform performance (Windows)
- Distributed execution overhead
- Cloud provider comparison (AWS, GCP, Azure)

### Advanced Metrics (Future)
- Percentile histograms (p50, p95, p99)
- Time-series trending
- Anomaly detection
- Performance forecasting

## Resources

### Documentation
- [Architecture Overview](architecture-overview.md)
- [Performance Monitoring](performance-monitoring.md)
- [Operations Guide](operations/README.md)

### Tools
- [Criterion.rs](https://github.com/bheisler/criterion.rs)
- [Flamegraph](https://github.com/flamegraph-rs/flamegraph)
- [cargo-instruments](https://github.com/cmyr/cargo-instruments)

### Community
- GitHub Issues: Report performance regressions
- GitHub Discussions: Share optimization strategies
- Discord: Real-time performance discussions

## Contributing

### Running Benchmarks Locally

```bash
# Install dependencies
cargo install cargo-flamegraph cargo-instruments

# Run benchmarks
cargo bench --bench production_benchmarks

# Generate flamegraph
cargo flamegraph --bench production_benchmarks

# View results
open target/criterion/report/index.html
```

### Submitting Results

When submitting benchmark results:
1. Include hardware specs (CPU, RAM, disk)
2. Include software versions (Rust, Docker, OS)
3. Include full criterion output
4. Include flamegraph if performance regression

### Adding New Benchmarks

Follow the 80/20 principle:
1. Focus on critical path operations
2. Measure production-relevant scenarios
3. Target 95th percentile performance
4. Document production impact
5. Include optimization opportunities

---

**Remember**: Focus on the 20% of metrics that matter for 80% of production performance. Optimize what matters most!

## Status Legend

- ✅ **Green**: Meets or exceeds target
- ⚠️  **Yellow**: Within acceptable range, review recommended
- 🔴 **Red**: Performance regression, action required
- ⏳ **Pending**: Benchmark not yet executed
- 🚧 **In Progress**: Optimization in progress

## Changelog

### v0.1.0 (2025-10-13)
- Initial performance baseline established
- Core benchmarks implemented
- Documentation created
- CI/CD integration planned

---

*This document follows the 80/20 principle: 80% of what you need to know about performance in 20% of the documentation complexity.*
