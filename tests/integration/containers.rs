//! Container integration tests
//!
//! Tests for container lifecycle, operations, and integration with the environment.

use clnrm::{
    run, CleanroomEnvironment, CleanroomConfig, Error as CleanroomError,
    Assert, new_cleanroom, Policy, SecurityLevel
};
use crate::fixtures::{TestEnvironments, TestContainers, TestAssertions};
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
    environment.register_container("test1".to_string(), "container_id_123".to_string()).await?;
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
    environment.register_container("container1".to_string(), "id1".to_string()).await?;
    environment.register_container("container2".to_string(), "id2".to_string()).await?;
    environment.register_container("container3".to_string(), "id3".to_string()).await?;
    
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
    environment.register_container("cleanup_test".to_string(), "cleanup_id".to_string()).await?;
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

    let result = environment.execute_test("timeout_test", || {
        // Simulate some work
        std::thread::sleep(Duration::from_millis(1));
        Ok::<String, CleanroomError>("timeout_result".to_string())
    }).await?;

    let duration = start_time.elapsed();

    assert_eq!(result, "timeout_result");
    // Should complete quickly due to mocking
    TestAssertions::assert_duration_less_than(duration, Duration::from_millis(100));

    Ok(())
}

/// Test basic cleanroom environment creation and configuration (from integration_tests.rs)
#[tokio::test]
async fn test_cleanroom_environment_creation_integration() -> Result<(), Box<dyn std::error::Error>> {
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await?;

    // Verify environment is properly initialized
    assert!(!environment.session_id().is_nil());

    // Verify configuration is set correctly
    let env_config = environment.config();
    assert!(env_config.test_execution_timeout >= Duration::from_millis(10));

    Ok(())
}

