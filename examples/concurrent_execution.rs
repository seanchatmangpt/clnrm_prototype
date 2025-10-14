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

    // Example 4: Shared state coordination
    println!("\n🤝 Example 4: Shared State Coordination");
    println!("----------------------------------------");

    let coordinator_task = orchestrator
        .spawn_task(
            "coordinator".to_string(),
            Box::new(|context| {
                Box::pin(async move {
                    println!("  🔄 Coordinator task started");

                    // Set shared state
                    context
                        .set_shared_state(
                            "phase".to_string(),
                            serde_json::Value::String("setup".to_string()),
                        )
                        .await;
                    context
                        .set_shared_state(
                            "count".to_string(),
                            serde_json::Value::Number(serde_json::Number::from(0)),
                        )
                        .await;

                    sleep(Duration::from_millis(50)).await;

                    // Update shared state
                    context
                        .set_shared_state(
                            "phase".to_string(),
                            serde_json::Value::String("running".to_string()),
                        )
                        .await;

                    println!("  ✅ Coordinator task completed");
                    Ok::<(), clnrm::error::CleanroomError>(())
                })
            }),
        )
        .await?;

    let worker_task = orchestrator
        .spawn_task(
            "worker".to_string(),
            Box::new(|context| {
                Box::pin(async move {
                    println!("  🔄 Worker task started");

                    // Wait a bit for coordinator to set up
                    sleep(Duration::from_millis(25)).await;

                    // Read shared state
                    if let Some(phase) = context.get_shared_state("phase").await {
                        println!("  📊 Current phase: {}", phase);
                    }

                    if let Some(count) = context.get_shared_state("count").await {
                        println!("  📊 Current count: {}", count);
                    }

                    println!("  ✅ Worker task completed");
                    Ok::<(), clnrm::error::CleanroomError>(())
                })
            }),
        )
        .await?;

    // Wait for both tasks
    let _ = orchestrator.wait_for_task(coordinator_task).await?;
    let _ = orchestrator.wait_for_task(worker_task).await?;

    // Example 5: Error handling and recovery
    println!("\n🔧 Example 5: Error Handling and Recovery");
    println!("------------------------------------------");

    let error_task = orchestrator
        .spawn_task(
            "error_task".to_string(),
            Box::new(|_context| {
                Box::pin(async move {
                    println!("  🔄 Error task started");
                    sleep(Duration::from_millis(50)).await;
                    println!("  ❌ Error task failed");
                    Err(clnrm::error::CleanroomError::internal_error(
                        "Simulated error".to_string(),
                    ))
                })
            }),
        )
        .await?;

    let result = orchestrator.wait_for_task(error_task).await?;
    match result {
        TaskResult::Failure(error) => {
            println!("  ✅ Error task failed as expected: {}", error);
        }
        _ => println!("  ❌ Unexpected result: {:?}", result),
    }

    // Example 6: Priority-based task execution
    println!("\n🎯 Example 6: Priority-based Task Execution");
    println!("-------------------------------------------");

    // Spawn tasks with different priorities
    let low_priority_task = orchestrator
        .spawn_task(
            "low_priority".to_string(),
            Box::new(|_context| {
                Box::pin(async move {
                    println!("  🔄 Low priority task started");
                    sleep(Duration::from_millis(100)).await;
                    println!("  ✅ Low priority task completed");
                    Ok::<(), clnrm::error::CleanroomError>(())
                })
            }),
        )
        .await?;

    let high_priority_task = orchestrator
        .spawn_task(
            "high_priority".to_string(),
            Box::new(|_context| {
                Box::pin(async move {
                    println!("  🔄 High priority task started");
                    sleep(Duration::from_millis(50)).await;
                    println!("  ✅ High priority task completed");
                    Ok::<(), clnrm::error::CleanroomError>(())
                })
            }),
        )
        .await?;

    // Wait for both tasks
    let _ = orchestrator.wait_for_task(high_priority_task).await?;
    let _ = orchestrator.wait_for_task(low_priority_task).await?;

    // Final statistics
    println!("\n📊 Final Statistics");
    println!("===================");

    let stats = orchestrator.get_stats().await;
    println!("  📈 Total tasks: {}", stats.total_tasks);
    println!("  ✅ Completed: {}", stats.tasks_completed);
    println!("  ❌ Failed: {}", stats.tasks_failed);
    println!("  🛑 Cancelled: {}", stats.tasks_cancelled);
    println!("  ⏰ Timed out: {}", stats.tasks_timed_out);
    println!("  📊 Peak concurrent: {}", stats.peak_concurrent_tasks);
    println!("  ⏱️  Average duration: {:.2}ms", stats.average_duration_ms);

    println!("\n🎉 Structured concurrency orchestrator example completed!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_orchestrator_basic_functionality() {
        let mut orchestrator = ConcurrencyOrchestrator::new();

        let task_id = orchestrator
            .spawn_task(
                "test_task".to_string(),
                Box::new(|_context| {
                    Box::pin(async move { Ok::<(), clnrm::error::CleanroomError>(()) })
                }),
            )
            .await
            .unwrap();

        let result = orchestrator.wait_for_task(task_id).await.unwrap();
        assert!(result.is_success());
    }

    #[tokio::test]
    async fn test_orchestrator_timeout() {
        let mut orchestrator = ConcurrencyOrchestrator::new();

        let task_id = orchestrator
            .spawn_task_with_timeout(
                "timeout_test".to_string(),
                Duration::from_millis(50),
                Box::new(|_context| {
                    Box::pin(async move {
                        sleep(Duration::from_millis(100)).await;
                        Ok::<(), clnrm::error::CleanroomError>(())
                    })
                }),
            )
            .await
            .unwrap();

        let result = orchestrator.wait_for_task(task_id).await.unwrap();
        assert!(result.is_timed_out());
    }

    #[tokio::test]
    async fn test_orchestrator_cancellation() {
        let mut orchestrator = ConcurrencyOrchestrator::new();

        let task_id = orchestrator
            .spawn_task(
                "cancellable_test".to_string(),
                Box::new(|mut context| {
                    Box::pin(async move {
                        context.wait_for_cancellation().await;
                        Ok::<(), clnrm::error::CleanroomError>(())
                    })
                }),
            )
            .await
            .unwrap();

        orchestrator.cancel_task(task_id).await.unwrap();
        let result = orchestrator.wait_for_task(task_id).await.unwrap();
        assert!(result.is_cancelled());
    }
}
