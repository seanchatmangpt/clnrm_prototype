//! Integration scenario tests
//!
//! Tests for complex scenarios and workflows that involve multiple
//! components working together in realistic usage patterns.

use clnrm::{CleanroomConfig, CleanroomEnvironment, Error as CleanroomError};
use std::time::Duration;

/// Test complete test execution workflow
#[tokio::test]
async fn test_complete_workflow() -> Result<(), Box<dyn std::error::Error>> {
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await?;

    // Execute multiple tests in a workflow
    let test_results = vec![
        environment.execute_test("setup", || Ok::<String, CleanroomError>("setup complete".to_string())).await?,
        environment.execute_test("test1", || Ok::<String, CleanroomError>("test1 passed".to_string())).await?,
        environment.execute_test("test2", || Ok::<String, CleanroomError>("test2 passed".to_string())).await?,
        environment.execute_test("cleanup", || Ok::<String, CleanroomError>("cleanup complete".to_string())).await?,
    ];

    assert_eq!(test_results.len(), 4);
    assert!(test_results.iter().all(|result| result.contains("complete") || result.contains("passed")));

    Ok(())
}

/// Test resource management across multiple tests
#[tokio::test]
async fn test_resource_management_workflow() -> Result<(), Box<dyn std::error::Error>> {
    let config = CleanroomConfig::default();
    let mut environment = CleanroomEnvironment::new(config).await?;

    // Register multiple containers for different purposes
    environment.register_container("database".to_string(), "db_container".to_string()).await?;
    environment.register_container("app".to_string(), "app_container".to_string()).await?;
    environment.register_container("cache".to_string(), "cache_container".to_string()).await?;

    assert_eq!(environment.get_container_count().await, 3);

    // Execute tests that use different resources
    let db_test = environment.execute_test("db_test", || Ok::<String, CleanroomError>("db test passed".to_string())).await?;
    let app_test = environment.execute_test("app_test", || Ok::<String, CleanroomError>("app test passed".to_string())).await?;
    let integration_test = environment.execute_test("integration_test", || Ok::<String, CleanroomError>("integration test passed".to_string())).await?;

    assert!(db_test.contains("db test passed"));
    assert!(app_test.contains("app test passed"));
    assert!(integration_test.contains("integration test passed"));

    // Cleanup all resources
    environment.cleanup().await?;
    assert_eq!(environment.get_container_count().await, 0);

    Ok(())
}

/// Test error recovery and resilience
#[tokio::test]
async fn test_error_recovery_scenario() -> Result<(), Box<dyn std::error::Error>> {
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await?;

    // Execute a mix of passing and failing tests
    let results = vec![
        environment.execute_test("pass1", || Ok::<String, CleanroomError>("pass".to_string())).await?,
        environment.execute_test("fail1", || Err::<String, CleanroomError>(CleanroomError::validation_error("fail"))).await,
        environment.execute_test("pass2", || Ok::<String, CleanroomError>("pass".to_string())).await?,
        environment.execute_test("fail2", || Err::<String, CleanroomError>(CleanroomError::validation_error("fail"))).await,
        environment.execute_test("pass3", || Ok::<String, CleanroomError>("pass".to_string())).await?,
    ];

    // Should have 3 passing results and 2 errors
    let passing_results: Vec<_> = results.iter().filter(|r| r.is_ok()).collect();
    let failing_results: Vec<_> = results.iter().filter(|r| r.is_err()).collect();

    assert_eq!(passing_results.len(), 3);
    assert_eq!(failing_results.len(), 2);

    Ok(())
}
