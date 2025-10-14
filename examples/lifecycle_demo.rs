//! Lifecycle Management System Demo
//!
//! Demonstrates the complete cleanroom lifecycle management:
//! - Environment initialization
//! - Test execution in cleanroom
//! - Metrics collection
//! - Resource cleanup

use clnrm::{CleanroomConfig, CleanroomEnvironment};
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🚀 Lifecycle Management System Demo\n");

    // 1. Initialize cleanroom configuration
    println!("📋 Step 1: Initialize Cleanroom Configuration");
    let config = CleanroomConfig::default();
    println!("   ✓ Created cleanroom config");
    println!("   ✓ Test timeout: {:?}", config.test_execution_timeout);
    println!("   ✓ Security level: {:?}\n", config.security_policy.security_level);

    // 2. Create cleanroom environment
    println!("🏗️  Step 2: Create Cleanroom Environment");
    let environment = Arc::new(CleanroomEnvironment::new(config).await?);
    println!("   ✓ Cleanroom environment created");
    println!("   ✓ Session ID: {}\n", environment.session_id());

    // 3. Execute tests
    println!("🧪 Step 3: Execute Tests");
    for i in 1..=3 {
        let result = environment
            .execute_test(&format!("lifecycle_test_{}", i), || {
                println!("   → Executing test {}", i);
                Ok::<i32, clnrm::Error>(i * 10)
            })
            .await?;
        println!("   ✓ Test {} completed with result: {}\n", i, result);
    }

    // 4. Collect metrics
    println!("📊 Step 4: Collect Metrics");
    let metrics = environment.get_metrics().await;
    println!("   ✓ Tests executed: {}", metrics.tests_executed);
    println!("   ✓ Tests passed: {}", metrics.tests_passed);
    println!("   ✓ Tests failed: {}", metrics.tests_failed);
    println!("   ✓ Total duration: {}ms\n", metrics.total_duration_ms);

    // 5. Cleanup
    println!("🧹 Step 5: Cleanup");
    environment.cleanup().await?;
    println!("   ✓ Environment cleaned up successfully\n");

    println!("🎉 Lifecycle Management Demo Completed Successfully!");
    Ok(())
}