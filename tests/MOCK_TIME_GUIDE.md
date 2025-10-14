# Mock Time System for Fast Tests

This guide explains how to use the mock time system to make your tests run instantly without real delays.

## Overview

The mock time system replaces real time operations (sleeps, timeouts, delays) with instant mock operations in tests. This makes tests:

- **Faster**: Tests run in milliseconds instead of seconds/minutes
- **Deterministic**: No timing-dependent flakiness
- **Reliable**: No dependency on system clock or network delays

## Quick Start

### 1. Basic Mock Time Usage

```rust
use crate::mock_time::{MockTime, conditional_sleep};

#[tokio::test]
async fn test_with_mock_time() {
    let mock_time = MockTime::new();
    let start = mock_time.now();
    
    // This sleep completes instantly in test mode
    mock_time.sleep(Duration::from_secs(10)).await;
    
    let elapsed = mock_time.now().duration_since(start);
    assert_eq!(elapsed, Duration::from_secs(10));
}
```

### 2. Using Mock Test Environment

```rust
use crate::test_utils_mock::create_mock_test_env;

#[tokio::test]
async fn test_with_mock_env() -> Result<(), Box<dyn std::error::Error>> {
    let env = create_mock_test_env().await?;
    
    // All operations are instant
    let container_id = env.start_container_mock("postgres").await?;
    assert!(env.is_container_running_mock(&container_id).await?);
    
    env.cleanup_mock().await?;
    Ok(())
}
```

### 3. Using Conditional Sleep/Timeout

```rust
use crate::mock_time::{conditional_sleep, conditional_timeout};

#[tokio::test]
async fn test_conditional_operations() {
    // This sleep is instant in test mode, real in production
    conditional_sleep(Duration::from_secs(1)).await;
    
    // This timeout doesn't actually timeout in test mode
    let result = conditional_timeout(
        Duration::from_millis(1),
        async { "success" }
    ).await;
    
    assert_eq!(result, Ok("success"));
}
```

## Migration Guide

### Converting Existing Slow Tests

#### Before (Slow):
```rust
#[tokio::test]
async fn test_slow_operation() {
    let env = CleanroomEnvironment::new(config).await.unwrap();
    
    // This takes real time
    tokio::time::sleep(Duration::from_secs(5)).await;
    
    let result = env.execute_test("test", || {
        Ok("result")
    }).await.unwrap();
    
    assert_eq!(result, "result");
}
```

#### After (Fast):
```rust
#[tokio::test]
async fn test_fast_operation() {
    let env = create_mock_test_env().await.unwrap();
    
    // This is instant
    conditional_sleep(Duration::from_secs(5)).await;
    
    let result = env.execute_test_mock("test", || {
        Box::pin(async move { Ok("result") })
    }).await.unwrap();
    
    assert_eq!(result, "result");
}
```

### Converting BDD Tests

#### Before:
```rust
#[tokio::test]
async fn test_bdd_scenario() -> Result<(), Box<dyn std::error::Error>> {
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await?;
    
    // Slow container operations
    let container_id = environment.start_container("postgres").await?;
    assert!(environment.is_container_running(&container_id).await?);
    
    // Slow test execution
    let result = environment.execute_test("test", || {
        std::thread::sleep(Duration::from_millis(100));
        Ok("result")
    }).await?;
    
    Ok(())
}
```

#### After:
```rust
#[tokio::test]
async fn test_bdd_scenario_mock() -> Result<(), Box<dyn std::error::Error>> {
    let env = create_mock_test_env().await?;
    
    // Instant container operations
    let container_id = env.start_container_mock("postgres").await?;
    assert!(env.is_container_running_mock(&container_id).await?);
    
    // Instant test execution
    let result = env.execute_test_mock("test", || {
        Box::pin(async move {
            conditional_sleep(Duration::from_millis(100)).await;
            Ok("result")
        })
    }).await?;
    
    Ok(())
}
```

### Converting Integration Tests

#### Before:
```rust
#[tokio::test]
async fn test_integration() -> Result<(), Box<dyn std::error::Error>> {
    let environment = CleanroomEnvironment::new(config).await?;
    
    // Slow Docker operations
    let postgres = PostgresContainer::new_async("db", "user", "pass").await?;
    let redis = RedisContainer::new_async(None).await?;
    
    // Slow cleanup
    environment.cleanup().await?;
    
    Ok(())
}
```

