# Core Team Test Best Practices

This document outlines the comprehensive test best practices for the Cleanroom Testing Framework, designed to ensure high-quality, maintainable, and efficient test suites.

## 🎯 Core Principles

### 1. **Test Pyramid Strategy**
```
    🔺 E2E Tests (5%)
   🔺🔺 Integration Tests (15%)
  🔺🔺🔺 Unit Tests (80%)
```

- **Unit Tests**: Fast, isolated, test single functions/classes
- **Integration Tests**: Test component interactions, moderate speed
- **E2E Tests**: Test complete workflows, slower but comprehensive

### 2. **FIRST Principles**
- **Fast**: Tests should complete in milliseconds, not seconds
- **Independent**: Tests should not depend on each other
- **Repeatable**: Tests should produce the same results every time
- **Self-Validating**: Tests should have clear pass/fail criteria
- **Timely**: Tests should be written alongside the code

### 3. **AAA Pattern**
```rust
#[tokio::test]
async fn test_example() -> Result<(), CleanroomError> {
    // Arrange - Set up test data and environment
    let environment = TestEnvironments::unit_test().await?;
    let expected_result = "expected_value";
    
    // Act - Execute the code under test
    let actual_result = environment.execute_test("test", || {
        Ok::<String, CleanroomError>(expected_result.to_string())
    }).await?;
    
    // Assert - Verify the results
    assert_eq!(actual_result, expected_result);
    Ok(())
}
```

## 🏗️ Test Organization

### Directory Structure
```
tests/
├── fixtures/           # Test infrastructure and utilities
├── core/              # Unit tests for core functionality
├── integration/       # Integration tests
├── performance/       # Performance and benchmark tests
├── security/          # Security-focused tests
├── e2e/              # End-to-end tests
└── docs/             # Test documentation and guides
```

### Test File Naming
- **Unit tests**: `test_[module_name].rs`
- **Integration tests**: `test_[feature]_integration.rs`
- **Performance tests**: `test_[feature]_performance.rs`
- **Security tests**: `test_[feature]_security.rs`

## 🛠️ Test Infrastructure

### 1. **Use Test Fixtures**
```rust
use clnrm::tests::fixtures::*;

#[tokio::test]
async fn test_with_fixtures() -> Result<(), CleanroomError> {
    // Use preset configurations
    let env = TestEnvironments::unit_test().await?;
    let config = TestConfigs::performance_test();
    let policy = TestPolicies::strict();
    
    // Use builders for custom configurations
    let custom_env = TestEnvironmentBuilder::new()
        .container_startup_timeout(Duration::from_millis(100))
        .max_concurrent_containers(5)
        .build()
        .await?;
    
    Ok(())
}
```

### 2. **Mock Time for Deterministic Tests**
```rust
use clnrm::tests::fixtures::MockTimeTestContext;

#[tokio::test]
async fn test_time_dependent_behavior() -> Result<(), CleanroomError> {
    let context = MockTimeTestContext::new();
    let start_time = context.now();
    
    // Advance time by 10 seconds
    context.advance(Duration::from_secs(10));
    
    // Test time-dependent behavior
    let elapsed = context.elapsed();
    assert_eq!(elapsed, Duration::from_secs(10));
    
    Ok(())
}
```

### 3. **Use Conditional Sleep**
```rust
use clnrm::conditional_sleep;

#[tokio::test]
async fn test_with_sleep() -> Result<(), CleanroomError> {
    // This completes immediately in test mode
    conditional_sleep(Duration::from_secs(10)).await;
    
    // Test continues immediately
    assert!(true);
    Ok(())
}
```

## 📝 Test Writing Guidelines

### 1. **Test Function Naming**
```rust
// Good: Descriptive and specific
#[tokio::test]
async fn test_environment_creation_with_custom_config() -> Result<(), CleanroomError> {
    // Test implementation
}

// Bad: Vague and unclear
#[tokio::test]
async fn test_environment() -> Result<(), CleanroomError> {
    // Test implementation
}
```

### 2. **Test Structure**
```rust
#[tokio::test]
async fn test_feature_behavior() -> Result<(), CleanroomError> {
    // Arrange: Set up test data
    let environment = TestEnvironments::unit_test().await?;
    let input_data = create_test_data();
    let expected_output = "expected_result";
    
    // Act: Execute the feature
    let actual_output = environment
        .execute_test("feature_test", || {
            Ok::<String, CleanroomError>(process_data(input_data))
        })
        .await?;
    
    // Assert: Verify results
    TestAssertions::assert_success(&Ok(actual_output.clone()));
    assert_eq!(actual_output, expected_output);
    
    // Cleanup: Ensure proper cleanup
    environment.cleanup().await?;
    
    Ok(())
}
```

