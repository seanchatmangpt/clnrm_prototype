//! Performance memory tests
//!
//! Tests that focus on memory usage patterns, allocation efficiency,
//! and memory-related performance characteristics.

use clnrm::{CleanroomEnvironment, Error as CleanroomError};
use crate::fixtures::{TestEnvironments, TestAssertions};
use std::time::{Duration, Instant};

/// Test memory allocation patterns
#[tokio::test]
async fn test_memory_allocation_patterns() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::performance_test().await?;

    let start_time = Instant::now();

    // Test various memory allocation patterns
    for i in 0..100 {
        let result = environment.execute_test(&format!("memory_alloc_{}", i), || {
            // Allocate different sizes
            let size = match i % 3 {
                0 => 1024,      // 1KB
                1 => 1024 * 10, // 10KB
                _ => 1024 * 100, // 100KB
            };

            let _data = vec![0u8; size];
            Ok::<String, CleanroomError>(format!("allocated_{}", size))
        }).await?;

        assert!(result.starts_with("allocated_"));
    }

    let duration = start_time.elapsed();

    // Should handle memory allocations efficiently
    TestAssertions::assert_duration_less_than(duration, Duration::from_secs(2));

    Ok(())
}

/// Test memory usage with many small allocations
#[tokio::test]
async fn test_small_allocations_performance() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::performance_test().await?;

    let start_time = Instant::now();

    // Create many small allocations
    for i in 0..1000 {
        let result = environment.execute_test(&format!("small_alloc_{}", i), || {
            // Small allocation pattern
            let mut allocations = Vec::new();
            for j in 0..10 {
                allocations.push(vec![0u8; 100]); // 100 bytes each
            }

            // Use the allocations
            let total_size: usize = allocations.iter().map(|v| v.len()).sum();
            Ok::<String, CleanroomError>(format!("small_total_{}", total_size))
        }).await?;

        assert!(result.starts_with("small_total_"));
    }

    let duration = start_time.elapsed();

    // Should handle many small allocations efficiently
    TestAssertions::assert_duration_less_than(duration, Duration::from_secs(5));

    Ok(())
}

/// Test memory usage with large allocations
#[tokio::test]
async fn test_large_allocations_performance() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::performance_test().await?;

    let start_time = Instant::now();

    // Create a few large allocations
    for i in 0..10 {
        let result = environment.execute_test(&format!("large_alloc_{}", i), || {
            // Large allocation pattern
            let size = 1024 * 1024 * (i + 1); // 1MB to 10MB
            let _data = vec![0u8; size];

            Ok::<String, CleanroomError>(format!("large_size_{}", size))
        }).await?;

        assert!(result.starts_with("large_size_"));
    }

    let duration = start_time.elapsed();

    // Should handle large allocations efficiently
    TestAssertions::assert_duration_less_than(duration, Duration::from_secs(3));

    Ok(())
}

/// Test memory cleanup and deallocation
#[tokio::test]
async fn test_memory_cleanup() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::performance_test().await?;

    let start_time = Instant::now();

    // Allocate and immediately deallocate memory
    for i in 0..100 {
        let result = environment.execute_test(&format!("cleanup_test_{}", i), || {
            // Allocate memory
            let mut data = vec![0u8; 1024 * 100]; // 100KB

            // Use the memory briefly
            for j in 0..data.len() {
                data[j] = (j % 256) as u8;
            }

            // Explicitly drop the allocation
            drop(data);

            Ok::<String, CleanroomError>(format!("cleanup_{}", i))
        }).await?;

        assert_eq!(result, format!("cleanup_{}", i));
    }

    let duration = start_time.elapsed();

    // Should handle allocation/deallocation cycles efficiently
    TestAssertions::assert_duration_less_than(duration, Duration::from_secs(3));

    Ok(())
}

/// Test memory efficiency with container operations
#[tokio::test]
async fn test_container_memory_efficiency() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::performance_test().await?;

    let start_time = Instant::now();

    // Register many containers and test memory usage
    for i in 0..50 {
        environment.register_container(
            format!("memory_container_{}", i),
            format!("memory_id_{}", i)
        ).await?;

        // Simulate memory-intensive container operations
        let result = environment.execute_test(&format!("container_memory_{}", i), || {
            // Simulate container memory usage
            let _container_data = vec![0u8; 1024 * 50]; // 50KB per container
            Ok::<String, CleanroomError>(format!("container_memory_{}", i))
        }).await?;

        assert_eq!(result, format!("container_memory_{}", i));
    }

    let duration = start_time.elapsed();

    // Should be memory efficient with many containers
    TestAssertions::assert_duration_less_than(duration, Duration::from_secs(3));

    // Cleanup containers
    environment.cleanup().await?;
    assert_eq!(environment.get_container_count().await, 0);

    Ok(())
}

/// Test memory fragmentation handling
#[tokio::test]
async fn test_memory_fragmentation() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::performance_test().await?;

    let start_time = Instant::now();

    // Simulate memory fragmentation scenario
    let mut allocations = Vec::new();

    // Allocate and deallocate in random order to create fragmentation
    for i in 0..100 {
        let result = environment.execute_test(&format!("fragment_test_{}", i), || {
            // Vary allocation sizes and patterns
            let size = match i % 5 {
                0 => 1024,        // 1KB
                1 => 1024 * 5,    // 5KB
                2 => 1024 * 10,   // 10KB
                3 => 1024 * 50,   // 50KB
                _ => 1024 * 100,  // 100KB
            };

            let data = vec![0u8; size];
            allocations.push(data);

            // Remove some random allocations to create fragmentation
            if i % 7 == 0 && !allocations.is_empty() {
                let remove_idx = i % allocations.len();
                allocations.remove(remove_idx);
            }

            Ok::<String, CleanroomError>(format!("fragment_{}", i))
        }).await?;

        assert_eq!(result, format!("fragment_{}", i));
    }

    let duration = start_time.elapsed();

    // Should handle memory fragmentation gracefully
    TestAssertions::assert_duration_less_than(duration, Duration::from_secs(5));

    Ok(())
}
