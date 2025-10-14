//! Example usage of mock time system in tests
//!
//! This file demonstrates how to convert existing slow tests to use
//! the mock time system for instant execution.

use crate::mock_time::{MockTime, MockTimeTestEnv, conditional_sleep, conditional_timeout};
use crate::test_utils_mock::{MockTestEnvironment, MockContainer, MockService, create_mock_test_env};
use clnrm::{CleanroomConfig, CleanroomEnvironment, Error as CleanroomError};
use std::time::Duration;

/// Example: Convert a slow BDD test to use mock time
#[tokio::test]
async fn test_cleanroom_environment_setup_mock() -> Result<(), Box<dyn std::error::Error>> {
    // Use mock time environment instead of real time
    let env = create_mock_test_env().await?;
    
    // This test now runs instantly instead of taking seconds
    let container_id = env.start_container_mock("postgres_container").await?;
    assert!(env.is_container_running_mock(&container_id).await?);
    
    // Mock cleanup is instant
    env.cleanup_mock().await?;
    assert!(!env.is_container_running_mock(&container_id).await?);
    
    Ok(())
}

/// Example: Convert a slow integration test to use mock time
#[tokio::test]
async fn test_container_lifecycle_mock() -> Result<(), Box<dyn std::error::Error>> {
    let env = create_mock_test_env().await?;
    
    // Register containers instantly
    env.environment().register_container("test1".to_string(), "container_id_123".to_string()).await?;
    assert!(env.environment().is_container_registered("test1").await);
    
    let container_count = env.environment().get_container_count().await;
    assert_eq!(container_count, 1);
    
    // Instant cleanup
    env.cleanup_mock().await?;
    assert_eq!(env.environment().get_container_count().await, 0);
    
    Ok(())
}

/// Example: Convert a slow performance test to use mock time
#[tokio::test]
async fn test_performance_mock() -> Result<(), Box<dyn std::error::Error>> {
    let env = create_mock_test_env().await?;
    
    // Execute test with mock time - runs instantly
    let result = env.execute_test_mock("performance_test", || {
        Box::pin(async move {
            // This sleep is now instant in mock mode
            conditional_sleep(Duration::from_secs(1)).await;
            Ok::<String, CleanroomError>("performance_result".to_string())
        })
    }).await?;
    
    assert_eq!(result, "performance_result");
    
    // Get metrics instantly
    let metrics = env.get_metrics_mock().await;
    assert!(metrics.total_tests >= 1);
    
    Ok(())
}

/// Example: Convert a slow timeout test to use mock time
#[tokio::test]
async fn test_timeout_handling_mock() -> Result<(), Box<dyn std::error::Error>> {
    let mock_time = MockTime::new();
    
    // Test timeout behavior with mock time
    let result = mock_time.timeout(
        Duration::from_millis(10),
        async {
            // This would normally take longer than the timeout
            conditional_sleep(Duration::from_secs(1)).await;
            "success"
        }
    ).await;
    
    // In mock mode, this should succeed because sleep is instant
    assert_eq!(result, Ok("success"));
    
    Ok(())
}

/// Example: Convert a slow concurrent test to use mock time
#[tokio::test]
async fn test_concurrent_execution_mock() -> Result<(), Box<dyn std::error::Error>> {
    let env = create_mock_test_env().await?;
    
    // Spawn multiple tasks that would normally take time
    let mut handles = Vec::new();
    
    for i in 0..5 {
        let env_clone = env.environment().clone();
        let handle = tokio::spawn(async move {
            // This sleep is now instant in mock mode
            conditional_sleep(Duration::from_millis(100)).await;
            env_clone.execute_test(&format!("test_{}", i), || {
                Ok::<i32, CleanroomError>(i)
            }).await
        });
        handles.push(handle);
    }
    
    // Wait for all tasks to complete (instantly in mock mode)
    for handle in handles {
        let result = handle.await?;
        assert!(result.is_ok());
    }
    
    Ok(())
}

/// Example: Convert a slow container test to use mock containers
#[tokio::test]
async fn test_mock_container_operations() -> Result<(), Box<dyn std::error::Error>> {
    let mut container = MockContainer::new("postgres");
    
    // These operations are now instant
    container.start().await?;
    assert!(container.is_running());
    
    container.stop().await?;
    assert!(!container.is_running());
    
    Ok(())
}

