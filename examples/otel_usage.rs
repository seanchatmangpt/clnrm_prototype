//! OpenTelemetry usage example for cleanroom testing
//!
//! This example demonstrates how to use OpenTelemetry for tracing, metrics, and logging
//! in cleanroom testing scenarios.

use clnrm::{
    conditional_sleep, CleanroomConfig, CleanroomEnvironment, ExportersConfig, LogLevel,
    OtelConfig, OtelManager, SpanStatus,
};
use opentelemetry::KeyValue;
use std::time::Duration;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize OpenTelemetry
    let otel_config = OtelConfig {
        service_name: "cleanroom-example".to_string(),
        service_version: "0.2.0".to_string(),
        environment: "development".to_string(),
        exporters: ExportersConfig::development(),
        ..OtelConfig::default()
    };

    let otel = OtelManager::new(otel_config).await?;

    // Create cleanroom environment
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await?;
    let session_id = environment.session_id();

    // Example 1: Container operations with tracing
    example_container_operations(&otel, &session_id).await?;

    // Example 2: Test execution with metrics
    example_test_execution(&otel, &session_id).await?;

    // Example 3: Scenario execution with logging
    example_scenario_execution(&otel, &session_id).await?;

    // Example 4: Error handling and observability
    example_error_handling(&otel, &session_id).await?;

    // Shutdown OpenTelemetry
    otel.shutdown().await?;

    Ok(())
}

async fn example_container_operations(
    otel: &OtelManager,
    session_id: &Uuid,
) -> Result<(), Box<dyn std::error::Error>> {
    let container_id = "example-container-123";

    // Start tracing container operations
    let span = otel.tracing.start_container_span("start", container_id);

    // Log container startup
    otel.logging.log_container_operation(
        LogLevel::Info,
        "start",
        container_id,
        "Starting container operation",
    );

    // Simulate container startup
    conditional_sleep(Duration::from_millis(100)).await;

    // Record metrics
    otel.metrics
        .record_container_operation("start", container_id);
    otel.metrics
        .record_container_startup_time(Duration::from_millis(100), container_id);
    otel.metrics
        .record_container_memory_usage(512.0, container_id);
    otel.metrics.record_container_cpu_usage(25.0, container_id);

    // Add span event
    otel.tracing.add_span_event(
        &span,
        "container_started",
        vec![
            KeyValue::new("container_id", container_id),
            KeyValue::new("startup_time_ms", 100),
        ],
    );

    // Set span status
    otel.tracing.set_span_status(&span, SpanStatus::Ok);

    // Span ends automatically when dropped
    drop(span);

    println!("Container operations example completed");
    Ok(())
}

async fn example_test_execution(
    otel: &OtelManager,
    session_id: &Uuid,
) -> Result<(), Box<dyn std::error::Error>> {
    let test_name = "integration_test";
    let session_id_str = session_id.to_string();

    // Start tracing test execution
    let span = otel.tracing.start_test_span(test_name, session_id);

    // Log test start
    otel.logging.log_test_operation(
        LogLevel::Info,
        test_name,
        &session_id_str,
        "Starting test execution",
    );

    // Record test execution
    otel.metrics
        .record_test_execution(test_name, &session_id_str);

    // Simulate test execution
    let start_time = std::time::Instant::now();
    conditional_sleep(Duration::from_millis(200)).await;
    let duration = start_time.elapsed();

    // Record test metrics
    otel.metrics
        .record_test_execution_time(duration, test_name, &session_id_str);

    // Simulate test success
    otel.metrics.record_test_success(test_name, &session_id_str);

    // Log test completion
    otel.logging.log_test_operation(
        LogLevel::Info,
        test_name,
        &session_id_str,
        "Test execution completed successfully",
    );

    // Add performance logging
    otel.logging.log_performance(
        "test_execution",
        duration,
        vec![
            KeyValue::new("test_name", test_name),
            KeyValue::new("session_id", session_id_str),
        ],
    );

    // Set span status
    otel.tracing.set_span_status(&span, SpanStatus::Ok);

    println!("Test execution example completed");
    Ok(())
}

async fn example_scenario_execution(
    otel: &OtelManager,
    session_id: &Uuid,
) -> Result<(), Box<dyn std::error::Error>> {
    let scenario_name = "deployment_scenario";
    let session_id_str = session_id.to_string();

    // Start tracing scenario execution
    let span = otel.tracing.start_scenario_span(scenario_name, "setup");

    // Log scenario start
    otel.logging.log_scenario_operation(
        LogLevel::Info,
        scenario_name,
        "setup",
        "Starting scenario execution",
    );

    // Record scenario execution
    otel.metrics
        .record_scenario_execution(scenario_name, &session_id_str);

    // Simulate scenario steps
    let steps = vec!["setup", "deploy", "verify", "cleanup"];
    let start_time = std::time::Instant::now();

    for step in steps {
        let step_span = otel.tracing.start_scenario_span(scenario_name, step);

        otel.logging.log_scenario_operation(
            LogLevel::Info,
            scenario_name,
            step,
            &format!("Executing step: {}", step),
        );

        // Simulate step execution
        conditional_sleep(Duration::from_millis(50)).await;

        otel.logging.log_scenario_operation(
            LogLevel::Info,
            scenario_name,
            step,
            &format!("Step completed: {}", step),
        );

        drop(step_span);
    }

    let duration = start_time.elapsed();

    // Record scenario metrics
    otel.metrics
        .record_scenario_execution_time(duration, scenario_name, &session_id_str);

    // Log scenario completion
    otel.logging.log_scenario_operation(
        LogLevel::Info,
        scenario_name,
        "complete",
        "Scenario execution completed successfully",
    );

    // Set span status
    otel.tracing.set_span_status(&span, SpanStatus::Ok);

    println!("Scenario execution example completed");
    Ok(())
}

async fn example_error_handling(
    otel: &OtelManager,
    session_id: &Uuid,
) -> Result<(), Box<dyn std::error::Error>> {
    let container_id = "error-container-456";

    // Start tracing with error handling
    let span = otel.tracing.start_container_span("start", container_id);

    // Simulate an error
    let result: Result<(), Box<dyn std::error::Error>> = Err("Container startup failed".into());

    match result {
        Ok(_) => {
            otel.tracing.set_span_status(&span, SpanStatus::Ok);
            otel.logging.log_container_operation(
                LogLevel::Info,
                "start",
                container_id,
                "Container started successfully",
            );
        }
        Err(e) => {
            // Record error in span
            otel.tracing.add_span_error(&span, &e);

            // Log error
            otel.logging.log_error(
                &e,
                vec![
                    KeyValue::new("container_id", container_id),
                    KeyValue::new("operation", "start"),
                ],
            );

            // Record failure metrics
            otel.metrics.record_test_failure(
                "container_startup",
                &session_id.to_string(),
                &e.to_string(),
            );
        }
    }

    println!("Error handling example completed");
    Ok(())
}
