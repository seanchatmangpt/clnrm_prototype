//! Unit Test Template
//!
//! This module provides a template for writing unit tests following best practices.

use clnrm::{Error as CleanroomError, conditional_sleep};
use crate::fixtures::{TestEnvironments, TestAssertions, MockTimeTestContext};
use std::time::Duration;

/// Template for unit tests
/// 
/// This template demonstrates the recommended structure for unit tests:
/// - Use test fixtures for setup
/// - Follow AAA pattern (Arrange, Act, Assert)
/// - Use appropriate assertions
/// - Handle errors properly
/// - Clean up resources
#[tokio::test]
async fn template_unit_test() -> Result<(), CleanroomError> {
    // Arrange: Set up test data and environment
    let environment = TestEnvironments::unit_test().await?;
    let test_data = create_test_data();
    let expected_result = "expected_value";
    
    // Act: Execute the code under test
    let actual_result = environment
        .execute_test("unit_test", || {
            Ok::<String, CleanroomError>(process_test_data(test_data))
        })
        .await?;
    
    // Assert: Verify the results
    assert_eq!(actual_result, expected_result);
    
    // Cleanup: Ensure proper cleanup
    environment.cleanup().await?;
    
    Ok(())
}

/// Template for unit tests with mock time
#[tokio::test]
async fn template_unit_test_with_mock_time() -> Result<(), CleanroomError> {
    // Arrange: Set up test data and mock time
    let context = MockTimeTestContext::new();
    let start_time = context.now();
    let test_duration = Duration::from_secs(10);
    
    // Act: Advance time and test time-dependent behavior
    context.advance(test_duration);
    let elapsed = context.elapsed();
    
    // Assert: Verify time behavior
    assert_eq!(elapsed, test_duration);
    
    Ok(())
}

/// Template for unit tests with conditional sleep
#[tokio::test]
async fn template_unit_test_with_conditional_sleep() -> Result<(), CleanroomError> {
    // Arrange: Set up test data
    let start_time = std::time::Instant::now();
    
    // Act: Use conditional sleep (completes immediately in test mode)
    conditional_sleep(Duration::from_secs(10)).await;
    
    // Assert: Verify that sleep was fast due to mocking
    let duration = start_time.elapsed();
    TestAssertions::assert_duration_less_than(duration, Duration::from_millis(100));
    
    Ok(())
}

/// Template for unit tests with error handling
#[tokio::test]
async fn template_unit_test_error_handling() -> Result<(), CleanroomError> {
    // Arrange: Set up test data
    let environment = TestEnvironments::unit_test().await?;
    
    // Act: Execute code that should fail
    let result = environment
        .execute_test("failing_test", || {
            Err::<String, CleanroomError>(CleanroomError::validation_error("Expected error"))
        })
        .await;
    
    // Assert: Verify error handling
    TestAssertions::assert_error(&result);
    TestAssertions::assert_error_message(&result, "Expected error");
    
    // Cleanup
    environment.cleanup().await?;
    
    Ok(())
}

/// Template for unit tests with multiple assertions
#[tokio::test]
async fn template_unit_test_multiple_assertions() -> Result<(), CleanroomError> {
    // Arrange: Set up test data
    let environment = TestEnvironments::unit_test().await?;
    let test_data = vec!["item1", "item2", "item3"];
    
    // Act: Execute the code under test
    let results = environment
        .execute_test("multiple_items_test", || {
            Ok::<Vec<String>, CleanroomError>(
                test_data.iter().map(|s| s.to_string()).collect()
            )
        })
        .await?;
    
    // Assert: Verify multiple conditions
    TestAssertions::assert_vec_length(&results, 3);
    TestAssertions::assert_vec_contains(&results, &"item1".to_string());
    TestAssertions::assert_vec_contains(&results, &"item2".to_string());
    TestAssertions::assert_vec_contains(&results, &"item3".to_string());
    
    // Cleanup
    environment.cleanup().await?;
    
    Ok(())
}

/// Template for unit tests with performance assertions
#[tokio::test]
async fn template_unit_test_performance() -> Result<(), CleanroomError> {
    // Arrange: Set up test data
    let start_time = std::time::Instant::now();
    
    // Act: Execute performance-critical code
    let result = perform_fast_operation().await?;
    
    // Assert: Verify performance characteristics
    let duration = start_time.elapsed();
    TestAssertions::assert_duration_less_than(duration, Duration::from_millis(10));
    assert_eq!(result, "fast_result");
    
    Ok(())
}

// Helper functions for templates

fn create_test_data() -> String {
    "test_data".to_string()
}

fn process_test_data(data: String) -> String {
    format!("processed_{}", data)
}

async fn perform_fast_operation() -> Result<String, CleanroomError> {
    // Simulate fast operation
    conditional_sleep(Duration::from_millis(1)).await;
    Ok("fast_result".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_template_functions() -> Result<(), CleanroomError> {
        // Test helper functions
        let test_data = create_test_data();
        assert_eq!(test_data, "test_data");
        
        let processed = process_test_data(test_data);
        assert_eq!(processed, "processed_test_data");
        
        let fast_result = perform_fast_operation().await?;
        assert_eq!(fast_result, "fast_result");
        
        Ok(())
    }
}