### 3. **Error Testing**
```rust
#[tokio::test]
async fn test_error_handling() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::unit_test().await?;
    
    // Test expected error
    let result = environment
        .execute_test("failing_test", || {
            Err::<String, CleanroomError>(CleanroomError::validation_error("Expected error"))
        })
        .await;
    
    // Verify error handling
    TestAssertions::assert_error(&result);
    TestAssertions::assert_error_message(&result, "Expected error");
    
    Ok(())
}
```

## 🚀 Performance Guidelines

### 1. **Test Speed Targets**
- **Unit tests**: <10ms each
- **Integration tests**: <100ms each
- **Performance tests**: <1s each
- **Total test suite**: <30s

### 2. **Optimization Techniques**
```rust
// Use appropriate test presets
let env = TestEnvironments::unit_test().await?;        // Fastest
let env = TestEnvironments::integration_test().await?; // Moderate
let env = TestEnvironments::performance_test().await?; // Slower but comprehensive

// Use conditional sleep instead of real sleep
conditional_sleep(Duration::from_millis(100)).await; // Instant in tests

// Use mock time for time-dependent tests
let context = MockTimeTestContext::new();
context.advance(Duration::from_secs(10)); // Instant
```

### 3. **Avoid Slow Operations**
```rust
// ❌ Bad: Real sleep operations
tokio::time::sleep(Duration::from_secs(10)).await;

// ✅ Good: Conditional sleep
conditional_sleep(Duration::from_secs(10)).await;

// ❌ Bad: Real Docker operations in unit tests
let container = DockerContainer::new("postgres:15").await?;

// ✅ Good: Mocked operations
let container = TestContainers::postgres();
```

## 🔒 Security Testing

### 1. **Security Test Categories**
```rust
// Test security policies
#[tokio::test]
async fn test_security_policy_enforcement() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::security_test().await?;
    let policy = TestPolicies::locked();
    
    // Test security enforcement
    // Implementation here
    
    Ok(())
}

// Test resource limits
#[tokio::test]
async fn test_resource_limits() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::security_test().await?;
    
    // Test resource limit enforcement
    // Implementation here
    
    Ok(())
}
```

### 2. **Security Test Patterns**
- Test all security levels (Permissive, Standard, Strict, Locked)
- Test resource limit enforcement
- Test network isolation
- Test filesystem isolation
- Test process isolation

## 📊 Test Coverage

### 1. **Coverage Targets**
- **Unit tests**: 90%+ line coverage
- **Integration tests**: 80%+ feature coverage
- **Critical paths**: 100% coverage
- **Security features**: 100% coverage

### 2. **Coverage Measurement**
```bash
# Run tests with coverage
cargo test --features coverage

# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage/
```

## 🧪 Test Types

### 1. **Unit Tests**
```rust
// Test individual functions/classes
#[tokio::test]
async fn test_config_validation() -> Result<(), CleanroomError> {
    let config = TestConfigs::unit_test();
    
    // Test validation logic
    assert!(config.validate().is_ok());
    
    Ok(())
}
```

### 2. **Integration Tests**
```rust
// Test component interactions
#[tokio::test]
async fn test_environment_container_integration() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::integration_test().await?;
    
    // Test environment-container interaction
    environment.register_container("test".to_string(), "id".to_string()).await?;
    assert!(environment.is_container_registered("test").await);
    
    Ok(())
}
```

### 3. **Performance Tests**
```rust
// Test performance characteristics
#[tokio::test]
async fn test_performance_benchmark() -> Result<(), CleanroomError> {
    let start = std::time::Instant::now();
    
    // Execute performance-critical operation
    let result = perform_operation().await?;
    
    let duration = start.elapsed();
    TestAssertions::assert_duration_less_than(duration, Duration::from_millis(100));
    
    Ok(())
}
```

## 🔄 Test Maintenance

### 1. **Regular Review**
- Review test coverage monthly
- Update tests when requirements change
- Remove obsolete tests
- Refactor slow tests

