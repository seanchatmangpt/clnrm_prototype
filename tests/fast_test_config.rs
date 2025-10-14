//! Fast test configuration for cleanroom testing framework
//!
//! This module provides optimized test configurations that avoid
//! Docker dependencies and use minimal timeouts for fast execution.

use clnrm::{CleanroomConfig, SecurityPolicy};
use std::time::Duration;

/// Ultra-fast test configuration for unit tests
pub fn ultra_fast_test_config() -> CleanroomConfig {
    CleanroomConfig {
        enable_singleton_containers: true,
        container_startup_timeout: Duration::from_micros(100),
        test_execution_timeout: Duration::from_micros(500),
        max_concurrent_containers: 1,
        enable_deterministic_execution: false,
        enable_coverage_tracking: false,
        enable_snapshot_testing: false,
        enable_tracing: false,
        security_policy: SecurityPolicy::default(),
        ..CleanroomConfig::default()
    }
}

/// Fast test configuration for integration tests
pub fn fast_integration_config() -> CleanroomConfig {
    CleanroomConfig {
        enable_singleton_containers: true,
        container_startup_timeout: Duration::from_millis(10),
        test_execution_timeout: Duration::from_millis(50),
        max_concurrent_containers: 2,
        enable_deterministic_execution: false,
        enable_coverage_tracking: false,
        enable_snapshot_testing: false,
        enable_tracing: false,
        security_policy: SecurityPolicy::default(),
        ..CleanroomConfig::default()
    }
}

/// Mock test configuration that avoids real container operations
pub fn mock_test_config() -> CleanroomConfig {
    CleanroomConfig {
        enable_singleton_containers: true,
        container_startup_timeout: Duration::from_micros(1),
        test_execution_timeout: Duration::from_micros(10),
        max_concurrent_containers: 1,
        enable_deterministic_execution: true,
        enable_coverage_tracking: false,
        enable_snapshot_testing: false,
        enable_tracing: false,
        security_policy: SecurityPolicy::default(),
        ..CleanroomConfig::default()
    }
}

/// Performance test configuration with minimal overhead
pub fn performance_test_config() -> CleanroomConfig {
    CleanroomConfig {
        enable_singleton_containers: true,
        container_startup_timeout: Duration::from_millis(1),
        test_execution_timeout: Duration::from_millis(10),
        max_concurrent_containers: 5,
        enable_deterministic_execution: false,
        enable_coverage_tracking: false,
        enable_snapshot_testing: false,
        enable_tracing: false,
        security_policy: SecurityPolicy::default(),
        ..CleanroomConfig::default()
    }
}

/// Concurrent test configuration for parallel execution
pub fn concurrent_test_config() -> CleanroomConfig {
    CleanroomConfig {
        enable_singleton_containers: true,
        container_startup_timeout: Duration::from_millis(5),
        test_execution_timeout: Duration::from_millis(25),
        max_concurrent_containers: 10,
        enable_deterministic_execution: false,
        enable_coverage_tracking: false,
        enable_snapshot_testing: false,
        enable_tracing: false,
        security_policy: SecurityPolicy::default(),
        ..CleanroomConfig::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ultra_fast_config_validation() {
        let config = ultra_fast_test_config();
        assert!(config.validate().is_ok());
        assert_eq!(config.container_startup_timeout, Duration::from_micros(100));
        assert_eq!(config.test_execution_timeout, Duration::from_micros(500));
    }

    #[test]
    fn test_fast_integration_config_validation() {
        let config = fast_integration_config();
        assert!(config.validate().is_ok());
        assert_eq!(config.container_startup_timeout, Duration::from_millis(10));
        assert_eq!(config.test_execution_timeout, Duration::from_millis(50));
    }

    #[test]
    fn test_mock_config_validation() {
        let config = mock_test_config();
        assert!(config.validate().is_ok());
        assert_eq!(config.container_startup_timeout, Duration::from_micros(1));
        assert_eq!(config.test_execution_timeout, Duration::from_micros(10));
    }

    #[test]
    fn test_performance_config_validation() {
        let config = performance_test_config();
        assert!(config.validate().is_ok());
        assert_eq!(config.container_startup_timeout, Duration::from_millis(1));
        assert_eq!(config.test_execution_timeout, Duration::from_millis(10));
    }

    #[test]
    fn test_concurrent_config_validation() {
        let config = concurrent_test_config();
        assert!(config.validate().is_ok());
        assert_eq!(config.container_startup_timeout, Duration::from_millis(5));
        assert_eq!(config.test_execution_timeout, Duration::from_millis(25));
        assert_eq!(config.max_concurrent_containers, 10);
    }
}