//! Security-related tests for cleanroom testing framework
//!
//! # Security Testing Best Practices
//!
//! ## Test Organization
//! ```rust
//! #[cfg(test)]
//! mod security_tests {
//!     use super::*;
//!     use proptest::prelude::*;
//!
//!     // 1. Unit tests for individual security functions
//!     #[test]
//!     fn test_security_policy_creation() { /* ... */ }
//!
//!     // 2. Integration tests for security components
//!     #[tokio::test]
//!     async fn test_security_policy_enforcement() { /* ... */ }
//!
//!     // 3. Property tests for security invariants
//!     proptest! {
//!         #[test]
//!         fn test_security_policy_properties(
//!             level in prop::sample::select(&[
//!                 SecurityLevel::Permissive,
//!                 SecurityLevel::Standard,
//!                 SecurityLevel::Strict,
//!                 SecurityLevel::Locked,
//!             ]),
//!         ) {
//!             // Test that security levels maintain invariants
//!             let policy = SecurityPolicy::with_security_level(level);
//!             prop_assert!(policy.validate().is_ok());
//!         }
//!     }
//! }
//! ```
//!
//! ## Security Test Patterns
//! - **Authorization Tests**: Test access control and permissions
//! - **Authentication Tests**: Test credential validation
//! - **Isolation Tests**: Test sandbox boundaries
//! - **Injection Tests**: Test against malicious inputs
//! - **Boundary Tests**: Test edge cases and limits
//!
//! ## Security Test Naming
//! ```rust
//! test_security_policy_locked_blocks_all_network_access
//! test_security_policy_permissive_allows_filesystem_access
//! test_security_policy_strict_validates_allowed_ports
//! test_security_policy_serialization_preserves_security_level
//! ```

use clnrm::{
    policy::{FilesystemIsolation, NetworkIsolation, SecurityLevel, SecurityPolicy},
    CleanroomConfig, CleanroomEnvironment, Policy,
};
use std::time::Duration;

#[tokio::test]
async fn test_security_policy_creation() {
    let policy = SecurityPolicy::default();
    assert_eq!(policy.security_level, SecurityLevel::Standard);
    assert!(policy.enable_network_isolation);
    assert!(policy.enable_filesystem_isolation);
}

#[tokio::test]
async fn test_security_policy_locked() {
    let locked_policy = SecurityPolicy::locked();
    assert_eq!(locked_policy.security_level, SecurityLevel::Locked);
    assert!(!locked_policy.allows_network());
    assert!(!locked_policy.allows_filesystem_access());
}

#[tokio::test]
async fn test_security_policy_permissive() {
    let permissive_policy = SecurityPolicy::permissive();
    assert_eq!(permissive_policy.security_level, SecurityLevel::Permissive);
    assert!(permissive_policy.allows_network());
    assert!(permissive_policy.allows_filesystem_access());
}

#[tokio::test]
async fn test_policy_enforcement() {
    let mut config = CleanroomConfig::default();
    config.security_policy = SecurityPolicy::locked();

    let environment = CleanroomEnvironment::new(config).await.unwrap();
    let policy = environment.get_policy().await.unwrap();

    assert_eq!(policy.security_level, SecurityLevel::Locked);
    assert!(!policy.allows_network());
}

#[tokio::test]
async fn test_network_isolation_levels() {
    // Test different network isolation levels
    let standard = SecurityPolicy {
        security_level: SecurityLevel::Standard,
        enable_network_isolation: true,
        ..Default::default()
    };

    let locked = SecurityPolicy {
        security_level: SecurityLevel::Locked,
        enable_network_isolation: true,
        ..Default::default()
    };

    assert!(standard.allows_network()); // Standard allows some network
    assert!(!locked.allows_network()); // Locked blocks all network
}

#[tokio::test]
async fn test_filesystem_isolation() {
    let policy = SecurityPolicy {
        security_level: SecurityLevel::Strict,
        enable_filesystem_isolation: true,
        allowed_paths: vec!["/tmp".to_string(), "/var".to_string()],
        ..Default::default()
    };

    // Test that filesystem isolation is properly configured
    assert!(policy.enable_filesystem_isolation);
    assert!(!policy.allowed_paths.is_empty());
}

#[tokio::test]
async fn test_policy_serialization() {
    let policy = SecurityPolicy {
        security_level: SecurityLevel::High,
        enable_network_isolation: true,
        enable_filesystem_isolation: true,
        allowed_ports: vec![8080, 443],
        ..Default::default()
    };

    // Test JSON serialization
    let json = serde_json::to_string(&policy).unwrap();
    assert!(json.contains("High"));
    assert!(json.contains("8080"));

    // Test deserialization
    let deserialized: SecurityPolicy = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.security_level, SecurityLevel::High);
    assert_eq!(deserialized.allowed_ports, vec![8080, 443]);
}

#[tokio::test]
async fn test_policy_validation() {
    let mut policy = SecurityPolicy::default();

    // Valid policy should pass validation
    assert!(policy.validate().is_ok());

    // Invalid policy with no allowed ports should fail
    policy.allowed_ports.clear();
    assert!(policy.validate().is_err());
}

#[tokio::test]
async fn test_environment_isolation() {
    let config = CleanroomConfig {
        security_policy: SecurityPolicy::locked(),
        ..Default::default()
    };

    let environment = CleanroomEnvironment::new(config).await.unwrap();

    // Test that environment respects security policy
    let policy = environment.get_policy().await.unwrap();
    assert!(!policy.allows_network());
    assert!(!policy.allows_filesystem_access());
}

#[tokio::test]
async fn test_policy_summary() {
    let policy = SecurityPolicy {
        security_level: SecurityLevel::Strict,
        enable_network_isolation: true,
        enable_filesystem_isolation: true,
        ..Default::default()
    };

    let summary = policy.summary();
    assert!(summary.contains("Strict"));
    assert!(summary.contains("Network Isolation"));
    assert!(summary.contains("Filesystem Isolation"));
}
