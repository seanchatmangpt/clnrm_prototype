//! Test configuration fixtures and builders
//!
//! Provides standardized test configurations for different testing scenarios.

use clnrm::{CleanroomConfig, SecurityLevel, SecurityPolicy, ResourceLimits};
use std::time::Duration;

/// Test configuration presets for different scenarios
pub struct TestConfigs;

impl TestConfigs {
    /// Ultra-fast configuration for unit tests
    pub fn unit_test() -> CleanroomConfig {
        CleanroomConfig {
            enable_singleton_containers: true,
            container_startup_timeout: Duration::from_micros(100),
            test_execution_timeout: Duration::from_micros(500),
            max_concurrent_containers: 1,
            enable_deterministic_execution: false,
            deterministic_seed: None,
            enable_coverage_tracking: false,
            enable_snapshot_testing: false,
            enable_tracing: false,
            resource_limits: ResourceLimits::default(),
            security_policy: SecurityPolicy::with_security_level(SecurityLevel::Low),
            performance_monitoring: clnrm::config::PerformanceMonitoringConfig::default(),
            container_customizers: std::collections::HashMap::new(),
        }
    }

    /// Fast configuration for integration tests
    pub fn integration_test() -> CleanroomConfig {
        CleanroomConfig {
            enable_singleton_containers: true,
            container_startup_timeout: Duration::from_millis(10),
            test_execution_timeout: Duration::from_millis(50),
            max_concurrent_containers: 2,
            enable_deterministic_execution: false,
            deterministic_seed: None,
            enable_coverage_tracking: false,
            enable_snapshot_testing: false,
            enable_tracing: false,
            resource_limits: ResourceLimits::default(),
            security_policy: SecurityPolicy::with_security_level(SecurityLevel::Medium),
            performance_monitoring: clnrm::config::PerformanceMonitoringConfig::default(),
            container_customizers: std::collections::HashMap::new(),
        }
    }

    /// Mock configuration that avoids real container operations
    pub fn mock_test() -> CleanroomConfig {
        CleanroomConfig {
            enable_singleton_containers: true,
            container_startup_timeout: Duration::from_micros(1),
            test_execution_timeout: Duration::from_micros(10),
            max_concurrent_containers: 1,
            enable_deterministic_execution: true,
            deterministic_seed: Some(42),
            enable_coverage_tracking: false,
            enable_snapshot_testing: false,
            enable_tracing: false,
            resource_limits: ResourceLimits::default(),
            security_policy: SecurityPolicy::with_security_level(SecurityLevel::Low),
            performance_monitoring: clnrm::config::PerformanceMonitoringConfig::default(),
            container_customizers: std::collections::HashMap::new(),
        }
    }

    /// Performance test configuration with minimal overhead
    pub fn performance_test() -> CleanroomConfig {
        CleanroomConfig {
            enable_singleton_containers: true,
            container_startup_timeout: Duration::from_millis(1),
            test_execution_timeout: Duration::from_millis(10),
            max_concurrent_containers: 5,
            enable_deterministic_execution: false,
            deterministic_seed: None,
            enable_coverage_tracking: false,
            enable_snapshot_testing: false,
            enable_tracing: false,
            resource_limits: ResourceLimits::default(),
            security_policy: SecurityPolicy::with_security_level(SecurityLevel::Medium),
            performance_monitoring: clnrm::config::PerformanceMonitoringConfig::default(),
            container_customizers: std::collections::HashMap::new(),
        }
    }

    /// Concurrent test configuration for parallel execution
    pub fn concurrent_test() -> CleanroomConfig {
        CleanroomConfig {
            enable_singleton_containers: true,
            container_startup_timeout: Duration::from_millis(5),
            test_execution_timeout: Duration::from_millis(25),
            max_concurrent_containers: 10,
            enable_deterministic_execution: false,
            deterministic_seed: None,
            enable_coverage_tracking: false,
            enable_snapshot_testing: false,
            enable_tracing: false,
            resource_limits: ResourceLimits::default(),
            security_policy: SecurityPolicy::with_security_level(SecurityLevel::Medium),
            performance_monitoring: clnrm::config::PerformanceMonitoringConfig::default(),
            container_customizers: std::collections::HashMap::new(),
        }
    }

    /// Stress test configuration for high-load scenarios
    pub fn stress_test() -> CleanroomConfig {
        CleanroomConfig {
            enable_singleton_containers: true,
            container_startup_timeout: Duration::from_millis(1),
            test_execution_timeout: Duration::from_millis(10),
            max_concurrent_containers: 100,
            enable_deterministic_execution: false,
            deterministic_seed: None,
            enable_coverage_tracking: false,
            enable_snapshot_testing: false,
            enable_tracing: false,
            resource_limits: ResourceLimits::default(),
            security_policy: SecurityPolicy::with_security_level(SecurityLevel::Medium),
            performance_monitoring: clnrm::config::PerformanceMonitoringConfig::default(),
            container_customizers: std::collections::HashMap::new(),
        }
    }

