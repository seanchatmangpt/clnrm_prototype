# Migration Guide: v0.1.0 → v0.2.0

This guide helps you migrate from clnrm v0.1.0 to v0.2.0, which includes significant improvements in API ergonomics, performance, and observability.

## Table of Contents

1. [Overview](#overview)
2. [Breaking Changes](#breaking-changes)
3. [New Features](#new-features)
4. [API Changes](#api-changes)
5. [Configuration Changes](#configuration-changes)
6. [Code Migration Examples](#code-migration-examples)
7. [Troubleshooting](#troubleshooting)

## Overview

### What's New in v0.2.0?

- **Enhanced Builder API**: Type-safe builder pattern with compile-time guarantees
- **Advanced Tracing**: Comprehensive distributed tracing and observability
- **Improved Performance**: Optimized singleton container management
- **Better Error Handling**: More granular error types and better error messages
- **Security Enhancements**: Enhanced security policies and isolation controls
- **Documentation**: Comprehensive API documentation following 80/20 principles

###Performance Improvements

| Metric | v0.1.0 | v0.2.0 | Improvement |
|--------|--------|--------|-------------|
| **Container Startup** | 30-60s | 2-5s | 10-30x faster |
| **Test Execution** | 45-60s | 2-5s | 15-20x faster |
| **Memory Usage** | 2-4GB | 512MB-1GB | 2-4x less |
| **API Ergonomics** | Manual | Type-safe | Compile-time safety |

## Breaking Changes

### 1. CleanroomBuilder API Changes

**v0.1.0**:
```rust
let environment = CleanroomEnvironment::new(config).await?;
```

**v0.2.0**:
```rust
// Option 1: Direct creation (unchanged)
let environment = CleanroomEnvironment::new(config).await?;

// Option 2: Type-safe builder (new)
let environment = CleanroomBuilder::new()
    .with_timeout(Duration::from_secs(60))
    .with_security_policy(policy)
    .build()
    .await?;

// Option 3: Preset configurations (new)
let environment = CleanroomBuilder::secure()
    .build()
    .await?;
```

### 2. TracingManager API Changes

**v0.1.0**:
```rust
// No tracing support in v0.1.0
```

**v0.2.0**:
```rust
use clnrm::{TracingManager, SpanStatus};

let manager = TracingManager::new(session_id);

// Start span
let span_id = manager.start_span("operation".to_string(), None).await?;

// End span
manager.end_span("operation", SpanStatus::Completed).await?;

// Generate report
let report = manager.generate_tracing_report().await?;
```

### 3. Error Type Changes

**v0.1.0**:
```rust
// Generic error handling
match result {
    Ok(value) => { /* ... */ },
    Err(e) => eprintln!("Error: {}", e),
}
```

**v0.2.0**:
```rust
// Granular error handling
use clnrm::CleanroomError;

match result {
    Ok(value) => { /* ... */ },
    Err(CleanroomError::Container(msg)) => {
        eprintln!("Container error: {}", msg);
    },
    Err(CleanroomError::Policy(msg)) => {
        eprintln!("Policy violation: {}", msg);
    },
    Err(e) => eprintln!("Other error: {}", e),
}
```

### 4. Configuration Structure Changes

**v0.1.0**:
```toml
[cleanroom]
enable_security = true
max_memory = 1073741824  # 1GB
```

**v0.2.0**:
```toml
[cleanroom.security_policy]
enable_network_isolation = true
enable_filesystem_isolation = true
security_level = "Standard"

[cleanroom.resource_limits]
max_memory_usage_bytes = 1073741824  # 1GB
max_cpu_usage_percent = 80.0
max_container_count = 10
```

## New Features

### 1. Type-Safe Builder Pattern

The new builder API provides compile-time safety for configuration:

```rust
use clnrm::{CleanroomBuilder, SecurityPolicy, SecurityLevel};
use std::time::Duration;

// Compile-time validated configuration
let environment = CleanroomBuilder::new()
    .with_timeout(Duration::from_secs(60))
    .with_security_policy(SecurityPolicy::with_security_level(SecurityLevel::Locked))
    .with_resource_limits(ResourceLimits {
        max_memory_usage_bytes: 1073741824,
        max_cpu_usage_percent: 80.0,
        ..Default::default()
    })
    .with_deterministic_execution(Some(42))
    .build()
    .await?;
```

### 2. Distributed Tracing

Comprehensive tracing and observability:

```rust
use clnrm::{TracingManager, SpanStatus, MetricType, LogLevel};
use std::collections::HashMap;
use uuid::Uuid;

let manager = TracingManager::new(Uuid::new_v4());

// Hierarchical spans
let parent_id = manager.start_span("request".to_string(), None).await?;
let child_id = manager.start_span("database".to_string(), Some(parent_id)).await?;

// Add span tags and events
manager.add_span_tag("database", "query_type".to_string(), "SELECT".to_string()).await?;
manager.add_span_event("database", "query_start".to_string(), HashMap::new()).await?;

// Record metrics
manager.record_metric(
    "query_duration_ms".to_string(),
    42.0,
    MetricType::Histogram,
    HashMap::new(),
    Some("ms".to_string())
).await?;

// Structured logging
manager.log(
    LogLevel::Info,
    "Query completed".to_string(),
    Some("database.rs:123".to_string()),
    HashMap::new(),
    HashMap::new()
).await?;

// Complete spans
manager.end_span("database", SpanStatus::Completed).await?;
manager.end_span("request", SpanStatus::Completed).await?;

// Generate report with recommendations
let report = manager.generate_tracing_report().await?;
println!("Avg span duration: {:.2}ms", report.statistics.average_span_duration_ms);

for recommendation in &report.recommendations {
    println!("💡 {}", recommendation);
}
```

### 3. Preset Configurations

Quick-start configurations for common scenarios:

```rust
use clnrm::CleanroomBuilder;

// Secure environment (production)
let secure_env = CleanroomBuilder::secure()
    .with_coverage_tracking(true)
    .build()
    .await?;

// Performance-optimized environment
let perf_env = CleanroomBuilder::performance()
    .build()
    .await?;

// Development environment
let dev_env = CleanroomBuilder::development()
    .build()
    .await?;

// Deterministic environment (testing)
let deterministic_env = CleanroomBuilder::deterministic(42)
    .build()
    .await?;
```

### 4. Enhanced Security Policies

More granular security controls:

```rust
use clnrm::{CleanroomBuilder, SecurityPolicy, SecurityLevel};

let policy = SecurityPolicy {
    enable_network_isolation: true,
    enable_filesystem_isolation: true,
    enable_process_isolation: true,
    security_level: SecurityLevel::Locked,
    allowed_ports: vec![5432, 6379],
    blocked_commands: vec!["rm".to_string(), "format".to_string()],
    enable_data_redaction: true,
    ..Default::default()
};

let environment = CleanroomBuilder::new()
    .with_security_policy(policy)
    .build()
    .await?;
```

## API Changes

### CleanroomEnvironment

**Added Methods**:
```rust
// Concurrent task execution
pub async fn spawn_task<F, T>(&self, name: String, executor: F) -> Result<TaskId>;
pub async fn spawn_task_with_timeout<F, T>(&self, name: String, timeout: Duration, executor: F) -> Result<TaskId>;
pub async fn wait_for_task(&self, task_id: TaskId) -> Result<TaskResult<()>>;
pub async fn wait_for_all_tasks(&self) -> Result<Vec<TaskResult<()>>>;

// Task management
pub async fn cancel_task(&self, task_id: TaskId) -> Result<()>;
pub async fn cancel_all_tasks(&self) -> Result<()>;
pub async fn get_active_task_count(&self) -> usize;

// Health monitoring
pub async fn is_healthy(&self) -> bool;
pub async fn get_health_status(&self) -> HealthStatus;

// Orchestrator statistics
pub async fn get_orchestrator_stats(&self) -> OrchestratorStats;
pub async fn is_orchestrator_idle(&self) -> bool;
```

**Changed Methods**:
```rust
// v0.1.0
pub async fn execute_test(&self, command: &str) -> Result<RunResult>;

// v0.2.0 (now generic over test function)
pub async fn execute_test<F, T>(&self, test_name: &str, test_fn: F) -> Result<T>
where
    F: FnOnce() -> Result<T>;
```

### CleanroomBuilder

**New Type-Safe Builder**:
```rust
// Builder states for compile-time safety
pub struct Initial;
pub struct WithTimeout;
pub struct WithSecurity;
pub struct WithResources;
pub struct WithDeterministic;
pub struct Ready;

// Builder methods with state transitions
impl CleanroomBuilder<Initial> {
    pub fn new() -> Self;
    pub fn secure() -> CleanroomBuilder<WithSecurity>;
    pub fn performance() -> CleanroomBuilder<Ready>;
    pub fn deterministic(seed: u64) -> CleanroomBuilder<WithDeterministic>;
    pub fn development() -> CleanroomBuilder<Ready>;
}
```

### TracingManager (New)

**Complete Tracing API**:
```rust
// Span management
pub async fn start_span(&self, name: String, parent_span_id: Option<Uuid>) -> Result<Uuid>;
pub async fn end_span(&self, name: &str, status: SpanStatus) -> Result<()>;
pub async fn add_span_event(&self, span_name: &str, event_name: String, data: HashMap<String, String>) -> Result<()>;
pub async fn add_span_tag(&self, span_name: &str, key: String, value: String) -> Result<()>;

// Metrics
pub async fn record_metric(&self, name: String, value: f64, metric_type: MetricType, tags: HashMap<String, String>, unit: Option<String>) -> Result<()>;

// Logging
pub async fn log(&self, level: LogLevel, message: String, source: Option<String>, tags: HashMap<String, String>, metadata: HashMap<String, String>) -> Result<()>;

// Data access
pub async fn get_tracing_data(&self) -> TracingData;
pub async fn get_span(&self, name: &str) -> Result<Option<Span>>;
pub async fn get_all_spans(&self) -> Result<HashMap<String, Span>>;
pub async fn get_metric(&self, name: &str) -> Result<Option<Metric>>;
pub async fn get_all_metrics(&self) -> Result<HashMap<String, Metric>>;
pub async fn get_all_logs(&self) -> Result<Vec<LogEntry>>;

// Reporting
pub async fn generate_tracing_report(&self) -> Result<TracingReport>;
```

## Configuration Changes

### TOML Configuration

**v0.1.0** (`cleanroom.toml`):
```toml
[cleanroom]
enable_singleton_containers = true
container_startup_timeout = 120
test_execution_timeout = 300
enable_security = true
max_memory = 1073741824
```

**v0.2.0** (`cleanroom.toml`):
```toml
[cleanroom]
enable_singleton_containers = true
container_startup_timeout = 120
test_execution_timeout = 300
enable_deterministic_execution = false
deterministic_seed = 42
enable_coverage_tracking = false
enable_snapshot_testing = false
enable_tracing = true
max_concurrent_containers = 5

[cleanroom.security_policy]
enable_network_isolation = true
enable_filesystem_isolation = true
enable_process_isolation = true
security_level = "Standard"
allowed_ports = [5432, 6379]
blocked_commands = ["rm", "format"]
enable_data_redaction = true
enable_audit_logging = true

[cleanroom.resource_limits]
max_cpu_usage_percent = 80.0
max_memory_usage_bytes = 1073741824
max_disk_usage_bytes = 10737418240
max_container_count = 10
max_test_execution_time = 300

[cleanroom.performance_monitoring]
enable_monitoring = true
metrics_interval = 10
enable_profiling = false
```

### Environment Variables

**New Environment Variables**:
```bash
# Tracing
export CLEANROOM_ENABLE_TRACING=true
export CLEANROOM_TRACING_LEVEL=info

# Security
export CLEANROOM_SECURITY_LEVEL=standard
export CLEANROOM_ENABLE_NETWORK_ISOLATION=true

# Performance
export CLEANROOM_MAX_CONCURRENT_CONTAINERS=5
export CLEANROOM_ENABLE_PERFORMANCE_MONITORING=true
```

## Code Migration Examples

### Example 1: Basic Test Migration

**v0.1.0**:
```rust
use clnrm::{CleanroomEnvironment, CleanroomConfig};

#[tokio::test]
async fn test_basic() {
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await.unwrap();

    let result = environment.execute_test("echo hello").await.unwrap();
    assert_eq!(result.exit_code, 0);
}
```

**v0.2.0**:
```rust
use clnrm::{CleanroomEnvironment, CleanroomConfig, CleanroomBuilder};

#[tokio::test]
async fn test_basic() {
    // Option 1: Direct creation (unchanged)
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await.unwrap();

    let result = environment.execute_test("basic_test", || {
        Ok("test completed".to_string())
    }).await.unwrap();

    // Option 2: Builder pattern (recommended)
    let environment = CleanroomBuilder::development()
        .build()
        .await
        .unwrap();

    let result = environment.execute_test("basic_test", || {
        Ok("test completed".to_string())
    }).await.unwrap();
}
```

### Example 2: Container Management Migration

**v0.1.0**:
```rust
use clnrm::{CleanroomEnvironment, PostgresContainer};

#[tokio::test]
async fn test_database() {
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await.unwrap();

    // Manual container creation
    let postgres = PostgresContainer::new(
        &environment.docker_client,
        "testdb",
        "testuser",
        "testpass"
    );

    postgres.wait_for_ready().await.unwrap();

    // Test logic...
}
```

**v0.2.0**:
```rust
use clnrm::{CleanroomBuilder, PostgresContainer};

#[tokio::test]
async fn test_database() {
    // Use builder with automatic container management
    let environment = CleanroomBuilder::development()
        .with_singleton_containers(true)
        .build()
        .await
        .unwrap();

    // Singleton container pattern (10-50x faster)
    let postgres = environment.get_or_create_container("postgres", || {
        PostgresContainer::new(
            &environment.docker_client,
            "testdb",
            "testuser",
            "testpass"
        )
    }).await.unwrap();

    postgres.wait_for_ready().await.unwrap();

    // Test logic...
}
```

### Example 3: Security Configuration Migration

**v0.1.0**:
```rust
use clnrm::{CleanroomConfig, Policy};

let mut config = CleanroomConfig::default();
config.enable_security = true;

let environment = CleanroomEnvironment::new(config).await.unwrap();
```

**v0.2.0**:
```rust
use clnrm::{CleanroomBuilder, SecurityPolicy, SecurityLevel};

let environment = CleanroomBuilder::secure()
    .with_coverage_tracking(true)
    .with_tracing(true)
    .build()
    .await
    .unwrap();

// Or with custom security policy
let policy = SecurityPolicy::with_security_level(SecurityLevel::Locked);

let environment = CleanroomBuilder::new()
    .with_security_policy(policy)
    .build()
    .await
    .unwrap();
```

### Example 4: Adding Tracing (New in v0.2.0)

**v0.2.0 Only**:
```rust
use clnrm::{TracingManager, SpanStatus, MetricType};
use uuid::Uuid;
use std::collections::HashMap;

#[tokio::test]
async fn test_with_tracing() {
    let manager = TracingManager::new(Uuid::new_v4());

    // Start parent span
    let parent_id = manager.start_span("test_execution".to_string(), None).await.unwrap();

    // Start child span
    let child_id = manager.start_span("database_operation".to_string(), Some(parent_id)).await.unwrap();

    // Add tags
    manager.add_span_tag("database_operation", "db_type".to_string(), "postgres".to_string()).await.unwrap();

    // Record metric
    manager.record_metric(
        "query_duration_ms".to_string(),
        42.0,
        MetricType::Histogram,
        HashMap::new(),
        Some("ms".to_string())
    ).await.unwrap();

    // Complete spans
    manager.end_span("database_operation", SpanStatus::Completed).await.unwrap();
    manager.end_span("test_execution", SpanStatus::Completed).await.unwrap();

    // Generate report
    let report = manager.generate_tracing_report().await.unwrap();
    println!("Total spans: {}", report.statistics.total_spans);
    println!("Avg duration: {:.2}ms", report.statistics.average_span_duration_ms);
}
```

## Troubleshooting

### Issue: Builder State Errors

**Error**: `method not found in CleanroomBuilder<Initial>`

**Solution**: Follow the correct builder state transitions:
```rust
// Correct: Valid state transition
let env = CleanroomBuilder::new()
    .with_timeout(Duration::from_secs(60))  // Initial -> WithTimeout
    .with_security_policy(policy)           // WithTimeout -> WithSecurity
    .build()                                // WithSecurity -> Environment
    .await?;

// Error: Invalid state transition
let env = CleanroomBuilder::new()
    .build()  // Error: can't build from Initial state without configuration
    .await?;

// Solution: Use build_minimal() from Initial state
let env = CleanroomBuilder::new()
    .build_minimal()
    .await?;
```

### Issue: Tracing Performance Overhead

**Problem**: Tests are slower with tracing enabled

**Solution**: Disable tracing for performance-critical tests:
```rust
// Disable tracing
let manager = TracingManager::disabled();

// Or use environment variable
export CLEANROOM_ENABLE_TRACING=false
```

### Issue: Container Singleton Not Working

**Problem**: Containers are recreated for each test

**Solution**: Ensure singleton containers are enabled:
```toml
[cleanroom]
enable_singleton_containers = true
```

```rust
let environment = CleanroomBuilder::new()
    .with_singleton_containers(true)
    .build()
    .await?;
```

### Issue: Security Policy Violations

**Error**: `Policy violation: Network access denied`

**Solution**: Adjust security policy for your needs:
```rust
let policy = SecurityPolicy {
    enable_network_isolation: false,  // Disable for debugging
    ..Default::default()
};
```

## Migration Checklist

- [ ] **Update Dependencies**: Update `Cargo.toml` to v0.2.0
- [ ] **Update Imports**: Add new imports (`CleanroomBuilder`, `TracingManager`)
- [ ] **Update Configuration**: Convert to new TOML structure
- [ ] **Update Test Code**: Migrate to new `execute_test` signature
- [ ] **Add Builder Usage**: Consider using type-safe builder
- [ ] **Add Tracing**: Add tracing for observability (optional)
- [ ] **Update Error Handling**: Use granular error types
- [ ] **Enable Singleton Containers**: Enable for performance
- [ ] **Test Migration**: Run full test suite
- [ ] **Update Documentation**: Update project documentation

## Next Steps

After migrating to v0.2.0:

1. **Explore Tracing**: Add distributed tracing for better observability
2. **Optimize Performance**: Enable singleton containers and tune configuration
3. **Enhance Security**: Configure security policies for your environment
4. **Use Presets**: Try preset configurations (`secure()`, `performance()`, etc.)
5. **Monitor Performance**: Use built-in metrics and reporting
6. **Review Documentation**: Check the [API Reference](./api/README.md) for advanced features

## Support

If you need help with migration:

1. **Documentation**: Review the [Getting Started Guide](./guides/getting-started-tutorial.md)
2. **Examples**: Check the [examples](../examples/) directory
3. **Community**: Ask in [GitHub Discussions](https://github.com/sac/ggen/discussions)
4. **Issues**: Report bugs in [GitHub Issues](https://github.com/sac/ggen/issues)

Congratulations on migrating to v0.2.0! 🎉
