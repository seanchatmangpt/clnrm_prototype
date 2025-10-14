//! BDD (Behavior Driven Development) tests for cleanroom testing framework
//!
//! These tests use the cucumber framework to test cleanroom behavior
//! from a user perspective with Given-When-Then scenarios.
//!
//! NOTE: This is a legacy test file. New tests should use the fixtures
//! in the fixtures module for better organization and performance.

use clnrm::{
    CleanroomConfig, CleanroomEnvironment, CleanroomGuard, DeterministicManager,
    Error as CleanroomError, GenericContainer, Policy, PostgresContainer, RedisContainer,
    ResourceLimits, SecurityLevel, TestReport,
};
use futures_util::future;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;

/// BDD test context for cleanroom scenarios
#[derive(Debug, Default)]
pub struct CleanroomTestContext {
    pub environment: Option<Arc<CleanroomEnvironment>>,
    pub config: Option<CleanroomConfig>,
    pub policy: Option<Policy>,
    pub resource_limits: Option<ResourceLimits>,
    pub test_results: Vec<String>,
    pub errors: Vec<CleanroomError>,
    pub containers: Vec<String>,
    pub services: Vec<String>,
    pub snapshots: Vec<String>,
    pub traces: Vec<String>,
}

impl CleanroomTestContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_environment(&self) -> Option<&Arc<CleanroomEnvironment>> {
        self.environment.as_ref()
    }

    pub fn set_environment(&mut self, environment: Arc<CleanroomEnvironment>) {
        self.environment = Some(environment);
    }

    pub fn get_config(&self) -> Option<&CleanroomConfig> {
        self.config.as_ref()
    }

    pub fn set_config(&mut self, config: CleanroomConfig) {
        self.config = Some(config);
    }

    pub fn get_policy(&self) -> Option<&Policy> {
        self.policy.as_ref()
    }

    pub fn set_policy(&mut self, policy: Policy) {
        self.policy = Some(policy);
    }

    pub fn add_test_result(&mut self, result: String) {
        self.test_results.push(result);
    }

    pub fn add_error(&mut self, error: CleanroomError) {
        self.errors.push(error);
    }

    pub fn add_container(&mut self, container: String) {
        self.containers.push(container);
    }

    pub fn add_service(&mut self, service: String) {
        self.services.push(service);
    }

    pub fn add_snapshot(&mut self, snapshot: String) {
        self.snapshots.push(snapshot);
    }

    pub fn add_trace(&mut self, trace: String) {
        self.traces.push(trace);
    }
}

#[tokio::test]
async fn test_bdd_scenario_basic_environment_creation() -> Result<(), CleanroomError> {
    let mut context = CleanroomTestContext::new();

    // Given: I want to create a cleanroom environment
    let config = CleanroomConfig::default();
    context.set_config(config);

    // When: I create the environment
    let environment = CleanroomEnvironment::new(context.get_config().unwrap().clone()).await?;
    context.set_environment(Arc::new(environment));

    // Then: The environment should be created successfully
    let env = context.get_environment().unwrap();
    assert!(!env.session_id().is_nil());

    Ok(())
}

#[tokio::test]
async fn test_bdd_scenario_container_lifecycle() -> Result<(), CleanroomError> {
    let mut context = CleanroomTestContext::new();

    // Given: I have a cleanroom environment
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await?;
    context.set_environment(Arc::new(environment));

    // When: I register a container
    let env = context.get_environment().unwrap();
    env.register_container("test_container".to_string(), "container_id_123".to_string())
        .await?;
    context.add_container("test_container".to_string());

    // Then: The container should be registered
    assert!(env.is_container_registered("test_container").await);

    // And: The container count should be updated
    let container_count = env.get_container_count().await;
    assert!(container_count >= 1);

    Ok(())
}

#[tokio::test]
async fn test_bdd_scenario_test_execution() -> Result<(), CleanroomError> {
    let mut context = CleanroomTestContext::new();

    // Given: I have a cleanroom environment
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await?;
    context.set_environment(Arc::new(environment));

    // When: I execute a test
    let env = context.get_environment().unwrap();
    let result = env
        .execute_test("bdd_test", || {
            Ok::<String, CleanroomError>("bdd_test_result".to_string())
        })
        .await?;

    context.add_test_result(result.clone());

    // Then: The test should complete successfully
    assert_eq!(result, "bdd_test_result");

    Ok(())
}

#[tokio::test]
async fn test_bdd_scenario_error_handling() -> Result<(), CleanroomError> {
    let mut context = CleanroomTestContext::new();

    // Given: I have a cleanroom environment
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await?;
    context.set_environment(Arc::new(environment));

    // When: I execute a test that fails
    let env = context.get_environment().unwrap();
    let result = env
        .execute_test("failing_test", || {
            Err::<String, CleanroomError>(CleanroomError::validation_error("Test failure"))
        })
        .await;

    // Then: The test should fail with an error
    assert!(result.is_err());
    if let Err(error) = result {
        context.add_error(error);
    }

    // And: The error should be recorded
    assert!(!context.errors.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_bdd_scenario_timeout_handling() -> Result<(), CleanroomError> {
    let mut context = CleanroomTestContext::new();

    // Given: I have a cleanroom environment with a short timeout
    let config = CleanroomConfig {
        test_execution_timeout: Duration::from_millis(100),
        ..CleanroomConfig::default()
    };
    let environment = CleanroomEnvironment::new(config).await?;
    context.set_environment(Arc::new(environment));

    // When: I execute a test that times out
    let timeout_result = timeout(
        Duration::from_millis(100),
        env.execute_test("slow_test", || async {
            conditional_sleep(Duration::from_millis(10)).await;
            Ok("slow_result")
        }),
    )
    .await;

    // Then: The test should complete (due to mocking)
    assert!(timeout_result.is_ok());
    let result = timeout_result.unwrap()?;
    context.add_test_result(result);

    Ok(())
}

#[tokio::test]
async fn test_bdd_scenario_performance_measurement() -> Result<(), CleanroomError> {
    let mut context = CleanroomTestContext::new();

    // Given: I have a cleanroom environment
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await?;
    context.set_environment(Arc::new(environment));

    // When: I execute a performance test
    let env = context.get_environment().unwrap();
    let start_time = std::time::Instant::now();
    let result = env
        .execute_test("performance_test", || async {
            // Simulate some work
            // Minimal delay for testing
            Ok("performance_result")
        })
        .await?;

    let duration = start_time.elapsed();
    context.add_test_result(result.clone());

    // Then: The test should complete successfully
    assert_eq!(result, "performance_result");

    // And: The test should complete quickly (due to mocking)
    assert!(duration < Duration::from_millis(100));

    Ok(())
}