### 2. **Test Documentation**
```rust
/// Test that verifies environment creation with custom configuration
/// 
/// This test ensures that:
/// - Environment can be created with custom config
/// - Configuration is properly applied
/// - Environment is ready for use
#[tokio::test]
async fn test_environment_creation_with_custom_config() -> Result<(), CleanroomError> {
    // Test implementation
}
```

### 3. **Test Data Management**
```rust
// Use consistent test data
fn create_test_data() -> TestData {
    TestData {
        name: "test_data".to_string(),
        value: 42,
        timestamp: std::time::SystemTime::now(),
    }
}

// Use builders for complex test data
let test_data = TestDataBuilder::new()
    .with_name("custom_name")
    .with_value(100)
    .build();
```

## 🚨 Common Pitfalls

### 1. **Avoid These Patterns**
```rust
// ❌ Bad: Tests that depend on external resources
#[tokio::test]
async fn test_with_real_docker() -> Result<(), CleanroomError> {
    let container = DockerContainer::new("postgres:15").await?; // Slow and unreliable
    // Test implementation
}

// ❌ Bad: Tests with real sleep
#[tokio::test]
async fn test_with_real_sleep() -> Result<(), CleanroomError> {
    tokio::time::sleep(Duration::from_secs(10)).await; // Slow
    // Test implementation
}

// ❌ Bad: Tests that depend on each other
#[tokio::test]
async fn test_first() -> Result<(), CleanroomError> {
    set_global_state("value"); // Affects other tests
    // Test implementation
}

// ❌ Bad: Tests without proper cleanup
#[tokio::test]
async fn test_without_cleanup() -> Result<(), CleanroomError> {
    let environment = CleanroomEnvironment::new(config).await?;
    // No cleanup - resources leak
}
```

### 2. **Use These Patterns Instead**
```rust
// ✅ Good: Use test fixtures
#[tokio::test]
async fn test_with_fixtures() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::unit_test().await?; // Fast and reliable
    // Test implementation
    environment.cleanup().await?; // Proper cleanup
    Ok(())
}

// ✅ Good: Use conditional sleep
#[tokio::test]
async fn test_with_conditional_sleep() -> Result<(), CleanroomError> {
    conditional_sleep(Duration::from_secs(10)).await; // Instant in tests
    // Test implementation
}

// ✅ Good: Independent tests
#[tokio::test]
async fn test_independent() -> Result<(), CleanroomError> {
    let local_state = "value"; // Local state, doesn't affect other tests
    // Test implementation
}
```

## 📋 Test Checklist

### Before Writing Tests
- [ ] Understand the requirement
- [ ] Identify test type (unit/integration/performance)
- [ ] Choose appropriate test fixtures
- [ ] Plan test data and scenarios

### While Writing Tests
- [ ] Follow AAA pattern (Arrange, Act, Assert)
- [ ] Use descriptive test names
- [ ] Use appropriate assertions
- [ ] Handle errors properly
- [ ] Clean up resources

### After Writing Tests
- [ ] Run tests locally
- [ ] Verify test speed (<100ms for unit tests)
- [ ] Check test coverage
- [ ] Review test documentation
- [ ] Ensure tests are independent

## 🎯 Quality Metrics

### Test Quality Indicators
- **Speed**: Tests complete quickly
- **Reliability**: Tests pass consistently
- **Maintainability**: Tests are easy to understand and modify
- **Coverage**: Tests cover critical functionality
- **Isolation**: Tests don't depend on each other

### Continuous Improvement
- Monitor test execution times
- Track test failure rates
- Review test coverage reports
- Refactor slow or flaky tests
- Update test documentation

## 📚 Resources

### Documentation
- [Test Fixtures Guide](fixtures/README.md)
- [Mock Time Guide](MOCK_TIME_GUIDE.md)
- [Performance Testing Guide](performance/README.md)
- [Security Testing Guide](security/README.md)

### Tools
- `cargo test` - Run tests
- `cargo tarpaulin` - Coverage analysis
- `cargo bench` - Performance benchmarks
- `cargo clippy` - Code quality checks

### Examples
- [Unit Test Examples](core/examples.rs)
- [Integration Test Examples](integration/examples.rs)
- [Performance Test Examples](performance/examples.rs)

---

**Remember**: Good tests are an investment in code quality, maintainability, and developer productivity. Follow these practices to create a robust, efficient, and maintainable test suite! 🎉
