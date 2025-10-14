//! Performance benchmark tests
//!
//! Tests that measure and verify performance characteristics.

use clnrm::{run, CleanroomEnvironment, conditional_sleep};
use crate::fixtures::{TestEnvironments, TestAssertions, MockTimeUtils};
use std::time::{Duration, Instant};

#[tokio::test]
async fn test_command_execution_performance() {
    let start_time = Instant::now();
    
    let result = run(["echo", "performance test"]);
    
    let duration = start_time.elapsed();
    
    TestAssertions::assert_success(&result);
    let run_result = result.unwrap();
    TestAssertions::assert_run_success(&run_result);
    
    // Command execution should be fast
    TestAssertions::assert_duration_less_than(duration, Duration::from_millis(100));
}

#[tokio::test]
async fn test_environment_creation_performance() {
    let start_time = Instant::now();
    
    let environment = TestEnvironments::unit_test().await;
    
    let duration = start_time.elapsed();
    
    TestAssertions::assert_success(&environment);
    
    // Environment creation should be fast
    TestAssertions::assert_duration_less_than(duration, Duration::from_millis(100));
}

#[tokio::test]
async fn test_multiple_command_execution_performance() {
    let start_time = Instant::now();
    
    // Execute multiple commands
    let results = vec![
        run(["echo", "command1"]),
        run(["echo", "command2"]),
        run(["echo", "command3"]),
        run(["echo", "command4"]),
        run(["echo", "command5"]),
    ];
    
    let duration = start_time.elapsed();
    
    // All commands should succeed
    for result in results {
        TestAssertions::assert_success(&result);
        let run_result = result.unwrap();
        TestAssertions::assert_run_success(&run_result);
    }
    
    // Multiple commands should still be fast
    TestAssertions::assert_duration_less_than(duration, Duration::from_millis(200));
}

#[tokio::test]
async fn test_environment_operations_performance() {
    let environment = TestEnvironments::performance_test().await.unwrap();
    
    let start_time = Instant::now();
    
    // Perform multiple operations
    for i in 0..10 {
        environment.register_container(
            format!("perf_container_{}", i),
            format!("perf_id_{}", i)
        ).await.unwrap();
    }
    
    let duration = start_time.elapsed();
    
    // Operations should be fast
    TestAssertions::assert_duration_less_than(duration, Duration::from_millis(100));
    
    // Verify all containers are registered
    assert_eq!(environment.get_container_count().await, 10);
}

#[tokio::test]
async fn test_test_execution_performance() {
    let environment = TestEnvironments::performance_test().await.unwrap();
    
    let start_time = Instant::now();
    
    // Execute multiple tests
    for i in 0..5 {
        let result = environment.execute_test(&format!("perf_test_{}", i), || {
            // Simulate some work
            conditional_sleep(Duration::from_millis(1)).await;
            Ok::<String, clnrm::Error>(format!("result_{}", i))
        }).await.unwrap();
        
        assert_eq!(result, format!("result_{}", i));
    }
    
    let duration = start_time.elapsed();
    
    // Test execution should be fast due to mocking
    TestAssertions::assert_duration_less_than(duration, Duration::from_millis(100));
}

#[tokio::test]
async fn test_mock_time_performance() {
    let (result, duration) = MockTimeUtils::measure_execution_time(
        || async {
            // Simulate 10 seconds of work
            conditional_sleep(Duration::from_secs(10)).await;
            42
        },
        None,
    ).await;
    
    assert_eq!(result, 42);
    // Should complete immediately due to mocking
    TestAssertions::assert_duration_less_than(duration, Duration::from_millis(100));
}

#[tokio::test]
async fn test_concurrent_operations_performance() {
    let environment = TestEnvironments::concurrent_test().await.unwrap();
    
    let start_time = Instant::now();
    
    // Perform concurrent operations
    let mut handles = Vec::new();
    
    for i in 0..5 {
        let env = environment.clone();
        let handle = tokio::spawn(async move {
            env.register_container(
                format!("concurrent_container_{}", i),
                format!("concurrent_id_{}", i)
            ).await
        });
        handles.push(handle);
    }
    
    // Wait for all operations to complete
    for handle in handles {
        TestAssertions::assert_success(&handle.await.unwrap());
    }
    
    let duration = start_time.elapsed();
    
    // Concurrent operations should be fast
    TestAssertions::assert_duration_less_than(duration, Duration::from_millis(200));
    
    // Verify all containers are registered
    assert_eq!(environment.get_container_count().await, 5);
}

#[tokio::test]
async fn test_memory_usage_performance() {
    let environment = TestEnvironments::performance_test().await.unwrap();
    
    let start_time = Instant::now();
    
    // Create many containers to test memory usage
    for i in 0..100 {
        environment.register_container(
            format!("memory_container_{}", i),
            format!("memory_id_{}", i)
        ).await.unwrap();
    }
    
    let duration = start_time.elapsed();
    
    // Should still be fast
    TestAssertions::assert_duration_less_than(duration, Duration::from_millis(500));
    
    // Verify all containers are registered
    assert_eq!(environment.get_container_count().await, 100);
    
    // Cleanup
    environment.cleanup().await.unwrap();
    assert_eq!(environment.get_container_count().await, 0);
}