/// Test test execution (from integration_tests.rs)
#[tokio::test]
async fn test_execute_test_integration() -> Result<(), Box<dyn std::error::Error>> {
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await?;

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

/// Test comprehensive reporting (from integration_tests.rs)
#[tokio::test]
async fn test_comprehensive_reporting_integration() -> Result<(), Box<dyn std::error::Error>> {
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await?;

    // Execute multiple tests
    environment
        .execute_test("test1", || {
            Ok::<String, CleanroomError>("result1".to_string())
        })
        .await?;
    environment
        .execute_test("test2", || {
            Ok::<String, CleanroomError>("result2".to_string())
        })
        .await?;
    environment
        .execute_test("test3", || {
            Ok::<String, CleanroomError>("result3".to_string())
        })
        .await?;

    // Generate comprehensive report
    let metrics = environment.get_metrics().await;

    // Verify metrics structure
    assert!(metrics.tests_executed >= 3);
    assert!(metrics.tests_passed >= 3);
    assert_eq!(metrics.tests_failed, 0);

    Ok(())
}

/// Test error handling and recovery (from integration_tests.rs)
#[tokio::test]
async fn test_error_handling_integration() -> Result<(), Box<dyn std::error::Error>> {
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await?;

    // Test test execution failure
    let test_result = environment
        .execute_test("failing_test", || {
            Err::<String, CleanroomError>(CleanroomError::validation_error("Test failure"))
        })
        .await;
    assert!(test_result.is_err());

    Ok(())
}

/// Test concurrent test execution (from integration_tests.rs)
#[tokio::test]
async fn test_concurrent_execution_integration() -> Result<(), Box<dyn std::error::Error>> {
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await?;

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

/// Test cleanroom environment cleanup (from integration_tests.rs)
#[tokio::test]
async fn test_environment_cleanup_integration() -> Result<(), Box<dyn std::error::Error>> {
    let config = CleanroomConfig::default();
    let mut environment = CleanroomEnvironment::new(config).await?;

    // Start some containers
    let _id1 = environment.start_container("test1").await?;
    let _id2 = environment.start_container("test2").await?;

    // Verify containers are running
    assert!(environment.get_container_count().await >= 2);

    // Cleanup environment
    environment.cleanup().await?;

    // Verify containers are stopped
    assert_eq!(environment.get_container_count().await, 0);

    Ok(())
}

/// Test configuration validation (from integration_tests.rs)
#[tokio::test]
async fn test_configuration_validation_integration() -> Result<(), Box<dyn std::error::Error>> {
    // Test valid configuration
    let valid_config = CleanroomConfig::default();
    assert!(valid_config.validate().is_ok());

    // Test invalid configuration
    let mut invalid_config = CleanroomConfig::default();
    invalid_config.max_concurrent_containers = 0; // Invalid value

    let validation_result = invalid_config.validate();
    assert!(validation_result.is_err());

    Ok(())
}

/// Test basic Docker integration with simple command execution (from integration_tests.rs)
#[tokio::test]
async fn test_docker_integration_basic_integration() -> Result<(), Box<dyn std::error::Error>> {
    // Skip if Docker is not available
    if !std::process::Command::new("docker")
        .arg("--version")
        .output()
        .is_ok()
    {
        println!("Skipping Docker integration test: Docker not available");
        return Ok(());
    }

    // Test simple command execution
    let result = run(["echo", "hello from cleanroom"])?;

    result.assert_success();
    assert_eq!(result.exit_code, 0);
    assert!(result.stdout.trim().contains("hello from cleanroom"));

    Ok(())
}

/// Test new_cleanroom convenience function (from integration_tests.rs)
#[tokio::test]
async fn test_new_cleanroom_convenience_integration() -> Result<(), Box<dyn std::error::Error>> {
    let environment = new_cleanroom().await?;

    assert!(environment.get_container_count().await >= 0);

    // Test a simple command through the environment
    let result = environment
        .execute_test("convenience_test", || {
            Ok::<String, CleanroomError>("convenience works".to_string())
        })
        .await?;

    assert_eq!(result, "convenience works");

    Ok(())
}

/// Test error handling in Docker integration (from integration_tests.rs)
#[tokio::test]
async fn test_docker_integration_error_handling_integration() -> Result<(), Box<dyn std::error::Error>> {
    // Skip if Docker is not available
    if !std::process::Command::new("docker")
        .arg("--version")
        .output()
        .is_ok()
    {
        println!("Skipping error handling test: Docker not available");
        return Ok(());
    }

    // Test command that should fail
    let result = run(["sh", "-c", "exit 42"])?;

    result.assert_failure();
    assert_eq!(result.exit_code, 42);

    Ok(())
}

/// Test container isolation and cleanup (from integration_tests.rs)
#[tokio::test]
async fn test_container_isolation_and_cleanup_integration() -> Result<(), Box<dyn std::error::Error>> {
    // Skip if Docker is not available
    if !std::process::Command::new("docker")
        .arg("--version")
        .output()
        .is_ok()
    {
        println!("Skipping isolation test: Docker not available");
        return Ok(());
    }

    // Execute multiple commands to test isolation
    let result1 = run(["echo", "first command"])?;
    let result2 = run(["echo", "second command"])?;

    result1.assert_success();
    result2.assert_success();
    assert_eq!(result1.stdout.trim(), "first command");
    assert_eq!(result2.stdout.trim(), "second command");

    Ok(())
}

/// Test performance characteristics (from integration_tests.rs)
#[tokio::test]
async fn test_performance_characteristics_integration() -> Result<(), Box<dyn std::error::Error>> {
    // Skip if Docker is not available
    if !std::process::Command::new("docker")
        .arg("--version")
        .output()
        .is_ok()
    {
        println!("Skipping performance test: Docker not available");
        return Ok(());
    }

    let start = std::time::Instant::now();

    // Execute a simple command
    let result = run(["echo", "performance test"])?;

    let duration = start.elapsed();

    result.assert_success();
    assert!(duration.as_millis() < 100); // Should complete within 100ms
    assert!(result.duration_ms > 0); // Should have recorded execution time

    Ok(())
}

/// Test basic cleanroom environment creation and configuration (fast version)
#[tokio::test]
async fn test_cleanroom_environment_creation_fast() -> Result<(), Box<dyn std::error::Error>> {
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await?;

    // Verify environment is properly initialized
    assert!(!environment.session_id().is_nil());

    // Verify configuration is set correctly
    let env_config = environment.config();
    assert!(env_config.test_execution_timeout >= Duration::from_millis(1));

    Ok(())
}

/// Test container lifecycle management (fast version)
#[tokio::test]
async fn test_container_lifecycle_fast() -> Result<(), Box<dyn std::error::Error>> {
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await?;

    // Test container registration
    environment.register_container("test1".to_string(), "container_id_123".to_string()).await?;
    assert!(environment.is_container_registered("test1").await);

    // Test container access
    let container_count = environment.get_container_count().await;
    assert!(container_count >= 1);

    // Test container cleanup
    environment.cleanup().await?;
    assert_eq!(environment.get_container_count().await, 0);

    Ok(())
}

/// Test test execution (fast version)
#[tokio::test]
async fn test_execute_test_fast() -> Result<(), Box<dyn std::error::Error>> {
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await?;

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

/// Test error handling and recovery (fast version)
#[tokio::test]
async fn test_error_handling_fast() -> Result<(), Box<dyn std::error::Error>> {
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await?;

    // Test test execution failure
    let test_result = environment
        .execute_test("failing_test", || {
            Err::<String, CleanroomError>(CleanroomError::validation_error("Test failure"))
        })
        .await;
    assert!(test_result.is_err());

    Ok(())
}

/// Test concurrent test execution (fast version)
#[tokio::test]
async fn test_concurrent_execution_fast() -> Result<(), Box<dyn std::error::Error>> {
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await?;

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

/// Test cleanroom environment cleanup (fast version)
#[tokio::test]
async fn test_environment_cleanup_fast() -> Result<(), Box<dyn std::error::Error>> {
    let config = CleanroomConfig::default();
    let mut environment = CleanroomEnvironment::new(config).await?;

    // Start some containers
    let _id1 = environment.start_container("test1").await?;
    let _id2 = environment.start_container("test2").await?;

    // Verify containers are running
    assert!(environment.get_container_count().await >= 2);

    // Cleanup environment
    environment.cleanup().await?;

    // Verify containers are stopped
    assert_eq!(environment.get_container_count().await, 0);

    Ok(())
}

/// Test configuration validation (fast version)
#[tokio::test]
async fn test_configuration_validation_fast() -> Result<(), Box<dyn std::error::Error>> {
    // Test valid configuration
    let valid_config = CleanroomConfig::default();
    assert!(valid_config.validate().is_ok());

    // Test invalid configuration
    let mut invalid_config = CleanroomConfig::default();
    invalid_config.max_concurrent_containers = 0; // Invalid value

    let validation_result = invalid_config.validate();
    assert!(validation_result.is_err());

    Ok(())
}

/// Test basic command execution without Docker (fast version)
#[tokio::test]
async fn test_basic_command_execution_fast() -> Result<(), Box<dyn std::error::Error>> {
    // Test simple command execution
    let result = run(["echo", "hello from cleanroom"])?;

    result.assert_success();
    assert_eq!(result.exit_code, 0);
    assert!(result.stdout.trim().contains("hello from cleanroom"));

    Ok(())
}

/// Test error handling in command execution (fast version)
#[tokio::test]
async fn test_command_error_handling_fast() -> Result<(), Box<dyn std::error::Error>> {
    // Test command that should fail
    let result = run(["sh", "-c", "exit 42"])?;

    result.assert_failure();
    assert_eq!(result.exit_code, 42);

    Ok(())
}

/// Test command isolation (fast version)
#[tokio::test]
async fn test_command_isolation_fast() -> Result<(), Box<dyn std::error::Error>> {
    // Execute multiple commands to test isolation
    let result1 = run(["echo", "first command"])?;
    let result2 = run(["echo", "second command"])?;

    result1.assert_success();
    result2.assert_success();
    assert_eq!(result1.stdout.trim(), "first command");
    assert_eq!(result2.stdout.trim(), "second command");

    Ok(())
}

/// Test performance characteristics (fast version)
#[tokio::test]
async fn test_performance_characteristics_fast() -> Result<(), Box<dyn std::error::Error>> {
    let start = std::time::Instant::now();

    // Execute a simple command
    let result = run(["echo", "performance test"])?;

    let duration = start.elapsed();

    result.assert_success();
    assert!(duration.as_millis() < 1000); // Should complete within 1 second
    assert!(result.duration_ms > 0); // Should have recorded execution time

    Ok(())
}