/// Example: Convert a slow service test to use mock services
#[tokio::test]
async fn test_mock_service_operations() -> Result<(), Box<dyn std::error::Error>> {
    let mut service = MockService::new("redis");
    
    // These operations are now instant
    service.start().await?;
    assert!(service.is_ready());
    
    Ok(())
}

/// Example: Convert a slow BDD test with multiple steps
#[tokio::test]
async fn test_comprehensive_bdd_mock() -> Result<(), Box<dyn std::error::Error>> {
    let env = create_mock_test_env().await?;
    
    // Given: A cleanroom environment (instant)
    let container_id = env.start_container_mock("test_container").await?;
    
    // When: I execute multiple tests (instant)
    let results = vec![
        env.execute_test_mock("test1", || {
            Box::pin(async move {
                conditional_sleep(Duration::from_millis(50)).await;
                Ok::<String, CleanroomError>("result1".to_string())
            })
        }).await?,
        env.execute_test_mock("test2", || {
            Box::pin(async move {
                conditional_sleep(Duration::from_millis(50)).await;
                Ok::<String, CleanroomError>("result2".to_string())
            })
        }).await?,
        env.execute_test_mock("test3", || {
            Box::pin(async move {
                conditional_sleep(Duration::from_millis(50)).await;
                Ok::<String, CleanroomError>("result3".to_string())
            })
        }).await?,
    ];
    
    // Then: All tests should complete successfully (instant)
    assert_eq!(results.len(), 3);
    assert_eq!(results[0], "result1");
    assert_eq!(results[1], "result2");
    assert_eq!(results[2], "result3");
    
    // And: I should get metrics (instant)
    let metrics = env.get_metrics_mock().await;
    assert!(metrics.total_tests >= 3);
    
    // And: Cleanup should work (instant)
    env.cleanup_mock().await?;
    
    Ok(())
}

/// Example: Convert a slow property test to use mock time
#[tokio::test]
async fn test_property_validation_mock() -> Result<(), Box<dyn std::error::Error>> {
    // Test with various timeout values - all instant in mock mode
    let timeouts = vec![
        Duration::from_millis(1),
        Duration::from_millis(10),
        Duration::from_millis(100),
        Duration::from_secs(1),
        Duration::from_secs(10),
    ];
    
    for timeout in timeouts {
        let mock_time = MockTime::new();
        let start = mock_time.now();
        
        // This sleep is now instant regardless of duration
        mock_time.sleep(timeout).await;
        
        let elapsed = mock_time.now().duration_since(start);
        assert_eq!(elapsed, timeout);
    }
    
    Ok(())
}

/// Example: Convert a slow Docker integration test to use mock time
#[tokio::test]
async fn test_docker_integration_mock() -> Result<(), Box<dyn std::error::Error>> {
    let env = create_mock_test_env().await?;
    
    // Simulate Docker operations that would normally take time
    let postgres_id = env.start_container_mock("postgres").await?;
    let redis_id = env.start_container_mock("redis").await?;
    
    // These checks are now instant
    assert!(env.is_container_running_mock(&postgres_id).await?);
    assert!(env.is_container_running_mock(&redis_id).await?);
    
    // Cleanup is instant
    env.cleanup_mock().await?;
    assert!(!env.is_container_running_mock(&postgres_id).await?);
    assert!(!env.is_container_running_mock(&redis_id).await?);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_time_advancement() {
        let mock_time = MockTime::new();
        let start = mock_time.now();
        
        // Advance time by 5 seconds
        mock_time.advance(Duration::from_secs(5));
        
        let elapsed = mock_time.now().duration_since(start);
        assert_eq!(elapsed, Duration::from_secs(5));
    }

    #[tokio::test]
    async fn test_conditional_sleep_speed() {
        let start = std::time::Instant::now();
        
        // This should complete almost instantly in test mode
        conditional_sleep(Duration::from_secs(10)).await;
        
        let elapsed = start.elapsed();
        // Should be much faster than 10 seconds
        assert!(elapsed < Duration::from_millis(100));
    }

    #[tokio::test]
    async fn test_conditional_timeout_success() {
        // This should not timeout in test mode
        let result = conditional_timeout(
            Duration::from_millis(1),
            async {
                conditional_sleep(Duration::from_secs(1)).await;
                "success"
            }
        ).await;
        
        assert_eq!(result, Ok("success"));
    }
}
