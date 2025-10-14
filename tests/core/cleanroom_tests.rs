//! Tests for cleanroom environment functionality
//!
//! This module contains tests for cleanroom environment creation, configuration,
//! session management, and test execution.

use crate::fixtures::{TestAssertions, TestEnvironments};
use clnrm::{CleanroomConfig, CleanroomEnvironment, Error as CleanroomError};
use std::time::Duration;

#[tokio::test]
async fn test_cleanroom_creation() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::unit_test().await?;
    assert!(!environment.session_id().is_nil());
    Ok(())
}

#[tokio::test]
async fn test_cleanroom_session_id() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::unit_test().await?;
    assert!(!environment.session_id().is_nil());
    Ok(())
}

#[tokio::test]
async fn test_cleanroom_config() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::unit_test().await?;
    let config = environment.config();

    // Test that timeout is reasonable (not 300 seconds)
    TestAssertions::assert_duration_less_than(
        config.test_execution_timeout,
        Duration::from_secs(1),
    );

    Ok(())
}

#[tokio::test]
async fn test_cleanroom_start_time() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::unit_test().await?;
    let start_time = environment.start_time();

    // Should complete quickly due to mocking
    TestAssertions::assert_duration_less_than(start_time.elapsed(), Duration::from_millis(100));

    Ok(())
}

#[tokio::test]
async fn test_cleanroom_execute_test() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::unit_test().await?;

    let result = environment
        .execute_test("test", || Ok::<i32, CleanroomError>(42))
        .await?;

    assert_eq!(result, 42);
    Ok(())
}

#[tokio::test]
async fn test_cleanroom_execute_test_failure() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::unit_test().await?;

    let result = environment
        .execute_test("test", || {
            Err::<i32, CleanroomError>(CleanroomError::validation_error("test error"))
        })
        .await;

    TestAssertions::assert_error(&result);
    Ok(())
}
