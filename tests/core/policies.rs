//! Core policy tests
//!
//! Tests for core policy functionality including Policy creation,
//! validation, and policy-based access control.

use clnrm::{Policy, SecurityLevel};

/// Test Policy functionality
#[test]
fn test_policy() {
    // Test default policy
    let policy = Policy::default();
    assert_eq!(policy.security.security_level, SecurityLevel::Standard);
    // Default policy does NOT allow network by default
    assert!(!policy.allows_network());

    // Test locked policy
    let locked_policy = Policy::locked();
    assert_eq!(locked_policy.security.security_level, SecurityLevel::Locked);
    assert!(!locked_policy.allows_network());

    // Test custom policy
    let custom_policy =
        Policy::with_security_level(SecurityLevel::High).with_network_isolation(false);
    assert_eq!(custom_policy.security.security_level, SecurityLevel::High);
    assert!(custom_policy.allows_network());

    // Test policy summary
    let summary = custom_policy.summary();
    assert!(summary.contains("Policy Summary"));
    assert!(summary.contains("Security Level"));
}

/// Test policy serialization
#[test]
fn test_policy_serialization() -> anyhow::Result<()> {
    let policy = Policy::with_security_level(SecurityLevel::High).with_network_isolation(false);

    // Test JSON serialization
    let json = serde_json::to_string(&policy)
        .map_err(|e| anyhow::anyhow!("JSON serialization failed: {}", e))?;
    assert!(json.contains("security_level"));
    assert!(json.contains("network"));

    // Test JSON deserialization
    let deserialized_policy: Policy = serde_json::from_str(&json)
        .map_err(|e| anyhow::anyhow!("JSON deserialization failed: {}", e))?;
    assert_eq!(
        deserialized_policy.security.security_level,
        policy.security.security_level
    );
    assert_eq!(
        deserialized_policy.security.enable_network_isolation,
        policy.security.enable_network_isolation
    );

    Ok(())
}
