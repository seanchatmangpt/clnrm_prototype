//! Production Performance Benchmarks for clnrm Testing Framework
//!
//! This benchmark suite establishes production baselines for the clnrm (cleanroom) testing
//! framework. It focuses on the 20% of metrics that matter for 80% of production performance.
//!
//! # Benchmark Categories
//!
//! 1. **Container Lifecycle** - Startup, shutdown, resource usage
//! 2. **Test Execution** - Execution time, throughput, concurrency
//! 3. **Memory Usage** - Allocation patterns, peak usage
//! 4. **I/O Performance** - File operations, container operations
//! 5. **Async Performance** - Task spawning, context switching, await overhead
//!
//! # Performance Targets (from README.md 80/20 claims)
//!
//! - Singleton containers: 80% faster than regular containers
//! - Container startup time: < 2s
//! - Test execution time: < 100ms for simple tests
//! - Memory usage: < 100MB for basic operations
//! - Concurrent test throughput: >= 10 tests/second
//!
//! # Usage
//!
//! ```bash
//! cargo bench --bench production_benchmarks
//! ```

use clnrm::cleanroom::{CleanroomConfig, CleanroomEnvironment};
use clnrm::error::{CleanroomError, Result};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::sync::Arc;
use std::time::Duration;

// =============================================================================
// CATEGORY 1: CONTAINER LIFECYCLE BENCHMARKS
// =============================================================================

/// Benchmark: CleanroomEnvironment creation time
///
/// **Target**: < 100ms (95th percentile)
/// **Production Impact**: High - affects test startup time
fn bench_environment_creation(c: &mut Criterion) {
    c.bench_function("environment_creation", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let config = CleanroomConfig::default();
                black_box(CleanroomEnvironment::new(config).await.unwrap())
            });
    });
}

/// Benchmark: CleanroomEnvironment creation with custom config
///
/// **Target**: < 150ms (95th percentile)
/// **Production Impact**: Medium - affects advanced test scenarios
fn bench_environment_creation_with_config(c: &mut Criterion) {
    c.bench_function("environment_creation_with_config", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let mut config = CleanroomConfig::default();
                config.enable_deterministic_execution = true;
                config.deterministic_seed = Some(42);
                config.test_execution_timeout = Duration::from_secs(300);

                black_box(CleanroomEnvironment::new(config).await.unwrap())
            });
    });
}

/// Benchmark: CleanroomEnvironment session initialization
///
/// **Target**: < 50ms (95th percentile)
/// **Production Impact**: High - affects every test session
fn bench_session_initialization(c: &mut Criterion) {
    c.bench_function("session_initialization", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let config = CleanroomConfig::default();
                let env = CleanroomEnvironment::new(config).await.unwrap();
                black_box(env.session_id())
            });
    });
}

/// Benchmark: Environment cleanup time
///
/// **Target**: < 200ms (95th percentile)
/// **Production Impact**: Medium - affects test cleanup
fn bench_environment_cleanup(c: &mut Criterion) {
    c.bench_function("environment_cleanup", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let config = CleanroomConfig::default();
                let mut env = CleanroomEnvironment::new(config).await.unwrap();
                black_box(env.cleanup().await.unwrap())
            });
    });
}

/// Benchmark: Container registration time
///
/// **Target**: < 10ms (95th percentile)
/// **Production Impact**: High - affects singleton pattern performance
fn bench_container_registration(c: &mut Criterion) {
    c.bench_function("container_registration", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let config = CleanroomConfig::default();
                let env = CleanroomEnvironment::new(config).await.unwrap();
                black_box(
                    env.register_container("test".to_string(), "container_123".to_string())
                        .await
                        .unwrap(),
                )
            });
    });
}

