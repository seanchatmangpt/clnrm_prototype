//! # Production Usage Examples for clnrm
//!
//! This example demonstrates comprehensive production-ready usage patterns for the
//! cleanroom testing framework, including security policies, observability, error
//! handling, and best practices for enterprise deployments.

use clnrm::{
    CleanroomBuilder, CleanroomConfig, CleanroomEnvironment, LogLevel, MetricType, Policy,
    PostgresContainer, RedisContainer, ResourceLimits, Result, SecurityLevel, SecurityPolicy,
    SpanStatus, TracingManager,
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

/// Example 1: Production Environment Setup
///
/// Demonstrates how to set up a production-ready cleanroom environment with:
/// - Strict security policies
/// - Resource limits
/// - Observability and tracing
/// - Error handling
async fn production_environment_setup() -> Result<Arc<CleanroomEnvironment>> {
    println!("=== Example 1: Production Environment Setup ===\n");

    // Configure strict security policy for production
    let security_policy = SecurityPolicy {
        enable_network_isolation: true,
        enable_filesystem_isolation: true,
        enable_process_isolation: true,
        security_level: SecurityLevel::Locked,
        allowed_ports: vec![5432, 6379, 8080], // Only allow specific ports
        blocked_commands: vec!["rm".to_string(), "format".to_string(), "dd".to_string()],
        enable_data_redaction: true,
        enable_audit_logging: true,
        ..Default::default()
    };

    // Configure resource limits to prevent resource exhaustion
    let resource_limits = ResourceLimits {
        max_cpu_usage_percent: 80.0,
        max_memory_usage_bytes: 8_589_934_592, // 8GB
        max_disk_usage_bytes: 107_374_182_400, // 100GB
        max_container_count: 10,
        max_test_execution_time: Duration::from_secs(600), // 10 minutes
        ..Default::default()
    };

    // Build production environment with type-safe builder
    let environment = CleanroomBuilder::new()
        .with_timeout(Duration::from_secs(300))
        .with_security_policy(security_policy)
        .with_resource_limits(resource_limits)
        .with_deterministic_execution(Some(42)) // Reproducible tests
        .with_coverage_tracking(true)
        .with_snapshot_testing(true)
        .with_tracing(true)
        .build()
        .await?;

    println!("Production environment created successfully");
    println!("Session ID: {}", environment.session_id());
    println!("Configuration validated\n");

    Ok(Arc::new(environment))
}

/// Example 2: Database Integration Testing
///
/// Demonstrates how to:
/// - Use singleton container pattern for performance
/// - Handle database initialization
/// - Run integration tests with proper cleanup
async fn database_integration_testing() -> Result<()> {
    println!("=== Example 2: Database Integration Testing ===\n");

    let environment = production_environment_setup().await?;

    // Create PostgreSQL container (singleton pattern - reused across tests)
    let postgres = environment
        .get_or_create_container("postgres", || {
            PostgresContainer::new(&environment.backend(), "testdb", "testuser", "testpass")
        })
        .await;

    match postgres {
        Ok(_) => {
            println!("PostgreSQL container created (first call takes 30-60s)");
            println!("Subsequent calls reuse container (2-5ms)\n");
        }
        Err(e) => {
            eprintln!("Failed to create PostgreSQL container: {}", e);
            return Err(e);
        }
    }

    // Run database integration test
    let result = environment
        .execute_test("database_integration", || {
            println!("Running database integration test...");
            // Your database test logic here
            Ok("Database integration test passed".to_string())
        })
        .await?;

    println!("Test result: {}\n", result);

    Ok(())
}

/// Example 3: Distributed Tracing and Observability
///
/// Demonstrates comprehensive observability with:
/// - Hierarchical spans
/// - Metrics collection
/// - Structured logging
/// - Performance monitoring
async fn observability_example() -> Result<()> {
    println!("=== Example 3: Distributed Tracing and Observability ===\n");

    let session_id = Uuid::new_v4();
    let tracing_manager = TracingManager::new(session_id);

    // Start root span for request
    let request_span_id = tracing_manager
        .start_span("http_request".to_string(), None)
        .await?;
    tracing_manager
        .add_span_tag("http_request", "method".to_string(), "POST".to_string())
        .await?;
    tracing_manager
        .add_span_tag(
            "http_request",
            "endpoint".to_string(),
            "/api/users".to_string(),
        )
        .await?;

    // Start child span for database query
    let db_span_id = tracing_manager
        .start_span("database_query".to_string(), Some(request_span_id))
        .await?;

    // Add event to database span
    let mut event_data = HashMap::new();
    event_data.insert(
        "query".to_string(),
        "SELECT * FROM users WHERE id = ?".to_string(),
    );
    tracing_manager
        .add_span_event("database_query", "query_start".to_string(), event_data)
        .await?;

    // Record query duration metric
    tracing_manager
        .record_metric(
            "database_query_duration_ms".to_string(),
            42.5,
            MetricType::Histogram,
            {
                let mut tags = HashMap::new();
                tags.insert("table".to_string(), "users".to_string());
                tags.insert("operation".to_string(), "SELECT".to_string());
                tags
            },
            Some("ms".to_string()),
        )
        .await?;

    // Complete database span
    tracing_manager
        .end_span("database_query", SpanStatus::Completed)
        .await?;

    // Start child span for cache lookup
    let cache_span_id = tracing_manager
        .start_span("cache_lookup".to_string(), Some(request_span_id))
        .await?;

    // Record cache hit metric
    tracing_manager
        .record_metric(
            "cache_hits_total".to_string(),
            1.0,
            MetricType::Counter,
            {
                let mut tags = HashMap::new();
                tags.insert("cache_type".to_string(), "redis".to_string());
                tags
            },
            Some("count".to_string()),
        )
        .await?;

    // Log cache hit
    tracing_manager
        .log(
            LogLevel::Info,
            "Cache hit for user data".to_string(),
            Some("cache.rs:123".to_string()),
            {
                let mut tags = HashMap::new();
                tags.insert("cache_key".to_string(), "user:123".to_string());
                tags
            },
            {
                let mut metadata = HashMap::new();
                metadata.insert("ttl_seconds".to_string(), "3600".to_string());
                metadata
            },
        )
        .await?;

    // Complete cache span
    tracing_manager
        .end_span("cache_lookup", SpanStatus::Completed)
        .await?;

    // Complete request span
    tracing_manager
        .end_span("http_request", SpanStatus::Completed)
        .await?;

    // Generate comprehensive tracing report
    let report = tracing_manager.generate_tracing_report().await?;

    println!("Tracing Report:");
    println!("  Session ID: {}", report.session_id);
    println!("  Total spans: {}", report.statistics.total_spans);
    println!("  Completed spans: {}", report.statistics.completed_spans);
    println!(
        "  Average span duration: {:.2}ms",
        report.statistics.average_span_duration_ms
    );
    println!("  Total metrics: {}", report.statistics.total_metrics);
    println!("  Total logs: {}", report.statistics.total_logs);

    if !report.recommendations.is_empty() {
        println!("\nRecommendations:");
        for recommendation in &report.recommendations {
            println!("  - {}", recommendation);
        }
    }

    println!();
    Ok(())
}

/// Example 4: Error Handling and Recovery
///
/// Demonstrates production-grade error handling with:
/// - Granular error types
/// - Retry logic
/// - Circuit breaker pattern
/// - Graceful degradation
async fn error_handling_example() -> Result<()> {
    println!("=== Example 4: Error Handling and Recovery ===\n");

    let environment = production_environment_setup().await?;

    // Attempt operation with retry logic
    let max_retries = 3;
    let mut retry_count = 0;

    loop {
        match environment
            .execute_test("flaky_test", || {
                println!("Attempt {} of {}", retry_count + 1, max_retries);

                // Simulate flaky test
                if retry_count < 2 {
                    Err(clnrm::Error::execution_error("Transient failure"))
                } else {
                    Ok("Test passed after retry".to_string())
                }
            })
            .await
        {
            Ok(result) => {
                println!("Success: {}\n", result);
                break;
            }
            Err(clnrm::Error::Execution(msg)) if retry_count < max_retries - 1 => {
                retry_count += 1;
                println!("Retrying due to error: {}", msg);
                tokio::time::sleep(Duration::from_millis(100 * (retry_count as u64))).await;
                continue;
            }
            Err(e) => {
                eprintln!("Failed after {} retries: {}\n", retry_count + 1, e);
                return Err(e);
            }
        }
    }

    Ok(())
}

/// Example 5: Concurrent Test Execution
///
/// Demonstrates how to run multiple tests concurrently with:
/// - Structured concurrency
/// - Task orchestration
/// - Resource management
/// - Result aggregation
async fn concurrent_testing_example() -> Result<()> {
    println!("=== Example 5: Concurrent Test Execution ===\n");

    let environment = production_environment_setup().await?;

    // Spawn multiple concurrent tasks
    let task1 = environment
        .spawn_task_with_timeout(
            "integration_test_1".to_string(),
            Duration::from_secs(30),
            |_ctx| {
                Box::pin(async move {
                    println!("Running integration test 1...");
                    tokio::time::sleep(Duration::from_millis(100)).await;
                    Ok(42)
                })
            },
        )
        .await?;

    let task2 = environment
        .spawn_task_with_timeout(
            "integration_test_2".to_string(),
            Duration::from_secs(30),
            |_ctx| {
                Box::pin(async move {
                    println!("Running integration test 2...");
                    tokio::time::sleep(Duration::from_millis(150)).await;
                    Ok(84)
                })
            },
        )
        .await?;

    let task3 = environment
        .spawn_task_with_timeout(
            "integration_test_3".to_string(),
            Duration::from_secs(30),
            |_ctx| {
                Box::pin(async move {
                    println!("Running integration test 3...");
                    tokio::time::sleep(Duration::from_millis(200)).await;
                    Ok(126)
                })
            },
        )
        .await?;

    println!(
        "Spawned {} concurrent tasks",
        environment.get_active_task_count().await
    );

    // Wait for all tasks to complete
    let results = environment.wait_for_all_tasks().await?;

    println!("All tasks completed:");
    for (i, result) in results.iter().enumerate() {
        match result.result {
            Ok(_) => println!(
                "  Task {} completed successfully in {:?}",
                i + 1,
                result.duration
            ),
            Err(ref e) => println!("  Task {} failed: {}", i + 1, e),
        }
    }

    // Get orchestrator statistics
    let stats = environment.get_orchestrator_stats().await;
    println!("\nOrchestrator Statistics:");
    println!("  Total tasks: {}", stats.total_tasks);
    println!("  Completed tasks: {}", stats.completed_tasks);
    println!("  Failed tasks: {}", stats.failed_tasks);
    println!("  Average task duration: {:?}", stats.average_task_duration);

    println!();
    Ok(())
}

/// Example 6: Health Monitoring and Metrics
///
/// Demonstrates system health monitoring with:
/// - Health checks
/// - Metrics collection
/// - Performance monitoring
/// - Alerting thresholds
async fn health_monitoring_example() -> Result<()> {
    println!("=== Example 6: Health Monitoring and Metrics ===\n");

    let environment = production_environment_setup().await?;

    // Check environment health
    let is_healthy = environment.is_healthy().await;
    println!(
        "Environment health: {}",
        if is_healthy { "HEALTHY" } else { "UNHEALTHY" }
    );

    // Get detailed health status
    let health_status = environment.get_health_status().await;
    println!("Health status: {:?}", health_status);

    // Run some tests to generate metrics
    for i in 1..=5 {
        environment
            .execute_test(&format!("test_{}", i), || {
                Ok(format!("Test {} completed", i))
            })
            .await?;
    }

    // Get comprehensive metrics
    let metrics = environment.get_metrics().await;

    println!("\nEnvironment Metrics:");
    println!("  Session ID: {}", metrics.session_id);
    println!("  Tests executed: {}", metrics.tests_executed);
    println!("  Tests passed: {}", metrics.tests_passed);
    println!("  Tests failed: {}", metrics.tests_failed);
    println!(
        "  Success rate: {:.2}%",
        (metrics.tests_passed as f64 / metrics.tests_executed as f64) * 100.0
    );
    println!("  Total duration: {}ms", metrics.total_duration_ms);
    println!(
        "  Average execution time: {:?}",
        metrics.average_execution_time
    );
    println!(
        "  Peak memory usage: {} bytes",
        metrics.peak_memory_usage_bytes
    );
    println!("  Peak CPU usage: {:.2}%", metrics.peak_cpu_usage_percent);
    println!("  Containers created: {}", metrics.containers_created);
    println!("  Containers destroyed: {}", metrics.containers_destroyed);

    // Check resource usage
    println!("\nResource Usage:");
    println!("  CPU: {:.2}%", metrics.resource_usage.cpu_usage_percent);
    println!(
        "  Memory: {} bytes",
        metrics.resource_usage.memory_usage_bytes
    );
    println!("  Disk: {} bytes", metrics.resource_usage.disk_usage_bytes);
    println!(
        "  Network sent: {} bytes",
        metrics.resource_usage.network_bytes_sent
    );
    println!(
        "  Network received: {} bytes",
        metrics.resource_usage.network_bytes_received
    );
    println!(
        "  Active containers: {}",
        metrics.resource_usage.container_count
    );

    println!();
    Ok(())
}

/// Example 7: Multi-Container Orchestration
///
/// Demonstrates orchestrating multiple services with:
/// - PostgreSQL database
/// - Redis cache
/// - Service dependencies
/// - Health checks
async fn multi_container_orchestration() -> Result<()> {
    println!("=== Example 7: Multi-Container Orchestration ===\n");

    let environment = production_environment_setup().await?;

    // Start PostgreSQL (database)
    println!("Starting PostgreSQL...");
    let postgres_result = environment
        .get_or_create_container("postgres", || {
            PostgresContainer::new(&environment.backend(), "testdb", "testuser", "testpass")
        })
        .await;

    match postgres_result {
        Ok(_) => println!("PostgreSQL ready"),
        Err(e) => {
            eprintln!("PostgreSQL failed: {}", e);
            return Err(e);
        }
    }

    // Start Redis (cache)
    println!("Starting Redis...");
    let redis_result = environment
        .get_or_create_container("redis", || {
            RedisContainer::new(&environment.backend(), Some("redis_password".to_string()))
        })
        .await;

    match redis_result {
        Ok(_) => println!("Redis ready"),
        Err(e) => {
            eprintln!("Redis failed: {}", e);
            return Err(e);
        }
    }

    // Run integration test with both services
    let result = environment
        .execute_test("multi_service_integration", || {
            println!("Running integration test with PostgreSQL and Redis...");
            // Your integration test logic here
            Ok("Multi-service integration test passed".to_string())
        })
        .await?;

    println!("Test result: {}", result);

    // Get container count
    let container_count = environment.get_container_count().await;
    println!("Active containers: {}", container_count);

    println!();
    Ok(())
}

/// Main function demonstrating all production examples
#[tokio::main]
async fn main() -> Result<()> {
    println!("Cleanroom Production Usage Examples\n");
    println!("=====================================\n");

    // Run all examples
    if let Err(e) = production_environment_setup().await {
        eprintln!("Example 1 failed: {}", e);
    }

    if let Err(e) = database_integration_testing().await {
        eprintln!("Example 2 failed: {}", e);
    }

    if let Err(e) = observability_example().await {
        eprintln!("Example 3 failed: {}", e);
    }

    if let Err(e) = error_handling_example().await {
        eprintln!("Example 4 failed: {}", e);
    }

    if let Err(e) = concurrent_testing_example().await {
        eprintln!("Example 5 failed: {}", e);
    }

    if let Err(e) = health_monitoring_example().await {
        eprintln!("Example 6 failed: {}", e);
    }

    if let Err(e) = multi_container_orchestration().await {
        eprintln!("Example 7 failed: {}", e);
    }

    println!("All examples completed!");
    println!("\nKey Takeaways:");
    println!("  1. Use CleanroomBuilder for type-safe configuration");
    println!("  2. Enable strict security policies for production");
    println!("  3. Configure resource limits to prevent exhaustion");
    println!("  4. Use singleton containers for 10-50x performance");
    println!("  5. Implement comprehensive error handling with retries");
    println!("  6. Use structured concurrency for parallel testing");
    println!("  7. Monitor health and metrics for observability");
    println!("  8. Use distributed tracing for complex workflows");

    Ok(())
}
