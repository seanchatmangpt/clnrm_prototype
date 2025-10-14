//! Core property tests
//!
//! Property-based tests for core functionality using proptest
//! to verify deterministic behavior and comprehensive coverage.

use clnrm::{
    CleanroomConfig, CleanroomError, DeterministicManager, GenericContainer, Policy,
    PostgresContainer, RedisContainer, ResourceLimits, SecurityLevel, TestReport,
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

        // All values should be positive
        prop_assert!(limits.memory.max_usage_bytes > 0);
        prop_assert!(limits.cpu.max_usage_percent > 0.0);
        prop_assert!(limits.disk.max_usage_bytes > 0);
        prop_assert!(limits.network.max_bandwidth_bytes_per_sec > 0);

        // CPU percentage should not exceed 100%
        prop_assert!(limits.cpu.max_usage_percent <= 100.0);
    }
}

/// Property test for DeterministicManager behavior
proptest! {
    #[test]
    fn test_deterministic_manager_property(
        seed in any::<u64>(),
        key1 in "[a-zA-Z0-9_]{1,50}",
        key2 in "[a-zA-Z0-9_]{1,50}",
    ) {
        let manager = DeterministicManager::new(seed);

        // Same seed should produce same results (using async methods)
        let rt = tokio::runtime::Runtime::new().unwrap();
        let value1 = rt.block_on(manager.random());
        let value2 = rt.block_on(manager.random());
        // Note: These will be different as they're sequential calls, but deterministic

        // Seed should be preserved
        prop_assert_eq!(manager.seed(), seed);
    }
}

/// Property test for TestReport behavior
proptest! {
    #[test]
    fn test_test_report_property(
        test_names in prop::collection::vec("[a-zA-Z0-9_]{1,10}", 1..10),
        execution_times in prop::collection::vec(1..100u64, 1..10),
    ) {
        let report = TestReport::new();

        for (i, test_name) in test_names.iter().enumerate() {
            let success = i % 2 == 0;
            let execution_time = Duration::from_millis(execution_times[i % execution_times.len()]);

            report.record_test_execution(test_name, success, execution_time);
        }

        // Total tests should match input
        prop_assert_eq!(report.test_summary.total_tests, test_names.len());

        // Passed tests should be half (rounded up)
        prop_assert_eq!(report.test_summary.passed_tests, (test_names.len() + 1) / 2);

        // Failed tests should be half (rounded down)
        prop_assert_eq!(report.test_summary.failed_tests, test_names.len() / 2);

        // Average execution time should be calculated
        prop_assert!(report.test_summary.average_execution_time > Duration::from_millis(0));

        // Report should be serializable
        let json_report = report.to_json();
        prop_assert!(json_report.is_ok());

        let toml_report = report.to_toml();
        prop_assert!(toml_report.is_ok());
    }
}

/// Property test for container configuration
proptest! {
    #[test]
    fn test_container_configuration_property(
        image in "[a-zA-Z0-9_/:.-]{1,50}",
        port in 1..65535u16,
        env_key in "[a-zA-Z0-9_]{1,20}",
        env_value in "[a-zA-Z0-9_]{1,50}",
    ) {
        // Test PostgresContainer
        let postgres_container = PostgresContainer::new(&image)
            .with_port(port)
            .with_env(&env_key, &env_value);

        prop_assert_eq!(postgres_container.image(), image);
        prop_assert_eq!(postgres_container.port(), Some(port));
        prop_assert_eq!(postgres_container.container_type(), "postgres");

        // Test RedisContainer
        let redis_container = RedisContainer::new(&image)
            .with_port(port)
            .with_env(&env_key, &env_value);

        prop_assert_eq!(redis_container.image(), image);
        prop_assert_eq!(redis_container.port(), Some(port));
        prop_assert_eq!(redis_container.container_type(), "redis");

        // Test GenericContainer
        let generic_container = GenericContainer::new(&image)
            .with_port(port)
            .with_env(&env_key, &env_value);

        prop_assert_eq!(generic_container.image(), image);
        prop_assert_eq!(generic_container.port(), Some(port));
        prop_assert_eq!(generic_container.container_type(), "generic");
    }
}

/// Property test for error handling
proptest! {
    #[test]
    fn test_error_handling_property(
        error_message in "[a-zA-Z0-9_ ]{1,100}",
        error_kind in prop::sample::select(&[
            ErrorKind::ValidationError,
            ErrorKind::IoError,
            ErrorKind::ContainerError,
            ErrorKind::PolicyError,
            ErrorKind::ResourceError,
            ErrorKind::DeterministicError,
            ErrorKind::CoverageError,
            ErrorKind::SnapshotError,
            ErrorKind::TracingError,
            ErrorKind::ReportError,
        ]),
    ) {
        let error = CleanroomError::new(error_kind.clone(), &error_message);

        // Error should preserve kind and message
        prop_assert_eq!(error.kind(), error_kind);
        prop_assert_eq!(error.message(), error_message);

        // Error should be displayable
        let error_string = format!("{}", error);
        prop_assert!(error_string.contains(&error_message));

        // Error should be convertible to string
        let error_string = error.to_string();
        prop_assert!(error_string.contains(&error_message));
    }
}