/// Benchmark: Multiple container registrations
///
/// **Target**: Linear scaling with container count
/// **Production Impact**: High - affects test suites with multiple services
fn bench_multiple_container_registrations(c: &mut Criterion) {
    let mut group = c.benchmark_group("multiple_container_registrations");

    for size in [1, 5, 10, 20, 50].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.to_async(tokio::runtime::Runtime::new().unwrap())
                .iter(|| async move {
                    let config = CleanroomConfig::default();
                    let env = CleanroomEnvironment::new(config).await.unwrap();

                    for i in 0..size {
                        env.register_container(
                            format!("container_{}", i),
                            format!("id_{}", i),
                        )
                        .await
                        .unwrap();
                    }

                    black_box(env.get_container_count().await)
                });
        });
    }

    group.finish();
}

// =============================================================================
// CATEGORY 2: TEST EXECUTION BENCHMARKS
// =============================================================================

/// Benchmark: Simple test execution
///
/// **Target**: < 100ms (95th percentile)
/// **Production Impact**: Critical - affects all tests
fn bench_simple_test_execution(c: &mut Criterion) {
    c.bench_function("simple_test_execution", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let config = CleanroomConfig::default();
                let env = CleanroomEnvironment::new(config).await.unwrap();

                black_box(
                    env.execute_test("simple_test", || Ok::<i32, CleanroomError>(42))
                        .await
                        .unwrap(),
                )
            });
    });
}

/// Benchmark: Test execution with metrics collection
///
/// **Target**: < 150ms (95th percentile)
/// **Production Impact**: High - affects tests with monitoring
fn bench_test_execution_with_metrics(c: &mut Criterion) {
    c.bench_function("test_execution_with_metrics", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let config = CleanroomConfig::default();
                let env = CleanroomEnvironment::new(config).await.unwrap();

                env.execute_test("test_with_metrics", || Ok::<(), CleanroomError>(()))
                    .await
                    .unwrap();

                black_box(env.get_metrics().await)
            });
    });
}

/// Benchmark: Sequential test execution throughput
///
/// **Target**: >= 10 tests/second
/// **Production Impact**: High - affects CI/CD pipeline speed
fn bench_sequential_test_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("sequential_test_throughput");

    for test_count in [10, 50, 100].iter() {
        group.throughput(Throughput::Elements(*test_count as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(test_count),
            test_count,
            |b, &test_count| {
                b.to_async(tokio::runtime::Runtime::new().unwrap())
                    .iter(|| async move {
                        let config = CleanroomConfig::default();
                        let env = CleanroomEnvironment::new(config).await.unwrap();

                        for i in 0..test_count {
                            env.execute_test(&format!("test_{}", i), || {
                                Ok::<(), CleanroomError>(())
                            })
                            .await
                            .unwrap();
                        }

                        black_box(env.get_metrics().await)
                    });
            },
        );
    }

    group.finish();
}

/// Benchmark: Concurrent task spawning
///
/// **Target**: < 50ms per task spawn
/// **Production Impact**: High - affects parallel test execution
fn bench_concurrent_task_spawning(c: &mut Criterion) {
    c.bench_function("concurrent_task_spawning", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let config = CleanroomConfig::default();
                let env = Arc::new(CleanroomEnvironment::new(config).await.unwrap());

                let task_id = env
                    .spawn_task("benchmark_task".to_string(), |_ctx| {
                        Box::pin(async move { Ok::<i32, CleanroomError>(42) })
                    })
                    .await
                    .unwrap();

                black_box(task_id)
            });
    });
}

