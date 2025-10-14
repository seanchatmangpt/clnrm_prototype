//! Test utilities and helpers for cleanroom tests
//!
//! This module contains test utilities and helpers for cleanroom tests

use clnrm::{
    CleanroomConfig, CleanroomEnvironment, Error as CleanroomError, ResourceLimits,
    SecurityLevel,
};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;

/// Create a test cleanroom environment with default configuration
pub async fn create_test_environment() -> Result<Arc<CleanroomEnvironment>, CleanroomError> {
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await?;
    Ok(Arc::new(environment))
}

/// Create a test cleanroom environment with custom configuration
pub async fn create_test_environment_with_config(
    config: CleanroomConfig,
) -> Result<Arc<CleanroomEnvironment>, CleanroomError> {
    let environment = CleanroomEnvironment::new(config).await?;
    Ok(Arc::new(environment))
}

/// Create test resource limits
pub fn create_test_resource_limits() -> ResourceLimits {
    ResourceLimits::default()
}

/// Wait for a condition to be true with timeout
pub async fn wait_for_condition<F, Fut>(
    condition: F,
    timeout_duration: Duration,
) -> Result<bool, CleanroomError>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = bool>,
{
    match timeout(timeout_duration, async move {
        loop {
            if condition().await {
                break true;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    {
        Ok(result) => Ok(result),
        Err(_) => Ok(false),
    }
}

/// Create a test policy with specified security level
pub fn create_test_policy(security_level: SecurityLevel) -> clnrm::Policy {
    clnrm::Policy::with_security_level(security_level)
}

/// Create a test configuration with custom settings
pub fn create_test_config() -> CleanroomConfig {
    let mut config = CleanroomConfig::default();
    config.test_execution_timeout = Duration::from_secs(30);
    config.container_startup_timeout = Duration::from_secs(5);
    config
}

/// Helper to create a mock backend for testing
pub fn create_mock_backend() -> clnrm::backend::MockBackend {
    clnrm::backend::MockBackend::new()
}

/// Mock test environment builder for fast, deterministic testing
pub struct MockTestEnvironmentBuilder {
    config: CleanroomConfig,
    mock_time: Option<crate::mock_time::MockTime>,
}

impl MockTestEnvironmentBuilder {
    /// Create a new mock test environment builder
    pub fn new() -> Self {
        Self {
            config: CleanroomConfig::default(),
            mock_time: None,
        }
    }

    /// Set the configuration
    pub fn with_config(mut self, config: CleanroomConfig) -> Self {
        self.config = config;
        self
    }

    /// Enable mock time
    pub fn with_mock_time(mut self, mock_time: crate::mock_time::MockTime) -> Self {
        self.mock_time = Some(mock_time);
        self
    }

    /// Build the mock test environment
    pub async fn build(self) -> Result<MockTestEnvironment, CleanroomError> {
        let environment = CleanroomEnvironment::new(self.config).await?;
        let mock_time = self.mock_time.unwrap_or_else(|| crate::mock_time::MockTime::new());

        Ok(MockTestEnvironment {
            environment: Arc::new(environment),
            mock_time,
        })
    }
}

impl Default for MockTestEnvironmentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Mock test environment with instant operations
pub struct MockTestEnvironment {
    environment: Arc<CleanroomEnvironment>,
    mock_time: crate::mock_time::MockTime,
}

impl MockTestEnvironment {
    /// Create a new mock test environment
    pub async fn new() -> Result<Self, CleanroomError> {
        Self::builder().build().await
    }

    /// Create a builder for the mock test environment
    pub fn builder() -> MockTestEnvironmentBuilder {
        MockTestEnvironmentBuilder::new()
    }

    /// Get the underlying environment
    pub fn environment(&self) -> &Arc<CleanroomEnvironment> {
        &self.environment
    }

    /// Get the mock time controller
    pub fn mock_time(&self) -> &crate::mock_time::MockTime {
        &self.mock_time
    }

    /// Execute a test with mock time
    pub async fn execute_test_mock<F, T>(&self, name: &str, test_fn: F) -> Result<T, CleanroomError>
    where
        F: FnOnce() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, CleanroomError>> + Send>>,
    {
        // Use mock timeout instead of real timeout
        let result = crate::mock_time::conditional_timeout(
            self.environment.config().test_execution_timeout,
            test_fn(),
        ).await;

        match result {
            Ok(Ok(value)) => Ok(value),
            Ok(Err(e)) => Err(e),
            Err(_) => Err(CleanroomError::timeout_error("Mock test timeout")),
        }
    }
}
