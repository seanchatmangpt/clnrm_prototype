//! Simple cleanroom usage example
//!
//! This example demonstrates basic usage of the cleanroom testing framework.

use clnrm::{run, CleanroomConfig, CleanroomEnvironment, CleanroomGuard};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Simple Cleanroom Usage Example");
    println!("{}", "=".repeat(40));

    // 1. Basic command execution using the simple run() function
    println!("\n📦 1. Basic Command Execution");
    let result = run(["echo", "Hello from Cleanroom!"])?;
    println!("✅ Command executed successfully");
    println!("   Exit code: {}", result.exit_code);
    println!("   Output: {}", result.stdout.trim());
    println!("   Backend: {}", result.backend);
    println!("   Duration: {}ms", result.duration_ms);

    // 2. Using CleanroomEnvironment for more control
    println!("\n🔧 2. Using CleanroomEnvironment");
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await?;

    // Create a guard for automatic cleanup
    let _guard = CleanroomGuard::new(Arc::new(environment.clone()));

    // Execute a simple test function
    let test_result = environment
        .execute_test("simple_test", || {
            println!("Running simple test...");
            Ok::<String, clnrm::Error>("test passed".to_string())
        })
        .await?;

    println!("✅ Test result: {}", test_result);

    // Get metrics
    let metrics = environment.get_metrics().await;
    println!("📊 Metrics:");
    println!("   Tests executed: {}", metrics.tests_executed);
    println!("   Tests passed: {}", metrics.tests_passed);
    println!("   Tests failed: {}", metrics.tests_failed);
    println!("   Total duration: {}ms", metrics.total_duration_ms);

    // 3. Execute another command to show metrics update
    println!("\n🔄 3. Executing Another Test");
    let _result2 = environment
        .execute_test("another_test", || {
            println!("Running another test...");
            Ok::<i32, clnrm::Error>(42)
        })
        .await?;

    let updated_metrics = environment.get_metrics().await;
    println!("📊 Updated Metrics:");
    println!("   Tests executed: {}", updated_metrics.tests_executed);
    println!("   Tests passed: {}", updated_metrics.tests_passed);
    println!(
        "   Success rate: {:.1}%",
        (updated_metrics.tests_passed as f64 / updated_metrics.tests_executed as f64) * 100.0
    );

    println!("\n✅ All examples completed successfully!");
    Ok(())
}