/// Benchmark: Multiple concurrent task spawning and completion
///
/// **Target**: < 500ms for 10 concurrent tasks
/// **Production Impact**: Critical - affects parallel test suites
fn bench_multiple_concurrent_tasks(c: &mut Criterion) {
    let mut group = c.benchmark_group("multiple_concurrent_tasks");

    for task_count in [5, 10, 20].iter() {
        group.throughput(Throughput::Elements(*task_count as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(task_count),
            task_count,
            |b, &task_count| {
                b.to_async(tokio::runtime::Runtime::new().unwrap())
                    .iter(|| async move {
                        let config = CleanroomConfig::default();
                        let env = Arc::new(CleanroomEnvironment::new(config).await.unwrap());

                        // Spawn multiple tasks concurrently
                        let mut task_ids = Vec::new();
                        for i in 0..task_count {
                            let task_id = env
                                .spawn_task(format!("task_{}", i), |_ctx| {
                                    Box::pin(async move {
                                        tokio::time::sleep(Duration::from_millis(10)).await;
                                        Ok::<i32, CleanroomError>(i)
                                    })
                                })
                                .await
                                .unwrap();
                            task_ids.push(task_id);
                        }

                        // Wait for all tasks to complete
                        black_box(env.wait_for_all_tasks().await.unwrap())
                    });
            },
        );
    }

    group.finish();
}

// =============================================================================
// CATEGORY 3: MEMORY USAGE BENCHMARKS
// =============================================================================

/// Benchmark: Memory allocation for environment creation
///
/// **Target**: < 100MB for basic operations
/// **Production Impact**: High - affects resource utilization
fn bench_environment_memory_allocation(c: &mut Criterion) {
    c.bench_function("environment_memory_allocation", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let config = CleanroomConfig::default();
                let env = CleanroomEnvironment::new(config).await.unwrap();
                let metrics = env.get_metrics().await;
                black_box(metrics.peak_memory_usage_bytes)
            });
    });
}

/// Benchmark: Memory usage with multiple containers
///
/// **Target**: < 500MB for 10 containers
/// **Production Impact**: Medium - affects large test suites
fn bench_memory_usage_multiple_containers(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage_multiple_containers");

    for container_count in [5, 10, 20].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(container_count),
            container_count,
            |b, &container_count| {
                b.to_async(tokio::runtime::Runtime::new().unwrap())
                    .iter(|| async move {
                        let config = CleanroomConfig::default();
                        let env = CleanroomEnvironment::new(config).await.unwrap();

                        for i in 0..container_count {
                            env.register_container(
                                format!("container_{}", i),
                                format!("id_{}", i),
                            )
                            .await
                            .unwrap();
                        }

                        let metrics = env.get_metrics().await;
                        black_box(metrics.peak_memory_usage_bytes)
                    });
            },
        );
    }

    group.finish();
}

/// Benchmark: Metrics object memory footprint
///
/// **Target**: < 10KB per metrics object
/// **Production Impact**: Medium - affects long-running test sessions
fn bench_metrics_memory_footprint(c: &mut Criterion) {
    c.bench_function("metrics_memory_footprint", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let config = CleanroomConfig::default();
                let env = CleanroomEnvironment::new(config).await.unwrap();

                // Execute some tests to populate metrics
                for i in 0..10 {
                    env.execute_test(&format!("test_{}", i), || Ok::<(), CleanroomError>(()))
                        .await
                        .unwrap();
                }

                black_box(env.get_metrics().await)
            });
    });
}

// =============================================================================
// CATEGORY 4: I/O PERFORMANCE BENCHMARKS
// =============================================================================

/// Benchmark: Container registry read operations
///
/// **Target**: < 1ms (95th percentile)
/// **Production Impact**: High - affects singleton pattern lookups
fn bench_container_registry_read(c: &mut Criterion) {
    c.bench_function("container_registry_read", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let config = CleanroomConfig::default();
                let env = CleanroomEnvironment::new(config).await.unwrap();

                env.register_container("test".to_string(), "id_123".to_string())
                    .await
                    .unwrap();

                black_box(env.is_container_registered("test").await)
            });
    });
}

/// Benchmark: Container registry write operations
///
/// **Target**: < 5ms (95th percentile)
/// **Production Impact**: High - affects container management
fn bench_container_registry_write(c: &mut Criterion) {
    c.bench_function("container_registry_write", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let config = CleanroomConfig::default();
                let env = CleanroomEnvironment::new(config).await.unwrap();

                black_box(
                    env.register_container("test".to_string(), "id_123".to_string())
                        .await
                        .unwrap(),
                )
            });
    });
}

/// Benchmark: Metrics update operations
///
/// **Target**: < 5ms (95th percentile)
/// **Production Impact**: High - affects test execution overhead
fn bench_metrics_update_operations(c: &mut Criterion) {
    c.bench_function("metrics_update_operations", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let config = CleanroomConfig::default();
                let env = CleanroomEnvironment::new(config).await.unwrap();

                black_box(
                    env.update_metrics(|metrics| {
                        metrics.tests_executed += 1;
                        metrics.tests_passed += 1;
                    })
                    .await
                    .unwrap(),
                )
            });
    });
}

