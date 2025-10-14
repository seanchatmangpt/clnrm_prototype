//! Integration Test Template
//!
//! This module provides a template for writing integration tests following best practices.

use clnrm::{Error as CleanroomError, conditional_sleep};
use crate::fixtures::{TestEnvironments, TestContainers, TestAssertions};
use std::time::Duration;

/// Template for integration tests
/// 
/// This template demonstrates the recommended structure for integration tests:
/// - Test component interactions
/// - Use integration test fixtures
/// - Test real workflows
/// - Verify end-to-end behavior
#[tokio::test]
async fn template_integration_test() -> Result<(), CleanroomError> {
    // Arrange: Set up integration test environment
    let environment = TestEnvironments::integration_test().await?;
    let container = TestContainers::postgres();
    
    // Act: Test component interaction
    environment.register_container("postgres".to_string(), "container_id".to_string()).await?;
    let is_registered = environment.is_container_registered("postgres").await;
    
    // Assert: Verify integration behavior
    assert!(is_registered);
    assert!(environment.get_container_count().await >= 1);
    
    // Cleanup
    environment.cleanup().await?;
    
    Ok(())
}

/// Template for integration tests with multiple components
#[tokio::test]
async fn template_integration_test_multiple_components() -> Result<(), CleanroomError> {
    // Arrange: Set up multiple components
    let environment = TestEnvironments::integration_test().await?;
    let postgres = TestContainers::postgres();
    let redis = TestContainers::redis();
    
    // Act: Test multiple component interactions
    environment.register_container("postgres".to_string(), "pg_id".to_string()).await?;
    environment.register_container("redis".to_string(), "redis_id".to_string()).await?;
    
    // Assert: Verify all components are integrated
    assert!(environment.is_container_registered("postgres").await);
    assert!(environment.is_container_registered("redis").await);
    assert_eq!(environment.get_container_count().await, 2);
    
    // Cleanup
    environment.cleanup().await?;
    
    Ok(())
}

/// Template for integration tests with workflows
#[tokio::test]
async fn template_integration_test_workflow() -> Result<(), CleanroomError> {
    // Arrange: Set up workflow environment
    let environment = TestEnvironments::integration_test().await?;
    
    // Act: Execute multi-step workflow
    let step1_result = environment
        .execute_test("workflow_step_1", || {
            Ok::<String, CleanroomError>("step1_complete".to_string())
        })
        .await?;
    
    let step2_result = environment
        .execute_test("workflow_step_2", || {
            Ok::<String, CleanroomError>("step2_complete".to_string())
        })
        .await?;
    
    let step3_result = environment
        .execute_test("workflow_step_3", || {
            Ok::<String, CleanroomError>("workflow_complete".to_string())
        })
        .await?;
    
    // Assert: Verify workflow completion
    assert_eq!(step1_result, "step1_complete");
    assert_eq!(step2_result, "step2_complete");
    assert_eq!(step3_result, "workflow_complete");
    
    // Cleanup
    environment.cleanup().await?;
    
    Ok(())
}

/// Template for integration tests with error scenarios
#[tokio::test]
async fn template_integration_test_error_scenarios() -> Result<(), CleanroomError> {
    // Arrange: Set up integration environment
    let environment = TestEnvironments::integration_test().await?;
    
    // Act: Test error handling in integration context
    let result = environment
        .execute_test("integration_error_test", || {
            Err::<String, CleanroomError>(CleanroomError::validation_error("Integration error"))
        })
        .await;
    
    // Assert: Verify error handling
    TestAssertions::assert_error(&result);
    TestAssertions::assert_error_message(&result, "Integration error");
    
    // Verify environment is still functional after error
    let recovery_result = environment
        .execute_test("recovery_test", || {
            Ok::<String, CleanroomError>("recovery_successful".to_string())
        })
        .await?;
    
    assert_eq!(recovery_result, "recovery_successful");
    
    // Cleanup
    environment.cleanup().await?;
    
    Ok(())
}

