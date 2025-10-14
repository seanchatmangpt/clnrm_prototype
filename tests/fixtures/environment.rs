//! Test environment fixtures and builders
//!
//! Provides standardized test environment creation and management.

use crate::fixtures::config::TestConfigs;
use clnrm::{CleanroomConfig, CleanroomEnvironment, Error as CleanroomError};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;

/// Test environment fixtures
pub struct TestEnvironments;

impl TestEnvironments {
    /// Create a unit test environment
    pub async fn unit_test() -> Result<Arc<CleanroomEnvironment>, CleanroomError> {
        let config = TestConfigs::unit_test();
        let environment = CleanroomEnvironment::new(config).await?;
        Ok(Arc::new(environment))
    }

    /// Create an integration test environment
    pub async fn integration_test() -> Result<Arc<CleanroomEnvironment>, CleanroomError> {
        let config = TestConfigs::integration_test();
        let environment = CleanroomEnvironment::new(config).await?;
        Ok(Arc::new(environment))
    }

    /// Create a mock test environment
    pub async fn mock_test() -> Result<Arc<CleanroomEnvironment>, CleanroomError> {
        let config = TestConfigs::mock_test();
        let environment = CleanroomEnvironment::new(config).await?;
        Ok(Arc::new(environment))
    }

    /// Create a performance test environment
    pub async fn performance_test() -> Result<Arc<CleanroomEnvironment>, CleanroomError> {
        let config = TestConfigs::performance_test();
        let environment = CleanroomEnvironment::new(config).await?;
        Ok(Arc::new(environment))
    }

    /// Create a concurrent test environment
    pub async fn concurrent_test() -> Result<Arc<CleanroomEnvironment>, CleanroomError> {
        let config = TestConfigs::concurrent_test();
        let environment = CleanroomEnvironment::new(config).await?;
        Ok(Arc::new(environment))
    }

    /// Create a stress test environment
    pub async fn stress_test() -> Result<Arc<CleanroomEnvironment>, CleanroomError> {
        let config = TestConfigs::stress_test();
        let environment = CleanroomEnvironment::new(config).await?;
        Ok(Arc::new(environment))
    }

    /// Create a comprehensive test environment
    pub async fn comprehensive_test() -> Result<Arc<CleanroomEnvironment>, CleanroomError> {
        let config = TestConfigs::comprehensive_test();
        let environment = CleanroomEnvironment::new(config).await?;
        Ok(Arc::new(environment))
    }

    /// Create a security test environment
    pub async fn security_test() -> Result<Arc<CleanroomEnvironment>, CleanroomError> {
        let config = TestConfigs::security_test();
        let environment = CleanroomEnvironment::new(config).await?;
        Ok(Arc::new(environment))
    }
}

/// Test environment builder for custom environments
pub struct TestEnvironmentBuilder {
    config: CleanroomConfig,
}

impl TestEnvironmentBuilder {
    /// Create a new test environment builder
    pub fn new() -> Self {
        Self {
            config: CleanroomConfig::default(),
        }
    }

    /// Start with a preset configuration
    pub fn with_preset(preset: fn() -> CleanroomConfig) -> Self {
        Self { config: preset() }
    }

    /// Set the configuration directly
    pub fn with_config(mut self, config: CleanroomConfig) -> Self {
        self.config = config;
        self
    }

    /// Set container startup timeout
    pub fn container_startup_timeout(mut self, timeout: Duration) -> Self {
        self.config.container_startup_timeout = timeout;
        self
    }

    /// Set test execution timeout
    pub fn test_execution_timeout(mut self, timeout: Duration) -> Self {
        self.config.test_execution_timeout = timeout;
        self
    }

    /// Set maximum concurrent containers
    pub fn max_concurrent_containers(mut self, max: usize) -> Self {
        self.config.max_concurrent_containers = max;
        self
    }

    /// Enable/disable singleton containers
    pub fn singleton_containers(mut self, enable: bool) -> Self {
        self.config.enable_singleton_containers = enable;
        self
    }

    /// Enable/disable deterministic execution
    pub fn deterministic_execution(mut self, enable: bool) -> Self {
        self.config.enable_deterministic_execution = enable;
        self
    }

