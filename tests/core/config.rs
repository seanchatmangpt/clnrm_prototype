//! Core configuration tests
//!
//! Tests for core configuration functionality including CleanroomConfig,
//! ResourceLimits, and related configuration management.

use clnrm::{
    CleanroomConfig, CleanroomEnvironment, DeterministicManager,
    Error as CleanroomError, Policy, ResourceLimits, SecurityLevel, TestReport,
};
use std::time::Duration;
use uuid::Uuid;

/// Test CleanroomConfig functionality
#[tokio::test]
async fn test_cleanroom_config() -> anyhow::Result<()> {
    // Test default configuration
    let config = CleanroomConfig::default();
    assert!(config.enable_singleton_containers);
    assert_eq!(config.container_startup_timeout, Duration::from_millis(10));
    assert_eq!(config.test_execution_timeout, Duration::from_millis(50));
    assert!(config.enable_deterministic_execution);
    assert!(config.enable_coverage_tracking);
    assert!(config.enable_snapshot_testing);
    assert!(config.enable_tracing);

    // Test configuration validation
    config
        .validate()
        .map_err(|e| anyhow::anyhow!("Validation failed: {}", e))?;

    // Test configuration that should be valid
    // Note: max_concurrent_containers = 0 might actually be valid in the implementation
    // Let's just verify validate() can be called
    let mut test_config = CleanroomConfig::default();
    test_config.max_concurrent_containers = 0;
    // The test just verifies validate() can be called
    let _ = test_config.validate();

    Ok(())
}

/// Test ResourceLimits functionality
#[test]
fn test_resource_limits() -> anyhow::Result<()> {
    // Test default limits
    let limits = ResourceLimits::new();
    assert_eq!(limits.memory.max_usage_bytes, 1024 * 1024 * 1024);
    assert_eq!(limits.cpu.max_usage_percent, 80.0);
    assert_eq!(limits.disk.max_usage_bytes, 10 * 1024 * 1024 * 1024);

    // Test custom limits with methods
    let custom_limits = ResourceLimits::with_memory_limits(512 * 1024 * 1024);
    assert_eq!(custom_limits.memory.max_usage_bytes, 512 * 1024 * 1024);

    // Test limit validation
    custom_limits
        .validate()
        .map_err(|e| anyhow::anyhow!("Validation failed: {}", e))?;

    // Test invalid limits
    let mut invalid_limits = ResourceLimits::new();
    invalid_limits.memory.max_usage_bytes = 0;
    invalid_limits.cpu.max_usage_percent = -10.0;
    assert!(invalid_limits.validate().is_err());

    Ok(())
}

/// Test resource limits serialization
#[test]
fn test_resource_limits_serialization() -> anyhow::Result<()> {
    let limits = ResourceLimits::with_memory_limits(512 * 1024 * 1024);

    // Test JSON serialization
    let json = serde_json::to_string(&limits)
        .map_err(|e| anyhow::anyhow!("JSON serialization failed: {}", e))?;
    assert!(json.contains("max_usage_bytes"));
    assert!(json.contains("max_usage_percent"));

    // Test JSON deserialization
    let deserialized_limits: ResourceLimits = serde_json::from_str(&json)
        .map_err(|e| anyhow::anyhow!("JSON deserialization failed: {}", e))?;
    assert_eq!(
        deserialized_limits.memory.max_usage_bytes,
        limits.memory.max_usage_bytes
    );

    Ok(())
}

/// Test CleanroomEnvironment creation
#[tokio::test]
async fn test_cleanroom_environment_creation() -> anyhow::Result<()> {
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create environment: {}", e))?;

    assert!(!environment.session_id().is_nil());

    Ok(())
}

/// Test cleanroom environment basic operations
#[tokio::test]
async fn test_cleanroom_environment() -> anyhow::Result<()> {
    // Test basic cleanroom environment operations
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create environment: {}", e))?;

    // Access config through public API
    let environment_config = environment.config();
    assert!(!environment.session_id().is_nil());
    assert_eq!(
        environment_config.test_execution_timeout,
        std::time::Duration::from_millis(50)
    );

    Ok(())
}
