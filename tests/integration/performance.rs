//! Integration performance tests
//!
//! Tests for performance characteristics and behavior under load
//! in integration scenarios.

use crate::fixtures::{TestAssertions, TestEnvironments};
use clnrm::{run, CleanroomEnvironment, Error as CleanroomError};
use std::time::{Duration, Instant};

/// Test performance under concurrent load
#[tokio::test]
async fn test_concurrent_performance() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::concurrent_test().await?;

    let start_time = Instant::now();

    // Execute multiple tests concurrently
    let mut handles = Vec::new();
    for i in 0..10 {
        let env_clone = environment.clone();
        let handle = tokio::spawn(async move {
            env_clone
                .execute_test(&format!("concurrent_test_{}", i), || {
                    Ok::<String, CleanroomError>(format!("result_{}", i))
                })
                .await
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

    // Verify all tests passed
    assert_eq!(results.len(), 10);
    for (i, result) in results.iter().enumerate() {
        assert_eq!(result, &format!("result_{}", i));
    }

    // Performance should be reasonable (less than 1 second for 10 tests)
    TestAssertions::assert_duration_less_than(duration, Duration::from_secs(1));

    Ok(())
}

/// Test memory usage during intensive operations
#[tokio::test]
async fn test_memory_usage_performance() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::integration_test().await?;

    let start_time = Instant::now();

    // Execute memory-intensive operations
    for i in 0..100 {
        let result = environment
            .execute_test(&format!("memory_test_{}", i), || {
                // Simulate memory allocation
                let _data = vec![0u8; 1024]; // 1KB per test
                Ok::<String, CleanroomError>(format!("memory_test_{}", i))
            })
            .await?;

        assert_eq!(result, format!("memory_test_{}", i));
    }

    let duration = start_time.elapsed();

    // Should complete within reasonable time
    TestAssertions::assert_duration_less_than(duration, Duration::from_secs(5));

    Ok(())
}

/// Test performance with large test suites
#[tokio::test]
async fn test_large_test_suite_performance() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::integration_test().await?;

    let start_time = Instant::now();

    // Execute a large number of tests
    for i in 0..50 {
        let result = environment
            .execute_test(&format!("suite_test_{}", i), || {
                Ok::<String, CleanroomError>(format!("suite_result_{}", i))
            })
            .await?;

        assert_eq!(result, format!("suite_result_{}", i));
    }

    let duration = start_time.elapsed();

    // Should scale reasonably (less than 2 seconds for 50 tests)
    TestAssertions::assert_duration_less_than(duration, Duration::from_secs(2));

    Ok(())
}

/// Test Docker command execution performance
#[tokio::test]
async fn test_docker_command_performance() -> Result<(), CleanroomError> {
    // Skip if Docker is not available
    if !std::process::Command::new("docker")
        .arg("--version")
        .output()
        .is_ok()
    {
        println!("Skipping Docker performance test: Docker not available");
        return Ok(());
    }

    let start_time = Instant::now();

    // Execute multiple Docker commands
    for i in 0..20 {
        let result = run(["echo", &format!("docker command {}", i)])?;
        TestAssertions::assert_success(&Ok(result));
        let run_result = result;
        TestAssertions::assert_run_success(&run_result);
        TestAssertions::assert_stdout_contains(&run_result, &format!("docker command {}", i));
    }

    let duration = start_time.elapsed();

    // Should complete within reasonable time
    TestAssertions::assert_duration_less_than(duration, Duration::from_secs(10));

    Ok(())
}
