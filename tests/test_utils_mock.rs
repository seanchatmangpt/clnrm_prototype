//! Mock test utilities for fast, deterministic testing
//!
//! This module provides utilities that replace real time operations
//! with instant mock operations in tests.

use crate::mock_time::{MockTime, MockTimeTestEnv, conditional_sleep, conditional_timeout};
use clnrm::{CleanroomConfig, CleanroomEnvironment, Error as CleanroomError};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;

/// Mock test environment builder
pub struct MockTestEnvironmentBuilder {
    config: CleanroomConfig,
    mock_time: Option<MockTime>,
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
    pub fn with_mock_time(mut self, mock_time: MockTime) -> Self {
        self.mock_time = Some(mock_time);
        self
    }

    /// Build the mock test environment
    pub async fn build(self) -> Result<MockTestEnvironment, CleanroomError> {
        let environment = CleanroomEnvironment::new(self.config).await?;
        let mock_time = self.mock_time.unwrap_or_else(MockTime::new);
        
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
    mock_time: MockTime,
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
    pub fn mock_time(&self) -> &MockTime {
        &self.mock_time
    }

    /// Execute a test with mock time
    pub async fn execute_test_mock<F, T>(&self, name: &str, test_fn: F) -> Result<T, CleanroomError>
    where
        F: FnOnce() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, CleanroomError>> + Send>>,
    {
        // Use mock timeout instead of real timeout
        let result = conditional_timeout(
            self.environment.config().test_execution_timeout,
            test_fn()
        ).await;
        
        match result {
            Ok(Ok(value)) => Ok(value),
            Ok(Err(e)) => Err(e),
            Err(_) => Err(CleanroomError::timeout_error("Mock test timeout")),
        }
    }

    /// Wait for a condition with mock time
    pub async fn wait_for_condition_mock<F, Fut>(
        &self,
        condition: F,
        timeout_duration: Duration,
    ) -> Result<bool, CleanroomError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = bool>,
    {
        let start_time = self.mock_time.now();
        
        while self.mock_time.now().duration_since(start_time) < timeout_duration {
            if condition().await {
                return Ok(true);
            }
            // Use mock sleep instead of real sleep
            conditional_sleep(Duration::from_millis(1)).await;
        }
        
        Ok(false)
    }

    /// Start a container with mock time
    pub async fn start_container_mock(&self, container_name: &str) -> Result<String, CleanroomError> {
        // Simulate container startup with mock time
        conditional_sleep(Duration::from_millis(10)).await;
        
        let container_id = format!("mock_{}", container_name);
        self.environment.register_container(container_name.to_string(), container_id.clone()).await?;
        
        Ok(container_id)
    }

    /// Check if container is running with mock time
    pub async fn is_container_running_mock(&self, container_id: &str) -> Result<bool, CleanroomError> {
        // Instant check in mock mode
        Ok(self.environment.is_container_registered(container_id).await)
    }

    /// Cleanup with mock time
    pub async fn cleanup_mock(&self) -> Result<(), CleanroomError> {
        // Instant cleanup in mock mode
        self.environment.cleanup().await
    }

    /// Get metrics with mock time
    pub async fn get_metrics_mock(&self) -> clnrm::Metrics {
        // Return mock metrics instantly
        self.environment.get_metrics().await
    }
}

/// Mock container operations
pub struct MockContainer {
    name: String,
    id: String,
    running: bool,
}

impl MockContainer {
    /// Create a new mock container
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            id: format!("mock_{}", uuid::Uuid::new_v4()),
            running: false,
        }
    }

    /// Start the container (instant in mock mode)
    pub async fn start(&mut self) -> Result<(), CleanroomError> {
        conditional_sleep(Duration::from_millis(1)).await;
        self.running = true;
        Ok(())
    }

    /// Stop the container (instant in mock mode)
    pub async fn stop(&mut self) -> Result<(), CleanroomError> {
        conditional_sleep(Duration::from_millis(1)).await;
        self.running = false;
        Ok(())
    }

    /// Check if container is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Get container ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get container name
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Mock service operations
pub struct MockService {
    name: String,
    ready: bool,
}

impl MockService {
    /// Create a new mock service
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ready: false,
        }
    }

    /// Start the service (instant in mock mode)
    pub async fn start(&mut self) -> Result<(), CleanroomError> {
        conditional_sleep(Duration::from_millis(1)).await;
        self.ready = true;
        Ok(())
    }

    /// Check if service is ready
    pub fn is_ready(&self) -> bool {
        self.ready
    }

    /// Get service name
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Helper function to create a fast mock test environment
pub async fn create_mock_test_env() -> Result<MockTestEnvironment, CleanroomError> {
    let config = CleanroomConfig {
        enable_singleton_containers: true,
        container_startup_timeout: Duration::from_millis(1),
        test_execution_timeout: Duration::from_millis(10),
        max_concurrent_containers: 5,
        enable_deterministic_execution: false,
        enable_coverage_tracking: false,
        enable_snapshot_testing: false,
        enable_tracing: false,
        ..CleanroomConfig::default()
    };

    MockTestEnvironment::builder()
        .with_config(config)
        .build()
        .await
}

/// Helper function to run a test with mock time
pub async fn run_with_mock_time<F, Fut, T>(test: F) -> T
where
    F: FnOnce(MockTime) -> Fut,
    Fut: std::future::Future<Output = T>,
{
    let mock_time = MockTime::new();
    test(mock_time).await
}

/// Helper function to run a test with mock time environment
pub async fn run_with_mock_env<F, Fut, T>(test: F) -> T
where
    F: FnOnce(MockTestEnvironment) -> Fut,
    Fut: std::future::Future<Output = T>,
{
    let env = create_mock_test_env().await.unwrap();
    test(env).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_mock_test_environment() {
        let env = create_mock_test_env().await.unwrap();
        
        let result = env.execute_test_mock("test", || {
            Box::pin(async move {
                conditional_sleep(Duration::from_secs(1)).await;
                Ok::<String, CleanroomError>("success".to_string())
            })
        }).await;
        
        assert_eq!(result.unwrap(), "success");
    }

    #[tokio::test]
    async fn test_mock_container() {
        let mut container = MockContainer::new("test");
        
        assert!(!container.is_running());
        
        container.start().await.unwrap();
        assert!(container.is_running());
        
        container.stop().await.unwrap();
        assert!(!container.is_running());
    }

    #[tokio::test]
    async fn test_mock_service() {
        let mut service = MockService::new("test");
        
        assert!(!service.is_ready());
        
        service.start().await.unwrap();
        assert!(service.is_ready());
    }

    #[tokio::test]
    async fn test_run_with_mock_time() {
        let result = run_with_mock_time(|mock_time| async move {
            let start = mock_time.now();
            mock_time.sleep(Duration::from_secs(5)).await;
            mock_time.now().duration_since(start)
        }).await;
        
        assert_eq!(result, Duration::from_secs(5));
    }

    #[tokio::test]
    async fn test_run_with_mock_env() {
        let result = run_with_mock_env(|env| async move {
            let container_id = env.start_container_mock("test").await.unwrap();
            env.is_container_running_mock(&container_id).await.unwrap()
        }).await;
        
        assert!(result);
    }
}
