# Core Team Test Best Practices

This document outlines the core team's testing best practices for the Cleanroom Testing Framework, following industry standards and FAANG-level quality requirements.

## 🎯 Core Principles

### 1. **Test Pyramid Structure**
```
    /\
   /  \     E2E Tests (5%)
  /____\    - Full system integration
 /      \   - Critical user journeys
/________\  Integration Tests (15%)
\        /  - Component interactions
 \      /   - Service boundaries
  \____/    Unit Tests (80%)
   \  /     - Individual functions
    \/      - Pure logic
```

### 2. **FAANG-Level Quality Standards**
- **Coverage**: 90%+ for unit tests, 80%+ for integration tests
- **Performance**: Unit tests <10ms, Integration tests <100ms
- **Reliability**: Zero flaky tests, deterministic results
- **Maintainability**: Clear, readable, well-documented tests

## 🏗️ Test Architecture

### Test Organization
```
tests/
├── unit/           # Pure unit tests (80%)
├── integration/    # Component integration (15%)
├── e2e/           # End-to-end tests (5%)
├── fixtures/      # Test infrastructure
├── helpers/       # Test utilities
└── benchmarks/    # Performance tests
```

### Test Categories

#### **Unit Tests** (`tests/unit/`)
- Test individual functions in isolation
- Mock all external dependencies
- Fast execution (<10ms each)
- High coverage (90%+)

#### **Integration Tests** (`tests/integration/`)
- Test component interactions
- Use real dependencies where appropriate
- Moderate execution time (<100ms each)
- Focus on critical paths

#### **End-to-End Tests** (`tests/e2e/`)
- Test complete user workflows
- Use real systems and containers
- Slower execution (<1s each)
- Cover critical user journeys

## 📋 Test Writing Standards

### 1. **Test Naming Convention**
```rust
// ✅ Good: Descriptive and specific
#[test]
fn test_user_authentication_with_valid_credentials_succeeds() {}

#[test]
fn test_user_authentication_with_invalid_credentials_fails() {}

// ❌ Bad: Vague and unclear
#[test]
fn test_auth() {}

#[test]
fn test_user() {}
```

### 2. **Test Structure (AAA Pattern)**
```rust
#[test]
fn test_scenario_description() {
    // Arrange - Set up test data and dependencies
    let config = TestConfigs::unit_test();
    let environment = TestEnvironments::unit_test().await?;
    
    // Act - Execute the code under test
    let result = environment.execute_test("test", || {
        Ok::<String, CleanroomError>("expected_result".to_string())
    }).await?;
    
    // Assert - Verify the results
    TestAssertions::assert_success(&result);
    assert_eq!(result, "expected_result");
}
```

### 3. **Error Handling Best Practices**
```rust
// ✅ Good: Proper error handling
#[tokio::test]
async fn test_operation_handles_errors_gracefully() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::unit_test().await?;
    
    let result = environment.execute_test("test", || {
        Err::<String, CleanroomError>(CleanroomError::validation_error("test error"))
    }).await;
    
    TestAssertions::assert_error(&result);
    Ok(())
}

// ❌ Bad: Panic on error
#[test]
fn test_operation() {
    let result = some_operation().unwrap(); // Don't do this
    assert_eq!(result, expected);
}
```

## 🚀 Performance Best Practices

### 1. **Fast Test Execution**
```rust
// ✅ Good: Use conditional sleep for time-dependent tests
#[tokio::test]
async fn test_timeout_behavior() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::unit_test().await?;
    
    // Use conditional sleep that completes immediately in test mode
    conditional_sleep(Duration::from_millis(100)).await;
    
    // Test continues immediately
    Ok(())
}

// ❌ Bad: Real sleep in tests
#[tokio::test]
async fn test_timeout_behavior() {
    tokio::time::sleep(Duration::from_millis(100)).await; // Slow!
}
```

### 2. **Mock External Dependencies**
```rust
// ✅ Good: Mock external services
#[tokio::test]
async fn test_container_creation_with_mock_backend() -> Result<(), CleanroomError> {
    let mock_backend = MockBackend::new();
    let environment = TestEnvironments::with_backend(mock_backend).await?;
    
    let container = environment.create_container("test").await?;
    assert!(container.is_running());
    
    Ok(())
}
```

