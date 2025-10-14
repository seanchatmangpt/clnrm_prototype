//! Tests for test utilities functionality
//!
//! This module contains tests for test environment builders, generators,
//! and assertion utilities.

use clnrm::cleanroom::ContainerStatus;
use clnrm::metrics_builder::ContainerMetricsBuilder;
use clnrm::test_utils::{assertions, generators, TestEnvironmentBuilder};
use std::time::{Duration, Instant};

#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::get_first
)]
#[tokio::test]
async fn test_environment_builder() {
    let env = TestEnvironmentBuilder::new()
        .with_singleton_containers(true)
        .with_timeout(Duration::from_secs(30))
        .build()
        .await
        .unwrap();

    // Test that environment was created successfully
    assert!(env.get_container_count().await > 0);
}

#[test]
fn test_generators() {
    let start_time = Instant::now();
    let metrics = generators::test_metrics("postgres", &start_time);
    assert_eq!(metrics.cpu_usage_percent, 5.0);

    let status = generators::test_status("running");
    assert_eq!(status, ContainerStatus::Running);

    let name = generators::test_container_name("test", 1);
    assert_eq!(name, "test_1");

    let conn_str = generators::test_connection_string("localhost", 5432, "testdb");
    assert!(conn_str.contains("postgresql://"));
}

#[test]
fn test_assertions() {
    let start_time = Instant::now();
    let metrics = ContainerMetricsBuilder::postgres(&start_time);

    assertions::assert_metrics_reasonable(&metrics)
        .unwrap_or_else(|e| panic!("Metrics should be reasonable: {}", e));
    assertions::assert_minimum_uptime(&metrics, 0)
        .unwrap_or_else(|e| panic!("Minimum uptime assertion should pass: {}", e));
    assertions::assert_memory_range(&metrics, 100, 200)
        .unwrap_or_else(|e| panic!("Memory range assertion should pass: {}", e));
}
