//! Core container tests
//!
//! Tests for core container functionality including container lifecycle,
//! management, and basic container operations.

use clnrm::{CleanroomConfig, CleanroomEnvironment, Error as CleanroomError};
use std::time::Duration;

/// Test container lifecycle management
#[tokio::test]
async fn test_container_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    let config = CleanroomConfig::default();
    let mut environment = CleanroomEnvironment::new(config).await?;

    // Test container registration
    environment
        .register_container("test1".to_string(), "container_id_123".to_string())
        .await?;
    assert!(environment.is_container_registered("test1").await);

    // Test container access
    let container_count = environment.get_container_count().await;
    assert!(container_count >= 1);

    // Test container cleanup
    environment.cleanup().await?;
    assert_eq!(environment.get_container_count().await, 0);

    Ok(())
}
