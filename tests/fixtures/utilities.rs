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