### 3. **Use Appropriate Test Presets**
```rust
// Unit tests: Ultra-fast, no external dependencies
let config = TestConfigs::unit_test(); // <1ms execution

// Integration tests: Fast, minimal external dependencies  
let config = TestConfigs::integration_test(); // <10ms execution

// Performance tests: Realistic but optimized
let config = TestConfigs::performance_test(); // <100ms execution
```

## 🛡️ Reliability Best Practices

### 1. **Deterministic Tests**
```rust
// ✅ Good: Use mock time for time-dependent tests
#[tokio::test]
async fn test_scheduled_task_execution() -> Result<(), CleanroomError> {
    let context = MockTimeTestContext::new();
    let scheduler = TaskScheduler::new();
    
    // Schedule task for 10 seconds from now
    scheduler.schedule_task("test_task", Duration::from_secs(10));
    
    // Advance time instantly
    context.advance(Duration::from_secs(10));
    
    // Task should be executed
    assert!(scheduler.is_task_executed("test_task"));
    
    Ok(())
}
```

### 2. **Isolated Tests**
```rust
// ✅ Good: Each test is independent
#[tokio::test]
async fn test_user_creation() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::unit_test().await?;
    // Test implementation
    Ok(())
}

#[tokio::test] 
async fn test_user_deletion() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::unit_test().await?;
    // Test implementation - independent of previous test
    Ok(())
}
```

### 3. **Proper Cleanup**
```rust
// ✅ Good: Automatic cleanup with RAII
#[tokio::test]
async fn test_resource_management() -> Result<(), CleanroomError> {
    let _guard = TestEnvironmentGuard::new(TestConfigs::unit_test()).await?;
    // Resources automatically cleaned up when guard drops
    Ok(())
}
```

## 📊 Test Coverage Standards

### Coverage Targets
- **Unit Tests**: 90%+ line coverage, 95%+ branch coverage
- **Integration Tests**: 80%+ line coverage, 85%+ branch coverage
- **E2E Tests**: 100% of critical user journeys

### Coverage Measurement
```bash
# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage/

# View coverage report
open coverage/tarpaulin-report.html
```

## 🔍 Test Quality Metrics

### 1. **Test Metrics Dashboard**
```rust
// Test execution metrics
pub struct TestMetrics {
    pub execution_time: Duration,
    pub memory_usage: usize,
    pub cpu_usage: f64,
    pub coverage_percentage: f64,
    pub flakiness_score: f64,
}
```

### 2. **Quality Gates**
- **Execution Time**: Unit tests <10ms, Integration <100ms
- **Memory Usage**: <100MB per test
- **Coverage**: Meet minimum thresholds
- **Flakiness**: 0% flaky tests

## 🧪 Test Types and Patterns

### 1. **Property-Based Testing**
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_config_validation_properties(
        timeout in 1u64..1000u64,
        containers in 1u32..100u32,
    ) {
        let config = CleanroomConfig {
            test_execution_timeout: Duration::from_secs(timeout),
            max_concurrent_containers: containers,
            ..TestConfigs::unit_test()
        };
        
        // Property: All valid configs should pass validation
        prop_assert!(config.validate().is_ok());
    }
}
```

### 2. **Table-Driven Tests**
```rust
#[test]
fn test_config_validation_cases() {
    let test_cases = vec![
        ("valid_config", TestConfigs::unit_test(), true),
        ("invalid_timeout", invalid_timeout_config(), false),
        ("invalid_containers", invalid_containers_config(), false),
    ];
    
    for (name, config, should_pass) in test_cases {
        let result = config.validate();
        assert_eq!(
            result.is_ok(),
            should_pass,
            "Test case '{}' failed",
            name
        );
    }
}
```

### 3. **Golden File Testing**
```rust
#[test]
fn test_output_format_matches_golden_file() {
    let output = generate_test_output();
    let golden_file = include_str!("fixtures/expected_output.txt");
    
    assert_eq!(output, golden_file);
}
```

## 🚨 Anti-Patterns to Avoid

### 1. **Don't Test Implementation Details**
```rust
// ❌ Bad: Testing private implementation
#[test]
fn test_internal_state() {
    let obj = MyObject::new();
    assert_eq!(obj.internal_counter, 0); // Testing private field
}

