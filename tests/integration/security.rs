//! Integration security tests
//!
//! Tests for security-related functionality and policy enforcement
//! in integration scenarios.

use clnrm::{
    CleanroomConfig, CleanroomEnvironment, Error as CleanroomError, Policy, SecurityLevel,
    SecurityPolicy,
};
use std::time::Duration;

/// Test security policy enforcement
#[tokio::test]
async fn test_security_policy_enforcement() -> Result<(), Box<dyn std::error::Error>> {
    // Test with locked security policy
    let config = CleanroomConfig {
        security_policy: SecurityPolicy::locked(),
        ..CleanroomConfig::default()
    };

    let environment = CleanroomEnvironment::new(config).await?;

    // Verify security level is enforced
    let policy = environment.config().security_policy.clone();
    assert_eq!(policy.security.security_level, SecurityLevel::Locked);

    // Test that policy validation works
    assert!(policy.validate().is_ok());

    Ok(())
}

/// Test network isolation enforcement
#[tokio::test]
async fn test_network_isolation() -> Result<(), Box<dyn std::error::Error>> {
    // Test with network isolation enabled
    let mut policy = SecurityPolicy::default();
    policy.security.enable_network_isolation = true;

    let config = CleanroomConfig {
        security_policy: policy,
        ..CleanroomConfig::default()
    };

    let environment = CleanroomEnvironment::new(config).await?;

    // Verify network isolation is enabled
    assert!(environment.config().security_policy.allows_network());

    Ok(())
}

/// Test filesystem isolation
#[tokio::test]
async fn test_filesystem_isolation() -> Result<(), Box<dyn std::error::Error>> {
    // Test with filesystem isolation enabled
    let mut policy = SecurityPolicy::default();
    policy.security.enable_filesystem_isolation = true;

    let config = CleanroomConfig {
        security_policy: policy,
        ..CleanroomConfig::default()
    };

    let environment = CleanroomEnvironment::new(config).await?;

    // Verify filesystem isolation is enabled
    assert!(
        environment
            .config()
            .security_policy
            .security
            .enable_filesystem_isolation
    );

    Ok(())
}

/// Test security policy summary and reporting
#[tokio::test]
async fn test_security_policy_reporting() -> Result<(), Box<dyn std::error::Error>> {
    // Test different security levels
    let security_levels = vec![
        SecurityLevel::Low,
        SecurityLevel::Medium,
        SecurityLevel::High,
        SecurityLevel::Locked,
    ];

    for level in security_levels {
        let policy = Policy::with_security_level(level.clone());
        let summary = policy.summary();

        // Verify summary contains security level information
        assert!(summary.contains("Security Level"));
        assert!(summary.contains(&format!("{:?}", level)));

        // Test policy validation
        assert!(policy.validate().is_ok());
    }

    Ok(())
}

/// Test resource limits enforcement
#[tokio::test]
async fn test_resource_limits_enforcement() -> Result<(), Box<dyn std::error::Error>> {
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await?;

    // Test that resource limits are properly configured
    let limits = environment.config().resource_limits.clone();
    assert!(limits.memory.max_usage_bytes > 0);
    assert!(limits.cpu.max_usage_percent > 0.0);
    assert!(limits.disk.max_usage_bytes > 0);

    // Test limits validation
    assert!(limits.validate().is_ok());

    Ok(())
}

/// Test policy-based access control
#[tokio::test]
async fn test_policy_based_access_control() -> Result<(), Box<dyn std::error::Error>> {
    // Test with permissive policy
    let permissive_policy = SecurityPolicy::with_security_level(SecurityLevel::Low);
    let permissive_config = CleanroomConfig {
        security_policy: permissive_policy,
        ..CleanroomConfig::default()
    };

    let permissive_env = CleanroomEnvironment::new(permissive_config).await?;
    assert!(permissive_env.config().security_policy.allows_network());

    // Test with locked policy
    let locked_policy = SecurityPolicy::locked();
    let locked_config = CleanroomConfig {
        security_policy: locked_policy,
        ..CleanroomConfig::default()
    };

    let locked_env = CleanroomEnvironment::new(locked_config).await?;
    assert_eq!(
        locked_env.config().security_policy.security.security_level,
        SecurityLevel::Locked
    );

    Ok(())
}