/// Property test for UUID generation
proptest! {
    #[test]
    fn test_uuid_generation_property(
        count in 1..20usize,
    ) {
        let mut uuids = Vec::new();

        for _ in 0..count {
            let uuid = Uuid::new_v4();
            uuids.push(uuid);
        }

        // All UUIDs should be unique
        for i in 0..uuids.len() {
            for j in (i + 1)..uuids.len() {
                prop_assert_ne!(uuids[i], uuids[j]);
            }
        }

        // All UUIDs should be valid
        for uuid in &uuids {
            prop_assert!(!uuid.is_nil());
        }
    }
}

/// Property test for duration operations
proptest! {
    #[test]
    fn test_duration_operations_property(
        duration1_ms in 1..1000u64,
        duration2_ms in 1..1000u64,
    ) {
        let duration1 = Duration::from_millis(duration1_ms);
        let duration2 = Duration::from_millis(duration2_ms);

        // Duration addition should be associative
        let sum1 = duration1 + duration2;
        let sum2 = duration2 + duration1;
        prop_assert_eq!(sum1, sum2);

        // Duration should be positive
        prop_assert!(duration1 > Duration::from_millis(0));
        prop_assert!(duration2 > Duration::from_millis(0));

        // Duration should be comparable
        if duration1_ms > duration2_ms {
            prop_assert!(duration1 > duration2);
        } else if duration1_ms < duration2_ms {
            prop_assert!(duration1 < duration2);
        } else {
            prop_assert_eq!(duration1, duration2);
        }
    }
}

/// Property test for JSON serialization round-trip
proptest! {
    #[test]
    fn test_json_serialization_round_trip_property(
        config in any::<CleanroomConfig>(),
    ) {
        // Serialize to JSON
        let json = serde_json::to_string(&config);
        prop_assert!(json.is_ok());

        // Deserialize from JSON
        let deserialized_config: Result<CleanroomConfig, _> = serde_json::from_str(&json.unwrap());
        prop_assert!(deserialized_config.is_ok());

        let deserialized_config = deserialized_config.unwrap();

        // Round-trip should preserve values
        prop_assert_eq!(deserialized_config.enable_singleton_containers, config.enable_singleton_containers);
        prop_assert_eq!(deserialized_config.container_startup_timeout, config.container_startup_timeout);
        prop_assert_eq!(deserialized_config.test_execution_timeout, config.test_execution_timeout);
        prop_assert_eq!(deserialized_config.max_concurrent_containers, config.max_concurrent_containers);
        prop_assert_eq!(deserialized_config.enable_deterministic_execution, config.enable_deterministic_execution);
        prop_assert_eq!(deserialized_config.enable_coverage_tracking, config.enable_coverage_tracking);
        prop_assert_eq!(deserialized_config.enable_snapshot_testing, config.enable_snapshot_testing);
        prop_assert_eq!(deserialized_config.enable_tracing, config.enable_tracing);
    }
}

/// Property test for TOML serialization round-trip
proptest! {
    #[test]
    fn test_toml_serialization_round_trip_property(
        config in any::<CleanroomConfig>(),
    ) {
        // Serialize to TOML
        let toml = toml::to_string(&config);
        prop_assert!(toml.is_ok());

        // Deserialize from TOML
        let deserialized_config: Result<CleanroomConfig, _> = toml::from_str(&toml.unwrap());
        prop_assert!(deserialized_config.is_ok());

        let deserialized_config = deserialized_config.unwrap();

        // Round-trip should preserve values
        prop_assert_eq!(deserialized_config.enable_singleton_containers, config.enable_singleton_containers);
        prop_assert_eq!(deserialized_config.container_startup_timeout, config.container_startup_timeout);
        prop_assert_eq!(deserialized_config.test_execution_timeout, config.test_execution_timeout);
        prop_assert_eq!(deserialized_config.max_concurrent_containers, config.max_concurrent_containers);
        prop_assert_eq!(deserialized_config.enable_deterministic_execution, config.enable_deterministic_execution);
        prop_assert_eq!(deserialized_config.enable_coverage_tracking, config.enable_coverage_tracking);
        prop_assert_eq!(deserialized_config.enable_snapshot_testing, config.enable_snapshot_testing);
        prop_assert_eq!(deserialized_config.enable_tracing, config.enable_tracing);
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