    /// Comprehensive test configuration with all features enabled
    pub fn comprehensive_test() -> CleanroomConfig {
        CleanroomConfig {
            enable_singleton_containers: true,
            container_startup_timeout: Duration::from_secs(5),
            test_execution_timeout: Duration::from_secs(10),
            max_concurrent_containers: 5,
            enable_deterministic_execution: true,
            deterministic_seed: Some(42),
            enable_coverage_tracking: true,
            enable_snapshot_testing: true,
            enable_tracing: true,
            resource_limits: ResourceLimits::default(),
            security_policy: SecurityPolicy::with_security_level(SecurityLevel::High),
            performance_monitoring: clnrm::config::PerformanceMonitoringConfig::default(),
            container_customizers: std::collections::HashMap::new(),
        }
    }

    /// Security-focused test configuration
    pub fn security_test() -> CleanroomConfig {
        CleanroomConfig {
            enable_singleton_containers: true,
            container_startup_timeout: Duration::from_secs(2),
            test_execution_timeout: Duration::from_secs(5),
            max_concurrent_containers: 3,
            enable_deterministic_execution: true,
            deterministic_seed: Some(42),
            enable_coverage_tracking: false,
            enable_snapshot_testing: false,
            enable_tracing: true,
            resource_limits: ResourceLimits::default(),
            security_policy: SecurityPolicy::with_security_level(SecurityLevel::High),
            performance_monitoring: clnrm::config::PerformanceMonitoringConfig::default(),
            container_customizers: std::collections::HashMap::new(),
        }
    }
}

/// Test configuration builder for custom configurations
pub struct TestConfigBuilder {
    config: CleanroomConfig,
}

impl TestConfigBuilder {
    /// Create a new test configuration builder
    pub fn new() -> Self {
        Self {
            config: CleanroomConfig::default(),
        }
    }

    /// Start with a preset configuration
    pub fn with_preset(preset: fn() -> CleanroomConfig) -> Self {
        Self { config: preset() }
    }

    /// Set container startup timeout
    pub fn container_startup_timeout(mut self, timeout: Duration) -> Self {
        self.config.container_startup_timeout = timeout;
        self
    }

    /// Set test execution timeout
    pub fn test_execution_timeout(mut self, timeout: Duration) -> Self {
        self.config.test_execution_timeout = timeout;
        self
    }

    /// Set maximum concurrent containers
    pub fn max_concurrent_containers(mut self, max: usize) -> Self {
        self.config.max_concurrent_containers = max;
        self
    }

    /// Enable/disable singleton containers
    pub fn singleton_containers(mut self, enable: bool) -> Self {
        self.config.enable_singleton_containers = enable;
        self
    }

    /// Enable/disable deterministic execution
    pub fn deterministic_execution(mut self, enable: bool) -> Self {
        self.config.enable_deterministic_execution = enable;
        self
    }

    /// Enable/disable coverage tracking
    pub fn coverage_tracking(mut self, enable: bool) -> Self {
        self.config.enable_coverage_tracking = enable;
        self
    }

    /// Enable/disable snapshot testing
    pub fn snapshot_testing(mut self, enable: bool) -> Self {
        self.config.enable_snapshot_testing = enable;
        self
    }

    /// Enable/disable tracing
    pub fn tracing(mut self, enable: bool) -> Self {
        self.config.enable_tracing = enable;
        self
    }

    /// Set security policy level
    pub fn security_policy(mut self, level: SecurityLevel) -> Self {
        self.config.security_policy = level;
        self
    }

    /// Build the configuration
    pub fn build(self) -> CleanroomConfig {
        self.config
    }
}

impl Default for TestConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_presets() {
        let unit_config = TestConfigs::unit_test();
        assert_eq!(
            unit_config.container_startup_timeout,
            Duration::from_micros(100)
        );
        assert_eq!(
            unit_config.test_execution_timeout,
            Duration::from_micros(500)
        );
        assert_eq!(unit_config.max_concurrent_containers, 1);

        let integration_config = TestConfigs::integration_test();
        assert_eq!(
            integration_config.container_startup_timeout,
            Duration::from_millis(10)
        );
        assert_eq!(
            integration_config.test_execution_timeout,
            Duration::from_millis(50)
        );
        assert_eq!(integration_config.max_concurrent_containers, 2);

        let mock_config = TestConfigs::mock_test();
        assert_eq!(
            mock_config.container_startup_timeout,
            Duration::from_micros(1)
        );
        assert_eq!(
            mock_config.test_execution_timeout,
            Duration::from_micros(10)
        );
        assert!(mock_config.enable_deterministic_execution);
    }

    #[test]
    fn test_config_builder() {
        let config = TestConfigBuilder::new()
            .container_startup_timeout(Duration::from_millis(100))
            .test_execution_timeout(Duration::from_millis(200))
            .max_concurrent_containers(5)
            .security_policy(SecurityLevel::Strict)
            .build();

        assert_eq!(config.container_startup_timeout, Duration::from_millis(100));
        assert_eq!(config.test_execution_timeout, Duration::from_millis(200));
        assert_eq!(config.max_concurrent_containers, 5);
        assert_eq!(config.security_policy, SecurityLevel::Strict);
    }

    #[test]
    fn test_config_builder_with_preset() {
        let config = TestConfigBuilder::with_preset(TestConfigs::unit_test)
            .max_concurrent_containers(3)
            .security_policy(SecurityLevel::Strict)
            .build();

        assert_eq!(config.container_startup_timeout, Duration::from_micros(100));
        assert_eq!(config.max_concurrent_containers, 3);
        assert_eq!(config.security_policy, SecurityLevel::Strict);
    }
}
