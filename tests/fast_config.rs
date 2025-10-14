//! Fast test configuration for optimized test execution
//!
//! This module provides optimized configurations for fast test execution
//! to meet the 10-second total runtime target.

use clnrm::CleanroomConfig;
use std::time::Duration;

/// Ultra-fast test configuration for maximum speed
pub fn ultra_fast_config() -> CleanroomConfig {
    CleanroomConfig {
        enable_singleton_containers: true,
        container_startup_timeout: Duration::from_millis(1),
        test_execution_timeout: Duration::from_millis(5),
        max_concurrent_containers: 1,
        enable_deterministic_execution: false,
        enable_coverage_tracking: false,
        enable_snapshot_testing: false,
        enable_tracing: false,
        security_policy: clnrm::policy::SecurityPolicy::default(),
        resource_limits: clnrm::ResourceLimits::default(),
    }
}

/// Fast test configuration for most tests
pub fn fast_config() -> CleanroomConfig {
    CleanroomConfig {
        enable_singleton_containers: true,
        container_startup_timeout: Duration::from_millis(5),
        test_execution_timeout: Duration::from_millis(10),
        max_concurrent_containers: 2,
        enable_deterministic_execution: false,
        enable_coverage_tracking: false,
        enable_snapshot_testing: false,
        enable_tracing: false,
        security_policy: clnrm::policy::SecurityPolicy::default(),
        resource_limits: clnrm::ResourceLimits::default(),
    }
}

/// Minimal test configuration for integration tests
pub fn minimal_config() -> CleanroomConfig {
    CleanroomConfig {
        enable_singleton_containers: true,
        container_startup_timeout: Duration::from_millis(10),
        test_execution_timeout: Duration::from_millis(50),
        max_concurrent_containers: 3,
        enable_deterministic_execution: false,
        enable_coverage_tracking: false,
        enable_snapshot_testing: false,
        enable_tracing: false,
        security_policy: clnrm::policy::SecurityPolicy::default(),
        resource_limits: clnrm::ResourceLimits::default(),
    }
}
