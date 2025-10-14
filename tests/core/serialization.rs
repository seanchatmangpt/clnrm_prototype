//! Serialization tests for cleanroom testing framework
//!
//! This module tests JSON and TOML serialization/deserialization of
//! configuration objects and other serializable types.

use clnrm::{
    config::{
        CleanroomConfig, ContainerCustomizer, PerformanceMonitoringConfig, PerformanceThresholds,
    },
    limits::{CpuLimits, DiskLimits, MemoryLimits, NetworkLimits, ResourceLimits},
    policy::{SecurityLevel, SecurityPolicy},
};
use std::{collections::HashMap, time::Duration};

#[test]
fn test_config_json_serialization() {
    let config = CleanroomConfig::default();

    // Test JSON serialization
    let json = serde_json::to_string(&config).unwrap();
    assert!(json.contains("enable_singleton_containers"));
    assert!(json.contains("container_startup_timeout"));
    assert!(json.contains("test_execution_timeout"));

    // Test JSON deserialization
    let deserialized: CleanroomConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(
        deserialized.enable_singleton_containers,
        config.enable_singleton_containers
    );
    assert_eq!(
        deserialized.container_startup_timeout,
        config.container_startup_timeout
    );
}

#[test]
fn test_config_toml_serialization() {
    let config = CleanroomConfig::default();

    // Test TOML serialization
    let toml = toml::to_string(&config).unwrap();
    assert!(toml.contains("enable_singleton_containers"));
    assert!(toml.contains("container_startup_timeout"));

    // Test TOML deserialization
    let deserialized: CleanroomConfig = toml::from_str(&toml.as_str()).unwrap();
    assert_eq!(
        deserialized.enable_singleton_containers,
        config.enable_singleton_containers
    );
}

#[test]
fn test_resource_limits_json_serialization() {
    let limits = ResourceLimits {
        memory: MemoryLimits {
            max_usage_bytes: 1024 * 1024 * 1024,          // 1GB
            max_swap_bytes: Some(2 * 1024 * 1024 * 1024), // 2GB
            max_rss_bytes: Some(512 * 1024 * 1024),       // 512MB
        },
        cpu: CpuLimits {
            max_usage_percent: 75.0,
            max_cores: Some(4),
            priority: 10,
        },
        disk: DiskLimits {
            max_usage_bytes: 50 * 1024 * 1024 * 1024, // 50GB
            max_files: Some(10000),
            max_file_size_bytes: Some(100 * 1024 * 1024), // 100MB
        },
        network: NetworkLimits {
            max_bandwidth_bytes_per_sec: 100 * 1024 * 1024, // 100MB/s
            max_connections: Some(1000),
            max_packets_per_sec: Some(10000),
        },
    };

    // Test JSON serialization
    let json = serde_json::to_string(&limits).unwrap();
    assert!(json.contains("1073741824")); // 1GB in bytes
    assert!(json.contains("75.0")); // CPU percentage

    // Test JSON deserialization
    let deserialized: ResourceLimits = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.memory.max_usage_bytes, 1024 * 1024 * 1024);
    assert_eq!(deserialized.cpu.max_usage_percent, 75.0);
}

#[test]
fn test_security_policy_serialization() {
    let policy = SecurityPolicy {
        security_level: SecurityLevel::High,
        enable_network_isolation: true,
        enable_filesystem_isolation: true,
        allowed_ports: vec![8080, 443, 22],
        allowed_paths: vec!["/tmp".to_string(), "/var/log".to_string()],
        ..Default::default()
    };

    // Test JSON serialization
    let json = serde_json::to_string(&policy).unwrap();
    assert!(json.contains("High"));
    assert!(json.contains("8080"));
    assert!(json.contains("443"));

    // Test JSON deserialization
    let deserialized: SecurityPolicy = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.security_level, SecurityLevel::High);
    assert_eq!(deserialized.allowed_ports, vec![8080, 443, 22]);
}

#[test]
fn test_performance_monitoring_serialization() {
    let monitoring = PerformanceMonitoringConfig {
        enable_monitoring: true,
        metrics_interval: Duration::from_secs(10),
        thresholds: PerformanceThresholds {
            max_cpu_usage_percent: 80.0,
            max_memory_usage_bytes: 2 * 1024 * 1024 * 1024, // 2GB
            max_test_execution_time: Duration::from_secs(5),
            max_container_startup_time: Duration::from_secs(2),
        },
        enable_profiling: true,
        enable_memory_tracking: true,
    };

    // Test JSON serialization
    let json = serde_json::to_string(&monitoring).unwrap();
    assert!(json.contains("true")); // enable_monitoring
    assert!(json.contains("10")); // metrics_interval

    // Test JSON deserialization
    let deserialized: PerformanceMonitoringConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.enable_monitoring, true);
    assert_eq!(deserialized.metrics_interval, Duration::from_secs(10));
}

