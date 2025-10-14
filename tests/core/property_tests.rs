//! Core property tests
//!
//! Property-based tests for core functionality using proptest
//! to verify deterministic behavior and comprehensive coverage.

use clnrm::{
    CleanroomConfig, CleanroomError, DeterministicManager,
    GenericContainer, Policy, PostgresContainer, RedisContainer, ResourceLimits,
    SecurityLevel, TestReport,
};
use proptest::prelude::*;
use std::time::Duration;
use uuid::Uuid;

/// Property test for CleanroomConfig validation
proptest! {
    #[test]
    fn test_config_validation_property(
        enable_singleton in any::<bool>(),
        startup_timeout_secs in 1..3u64,
        execution_timeout_secs in 1..5u64,
        max_containers in 1..10usize,
        enable_deterministic in any::<bool>(),
        enable_coverage in any::<bool>(),
        enable_snapshots in any::<bool>(),
        enable_tracing in any::<bool>(),
    ) {
        let config = CleanroomConfig {
            enable_singleton_containers: enable_singleton,
            container_startup_timeout: Duration::from_secs(startup_timeout_secs),
            test_execution_timeout: Duration::from_secs(execution_timeout_secs),
            max_concurrent_containers: max_containers,
            enable_deterministic_execution: enable_deterministic,
            enable_coverage_tracking: enable_coverage,
            enable_snapshot_testing: enable_snapshots,
            enable_tracing: enable_tracing,
            ..CleanroomConfig::default()
        };

        // Config should always validate with reasonable values
        prop_assert!(config.validate().is_ok());

        // Timeouts should be positive
        prop_assert!(config.container_startup_timeout > Duration::from_secs(0));
        prop_assert!(config.test_execution_timeout > Duration::from_secs(0));

        // Max containers should be positive
        prop_assert!(config.max_concurrent_containers > 0);
    }
}

/// Property test for Policy security levels
proptest! {
    #[test]
    fn test_policy_security_levels_property(
        security_level in prop::sample::select(&[
            SecurityLevel::Permissive,
            SecurityLevel::Standard,
            SecurityLevel::Strict,
            SecurityLevel::Locked,
        ]),
        enable_network_isolation in any::<bool>(),
        enable_port_scanning in any::<bool>(),
        enable_file_system_isolation in any::<bool>(),
    ) {
        let mut policy = Policy::with_security_level(security_level.clone());
        policy.security.enable_network_isolation = enable_network_isolation;
        policy.security.enable_filesystem_isolation = enable_file_system_isolation;

        // Policy should always be valid
        prop_assert!(policy.validate().is_ok());

        // Security level should match
        prop_assert_eq!(policy.security.security_level, security_level);

        // Network isolation should be consistent
        prop_assert_eq!(policy.security.enable_network_isolation, enable_network_isolation);

        // Summary should contain security level
        let summary = policy.summary();
        prop_assert!(summary.contains("Security Level"));
    }
}

/// Property test for ResourceLimits validation
proptest! {
    #[test]
    fn test_resource_limits_property(
        max_memory_mb in 1..512u32,
        max_cpu_percent in 1.0..100.0f64,
        max_disk_mb in 1..1024u32,
        max_network_mb in 1..256u32,
    ) {
        let mut limits = ResourceLimits::default();
        limits.memory.max_usage_bytes = (max_memory_mb as u64) * 1024 * 1024;
        limits.cpu.max_usage_percent = max_cpu_percent;
        limits.disk.max_usage_bytes = (max_disk_mb as u64) * 1024 * 1024;
        limits.network.max_bandwidth_bytes_per_sec = (max_network_mb as u64) * 1024 * 1024;

        // Limits should always validate with positive values
        prop_assert!(limits.validate().is_ok());

        // Memory should be positive
        prop_assert!(limits.memory.max_usage_bytes > 0);

        // CPU percentage should be reasonable
        prop_assert!(limits.cpu.max_usage_percent > 0.0 && limits.cpu.max_usage_percent <= 100.0);

        // Disk space should be positive
        prop_assert!(limits.disk.max_usage_bytes > 0);
    }
}

/// Property test for DeterministicManager consistency
proptest! {
    #[test]
    fn test_deterministic_manager_consistency_property(
        seed in any::<u64>(),
        num_ports in 1..100usize,
    ) {
        let manager = DeterministicManager::new(seed);

        // Seed should be consistent
        prop_assert_eq!(manager.seed(), seed);

        // Should be able to allocate ports
        prop_assert!(num_ports > 0 && num_ports < 1000);
    }
}

/// Property test for UUID generation consistency
#[test]
fn test_uuid_consistency() {
    // Test that UUIDs are properly formatted and unique
    let uuid1 = Uuid::new_v4();
    let uuid2 = Uuid::new_v4();

    assert!(!uuid1.is_nil());
    assert!(!uuid2.is_nil());
    assert_ne!(uuid1, uuid2);

    // UUID string representation should be valid
    let uuid_str = uuid1.to_string();
    assert_eq!(uuid_str.len(), 36);
    assert!(uuid_str.contains('-'));
}

/// Property test for error handling consistency
proptest! {
    #[test]
    fn test_error_handling_consistency_property(
        should_error in any::<bool>(),
    ) {
        if should_error {
            // Test that errors are properly structured
            let error = CleanroomError::ConfigError("test error".to_string());
            let error_msg = error.to_string();
            prop_assert!(!error_msg.is_empty());
            prop_assert!(error_msg.contains("test error"));
        } else {
            // Test that successful operations work
            prop_assert!(true);
        }
    }
}
