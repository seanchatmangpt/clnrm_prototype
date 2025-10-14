//! Performance stress tests
//!
//! Tests that stress the system under high load to identify
//! performance bottlenecks and stability issues.

use clnrm::{run, CleanroomEnvironment, Error as CleanroomError};
use crate::fixtures::{TestEnvironments, TestAssertions};
use std::time::{Duration, Instant};

/// Test system under high concurrent load
#[tokio::test]
async fn test_high_concurrent_load() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::concurrent_test().await?;

    let start_time = Instant::now();

    // Execute many tests concurrently
    let mut handles = Vec::new();
    for i in 0..100 {
        let env_clone = environment.clone();
        let handle = tokio::spawn(async move {
            env_clone.execute_test(&format!("stress_test_{}", i), || {
                Ok::<String, CleanroomError>(format!("stress_result_{}", i))
            }).await
        });
        handles.push(handle);
    }

    // Wait for all tests to complete
    let mut results = Vec::new();
    for handle in handles {
        let result = handle.await.unwrap()?;
        results.push(result);
    }

    let duration = start_time.elapsed();

    // Verify all tests completed
    assert_eq!(results.len(), 100);

    // Should complete within reasonable time (under 5 seconds)
    TestAssertions::assert_duration_less_than(duration, Duration::from_secs(5));

    Ok(())
}

/// Test system under sustained load
#[tokio::test]
async fn test_sustained_load() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::stress_test().await?;

    let start_time = Instant::now();

    // Execute tests continuously for a period
    for batch in 0..10 {
        let mut handles = Vec::new();
        for i in 0..20 {
            let env_clone = environment.clone();
            let test_id = batch * 20 + i;
            let handle = tokio::spawn(async move {
                env_clone.execute_test(&format!("sustained_test_{}", test_id), || {
                    Ok::<String, CleanroomError>(format!("sustained_result_{}", test_id))
                }).await
            });
            handles.push(handle);
        }

        // Wait for batch to complete
        for handle in handles {
            let result = handle.await.unwrap()?;
            assert!(result.starts_with("sustained_result_"));
        }
    }

    let duration = start_time.elapsed();

    // Should complete within reasonable time (under 10 seconds for 200 tests)
    TestAssertions::assert_duration_less_than(duration, Duration::from_secs(10));

    Ok(())
}

/// Test resource exhaustion scenarios
#[tokio::test]
async fn test_resource_exhaustion() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::stress_test().await?;

    let start_time = Instant::now();

    // Try to register many containers
    for i in 0..1000 {
        let result = environment.register_container(
            format!("exhaustion_container_{}", i),
            format!("exhaustion_id_{}", i)
        ).await;

        // Some may fail due to resource limits, which is expected
        if let Err(e) = result {
            println!("Expected resource exhaustion at container {}: {}", i, e);
            break;
        }
    }

    let duration = start_time.elapsed();

    // Should handle resource exhaustion gracefully
    TestAssertions::assert_duration_less_than(duration, Duration::from_secs(5));

    Ok(())
}

/// Test system recovery after stress
#[tokio::test]
async fn test_stress_recovery() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::stress_test().await?;

    // Stress the system
    let mut handles = Vec::new();
    for i in 0..50 {
        let env_clone = environment.clone();
        let handle = tokio::spawn(async move {
            env_clone.execute_test(&format!("recovery_test_{}", i), || {
                Ok::<String, CleanroomError>(format!("recovery_result_{}", i))
            }).await
        });
        handles.push(handle);
    }

    // Wait for stress tests to complete
    for handle in handles {
        let result = handle.await.unwrap()?;
        assert!(result.starts_with("recovery_result_"));
    }

    // System should still be responsive after stress
    let recovery_start = Instant::now();
    let result = environment.execute_test("post_stress_test", || {
        Ok::<String, CleanroomError>("post_stress_result".to_string())
    }).await?;

    let recovery_duration = recovery_start.elapsed();

    assert_eq!(result, "post_stress_result");
    // Should recover quickly
    TestAssertions::assert_duration_less_than(recovery_duration, Duration::from_millis(100));

    Ok(())
}

/// Test Docker stress scenarios
#[tokio::test]
async fn test_docker_stress() -> Result<(), CleanroomError> {
    // Skip if Docker is not available
    if !std::process::Command::new("docker")
        .arg("--version")
        .output()
        .is_ok()
    {
        println!("Skipping Docker stress test: Docker not available");
        return Ok(());
    }

    let start_time = Instant::now();

    // Execute many Docker commands rapidly
    for i in 0..50 {
        let result = run(["echo", &format!("docker stress test {}", i)])?;
        TestAssertions::assert_success(&result);
    }

    let duration = start_time.elapsed();

    // Should handle Docker stress reasonably
    TestAssertions::assert_duration_less_than(duration, Duration::from_secs(15));

    Ok(())
}