#[test]
fn test_container_customizer_serialization() {
    use clnrm::config::{ContainerResourceLimits, HealthCheckConfig, PortMapping, VolumeMount};

    let customizer = ContainerCustomizer {
        name: "test_container".to_string(),
        env_vars: HashMap::from([
            ("ENV_VAR1".to_string(), "value1".to_string()),
            ("ENV_VAR2".to_string(), "value2".to_string()),
        ]),
        volume_mounts: vec![VolumeMount {
            host_path: "/host/data".to_string(),
            container_path: "/app/data".to_string(),
            read_only: false,
        }],
        port_mappings: vec![PortMapping {
            container_port: 8080,
            host_port: Some(8080),
            protocol: "tcp".to_string(),
        }],
        resource_limits: ContainerResourceLimits {
            cpu_limit: 1.0,
            memory_limit_bytes: 512 * 1024 * 1024,     // 512MB
            disk_limit_bytes: 1024 * 1024 * 1024,      // 1GB
            network_bandwidth_limit: 50 * 1024 * 1024, // 50MB/s
        },
        health_check: HealthCheckConfig {
            command: "curl -f http://localhost:8080/health".to_string(),
            interval: Duration::from_secs(30),
            timeout: Duration::from_secs(10),
            retries: 3,
            start_period: Duration::from_secs(2),
        },
        init_commands: vec![
            "mkdir -p /app/data".to_string(),
            "chmod 755 /app/data".to_string(),
        ],
    };

    // Test JSON serialization
    let json = serde_json::to_string(&customizer).unwrap();
    assert!(json.contains("test_container"));
    assert!(json.contains("ENV_VAR1"));
    assert!(json.contains("8080"));

    // Test JSON deserialization
    let deserialized: ContainerCustomizer = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.name, "test_container");
    assert_eq!(deserialized.env_vars.len(), 2);
    assert_eq!(deserialized.env_vars["ENV_VAR1"], "value1");
}

#[test]
fn test_serialization_round_trip() {
    // Test that serialization and deserialization preserves all data
    let original_config = CleanroomConfig {
        enable_singleton_containers: false,
        container_startup_timeout: Duration::from_secs(45),
        test_execution_timeout: Duration::from_secs(5),
        max_concurrent_containers: 5,
        enable_deterministic_execution: true,
        deterministic_seed: Some(12345),
        enable_coverage_tracking: false,
        enable_snapshot_testing: true,
        enable_tracing: false,
        resource_limits: ResourceLimits {
            memory: MemoryLimits {
                max_usage_bytes: 256 * 1024 * 1024,      // 256MB
                max_swap_bytes: Some(512 * 1024 * 1024), // 512MB
                max_rss_bytes: Some(128 * 1024 * 1024),  // 128MB
            },
            cpu: CpuLimits {
                max_usage_percent: 25.0,
                max_cores: Some(2),
                priority: 5,
            },
            ..Default::default()
        },
        security_policy: SecurityPolicy {
            security_level: SecurityLevel::Strict,
            enable_network_isolation: true,
            enable_filesystem_isolation: true,
            allowed_ports: vec![80, 443],
            allowed_paths: vec!["/tmp".to_string()],
            ..Default::default()
        },
        performance_monitoring: PerformanceMonitoringConfig {
            enable_monitoring: false,
            metrics_interval: Duration::from_secs(30),
            ..Default::default()
        },
        container_customizers: HashMap::new(),
    };

    // Test JSON round-trip
    let json = serde_json::to_string(&original_config).unwrap();
    let json_restored: CleanroomConfig = serde_json::from_str(&json).unwrap();

    assert_eq!(
        json_restored.enable_singleton_containers,
        original_config.enable_singleton_containers
    );
    assert_eq!(
        json_restored.container_startup_timeout,
        original_config.container_startup_timeout
    );
    assert_eq!(
        json_restored.security_policy.security_level,
        original_config.security_policy.security_level
    );
    assert_eq!(
        json_restored.resource_limits.memory.max_usage_bytes,
        original_config.resource_limits.memory.max_usage_bytes
    );

    // Test TOML round-trip
    let toml = toml::to_string(&original_config).unwrap();
    let toml_restored: CleanroomConfig = toml::from_str(&toml).unwrap();

    assert_eq!(
        toml_restored.enable_singleton_containers,
        original_config.enable_singleton_containers
    );
    assert_eq!(
        toml_restored.container_startup_timeout,
        original_config.container_startup_timeout
    );
    assert_eq!(
        toml_restored.security_policy.security_level,
        original_config.security_policy.security_level
    );
}