    /// Enable/disable coverage tracking
    pub fn coverage_tracking(mut self, enable: bool) -> Self {
        self.config.enable_coverage_tracking = enable;
        self
    }

    /// Enable/disable snapshot testing
    pub fn snapshot_testing(mut self, enable: bool) -> Self {
        self.config.enable_snapshot_testing = enable;
        self
    }

    /// Enable/disable tracing
    pub fn tracing(mut self, enable: bool) -> Self {
        self.config.enable_tracing = enable;
        self
    }

    /// Build the test environment
    pub async fn build(self) -> Result<Arc<CleanroomEnvironment>, CleanroomError> {
        let environment = CleanroomEnvironment::new(self.config).await?;
        Ok(Arc::new(environment))
    }

    /// Build the test environment with timeout
    pub async fn build_with_timeout(
        self,
        timeout_duration: Duration,
    ) -> Result<Arc<CleanroomEnvironment>, CleanroomError> {
        let result = timeout(timeout_duration, self.build()).await;
        match result {
            Ok(Ok(environment)) => Ok(environment),
            Ok(Err(e)) => Err(e),
            Err(_) => Err(CleanroomError::validation_error(
                "Environment creation timeout",
            )),
        }
    }
}

impl Default for TestEnvironmentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Test environment guard for automatic cleanup
pub struct TestEnvironmentGuard {
    environment: Arc<CleanroomEnvironment>,
}

impl TestEnvironmentGuard {
    /// Create a new test environment guard
    pub async fn new(config: CleanroomConfig) -> Result<Self, CleanroomError> {
        let environment = CleanroomEnvironment::new(config).await?;
        Ok(Self {
            environment: Arc::new(environment),
        })
    }

    /// Create from a preset configuration
    pub async fn with_preset(preset: fn() -> CleanroomConfig) -> Result<Self, CleanroomError> {
        Self::new(preset()).await
    }

    /// Get the environment
    pub fn environment(&self) -> &Arc<CleanroomEnvironment> {
        &self.environment
    }

    /// Get the session ID
    pub fn session_id(&self) -> uuid::Uuid {
        self.environment.session_id()
    }

    /// Get the configuration
    pub fn config(&self) -> &CleanroomConfig {
        self.environment.config()
    }
}

impl Drop for TestEnvironmentGuard {
    fn drop(&mut self) {
        // Note: In a real implementation, you might want to call cleanup here
        // For now, we'll rely on the environment's own cleanup
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_environment_presets() {
        let unit_env = TestEnvironments::unit_test().await;
        assert!(unit_env.is_ok());

        let integration_env = TestEnvironments::integration_test().await;
        assert!(integration_env.is_ok());

        let mock_env = TestEnvironments::mock_test().await;
        assert!(mock_env.is_ok());
    }

    #[tokio::test]
    async fn test_environment_builder() {
        let environment = TestEnvironmentBuilder::new()
            .container_startup_timeout(Duration::from_millis(100))
            .test_execution_timeout(Duration::from_millis(200))
            .max_concurrent_containers(5)
            .build()
            .await;

        assert!(environment.is_ok());
        let env = environment.unwrap();
        assert_eq!(
            env.config().container_startup_timeout,
            Duration::from_millis(100)
        );
        assert_eq!(
            env.config().test_execution_timeout,
            Duration::from_millis(200)
        );
        assert_eq!(env.config().max_concurrent_containers, 5);
    }

    #[tokio::test]
    async fn test_environment_builder_with_preset() {
        let environment = TestEnvironmentBuilder::with_preset(TestConfigs::unit_test)
            .max_concurrent_containers(3)
            .build()
            .await;

        assert!(environment.is_ok());
        let env = environment.unwrap();
        assert_eq!(
            env.config().container_startup_timeout,
            Duration::from_micros(100)
        );
        assert_eq!(env.config().max_concurrent_containers, 3);
    }

    #[tokio::test]
    async fn test_environment_guard() {
        let guard = TestEnvironmentGuard::with_preset(TestConfigs::unit_test).await;
        assert!(guard.is_ok());

        let guard = guard.unwrap();
        assert!(!guard.session_id().is_nil());
        assert_eq!(
            guard.config().container_startup_timeout,
            Duration::from_micros(100)
        );
    }
}
