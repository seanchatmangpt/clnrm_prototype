//! Environment management tests
//!
//! Tests for CleanroomEnvironment creation, configuration, and lifecycle.

use crate::fixtures::{TestAssertions, TestEnvironmentBuilder, TestEnvironments};
use clnrm::{CleanroomConfig, CleanroomEnvironment, Error as CleanroomError};
use std::time::Duration;

#[tokio::test]
async fn test_environment_creation() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::unit_test().await?;

    // Verify environment is properly initialized
    assert!(!environment.session_id().is_nil());

    // Verify configuration is set correctly
    let config = environment.config();
    assert!(config.test_execution_timeout >= Duration::from_millis(1));

    Ok(())
}

#[tokio::test]
async fn test_environment_creation_with_custom_config() -> Result<(), CleanroomError> {
    let environment = TestEnvironmentBuilder::new()
        .container_startup_timeout(Duration::from_millis(100))
        .test_execution_timeout(Duration::from_millis(200))
        .max_concurrent_containers(5)
        .build()
        .await?;

    let config = environment.config();
    assert_eq!(config.container_startup_timeout, Duration::from_millis(100));
    assert_eq!(config.test_execution_timeout, Duration::from_millis(200));
    assert_eq!(config.max_concurrent_containers, 5);

    Ok(())
}

#[tokio::test]
async fn test_environment_presets() -> Result<(), CleanroomError> {
    // Test all environment presets
    let unit_env = TestEnvironments::unit_test().await?;
    let integration_env = TestEnvironments::integration_test().await?;
    let mock_env = TestEnvironments::mock_test().await?;
    let performance_env = TestEnvironments::performance_test().await?;
    let concurrent_env = TestEnvironments::concurrent_test().await?;
    let comprehensive_env = TestEnvironments::comprehensive_test().await?;
    let security_env = TestEnvironments::security_test().await?;

    // Verify all environments are created successfully
    assert!(!unit_env.session_id().is_nil());
    assert!(!integration_env.session_id().is_nil());
    assert!(!mock_env.session_id().is_nil());
    assert!(!performance_env.session_id().is_nil());
    assert!(!concurrent_env.session_id().is_nil());
    assert!(!comprehensive_env.session_id().is_nil());
    assert!(!security_env.session_id().is_nil());

    Ok(())
}

#[tokio::test]
async fn test_environment_session_ids_unique() -> Result<(), CleanroomError> {
    let env1 = TestEnvironments::unit_test().await?;
    let env2 = TestEnvironments::unit_test().await?;

    // Session IDs should be unique
    assert_ne!(env1.session_id(), env2.session_id());

    Ok(())
}

#[tokio::test]
async fn test_environment_configuration_access() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::unit_test().await?;
    let config = environment.config();

    // Verify we can access configuration
    assert!(config.enable_singleton_containers);
    assert!(config.container_startup_timeout > Duration::from_secs(0));
    assert!(config.test_execution_timeout > Duration::from_secs(0));
    assert!(config.max_concurrent_containers > 0);

    Ok(())
}

#[tokio::test]
async fn test_environment_cleanup() -> Result<(), CleanroomError> {
    let environment = TestEnvironments::unit_test().await?;

    // Test that cleanup can be called without error
    environment.cleanup().await?;

    Ok(())
}

#[tokio::test]
async fn test_environment_with_timeout() -> Result<(), CleanroomError> {
    let environment = TestEnvironmentBuilder::new()
        .container_startup_timeout(Duration::from_millis(50))
        .build_with_timeout(Duration::from_secs(1))
        .await?;

    assert!(!environment.session_id().is_nil());

    Ok(())
}

#[tokio::test]
async fn test_environment_builder_with_preset() -> Result<(), CleanroomError> {
    use crate::fixtures::config::TestConfigs;

    let environment = TestEnvironmentBuilder::with_preset(TestConfigs::unit_test)
        .max_concurrent_containers(3)
        .build()
        .await?;

    let config = environment.config();
    assert_eq!(config.max_concurrent_containers, 3);
    assert_eq!(config.container_startup_timeout, Duration::from_micros(100));

    Ok(())
}
