//! Validation tests for cleanroom testing framework
//!
//! This module tests configuration validation, input validation, and
//! constraint checking to ensure system integrity and prevent invalid states.

use clnrm::{
    config::{CleanroomConfig, PerformanceMonitoringConfig, PerformanceThresholds},
    limits::{CpuLimits, MemoryLimits, ResourceLimits},
    policy::{SecurityLevel, SecurityPolicy},
    CleanroomEnvironment, Error,
};
use std::{collections::HashMap, time::Duration};

#[tokio::test]
async fn test_config_validation_success() {
    let config = CleanroomConfig::default();
    assert!(config.validate().is_ok());
}

#[tokio::test]
async fn test_config_validation_zero_timeout() {
    let mut config = CleanroomConfig::default();
    config.container_startup_timeout = Duration::from_secs(0);

    let result = config.validate();
    assert!(result.is_err());
    if let Err(Error::ValidationError(msg)) = result {
        assert!(msg.contains("timeout"));
    }
}

#[tokio::test]
async fn test_config_validation_zero_test_timeout() {
    let mut config = CleanroomConfig::default();
    config.test_execution_timeout = Duration::from_secs(0);

    let result = config.validate();
    assert!(result.is_err());
    if let Err(Error::ValidationError(msg)) = result {
        assert!(msg.contains("timeout"));
    }
}

#[tokio::test]
async fn test_config_validation_empty_security_ports() {
    let mut config = CleanroomConfig::default();
    config.security_policy.allowed_ports.clear();

    let result = config.validate();
    assert!(result.is_err());
    if let Err(Error::ValidationError(msg)) = result {
        assert!(msg.contains("port"));
    }
}

#[tokio::test]
async fn test_config_validation_zero_metrics_interval() {
    let mut config = CleanroomConfig::default();
    config.performance_monitoring.metrics_interval = Duration::from_secs(0);

    let result = config.validate();
    assert!(result.is_err());
    if let Err(Error::ValidationError(msg)) = result {
        assert!(msg.contains("interval"));
    }
}

#[tokio::test]
async fn test_resource_limits_validation() {
    let mut limits = ResourceLimits::default();

    // Valid limits should pass
    assert!(limits.validate().is_ok());

    // Invalid CPU percentage should fail
    limits.cpu.max_usage_percent = -10.0;
    assert!(limits.validate().is_err());

    // Reset and test invalid memory
    limits.cpu.max_usage_percent = 50.0;
    limits.memory.max_usage_bytes = 0;
    assert!(limits.validate().is_err());
}

#[tokio::test]
async fn test_security_policy_validation() {
    let mut policy = SecurityPolicy::default();

    // Valid policy should pass
    assert!(policy.validate().is_ok());

    // Empty allowed ports should fail
    policy.allowed_ports.clear();
    assert!(policy.validate().is_err());
}

#[tokio::test]
async fn test_performance_monitoring_validation() {
    let mut config = PerformanceMonitoringConfig::default();

    // Valid config should pass
    assert!(config.validate().is_ok());

    // Zero interval should fail
    config.metrics_interval = Duration::from_secs(0);
    assert!(config.validate().is_err());
}

#[tokio::test]
async fn test_environment_validation() {
    // Test that environment creation validates configuration
    let invalid_config = CleanroomConfig {
        container_startup_timeout: Duration::from_secs(0),
        ..Default::default()
    };

    let result = CleanroomEnvironment::new(invalid_config).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_container_customizer_validation() {
    use clnrm::config::{
        ContainerCustomizer, ContainerResourceLimits, HealthCheckConfig, PortMapping, VolumeMount,
    };

    let customizer = ContainerCustomizer {
        name: "test".to_string(),
        env_vars: HashMap::new(),
        volume_mounts: vec![VolumeMount {
            host_path: "/tmp".to_string(),
            container_path: "/app".to_string(),
            read_only: false,
        }],
        port_mappings: vec![PortMapping {
            container_port: 8080,
            host_port: Some(8080),
            protocol: "tcp".to_string(),
        }],
        resource_limits: ContainerResourceLimits::default(),
        health_check: HealthCheckConfig::default(),
        init_commands: vec!["echo 'init'".to_string()],
    };

    // Test that customizer can be created and serialized
    let json = serde_json::to_string(&customizer).unwrap();
    assert!(json.contains("test"));
    assert!(json.contains("8080"));

    let deserialized: ContainerCustomizer = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.name, "test");
}

#[tokio::test]
async fn test_config_merge_validation() {
    let mut config1 = CleanroomConfig::default();
    let mut config2 = CleanroomConfig::default();

    // Make config2 have some different values
    config2.container_startup_timeout = Duration::from_secs(2);
    config2.enable_deterministic_execution = false;

    config1.merge(&config2);

    // Check that merge worked correctly
    assert_eq!(config1.container_startup_timeout, Duration::from_secs(2));
    assert_eq!(config1.enable_deterministic_execution, false);
    assert!(config1.validate().is_ok());
}

#[tokio::test]
async fn test_validation_error_messages() {
    let mut config = CleanroomConfig::default();

    // Test different validation error scenarios
    config.container_startup_timeout = Duration::from_secs(0);
    let result = config.validate();
    assert!(result.is_err());

    if let Err(Error::ValidationError(msg)) = result {
        assert!(!msg.is_empty());
        assert!(msg.contains("timeout"));
    }

    // Test resource validation errors
    config.container_startup_timeout = Duration::from_secs(30);
    config.resource_limits.cpu.max_usage_percent = -1.0;

    let result = config.validate();
    assert!(result.is_err());
}

#[tokio::test]
async fn test_config_summary_validation() {
    let config = CleanroomConfig::default();
    let summary = config.summary();

    // Summary should contain key information
    assert!(summary.contains("Cleanroom Configuration"));
    assert!(summary.contains("Singleton Containers"));
    assert!(summary.contains("Deterministic Execution"));
    assert!(summary.contains("30")); // Default timeout value
}

#[tokio::test]
async fn test_environment_creation_validation() {
    // Test that environment creation properly validates
    let valid_config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(valid_config).await;
    assert!(environment.is_ok());

    // Test with invalid config
    let invalid_config = CleanroomConfig {
        container_startup_timeout: Duration::from_secs(0),
        ..Default::default()
    };

    let environment = CleanroomEnvironment::new(invalid_config).await;
    assert!(environment.is_err());
}
