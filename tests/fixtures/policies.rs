//! Test policy fixtures and builders
//!
//! Provides standardized test policy creation and management.

use clnrm::{Policy, SecurityLevel};
use std::collections::HashMap;

/// Test policy fixtures
pub struct TestPolicies;

impl TestPolicies {
    /// Create a permissive test policy
    pub fn permissive() -> Policy {
        Policy {
            security_level: SecurityLevel::Permissive,
            network: clnrm::NetworkPolicy {
                enable_network_isolation: false,
                enable_port_scanning: false,
                enable_file_system_isolation: false,
            },
            ..Policy::default()
        }
    }

    /// Create a standard test policy
    pub fn standard() -> Policy {
        Policy {
            security_level: SecurityLevel::Standard,
            network: clnrm::NetworkPolicy {
                enable_network_isolation: true,
                enable_port_scanning: false,
                enable_file_system_isolation: true,
            },
            ..Policy::default()
        }
    }

    /// Create a strict test policy
    pub fn strict() -> Policy {
        Policy {
            security_level: SecurityLevel::Strict,
            network: clnrm::NetworkPolicy {
                enable_network_isolation: true,
                enable_port_scanning: true,
                enable_file_system_isolation: true,
            },
            ..Policy::default()
        }
    }

    /// Create a locked test policy
    pub fn locked() -> Policy {
        Policy {
            security_level: SecurityLevel::Locked,
            network: clnrm::NetworkPolicy {
                enable_network_isolation: true,
                enable_port_scanning: true,
                enable_file_system_isolation: true,
            },
            ..Policy::default()
        }
    }

    /// Create a network-isolated test policy
    pub fn network_isolated() -> Policy {
        Policy {
            security_level: SecurityLevel::Standard,
            network: clnrm::NetworkPolicy {
                enable_network_isolation: true,
                enable_port_scanning: false,
                enable_file_system_isolation: false,
            },
            ..Policy::default()
        }
    }

    /// Create a filesystem-isolated test policy
    pub fn filesystem_isolated() -> Policy {
        Policy {
            security_level: SecurityLevel::Standard,
            network: clnrm::NetworkPolicy {
                enable_network_isolation: false,
                enable_port_scanning: false,
                enable_file_system_isolation: true,
            },
            ..Policy::default()
        }
    }
}

/// Test policy builder for custom policies
pub struct TestPolicyBuilder {
    policy: Policy,
}

impl TestPolicyBuilder {
    /// Create a new test policy builder
    pub fn new() -> Self {
        Self {
            policy: Policy::default(),
        }
    }

    /// Start with a preset policy
    pub fn with_preset(preset: fn() -> Policy) -> Self {
        Self { policy: preset() }
    }

    /// Set the security level
    pub fn security_level(mut self, level: SecurityLevel) -> Self {
        self.policy.security_level = level;
        self
    }

    /// Enable/disable network isolation
    pub fn network_isolation(mut self, enable: bool) -> Self {
        self.policy.network.enable_network_isolation = enable;
        self
    }

    /// Enable/disable port scanning
    pub fn port_scanning(mut self, enable: bool) -> Self {
        self.policy.network.enable_port_scanning = enable;
        self
    }

    /// Enable/disable filesystem isolation
    pub fn filesystem_isolation(mut self, enable: bool) -> Self {
        self.policy.network.enable_file_system_isolation = enable;
        self
    }

    /// Build the policy
    pub fn build(self) -> Policy {
        self.policy
    }
}

impl Default for TestPolicyBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_presets() {
        let permissive = TestPolicies::permissive();
        assert_eq!(permissive.security_level, SecurityLevel::Permissive);
        assert!(!permissive.network.enable_network_isolation);

        let standard = TestPolicies::standard();
        assert_eq!(standard.security_level, SecurityLevel::Standard);
        assert!(standard.network.enable_network_isolation);

        let strict = TestPolicies::strict();
        assert_eq!(strict.security_level, SecurityLevel::Strict);
        assert!(strict.network.enable_network_isolation);
        assert!(strict.network.enable_port_scanning);

        let locked = TestPolicies::locked();
        assert_eq!(locked.security_level, SecurityLevel::Locked);
        assert!(locked.network.enable_network_isolation);
        assert!(locked.network.enable_port_scanning);
        assert!(locked.network.enable_file_system_isolation);
    }

    #[test]
    fn test_policy_builder() {
        let policy = TestPolicyBuilder::new()
            .security_level(SecurityLevel::Strict)
            .network_isolation(true)
            .port_scanning(true)
            .filesystem_isolation(true)
            .build();

        assert_eq!(policy.security_level, SecurityLevel::Strict);
        assert!(policy.network.enable_network_isolation);
        assert!(policy.network.enable_port_scanning);
        assert!(policy.network.enable_file_system_isolation);
    }

    #[test]
    fn test_policy_builder_with_preset() {
        let policy = TestPolicyBuilder::with_preset(TestPolicies::permissive)
            .network_isolation(true)
            .port_scanning(true)
            .build();

        assert_eq!(policy.security_level, SecurityLevel::Permissive);
        assert!(policy.network.enable_network_isolation);
        assert!(policy.network.enable_port_scanning);
    }
}
