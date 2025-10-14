//! Example demonstrating structured concurrency with CleanroomEnvironment
//!
//! This example shows how to use the CleanroomEnvironment to manage
//! concurrent tasks with proper cancellation, timeout handling, and resource cleanup.

use clnrm::{CleanroomConfig, CleanroomEnvironment};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Structured Concurrency Example");
    println!("==================================");

    // Create cleanroom environment
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await?;

    // Example 1: Basic concurrent task execution
    println!("\n📋 Example 1: Basic Concurrent Tasks");
    println!("------------------------------------");

    let mut task_ids = Vec::new();
    for i in 0..5 {
        let task_id = environment
            .spawn_task(
                format!("worker_{}", i),
                Box::new(move |mut context| {
                    Box::pin(async move {
                        println!("  🔄 Task {} started", i);

                        // Simulate work
                        tokio::time::sleep(Duration::from_millis(100 * (i + 1))).await;

                        // Check for cancellation
                        if context.is_cancelled() {
                            println!("  ❌ Task {} was cancelled", i);
                            return Err(clnrm::Error::internal_error("Task cancelled"));
                        }

                        println!("  ✅ Task {} completed", i);
                        Ok::<(), clnrm::Error>(())
                    })
                }),
            )
            .await?;
        task_ids.push(task_id);
    }

    println!("  📊 Spawned {} tasks", task_ids.len());
    println!(
        "  ⏳ Active tasks: {}",
        environment.get_active_task_count().await
    );

    // Wait for all tasks to complete
    let results = environment.wait_for_all().await?;
    println!("  📈 Completed {} tasks", results.len());

    let successful = results.iter().filter(|r| r.is_ok()).count();
    println!("  ✅ Successful: {}", successful);

    // Example 2: Task with timeout
    println!("\n⏰ Example 2: Task with Timeout");
    println!("-------------------------------");

    let timeout_task = environment
        .spawn_task_with_timeout(
            "timeout_task".to_string(),
            Duration::from_millis(200),
            |_context| {
                Box::pin(async move {
                    println!("  🔄 Timeout task started");
                    // This will take longer than the timeout
                    tokio::time::sleep(Duration::from_millis(500)).await;
                    println!("  ✅ Timeout task completed (should not reach here)");
                    Ok::<(), clnrm::Error>(())
                })
            },
        )
        .await?;

    let result = environment.wait_for_task(timeout_task).await?;
    match result {
        Ok(_) => println!("  ⏰ Task completed unexpectedly"),
        Err(e) => println!("  ⏰ Task failed/timed out: {}", e),
        _ => println!("  ❌ Unexpected result: {:?}", result),
    }

    // Example 3: Task cancellation
    println!("\n🛑 Example 3: Task Cancellation");
    println!("--------------------------------");

    let cancellable_task = environment
        .spawn_task(
            "cancellable_task".to_string(),
            Box::new(|mut context| {
                Box::pin(async move {
                    println!("  🔄 Cancellable task started");

                    // Check for cancellation in a loop
                    for i in 0..10 {
                        if context.is_cancelled() {
                            println!("  🛑 Cancellable task received cancellation signal at iteration {}", i);
                            return Ok::<(), clnrm::Error>(());
                        }
                        tokio::time::sleep(Duration::from_millis(50)).await;
                    }

                    println!("  ✅ Cancellable task completed normally");
                    Ok::<(), clnrm::Error>(())
                })
            }),
        )
        .await?;

    // Cancel the task after a short delay
    tokio::time::sleep(Duration::from_millis(200)).await;
    environment.cancel_task(cancellable_task).await?;

    let result = environment.wait_for_task(cancellable_task).await?;
    match result {
        Ok(_) => println!("  ✅ Task completed successfully"),
        Err(e) => println!("  🛑 Task was cancelled: {}", e),
    }

    // Example 4: Environment cleanup
    println!("\n🧹 Example 4: Environment Cleanup");
    println!("--------------------------------");

    // Get final metrics
    let metrics = environment.get_metrics().await;
    println!("  📊 Final metrics:");
    println!("     Tests executed: {}", metrics.tests_executed);
    println!("     Tests passed: {}", metrics.tests_passed);
    println!("     Tests failed: {}", metrics.tests_failed);
    println!("     Total duration: {}ms", metrics.total_duration_ms);

    // Cleanup environment
    environment.cleanup().await?;
    println!("  ✅ Environment cleaned up successfully");

    println!("\n🎯 All concurrency examples completed successfully!");
    Ok(())
}