/// Benchmark: Concurrent metrics reads
///
/// **Target**: < 10ms for 100 concurrent reads
/// **Production Impact**: Medium - affects monitoring dashboards
fn bench_concurrent_metrics_reads(c: &mut Criterion) {
    c.bench_function("concurrent_metrics_reads", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let config = CleanroomConfig::default();
                let env = Arc::new(CleanroomEnvironment::new(config).await.unwrap());

                // Spawn multiple concurrent metric reads
                let mut handles = Vec::new();
                for _ in 0..100 {
                    let env_clone = Arc::clone(&env);
                    let handle = tokio::spawn(async move { env_clone.get_metrics().await });
                    handles.push(handle);
                }

                // Wait for all reads to complete
                for handle in handles {
                    handle.await.unwrap();
                }

                black_box(())
            });
    });
}

// =============================================================================
// CATEGORY 5: ASYNC PERFORMANCE BENCHMARKS
// =============================================================================

/// Benchmark: Async task spawning overhead
///
/// **Target**: < 1ms per spawn
/// **Production Impact**: Critical - affects all async operations
fn bench_async_task_spawning_overhead(c: &mut Criterion) {
    c.bench_function("async_task_spawning_overhead", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                black_box(
                    tokio::spawn(async { 42 })
                        .await
                        .unwrap(),
                )
            });
    });
}

/// Benchmark: Async context switching
///
/// **Target**: < 10ms for 100 context switches
/// **Production Impact**: High - affects concurrent test execution
fn bench_async_context_switching(c: &mut Criterion) {
    c.bench_function("async_context_switching", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                for _ in 0..100 {
                    tokio::task::yield_now().await;
                }
                black_box(())
            });
    });
}

/// Benchmark: Async RwLock contention
///
/// **Target**: < 50ms for 100 lock operations
/// **Production Impact**: High - affects shared state access
fn bench_async_rwlock_contention(c: &mut Criterion) {
    c.bench_function("async_rwlock_contention", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                use tokio::sync::RwLock;
                let lock = Arc::new(RwLock::new(0));

                let mut handles = Vec::new();
                for _ in 0..100 {
                    let lock_clone = Arc::clone(&lock);
                    let handle = tokio::spawn(async move {
                        let mut data = lock_clone.write().await;
                        *data += 1;
                    });
                    handles.push(handle);
                }

                for handle in handles {
                    handle.await.unwrap();
                }

                black_box(())
            });
    });
}

/// Benchmark: Async channel throughput
///
/// **Target**: >= 100,000 messages/second
/// **Production Impact**: Medium - affects task communication
fn bench_async_channel_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("async_channel_throughput");

    for message_count in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*message_count as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(message_count),
            message_count,
            |b, &message_count| {
                b.to_async(tokio::runtime::Runtime::new().unwrap())
                    .iter(|| async move {
                        let (tx, mut rx) = tokio::sync::mpsc::channel(1000);

                        // Sender task
                        let sender = tokio::spawn(async move {
                            for i in 0..message_count {
                                tx.send(i).await.unwrap();
                            }
                        });

                        // Receiver task
                        let receiver = tokio::spawn(async move {
                            let mut count = 0;
                            while let Some(_msg) = rx.recv().await {
                                count += 1;
                                if count == message_count {
                                    break;
                                }
                            }
                            count
                        });

                        sender.await.unwrap();
                        black_box(receiver.await.unwrap())
                    });
            },
        );
    }

    group.finish();
}

// =============================================================================
// CATEGORY 6: SINGLETON CONTAINER PATTERN BENCHMARKS
// =============================================================================

/// Benchmark: Container startup time (singleton pattern baseline)
///
/// **Target**: < 2s for first container
/// **Production Impact**: Critical - establishes singleton pattern benefits
fn bench_container_startup_baseline(c: &mut Criterion) {
    c.bench_function("container_startup_baseline", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let config = CleanroomConfig::default();
                let env = CleanroomEnvironment::new(config).await.unwrap();
                let container_id = env.start_container("test_container").await.unwrap();
                black_box(container_id)
            });
    });
}