#[test]
fn test_duration_serialization() {
    // Test that Duration serializes correctly in different contexts
    let config = CleanroomConfig::default();

    let json = serde_json::to_string(&config).unwrap();
    // JSON doesn't support Duration natively, so it should be serialized as seconds
    assert!(json.contains("30") || json.contains("300")); // Default timeout values

    let toml = toml::to_string(&config).unwrap();
    assert!(toml.contains("30"));
    assert!(toml.contains("300"));
}

#[test]
fn test_complex_nested_serialization() {
    // Test serialization of complex nested structures
    let complex_config = CleanroomConfig {
        enable_singleton_containers: true,
        container_startup_timeout: Duration::from_secs(15),
        test_execution_timeout: Duration::from_secs(3),
        max_concurrent_containers: 8,
        enable_deterministic_execution: true,
        deterministic_seed: Some(98765),
        enable_coverage_tracking: true,
        enable_snapshot_testing: true,
        enable_tracing: true,
        resource_limits: ResourceLimits {
            memory: MemoryLimits {
                max_usage_bytes: 4 * 1024 * 1024 * 1024,      // 4GB
                max_swap_bytes: Some(8 * 1024 * 1024 * 1024), // 8GB
                max_rss_bytes: Some(2 * 1024 * 1024 * 1024),  // 2GB
            },
            cpu: CpuLimits {
                max_usage_percent: 90.0,
                max_cores: Some(8),
                priority: 20,
            },
            disk: DiskLimits {
                max_usage_bytes: 100 * 1024 * 1024 * 1024, // 100GB
                max_files: Some(50000),
                max_file_size_bytes: Some(1024 * 1024 * 1024), // 1GB
            },
            network: NetworkLimits {
                max_bandwidth_bytes_per_sec: 1000 * 1024 * 1024, // 1GB/s
                max_connections: Some(10000),
                max_packets_per_sec: Some(100000),
            },
        },
        security_policy: SecurityPolicy {
            security_level: SecurityLevel::Standard,
            enable_network_isolation: false,
            enable_filesystem_isolation: true,
            allowed_ports: vec![22, 80, 443, 8080],
            allowed_paths: vec![
                "/tmp".to_string(),
                "/var/log".to_string(),
                "/home".to_string(),
            ],
            ..Default::default()
        },
        performance_monitoring: PerformanceMonitoringConfig {
            enable_monitoring: true,
            metrics_interval: Duration::from_secs(5),
            thresholds: PerformanceThresholds {
                max_cpu_usage_percent: 95.0,
                max_memory_usage_bytes: 8 * 1024 * 1024 * 1024, // 8GB
                max_test_execution_time: Duration::from_secs(5),
                max_container_startup_time: Duration::from_secs(45),
            },
            enable_profiling: true,
            enable_memory_tracking: true,
        },
        container_customizers: HashMap::from([(
            "web_server".to_string(),
            ContainerCustomizer {
                name: "web_server".to_string(),
                env_vars: HashMap::from([
                    ("PORT".to_string(), "8080".to_string()),
                    ("ENVIRONMENT".to_string(), "production".to_string()),
                ]),
                volume_mounts: vec![],
                port_mappings: vec![],
                resource_limits: Default::default(),
                health_check: Default::default(),
                init_commands: vec![],
            },
        )]),
    };

    // Test that complex config serializes without errors
    let json = serde_json::to_string(&complex_config).unwrap();
    let toml = toml::to_string(&complex_config).unwrap();

    // Both should contain key elements
    assert!(json.contains("web_server"));
    assert!(json.contains("8080"));
    assert!(toml.contains("web_server"));
    assert!(toml.contains("8080"));

    // Test that deserialization preserves complexity
    let json_restored: CleanroomConfig = serde_json::from_str(&json).unwrap();
    let toml_restored: CleanroomConfig = toml::from_str(&toml).unwrap();

    assert_eq!(json_restored.container_customizers.len(), 1);
    assert_eq!(toml_restored.container_customizers.len(), 1);
    assert!(json_restored
        .container_customizers
        .contains_key("web_server"));
    assert!(toml_restored
        .container_customizers
        .contains_key("web_server"));
}