// ✅ Good: Testing public behavior
#[test]
fn test_object_initialization() {
    let obj = MyObject::new();
    assert_eq!(obj.get_count(), 0); // Testing public interface
}
```

### 2. **Don't Create Brittle Tests**
```rust
// ❌ Bad: Testing exact string formatting
#[test]
fn test_error_message_format() {
    let error = MyError::new("test");
    assert_eq!(error.to_string(), "Error: test at line 42"); // Brittle!
}

// ✅ Good: Testing error content
#[test]
fn test_error_message_content() {
    let error = MyError::new("test");
    assert!(error.to_string().contains("test")); // Flexible
}
```

### 3. **Don't Ignore Test Failures**
```rust
// ❌ Bad: Ignoring test failures
#[test]
#[ignore] // Don't do this without good reason
fn test_flaky_operation() {
    // Test implementation
}

// ✅ Good: Fix the underlying issue
#[test]
fn test_deterministic_operation() -> Result<(), CleanroomError> {
    // Properly implemented test
    Ok(())
}
```

## 📚 Test Documentation

### 1. **Test Documentation Standards**
```rust
/// Tests the user authentication flow with valid credentials.
///
/// This test verifies that:
/// - Valid credentials are accepted
/// - User session is created
/// - Authentication token is generated
/// - User permissions are loaded
///
/// # Test Data
/// - Username: "testuser"
/// - Password: "testpass123"
/// - Expected permissions: ["read", "write"]
///
/// # Assertions
/// - Authentication succeeds
/// - Session ID is non-empty
/// - Token is valid format
/// - Permissions match expected
#[tokio::test]
async fn test_user_authentication_with_valid_credentials_succeeds() -> Result<(), CleanroomError> {
    // Test implementation
    Ok(())
}
```

### 2. **Test Plan Documentation**
```rust
//! Test Plan: Container Lifecycle Management
//!
//! ## Test Coverage
//! - [x] Container creation
//! - [x] Container startup
//! - [x] Container health checks
//! - [x] Container cleanup
//! - [x] Error handling
//!
//! ## Test Data
//! - Test containers: alpine, nginx, postgres
//! - Test scenarios: success, timeout, failure
//!
//! ## Expected Outcomes
//! - All containers start successfully
//! - Health checks pass
//! - Cleanup completes without errors
```

## 🔧 Test Tooling

### 1. **Required Tools**
```toml
[dev-dependencies]
# Test frameworks
tokio-test = "0.4"
proptest = "1.0"
criterion = "0.5"

# Coverage
tarpaulin = "0.27"

# Mocking
mockall = "0.11"
wiremock = "0.5"

# Assertions
assert_matches = "1.5"
pretty_assertions = "1.4"
```

### 2. **Test Utilities**
```rust
// Test assertion macros
macro_rules! assert_success {
    ($result:expr) => {
        match $result {
            Ok(value) => value,
            Err(error) => panic!("Expected success, got error: {:?}", error),
        }
    };
}

// Test data generators
pub fn generate_test_config() -> CleanroomConfig {
    TestConfigs::unit_test()
}

pub fn generate_test_environment() -> impl Future<Output = Result<Arc<CleanroomEnvironment>, CleanroomError>> {
    TestEnvironments::unit_test()
}
```

## 📈 Continuous Improvement

### 1. **Test Review Checklist**
- [ ] Test name clearly describes what is being tested
- [ ] Test follows AAA pattern (Arrange, Act, Assert)
- [ ] Test is deterministic and doesn't depend on external state
- [ ] Test uses appropriate fixtures and mocks
- [ ] Test has proper error handling
- [ ] Test execution time is within limits
- [ ] Test covers the intended behavior, not implementation
- [ ] Test is properly documented

### 2. **Regular Test Maintenance**
- **Weekly**: Review test execution times and fix slow tests
- **Monthly**: Review test coverage and add missing tests
- **Quarterly**: Refactor tests for better maintainability
- **Annually**: Update test frameworks and tools

## 🎯 Success Metrics

### Quality Metrics
- **Test Coverage**: 90%+ unit, 80%+ integration
- **Execution Speed**: Unit <10ms, Integration <100ms
- **Reliability**: 0% flaky tests
- **Maintainability**: Clear, documented, well-organized

### Process Metrics
- **Test Review Time**: <30 minutes per test
- **Test Fix Time**: <2 hours for failing tests
- **Test Creation Time**: <1 hour for new tests
- **Test Suite Execution**: <30 seconds total

This comprehensive guide ensures that our test suite meets FAANG-level quality standards while maintaining high performance and reliability.