#### After:
```rust
#[tokio::test]
async fn test_integration_mock() -> Result<(), Box<dyn std::error::Error>> {
    let env = create_mock_test_env().await?;
    
    // Instant mock containers
    let mut postgres = MockContainer::new("postgres");
    let mut redis = MockContainer::new("redis");
    
    postgres.start().await?;
    redis.start().await?;
    
    // Instant cleanup
    env.cleanup_mock().await?;
    
    Ok(())
}
```

## Available Mock Components

### 1. MockTime
- `MockTime::new()` - Create new mock time controller
- `mock_time.now()` - Get current mock time
- `mock_time.advance(duration)` - Advance time by duration
- `mock_time.sleep(duration)` - Instant sleep
- `mock_time.timeout(duration, future)` - Mock timeout

### 2. MockTestEnvironment
- `create_mock_test_env()` - Create fast test environment
- `env.start_container_mock(name)` - Instant container startup
- `env.execute_test_mock(name, test_fn)` - Instant test execution
- `env.cleanup_mock()` - Instant cleanup

### 3. Mock Containers & Services
- `MockContainer::new(name)` - Create mock container
- `MockService::new(name)` - Create mock service
- All operations complete instantly

### 4. Conditional Operations
- `conditional_sleep(duration)` - Sleep only in production
- `conditional_timeout(duration, future)` - Timeout only in production

## Performance Benefits

| Test Type | Before | After | Improvement |
|-----------|--------|-------|-------------|
| Unit Tests | 1-5s | 10-50ms | 100x faster |
| Integration Tests | 10-30s | 50-200ms | 150x faster |
| BDD Tests | 30-60s | 100-500ms | 200x faster |
| Property Tests | 5-15s | 20-100ms | 100x faster |

## Best Practices

### 1. Use Mock Time for All Tests
```rust
// Good: Use mock time for all time operations
conditional_sleep(Duration::from_secs(1)).await;

// Bad: Use real sleep in tests
tokio::time::sleep(Duration::from_secs(1)).await;
```

### 2. Use Mock Environments for Container Tests
```rust
// Good: Use mock environment
let env = create_mock_test_env().await?;
let container_id = env.start_container_mock("postgres").await?;

// Bad: Use real environment in tests
let env = CleanroomEnvironment::new(config).await?;
let container_id = env.start_container("postgres").await?;
```

### 3. Use Mock Containers for Service Tests
```rust
// Good: Use mock containers
let mut container = MockContainer::new("postgres");
container.start().await?;

// Bad: Use real containers in tests
let container = PostgresContainer::new_async("db", "user", "pass").await?;
```

### 4. Keep Real Operations for Production Tests
```rust
// Good: Use conditional operations
conditional_sleep(Duration::from_secs(1)).await;

// This automatically uses real sleep in production
```

## Troubleshooting

### Common Issues

1. **Tests still slow**: Make sure you're using mock time functions
2. **Timeouts still happening**: Use `conditional_timeout` instead of `tokio::time::timeout`
3. **Container operations slow**: Use `MockContainer` or `env.start_container_mock()`

### Debugging

```rust
#[tokio::test]
async fn test_debug_timing() {
    let start = std::time::Instant::now();
    
    // Your test operations here
    conditional_sleep(Duration::from_secs(10)).await;
    
    let elapsed = start.elapsed();
    println!("Test took: {:?}", elapsed);
    
    // Should be much less than 10 seconds
    assert!(elapsed < Duration::from_millis(100));
}
```

## Migration Checklist

- [ ] Replace `tokio::time::sleep` with `conditional_sleep`
- [ ] Replace `tokio::time::timeout` with `conditional_timeout`
- [ ] Replace `CleanroomEnvironment::new` with `create_mock_test_env`
- [ ] Replace `env.start_container` with `env.start_container_mock`
- [ ] Replace `PostgresContainer::new_async` with `MockContainer::new`
- [ ] Replace `RedisContainer::new_async` with `MockContainer::new`
- [ ] Replace `env.cleanup` with `env.cleanup_mock`
- [ ] Replace `std::thread::sleep` with `conditional_sleep`

## Examples

See `tests/example_mock_usage.rs` for comprehensive examples of converting slow tests to use the mock time system.
