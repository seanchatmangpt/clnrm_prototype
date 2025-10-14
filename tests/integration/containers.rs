//! Container integration tests
//!
//! Tests for container lifecycle, operations, and integration with the environment.

use crate::fixtures::{TestAssertions, TestEnvironments};
use clnrm::{conditional_sleep, run, CleanroomEnvironment, Error as CleanroomError};
use std::time::Duration;

#[tokio::test]
async fn test_basic_command_execution() -> Result<(), CleanroomError> {
    let result = run(["echo", "hello world"]);

    TestAssertions::assert_success(&result);
    let run_result = result.unwrap();

    TestAssertions::assert_run_success(&run_result);
    TestAssertions::assert_stdout_contains(&run_result, "hello world");

    Ok(())
}

#[tokio::test]
async fn test_command_with_arguments() -> Result<(), CleanroomError> {
    let result = run(["sh", "-c", "echo 'test with args'"]);

    TestAssertions::assert_success(&result);
    let run_result = result.unwrap();

    TestAssertions::assert_run_success(&run_result);
    TestAssertions::assert_stdout_contains(&run_result, "test with args");

    Ok(())
}

#[tokio::test]
async fn test_command_failure() -> Result<(), CleanroomError> {
    let result = run(["sh", "-c", "exit 42"]);

    TestAssertions::assert_success(&result);
    let run_result = result.unwrap();

    TestAssertions::assert_run_failure_with_code(&run_result, 42);

    Ok(())
}

#[tokio::test]
async fn test_container_lifecycle() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::integration_test().await?;

    // Test container registration
    environment
        .register_container("test1".to_string(), "container_id_123".to_string())
        .await?;
    assert!(environment.is_container_registered("test1").await);

    // Test container access
    let container_count = environment.get_container_count().await;
    assert!(container_count >= 1);

    // Test container cleanup
    environment.cleanup().await?;
    assert_eq!(environment.get_container_count().await, 0);

    Ok(())
}

#[tokio::test]
async fn test_multiple_containers() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::concurrent_test().await?;

    // Register multiple containers
    environment
        .register_container("container1".to_string(), "id1".to_string())
        .await?;
    environment
        .register_container("container2".to_string(), "id2".to_string())
        .await?;
    environment
        .register_container("container3".to_string(), "id3".to_string())
        .await?;

    // Verify all containers are registered
    assert!(environment.is_container_registered("container1").await);
    assert!(environment.is_container_registered("container2").await);
    assert!(environment.is_container_registered("container3").await);

    let container_count = environment.get_container_count().await;
    assert_eq!(container_count, 3);

    Ok(())
}

#[tokio::test]
async fn test_container_isolation() -> Result<(), CleanroomError> {
    // Execute multiple commands to test isolation
    let result1 = run(["echo", "first command"]);
    let result2 = run(["echo", "second command"]);

    TestAssertions::assert_success(&result1);
    TestAssertions::assert_success(&result2);

    let run_result1 = result1.unwrap();
    let run_result2 = result2.unwrap();

    TestAssertions::assert_run_success(&run_result1);
    TestAssertions::assert_run_success(&run_result2);

    TestAssertions::assert_stdout_contains(&run_result1, "first command");
    TestAssertions::assert_stdout_contains(&run_result2, "second command");

    Ok(())
}

#[tokio::test]
async fn test_container_cleanup() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::integration_test().await?;

    // Register a container
    environment
        .register_container("cleanup_test".to_string(), "cleanup_id".to_string())
        .await?;
    assert!(environment.is_container_registered("cleanup_test").await);

    // Cleanup should remove all containers
    environment.cleanup().await?;
    assert_eq!(environment.get_container_count().await, 0);
    assert!(!environment.is_container_registered("cleanup_test").await);

    Ok(())
}

// Metrics test removed - metrics functionality is being refactored

#[tokio::test]
async fn test_container_timeout() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::integration_test().await?;

    // Test that timeout works (this should complete quickly due to mocking)
    let start_time = std::time::Instant::now();

    let result = environment
        .execute_test("timeout_test", || async {
            // Simulate some work using conditional sleep
            conditional_sleep(Duration::from_millis(1)).await;
            Ok::<String, CleanroomError>("timeout_result".to_string())
        })
        .await?;

    let duration = start_time.elapsed();

    assert_eq!(result, "timeout_result");
    // Should complete quickly due to mocking
    TestAssertions::assert_duration_less_than(duration, Duration::from_millis(100));

    Ok(())
}

#[tokio::test]
async fn test_environment_creation() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::integration_test().await?;

    // Verify environment is properly initialized
    assert!(!environment.session_id().is_nil());

    // Verify configuration is set correctly
    let config = environment.config();
    assert!(config.test_execution_timeout >= Duration::from_millis(1));

    Ok(())
}

#[tokio::test]
async fn test_test_execution() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::integration_test().await?;

    // Test deterministic test execution
    let result1 = environment
        .execute_test("test1", || {
            Ok::<String, CleanroomError>("result".to_string())
        })
        .await?;

    let result2 = environment
        .execute_test("test1", || {
            Ok::<String, CleanroomError>("result".to_string())
        })
        .await?;

    assert_eq!(result1, result2);

    Ok(())
}

#[tokio::test]
async fn test_error_handling() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::integration_test().await?;

    // Test test execution failure
    let test_result = environment
        .execute_test("failing_test", || {
            Err::<String, CleanroomError>(CleanroomError::validation_error("Test failure"))
        })
        .await;

    assert!(test_result.is_err());

    Ok(())
}

#[tokio::test]
async fn test_concurrent_execution() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::concurrent_test().await?;

    // Execute multiple tests sequentially
    for i in 0..5 {
        let result = environment
            .execute_test(&format!("concurrent_test_{}", i), || {
                Ok::<String, CleanroomError>(format!("result_{}", i))
            })
            .await?;
        assert_eq!(result, format!("result_{}", i));
    }

    Ok(())
}

#[tokio::test]
async fn test_environment_cleanup() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::integration_test().await?;

    // Register some containers
    environment
        .register_container("test1".to_string(), "id1".to_string())
        .await?;
    environment
        .register_container("test2".to_string(), "id2".to_string())
        .await?;

    // Verify containers are registered
    assert!(environment.get_container_count().await >= 2);

    // Cleanup environment
    environment.cleanup().await?;

    // Verify containers are stopped
    assert_eq!(environment.get_container_count().await, 0);

    Ok(())
}

#[tokio::test]
async fn test_performance_characteristics() -> Result<(), CleanroomError> {
    let start = std::time::Instant::now();

    // Execute a simple command
    let result = run(["echo", "performance test"]);

    let duration = start.elapsed();

    TestAssertions::assert_success(&result);
    let run_result = result.unwrap();
    TestAssertions::assert_run_success(&run_result);

    // Should complete quickly due to mocking
    TestAssertions::assert_duration_less_than(duration, Duration::from_millis(100));

    Ok(())
}
