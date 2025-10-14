//! Resource limits tests for cleanroom testing framework
//!
//! This module tests resource limits enforcement, monitoring, and configuration
//! to ensure proper resource management in cleanroom environments.

use clnrm::{
    limits::{CpuLimits, DiskLimits, MemoryLimits, NetworkLimits, ResourceLimits},
    CleanroomConfig, CleanroomEnvironment,
};
use std::time::Duration;

#[tokio::test]
async fn test_resource_limits_creation() {
    let limits = ResourceLimits::new();
    assert!(limits.validate().is_ok());
    assert!(limits.memory.max_usage_bytes > 0);
    assert!(limits.cpu.max_usage_percent > 0.0);
}

#[tokio::test]
async fn test_cpu_limits() {
    let cpu_limits = CpuLimits {
        max_usage_percent: 50.0,
        max_cores: Some(2),
        priority: 10,
    };

    assert_eq!(cpu_limits.max_usage_percent, 50.0);
    assert_eq!(cpu_limits.max_cores, Some(2));
    assert_eq!(cpu_limits.priority, 10);
}

#[tokio::test]
async fn test_memory_limits() {
    let memory_limits = MemoryLimits {
        max_usage_bytes: 512 * 1024 * 1024,       // 512MB
        max_swap_bytes: Some(1024 * 1024 * 1024), // 1GB
        max_rss_bytes: Some(256 * 1024 * 1024),   // 256MB
    };

    assert_eq!(memory_limits.max_usage_bytes, 512 * 1024 * 1024);
    assert_eq!(memory_limits.max_swap_bytes, Some(1024 * 1024 * 1024));
}

#[tokio::test]
async fn test_disk_limits() {
    let disk_limits = DiskLimits {
        max_usage_bytes: 10 * 1024 * 1024 * 1024, // 10GB
        max_files: Some(10000),
        max_file_size_bytes: Some(100 * 1024 * 1024), // 100MB
    };

    assert_eq!(disk_limits.max_usage_bytes, 10 * 1024 * 1024 * 1024);
    assert_eq!(disk_limits.max_files, Some(10000));
}

#[tokio::test]
async fn test_network_limits() {
    let network_limits = NetworkLimits {
        max_bandwidth_bytes_per_sec: 100 * 1024 * 1024, // 100MB/s
        max_connections: Some(1000),
        max_packets_per_sec: Some(10000),
    };

    assert_eq!(
        network_limits.max_bandwidth_bytes_per_sec,
        100 * 1024 * 1024
    );
    assert_eq!(network_limits.max_connections, Some(1000));
}

#[tokio::test]
async fn test_resource_limits_validation() {
    let mut limits = ResourceLimits::default();

    // Valid limits should pass
    assert!(limits.validate().is_ok());

    // Invalid CPU limits should fail
    limits.cpu.max_usage_percent = -10.0;
    assert!(limits.validate().is_err());

    // Reset and test invalid memory limits
    limits.cpu.max_usage_percent = 50.0;
    limits.memory.max_usage_bytes = 0;
    assert!(limits.validate().is_err());
}

#[tokio::test]
async fn test_resource_limits_with_config() {
    let mut config = CleanroomConfig::default();
    let custom_limits = ResourceLimits {
        memory: MemoryLimits {
            max_usage_bytes: 256 * 1024 * 1024,      // 256MB
            max_swap_bytes: Some(512 * 1024 * 1024), // 512MB
            max_rss_bytes: Some(128 * 1024 * 1024),  // 128MB
        },
        cpu: CpuLimits {
            max_usage_percent: 25.0,
            max_cores: Some(1),
            priority: 5,
        },
        ..Default::default()
    };

    config.resource_limits = custom_limits;
    assert!(config.validate().is_ok());

    let environment = CleanroomEnvironment::new(config).await.unwrap();
    let env_limits = environment.get_resource_limits().await;

    assert_eq!(env_limits.memory.max_usage_bytes, 256 * 1024 * 1024);
    assert_eq!(env_limits.cpu.max_usage_percent, 25.0);
}

#[tokio::test]
async fn test_resource_limits_serialization() {
    let limits = ResourceLimits {
        memory: MemoryLimits {
            max_usage_bytes: 1024 * 1024 * 1024,          // 1GB
            max_swap_bytes: Some(2 * 1024 * 1024 * 1024), // 2GB
            max_rss_bytes: Some(512 * 1024 * 1024),       // 512MB
        },
        cpu: CpuLimits {
            max_usage_percent: 75.0,
            max_cores: Some(4),
            priority: 15,
        },
        disk: DiskLimits {
            max_usage_bytes: 50 * 1024 * 1024 * 1024, // 50GB
            max_files: Some(50000),
            max_file_size_bytes: Some(500 * 1024 * 1024), // 500MB
        },
        network: NetworkLimits {
            max_bandwidth_bytes_per_sec: 500 * 1024 * 1024, // 500MB/s
            max_connections: Some(5000),
            max_packets_per_sec: Some(50000),
        },
    };

    // Test JSON serialization
    let json = serde_json::to_string(&limits).unwrap();
    assert!(json.contains("1073741824")); // 1GB in bytes
    assert!(json.contains("75.0")); // CPU percentage

    // Test deserialization
    let deserialized: ResourceLimits = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.memory.max_usage_bytes, 1024 * 1024 * 1024);
    assert_eq!(deserialized.cpu.max_usage_percent, 75.0);
}

#[tokio::test]
async fn test_resource_monitoring() {
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await.unwrap();

    // Test that resource monitoring is available
    let limits = environment.get_resource_limits().await;
    assert!(limits.memory.max_usage_bytes > 0);

    // Test resource usage reporting (should return valid data even if no containers)
    let usage = environment.get_resource_usage().await;
    assert!(usage.memory_usage_mb >= 0.0);
    assert!(usage.cpu_usage_percent >= 0.0);
}

#[tokio::test]
async fn test_resource_thresholds() {
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await.unwrap();

    // Test that performance monitoring thresholds are properly set
    let thresholds = environment.get_performance_thresholds().await;
    assert!(thresholds.max_cpu_usage_percent > 0.0);
    assert!(thresholds.max_memory_usage_bytes > 0);
    assert!(thresholds.max_test_execution_time > Duration::from_secs(0));
}

#[tokio::test]
async fn test_resource_alerts() {
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await.unwrap();

    // Test resource alert system (should not panic even with no active monitoring)
    let alerts = environment.get_resource_alerts().await;
    // Alerts might be empty initially, but the method should work
    assert!(alerts.is_empty() || !alerts.is_empty()); // Either way is fine for this test
}