/// Benchmark: Container lookup time (singleton pattern reuse)
///
/// **Target**: < 5ms (80% faster than startup)
/// **Production Impact**: Critical - demonstrates singleton pattern efficiency
fn bench_container_lookup_singleton(c: &mut Criterion) {
    c.bench_function("container_lookup_singleton", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let config = CleanroomConfig::default();
                let env = CleanroomEnvironment::new(config).await.unwrap();

                // First call - creates container
                let _container_id = env.start_container("test_container").await.unwrap();

                // Subsequent call - lookup
                black_box(env.is_container_registered("test_container").await)
            });
    });
}

/// Benchmark: Health check operations
///
/// **Target**: < 50ms per health check
/// **Production Impact**: Medium - affects container validation
fn bench_health_check_operations(c: &mut Criterion) {
    c.bench_function("health_check_operations", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let config = CleanroomConfig::default();
                let env = CleanroomEnvironment::new(config).await.unwrap();
                black_box(env.is_healthy().await)
            });
    });
}

/// Benchmark: Orchestrator statistics collection
///
/// **Target**: < 10ms per stats collection
/// **Production Impact**: Medium - affects monitoring overhead
fn bench_orchestrator_stats_collection(c: &mut Criterion) {
    c.bench_function("orchestrator_stats_collection", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async {
                let config = CleanroomConfig::default();
                let env = Arc::new(CleanroomEnvironment::new(config).await.unwrap());

                // Spawn some tasks to populate stats
                for i in 0..5 {
                    env.spawn_task(format!("task_{}", i), |_ctx| {
                        Box::pin(async move { Ok::<(), CleanroomError>(()) })
                    })
                    .await
                    .unwrap();
                }

                black_box(env.get_orchestrator_stats().await)
            });
    });
}

// =============================================================================
// BENCHMARK CONFIGURATION
// =============================================================================

criterion_group!(
    name = container_lifecycle;
    config = Criterion::default()
        .sample_size(100)
        .measurement_time(Duration::from_secs(10));
    targets =
        bench_environment_creation,
        bench_environment_creation_with_config,
        bench_session_initialization,
        bench_environment_cleanup,
        bench_container_registration,
        bench_multiple_container_registrations
);

criterion_group!(
    name = test_execution;
    config = Criterion::default()
        .sample_size(100)
        .measurement_time(Duration::from_secs(10));
    targets =
        bench_simple_test_execution,
        bench_test_execution_with_metrics,
        bench_sequential_test_throughput,
        bench_concurrent_task_spawning,
        bench_multiple_concurrent_tasks
);

criterion_group!(
    name = memory_usage;
    config = Criterion::default()
        .sample_size(50)
        .measurement_time(Duration::from_secs(10));
    targets =
        bench_environment_memory_allocation,
        bench_memory_usage_multiple_containers,
        bench_metrics_memory_footprint
);

criterion_group!(
    name = io_performance;
    config = Criterion::default()
        .sample_size(100)
        .measurement_time(Duration::from_secs(10));
    targets =
        bench_container_registry_read,
        bench_container_registry_write,
        bench_metrics_update_operations,
        bench_concurrent_metrics_reads
);

criterion_group!(
    name = async_performance;
    config = Criterion::default()
        .sample_size(100)
        .measurement_time(Duration::from_secs(10));
    targets =
        bench_async_task_spawning_overhead,
        bench_async_context_switching,
        bench_async_rwlock_contention,
        bench_async_channel_throughput
);

criterion_group!(
    name = singleton_pattern;
    config = Criterion::default()
        .sample_size(50)
        .measurement_time(Duration::from_secs(10));
    targets =
        bench_container_startup_baseline,
        bench_container_lookup_singleton,
        bench_health_check_operations,
        bench_orchestrator_stats_collection
);

criterion_main!(
    container_lifecycle,
    test_execution,
    memory_usage,
    io_performance,
    async_performance,
    singleton_pattern
);