/// Template for integration tests with concurrent operations
#[tokio::test]
async fn template_integration_test_concurrent_operations() -> Result<(), CleanroomError> {
    // Arrange: Set up concurrent test environment
    let environment = TestEnvironments::concurrent_test().await?;
    
    // Act: Execute concurrent operations
    let mut handles = Vec::new();
    
    for i in 0..5 {
        let env = environment.clone();
        let handle = tokio::spawn(async move {
            env.execute_test(&format!("concurrent_test_{}", i), || {
                Ok::<String, CleanroomError>(format!("result_{}", i))
            }).await
        });
        handles.push(handle);
    }
    
    // Wait for all operations to complete
    let mut results = Vec::new();
    for handle in handles {
        let result = handle.await.unwrap()?;
        results.push(result);
    }
    
    // Assert: Verify concurrent operations
    assert_eq!(results.len(), 5);
    for i in 0..5 {
        assert_eq!(results[i], format!("result_{}", i));
    }
    
    // Cleanup
    environment.cleanup().await?;
    
    Ok(())
}

/// Template for integration tests with performance requirements
#[tokio::test]
async fn template_integration_test_performance() -> Result<(), CleanroomError> {
    // Arrange: Set up performance test environment
    let environment = TestEnvironments::performance_test().await?;
    let start_time = std::time::Instant::now();
    
    // Act: Execute performance-critical integration
    let result = environment
        .execute_test("performance_integration_test", || {
            Ok::<String, CleanroomError>("performance_result".to_string())
        })
        .await?;
    
    // Assert: Verify performance requirements
    let duration = start_time.elapsed();
    TestAssertions::assert_duration_less_than(duration, Duration::from_millis(100));
    assert_eq!(result, "performance_result");
    
    // Cleanup
    environment.cleanup().await?;
    
    Ok(())
}

/// Template for integration tests with resource management
#[tokio::test]
async fn template_integration_test_resource_management() -> Result<(), CleanroomError> {
    // Arrange: Set up resource test environment
    let environment = TestEnvironments::integration_test().await?;
    
    // Act: Test resource allocation and deallocation
    environment.register_container("resource_test".to_string(), "resource_id".to_string()).await?;
    assert_eq!(environment.get_container_count().await, 1);
    
    // Test resource cleanup
    environment.cleanup().await?;
    assert_eq!(environment.get_container_count().await, 0);
    
    // Test resource reallocation after cleanup
    environment.register_container("resource_test_2".to_string(), "resource_id_2".to_string()).await?;
    assert_eq!(environment.get_container_count().await, 1);
    
    // Final cleanup
    environment.cleanup().await?;
    
    Ok(())
}

/// Template for integration tests with state management
#[tokio::test]
async fn template_integration_test_state_management() -> Result<(), CleanroomError> {
    // Arrange: Set up stateful test environment
    let environment = TestEnvironments::integration_test().await?;
    
    // Act: Test state transitions
    let state1 = environment
        .execute_test("state_transition_1", || {
            Ok::<String, CleanroomError>("state_1".to_string())
        })
        .await?;
    
    let state2 = environment
        .execute_test("state_transition_2", || {
            Ok::<String, CleanroomError>("state_2".to_string())
        })
        .await?;
    
    let final_state = environment
        .execute_test("final_state", || {
            Ok::<String, CleanroomError>("final_state".to_string())
        })
        .await?;
    
    // Assert: Verify state management
    assert_eq!(state1, "state_1");
    assert_eq!(state2, "state_2");
    assert_eq!(final_state, "final_state");
    
    // Cleanup
    environment.cleanup().await?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_integration_template_helpers() -> Result<(), CleanroomError> {
        // Test that integration test templates work correctly
        let environment = TestEnvironments::integration_test().await?;
        
        // Test basic integration functionality
        let result = environment
            .execute_test("template_test", || {
                Ok::<String, CleanroomError>("template_success".to_string())
            })
            .await?;
        
        assert_eq!(result, "template_success");
        
        // Cleanup
        environment.cleanup().await?;
        
        Ok(())
    }
}
