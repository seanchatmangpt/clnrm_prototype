//! Examples demonstrating observability setup
//!
//! This example shows how to set up comprehensive observability for the
//! cleanroom environment using tracing, metrics, and span collection.

use clnrm::{
    CleanroomConfig, CleanroomEnvironment, CleanroomBuilder,
    ObservabilityLayer, TracingLevel, Metrics, TracingManager
};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Cleanroom Observability Setup Examples");
    println!("=======================================");

    // Example 1: Basic observability layer setup
    println!("\n1. Basic Observability Layer Setup");
    let layer = ObservabilityLayer::new()
        .with_tracing_level(TracingLevel::Info)
        .with_sampling_rate(1.0);

    println!("✓ Observability layer created");
    println!("✓ Tracing level: {:?}", layer.tracing_config().level);
    println!(
        "✓ Sampling rate: {:.1}",
        layer.tracing_config().sampling_rate
    );

    // Example 2: Advanced observability configuration
    println!("\n2. Advanced Observability Configuration");
    let environment = CleanroomBuilder::new()
        .with_timeout(Duration::from_secs(30))
        .build()
        .await?;

    let layer = ObservabilityLayer::new()
        .with_tracing_level(TracingLevel::Debug)
        .with_tokio_console()
        .with_distributed_tracing()
        .with_sampling_rate(0.8);

    let manager = layer.attach(&environment)?;
    println!("✓ Advanced observability layer attached");
    println!(
        "✓ Tokio console integration: {}",
        manager.tracing_config.tokio_console
    );
    println!(
        "✓ Distributed tracing: {}",
        manager.tracing_config.distributed_tracing
    );

    // Example 3: Tracing manager usage
    println!("\n3. Tracing Manager Usage");
    let session_id = Uuid::new_v4();
    let tracing_manager = TracingManager::new(session_id);
    
    println!("✓ Tracing manager created with session ID: {}", session_id);
    println!("✓ Tracing enabled: {}", tracing_manager.is_enabled());

    // Example 4: Basic span operations
    println!("\n4. Basic Span Operations");
    let span_id = tracing_manager
        .start_span("test_span".to_string(), None)
        .await?;
    
    println!("✓ Started span: {}", span_id);
    
    // Simulate some work
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    tracing_manager
        .end_span("test_span", clnrm::tracing::SpanStatus::Completed)
        .await?;
    
    println!("✓ Ended span: test_span");

    // Example 5: Metrics collection
    println!("\n5. Metrics Collection");
    let metrics = Metrics {
        timestamp: Instant::now(),
        session_id,
        resource_usage: clnrm::observability::ResourceUsageMetrics {
            cpu_usage_percent: 25.5,
            memory_usage_bytes: 1024 * 1024 * 512, // 512 MB
            disk_usage_bytes: 1024 * 1024 * 1024,  // 1 GB
            network_bytes_sent: 1024 * 100,
            network_bytes_received: 1024 * 200,
        },
        performance: clnrm::observability::PerformanceMetrics {
            avg_test_execution_time: Duration::from_millis(150),
            container_startup_time: Duration::from_millis(2000),
            total_execution_time: Duration::from_secs(30),
            throughput: 10.5,
        },
        containers: clnrm::observability::ContainerMetrics {
            total_created: 5,
            total_destroyed: 3,
            currently_running: 2,
            avg_lifetime: Duration::from_secs(60),
        },
        tests: clnrm::observability::TestMetrics {
            total_executed: 100,
            total_passed: 95,
            total_failed: 5,
            avg_execution_time: Duration::from_millis(120),
        },
    };

    println!("✓ Metrics collected:");
    println!("  - CPU usage: {:.1}%", metrics.resource_usage.cpu_usage_percent);
    println!("  - Memory usage: {} MB", metrics.resource_usage.memory_usage_bytes / (1024 * 1024));
    println!("  - Tests executed: {}", metrics.tests.total_executed);
    println!("  - Success rate: {:.1}%", 
        (metrics.tests.total_passed as f64 / metrics.tests.total_executed as f64) * 100.0);

    println!("\n✓ All observability examples completed successfully!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_observability_layer_creation() {
        let layer = ObservabilityLayer::new();
        assert!(layer.metrics_config().enabled);
        assert!(layer.tracing_config().enabled);
    }

    #[tokio::test]
    async fn test_observability_layer_configuration() {
        let layer = ObservabilityLayer::new()
            .with_tracing_level(TracingLevel::Debug)
            .with_sampling_rate(0.5)
            .with_tokio_console()
            .with_distributed_tracing();

        assert_eq!(layer.tracing_config().level, TracingLevel::Debug);
        assert_eq!(layer.tracing_config().sampling_rate, 0.5);
        assert!(layer.tracing_config().tokio_console);
        assert!(layer.tracing_config().distributed_tracing);
    }

    #[tokio::test]
    async fn test_tracing_manager() {
        let session_id = Uuid::new_v4();
        let manager = TracingManager::new(session_id);
        
        assert_eq!(manager.session_id, session_id);
        assert!(manager.is_enabled());
    }

    #[tokio::test]
    async fn test_span_operations() {
        let session_id = Uuid::new_v4();
        let manager = TracingManager::new(session_id);
        
        let span_id = manager
            .start_span("test_span".to_string(), None)
            .await
            .unwrap();
        
        assert!(!span_id.is_empty());
        
        manager
            .end_span("test_span", clnrm::tracing::SpanStatus::Completed)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_metrics_creation() {
        let session_id = Uuid::new_v4();
        let metrics = Metrics {
            timestamp: Instant::now(),
            session_id,
            resource_usage: clnrm::observability::ResourceUsageMetrics {
                cpu_usage_percent: 50.0,
                memory_usage_bytes: 1024 * 1024,
                disk_usage_bytes: 2 * 1024 * 1024,
                network_bytes_sent: 1000,
                network_bytes_received: 2000,
            },
            performance: clnrm::observability::PerformanceMetrics {
                avg_test_execution_time: Duration::from_millis(100),
                container_startup_time: Duration::from_millis(1500),
                total_execution_time: Duration::from_secs(20),
                throughput: 15.0,
            },
            containers: clnrm::observability::ContainerMetrics {
                total_created: 3,
                total_destroyed: 1,
                currently_running: 2,
                avg_lifetime: Duration::from_secs(45),
            },
            tests: clnrm::observability::TestMetrics {
                total_executed: 50,
                total_passed: 48,
                total_failed: 2,
                avg_execution_time: Duration::from_millis(80),
            },
        };

        assert_eq!(metrics.session_id, session_id);
        assert_eq!(metrics.resource_usage.cpu_usage_percent, 50.0);
        assert_eq!(metrics.tests.total_executed, 50);
    }
}