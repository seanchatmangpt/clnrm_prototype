//! Integration tests for mock time functionality
//!
//! This test verifies that all sleep calls are properly mocked and tests run fast.

use clnrm::{conditional_sleep, conditional_timeout, MockTime, MockTimeTestEnv};
use std::time::{Duration, Instant};

#[tokio::test]
async fn test_conditional_sleep_performance() {
    let start = Instant::now();
    
    // This should complete immediately in test mode
    conditional_sleep(Duration::from_secs(10)).await;
    
    let duration = start.elapsed();
    // Should be much faster than 10 seconds
    assert!(duration < Duration::from_millis(100), "Sleep took too long: {:?}", duration);
}

#[tokio::test]
async fn test_conditional_timeout_performance() {
    let start = Instant::now();
    
    // This should not timeout in test mode
    let result = conditional_timeout(
        Duration::from_millis(1),
        async { 
            // Simulate some work
            conditional_sleep(Duration::from_secs(5)).await;
            "success" 
        }
    ).await;
    
    let duration = start.elapsed();
    assert_eq!(result, Ok("success"));
    // Should be much faster than 5 seconds
    assert!(duration < Duration::from_millis(100), "Timeout test took too long: {:?}", duration);
}

#[tokio::test]
async fn test_mock_time_basic_functionality() {
    let mock_time = MockTime::new();
    let start = mock_time.now();
    
    // Advance time by 5 seconds
    mock_time.advance(Duration::from_secs(5));
    let after_advance = mock_time.now();
    
    assert_eq!(after_advance.duration_since(start), Duration::from_secs(5));
}

#[tokio::test]
async fn test_mock_sleep_immediate_completion() {
    let mock_time = MockTime::new();
    let start = mock_time.now();
    let test_start = Instant::now();
    
    // Mock sleep should complete immediately
    mock_time.sleep(Duration::from_secs(10)).await;
    
    let after_sleep = mock_time.now();
    let test_duration = test_start.elapsed();
    
    // Mock time should advance
    assert_eq!(after_sleep.duration_since(start), Duration::from_secs(10));
    // But test should complete quickly
    assert!(test_duration < Duration::from_millis(100), "Mock sleep took too long: {:?}", test_duration);
}

#[tokio::test]
async fn test_mock_timeout_no_timeout() {
    let mock_time = MockTime::new();
    let start = Instant::now();
    
    // Mock timeout should not actually timeout
    let result = mock_time.timeout(
        Duration::from_millis(1),
        async { 
            conditional_sleep(Duration::from_secs(1)).await;
            "success" 
        }
    ).await;
    
    let duration = start.elapsed();
    assert_eq!(result, Ok("success"));
    // Should complete quickly
    assert!(duration < Duration::from_millis(100), "Mock timeout took too long: {:?}", duration);
}

#[tokio::test]
async fn test_mock_time_test_env() {
    let env = MockTimeTestEnv::new();
    let start = Instant::now();
    
    let result = env.run_test(|mock_time| async move {
        let start = mock_time.now();
        mock_time.sleep(Duration::from_secs(5)).await;
        mock_time.now().duration_since(start)
    }).await;
    
    let test_duration = start.elapsed();
    assert_eq!(result, Duration::from_secs(5));
    // Test should complete quickly
    assert!(test_duration < Duration::from_millis(100), "Mock time env test took too long: {:?}", test_duration);
}

#[tokio::test]
async fn test_advance_and_run() {
    let env = MockTimeTestEnv::new();
    let start = Instant::now();
    
    let result = env.advance_and_run(
        Duration::from_secs(10),
        |mock_time| async move {
            mock_time.now()
        }
    ).await;
    
    let test_duration = start.elapsed();
    // Time should be advanced by 10 seconds
    assert!(result.duration_since(Instant::now() - Duration::from_secs(10)) < Duration::from_secs(1));
    // Test should complete quickly
    assert!(test_duration < Duration::from_millis(100), "Advance and run took too long: {:?}", test_duration);
}

#[tokio::test]
async fn test_multiple_conditional_sleeps() {
    let start = Instant::now();
    
    // Multiple sleeps should all complete immediately
    conditional_sleep(Duration::from_secs(1)).await;
    conditional_sleep(Duration::from_secs(2)).await;
    conditional_sleep(Duration::from_secs(3)).await;
    conditional_sleep(Duration::from_secs(4)).await;
    conditional_sleep(Duration::from_secs(5)).await;
    
    let duration = start.elapsed();
    // Total time should be much less than 15 seconds
    assert!(duration < Duration::from_millis(100), "Multiple sleeps took too long: {:?}", duration);
}

#[tokio::test]
async fn test_nested_conditional_operations() {
    let start = Instant::now();
    
    let result = conditional_timeout(
        Duration::from_millis(1),
        async {
            conditional_sleep(Duration::from_secs(1)).await;
            conditional_timeout(
                Duration::from_millis(1),
                async {
                    conditional_sleep(Duration::from_secs(2)).await;
                    "nested_success"
                }
            ).await
        }
    ).await;
    
    let duration = start.elapsed();
    assert_eq!(result, Ok(Ok("nested_success")));
    // Should complete quickly despite nested operations
    assert!(duration < Duration::from_millis(100), "Nested operations took too long: {:?}", duration);
}
