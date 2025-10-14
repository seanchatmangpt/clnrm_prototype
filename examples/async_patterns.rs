//! Examples demonstrating async execution patterns
//!
//! This example shows how to use the CleanroomEnvironment with various async patterns,
//! including timeout handling, cancellation, and concurrent execution.

use clnrm::{CleanroomConfig, CleanroomEnvironment};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Cleanroom Async Patterns Examples");
    println!("==================================");

    // Create environment
    let mut config = CleanroomConfig::default();
    config.test_execution_timeout = Duration::from_secs(60);
    let environment = CleanroomEnvironment::new(config).await?;

    // Example 1: Basic async execution
    println!("\n1. Basic Async Execution");
    let result = environment
        .execute_test("async_test", || {
            // Simulate async work
            std::thread::sleep(Duration::from_millis(100));
            Ok::<i32, clnrm::Error>(42)
        })
        .await?;

    println!("✓ Result: {}", result);

    // Example 2: Multiple test executions
    println!("\n2. Multiple Test Executions");
    for i in 1..=3 {
        let result = environment
            .execute_test(&format!("test_{}", i), || {
                std::thread::sleep(Duration::from_millis(50));
                Ok::<i32, clnrm::Error>(i)
            })
            .await?;
        println!("✓ Test {} result: {}", i, result);
    }

    // Example 3: Get metrics
    println!("\n3. Metrics Collection");
    let metrics = environment.get_metrics().await;
    println!("✓ Tests executed: {}", metrics.tests_executed);
    println!("✓ Tests passed: {}", metrics.tests_passed);
    println!("✓ Total duration: {}ms", metrics.total_duration_ms);

    // Example 4: Error handling
    println!("\n4. Error Handling");
    let result = environment
        .execute_test("failing_test", || {
            Err::<i32, clnrm::Error>(clnrm::Error::validation_error("Test failure"))
        })
        .await;
    
    match result {
        Ok(_) => println!("✗ Expected failure but got success"),
        Err(e) => println!("✓ Expected failure: {}", e),
    }

    println!("\n=== All Async Patterns Examples Completed Successfully ===");
    Ok(())
}