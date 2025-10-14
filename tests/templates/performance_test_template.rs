//! Performance Test Template
//!
//! This module provides a template for writing performance tests following best practices.

use clnrm::{Error as CleanroomError, conditional_sleep};
use crate::fixtures::{TestEnvironments, TestAssertions, MockTimeTestContext};
use std::time::{Duration, Instant};

/// Template for performance tests
/// 
/// This template demonstrates the recommended structure for performance tests:
/// - Measure execution time
/// - Test performance characteristics
/// - Verify performance requirements
/// - Use performance test fixtures
#[tokio::test]
async fn template_performance_test() -> Result<(), CleanroomError> {
    // Arrange: Set up performance test environment
    let environment = TestEnvironments::performance_test().await?;
    let start_time = Instant::now();
    
    // Act: Execute performance-critical operation
    let result = environment
        .execute_test("performance_test", || {
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

/// Template for performance tests with multiple iterations
#[tokio::test]
async fn template_performance_test_multiple_iterations() -> Result<(), CleanroomError> {
    // Arrange: Set up performance test environment
    let environment = TestEnvironments::performance_test().await?;
    let iterations = 10;
    let mut total_duration = Duration::from_secs(0);
    
    // Act: Execute multiple iterations
    for i in 0..iterations {
        let start_time = Instant::now();
        
        let result = environment
            .execute_test(&format!("performance_iteration_{}", i), || {
                Ok::<String, CleanroomError>(format!("result_{}", i))
            })
            .await?;
        
        let iteration_duration = start_time.elapsed();
        total_duration += iteration_duration;
        
        assert_eq!(result, format!("result_{}", i));
    }
    
    // Assert: Verify performance requirements
    let average_duration = total_duration / iterations;
    TestAssertions::assert_duration_less_than(average_duration, Duration::from_millis(50));
    
    // Cleanup
    environment.cleanup().await?;
    
    Ok(())
}

/// Template for performance tests with concurrent operations
#[tokio::test]
async fn template_performance_test_concurrent_operations() -> Result<(), CleanroomError> {
    // Arrange: Set up concurrent performance test environment
    let environment = TestEnvironments::concurrent_test().await?;
    let start_time = Instant::now();
    
    // Act: Execute concurrent operations
    let mut handles = Vec::new();
    
    for i in 0..5 {
        let env = environment.clone();
        let handle = tokio::spawn(async move {
            let result = env
                .execute_test(&format!("concurrent_performance_{}", i), || {
                    Ok::<String, CleanroomError>(format!("concurrent_result_{}", i))
                })
                .await?;
            Ok::<String, CleanroomError>(result)
        });
        handles.push(handle);
    }
    
    // Wait for all operations to complete
    let mut results = Vec::new();
    for handle in handles {
        let result = handle.await.unwrap()?;
        results.push(result);
    }
    
    // Assert: Verify concurrent performance
    let total_duration = start_time.elapsed();
    TestAssertions::assert_duration_less_than(total_duration, Duration::from_millis(200));
    
    assert_eq!(results.len(), 5);
    for i in 0..5 {
        assert_eq!(results[i], format!("concurrent_result_{}", i));
    }
    
    // Cleanup
    environment.cleanup().await?;
    
    Ok(())
}

/// Template for performance tests with memory usage
#[tokio::test]
async fn template_performance_test_memory_usage() -> Result<(), CleanroomError> {
    // Arrange: Set up memory performance test environment
    let environment = TestEnvironments::performance_test().await?;
    let start_time = Instant::now();
    
    // Act: Execute memory-intensive operation
    let result = environment
        .execute_test("memory_performance_test", || {
            // Simulate memory-intensive operation
            let large_data = vec![0u8; 1024 * 1024]; // 1MB of data
            Ok::<usize, CleanroomError>(large_data.len())
        })
        .await?;
    
    // Assert: Verify memory performance
    let duration = start_time.elapsed();
    TestAssertions::assert_duration_less_than(duration, Duration::from_millis(100));
    assert_eq!(result, 1024 * 1024);
    
    // Cleanup
    environment.cleanup().await?;
    
    Ok(())
}

/// Template for performance tests with mock time
#[tokio::test]
async fn template_performance_test_with_mock_time() -> Result<(), CleanroomError> {
    // Arrange: Set up mock time performance test
    let context = MockTimeTestContext::new();
    let start_time = context.now();
    
    // Act: Execute time-dependent performance test
    context.advance(Duration::from_secs(10));
    let elapsed = context.elapsed();
    
    // Assert: Verify mock time performance
    assert_eq!(elapsed, Duration::from_secs(10));
    
    // Test that mock time operations are fast
    let real_start = Instant::now();
    context.advance(Duration::from_secs(100));
    let real_duration = real_start.elapsed();
    
    TestAssertions::assert_duration_less_than(real_duration, Duration::from_millis(10));
    
    Ok(())
}

/// Template for performance tests with conditional sleep
#[tokio::test]
async fn template_performance_test_with_conditional_sleep() -> Result<(), CleanroomError> {
    // Arrange: Set up conditional sleep performance test
    let start_time = Instant::now();
    
    // Act: Use conditional sleep (should be fast in test mode)
    conditional_sleep(Duration::from_secs(10)).await;
    
    // Assert: Verify conditional sleep performance
    let duration = start_time.elapsed();
    TestAssertions::assert_duration_less_than(duration, Duration::from_millis(100));
    
    Ok(())
}

/// Template for performance tests with resource limits
#[tokio::test]
async fn template_performance_test_resource_limits() -> Result<(), CleanroomError> {
    // Arrange: Set up resource-limited performance test
    let environment = TestEnvironments::performance_test().await?;
    let start_time = Instant::now();
    
    // Act: Execute operation within resource limits
    let result = environment
        .execute_test("resource_limited_performance_test", || {
            // Simulate resource-limited operation
            conditional_sleep(Duration::from_millis(10)).await;
            Ok::<String, CleanroomError>("resource_limited_result".to_string())
        })
        .await?;
    
    // Assert: Verify resource-limited performance
    let duration = start_time.elapsed();
    TestAssertions::assert_duration_less_than(duration, Duration::from_millis(100));
    assert_eq!(result, "resource_limited_result");
    
    // Cleanup
    environment.cleanup().await?;
    
    Ok(())
}

/// Template for performance tests with scalability
#[tokio::test]
async fn template_performance_test_scalability() -> Result<(), CleanroomError> {
    // Arrange: Set up scalability performance test
    let environment = TestEnvironments::performance_test().await?;
    let scales = vec![1, 5, 10];
    let mut results = Vec::new();
    
    // Act: Test performance at different scales
    for scale in scales {
        let start_time = Instant::now();
        
        for i in 0..scale {
            let result = environment
                .execute_test(&format!("scalability_test_{}_{}", scale, i), || {
                    Ok::<String, CleanroomError>(format!("scale_{}_result_{}", scale, i))
                })
                .await?;
            
            assert_eq!(result, format!("scale_{}_result_{}", scale, i));
        }
        
        let duration = start_time.elapsed();
        results.push((scale, duration));
    }
    
    // Assert: Verify scalability performance
    for (scale, duration) in results {
        // Performance should scale reasonably (not linearly)
        let max_duration = Duration::from_millis(scale as u64 * 50);
        TestAssertions::assert_duration_less_than(duration, max_duration);
    }
    
    // Cleanup
    environment.cleanup().await?;
    
    Ok(())
}

/// Template for performance tests with benchmarking
#[tokio::test]
async fn template_performance_test_benchmarking() -> Result<(), CleanroomError> {
    // Arrange: Set up benchmarking test
    let environment = TestEnvironments::performance_test().await?;
    let benchmark_iterations = 100;
    let mut durations = Vec::new();
    
    // Act: Run benchmark iterations
    for i in 0..benchmark_iterations {
        let start_time = Instant::now();
        
        let result = environment
            .execute_test(&format!("benchmark_{}", i), || {
                Ok::<String, CleanroomError>(format!("benchmark_result_{}", i))
            })
            .await?;
        
        let duration = start_time.elapsed();
        durations.push(duration);
        
        assert_eq!(result, format!("benchmark_result_{}", i));
    }
    
    // Assert: Verify benchmarking results
    let total_duration: Duration = durations.iter().sum();
    let average_duration = total_duration / benchmark_iterations;
    
    // Average should be very fast due to mocking
    TestAssertions::assert_duration_less_than(average_duration, Duration::from_millis(10));
    
    // All individual operations should be fast
    for duration in durations {
        TestAssertions::assert_duration_less_than(duration, Duration::from_millis(50));
    }
    
    // Cleanup
    environment.cleanup().await?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_performance_template_helpers() -> Result<(), CleanroomError> {
        // Test that performance test templates work correctly
        let environment = TestEnvironments::performance_test().await?;
        
        // Test basic performance functionality
        let start_time = Instant::now();
        let result = environment
            .execute_test("template_performance_test", || {
                Ok::<String, CleanroomError>("template_performance_success".to_string())
            })
            .await?;
        
        let duration = start_time.elapsed();
        
        assert_eq!(result, "template_performance_success");
        TestAssertions::assert_duration_less_than(duration, Duration::from_millis(100));
        
        // Cleanup
        environment.cleanup().await?;
        
        Ok(())
    }
}
