//! Common test utilities and shared test code for cleanroom framework

use clnrm::{
    CleanroomConfig, CleanroomEnvironment, Error as CleanroomError, GenericContainer, Policy,
    PostgresContainer, RedisContainer, ResourceLimits, SecurityLevel,
};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;

/// Common test timeout duration
pub const TEST_TIMEOUT: Duration = Duration::from_secs(30);

/// Common test configuration
pub fn default_test_config() -> CleanroomConfig {
    CleanroomConfig {
        enable_singleton_containers: true,
        container_startup_timeout: Duration::from_secs(10),
        test_execution_timeout: Duration::from_secs(30),
        max_concurrent_containers: 5,
        enable_deterministic_execution: true,
        enable_coverage_tracking: true,
        enable_snapshot_testing: true,
        enable_tracing: true,
        security_policy: SecurityPolicy::default(),
        ..CleanroomConfig::default()
    }
}

/// Fast test configuration for quick tests
pub fn fast_test_config() -> CleanroomConfig {
    CleanroomConfig {
        enable_singleton_containers: true,
        container_startup_timeout: Duration::from_secs(5),
        test_execution_timeout: Duration::from_secs(10),
        max_concurrent_containers: 3,
        enable_deterministic_execution: false,
        enable_coverage_tracking: false,
        enable_snapshot_testing: false,
        enable_tracing: false,
        security_policy: SecurityPolicy::default(),
        ..CleanroomConfig::default()
    }
}

/// Comprehensive test configuration
pub fn comprehensive_test_config() -> CleanroomConfig {
    CleanroomConfig {
        enable_singleton_containers: true,
        container_startup_timeout: Duration::from_secs(60),
        test_execution_timeout: Duration::from_secs(300),
        max_concurrent_containers: 10,
        enable_deterministic_execution: true,
        enable_coverage_tracking: true,
        enable_snapshot_testing: true,
        enable_tracing: true,
        security_policy: SecurityPolicy::default(),
        ..CleanroomConfig::default()
    }
}

/// Create a test environment with default configuration
pub async fn create_test_environment() -> Result<Arc<CleanroomEnvironment>, CleanroomError> {
    let config = default_test_config();
    let environment = CleanroomEnvironment::new(config).await?;
    Ok(Arc::new(environment))
}

/// Create a test environment with custom configuration
pub async fn create_test_environment_with_config(
    config: CleanroomConfig,
) -> Result<Arc<CleanroomEnvironment>, CleanroomError> {
    let environment = CleanroomEnvironment::new(config).await?;
    Ok(Arc::new(environment))
}

/// Create a test Postgres container
pub fn create_test_postgres_container() -> Result<PostgresContainer> {
    PostgresContainer::new("testdb", "testuser", "testpass")
}

/// Create a test Redis container
pub fn create_test_redis_container() -> Result<RedisContainer> {
    RedisContainer::new(None)
}

/// Create a test generic container
pub fn create_test_generic_container() -> Result<GenericContainer> {
    GenericContainer::new("test", "nginx", "latest")
}

/// Create a test policy
pub fn create_test_policy() -> Policy {
    Policy::with_security_level(SecurityLevel::Standard).with_network_isolation(false)
}

/// Create test resource limits
pub fn create_test_resource_limits() -> ResourceLimits {
    ResourceLimits::new()
        .with_memory_limits(512 * 1024 * 1024) // 512 MB in bytes
        .with_cpu_limits(50.0, 2) // 50% CPU, 2 cores
}

/// Wait for a condition to be true with timeout
pub async fn wait_for_condition<F, Fut>(
    condition: F,
    timeout_duration: Duration,
) -> Result<bool, CleanroomError>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = bool>,
{
    let start_time = std::time::Instant::now();
    while start_time.elapsed() < timeout_duration {
        if condition().await {
            return Ok(true);
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    Ok(false)
}

/// Execute a test with timeout
pub async fn execute_test_with_timeout<F, Fut, T>(
    test: F,
    timeout_duration: Duration,
) -> Result<T, CleanroomError>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, CleanroomError>>,
{
    timeout(timeout_duration, test())
        .await
        .map_err(|_| CleanroomError::validation_error("Test timeout"))?
}

/// Generate test data
pub fn generate_test_data(size: usize) -> serde_json::Value {
    let mut data = serde_json::Map::new();
    for i in 0..size {
        data.insert(
            format!("key_{}", i),
            serde_json::Value::String(format!("value_{}", i)),
        );
    }
    serde_json::Value::Object(data)
}

/// Create test snapshot data
pub fn create_test_snapshot_data() -> serde_json::Value {
    serde_json::json!({
        "test_data": "snapshot_value",
        "timestamp": "2024-01-01T00:00:00Z",
        "metadata": {
            "version": "1.0",
            "environment": "test"
        }
    })
}

/// Test assertion helpers
pub mod assertions {
    use super::*;

    /// Assert that two values are approximately equal (for floating point comparisons)
    pub fn assert_approx_eq(a: f64, b: f64, epsilon: f64) {
        assert!(
            (a - b).abs() < epsilon,
            "{} is not approximately equal to {} (epsilon: {})",
            a,
            b,
            epsilon
        );
    }

    /// Assert that a duration is within expected range
    pub fn assert_duration_in_range(duration: Duration, min: Duration, max: Duration) {
        assert!(
            duration >= min && duration <= max,
            "Duration {:?} is not in range [{:?}, {:?}]",
            duration,
            min,
            max
        );
    }

    /// Assert that a string contains expected content
    pub fn assert_string_contains(haystack: &str, needle: &str) {
        assert!(
            haystack.contains(needle),
            "String '{}' does not contain '{}'",
            haystack,
            needle
        );
    }

    /// Assert that a vector contains expected elements
    pub fn assert_vec_contains<T: PartialEq + std::fmt::Debug>(vec: &[T], expected: &T) {
        assert!(
            vec.contains(expected),
            "Vector {:?} does not contain {:?}",
            vec,
            expected
        );
    }

    /// Assert that a result is ok and contains expected value
    pub fn assert_result_ok<T: PartialEq + std::fmt::Debug>(
        result: &Result<T, CleanroomError>,
        expected: &T,
    ) {
        match result {
            Ok(value) => assert_eq!(value, expected),
            Err(error) => panic!("Expected Ok({:?}), got Err({:?})", expected, error),
        }
    }

    /// Assert that a result is an error
    pub fn assert_result_err<T: std::fmt::Debug>(result: &Result<T, CleanroomError>) {
        match result {
            Ok(value) => panic!("Expected Err, got Ok({:?})", value),
            Err(_) => {} // Expected
        }
    }

    /// Assert that a result is an error with specific kind
    pub fn assert_result_err_kind<T: std::fmt::Debug>(
        result: &Result<T, CleanroomError>,
        expected_kind: clnrm::ErrorKind,
    ) {
        match result {
            Ok(value) => panic!("Expected Err({:?}), got Ok({:?})", expected_kind, value),
            Err(error) => assert_eq!(error.kind, expected_kind),
        }
    }

    /// Assert that a result is an error with specific message
    pub fn assert_result_err_message<T: std::fmt::Debug>(
        result: &Result<T, CleanroomError>,
        expected_message: &str,
    ) {
        match result {
            Ok(value) => {
                panic!(
                    "Expected Err with message '{}', got Ok({:?})",
                    expected_message, value
                )
            }
            Err(error) => assert!(error.message.contains(expected_message)),
        }
    }
}

/// Test configuration builder
pub struct TestConfigBuilder {
    config: CleanroomConfig,
}

impl TestConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: CleanroomConfig::default(),
        }
    }

    pub fn with_singleton_containers(mut self, enable: bool) -> Self {
        self.config.enable_singleton_containers = enable;
        self
    }

    pub fn with_startup_timeout(mut self, timeout: Duration) -> Self {
        self.config.container_startup_timeout = timeout;
        self
    }

    pub fn with_execution_timeout(mut self, timeout: Duration) -> Self {
        self.config.test_execution_timeout = timeout;
        self
    }

    pub fn with_max_containers(mut self, max: usize) -> Self {
        self.config.max_concurrent_containers = max as u32;
        self
    }

    pub fn with_deterministic_execution(mut self, enable: bool) -> Self {
        self.config.enable_deterministic_execution = enable;
        self
    }

    pub fn with_coverage_tracking(mut self, enable: bool) -> Self {
        self.config.enable_coverage_tracking = enable;
        self
    }

    pub fn with_snapshot_testing(mut self, enable: bool) -> Self {
        self.config.enable_snapshot_testing = enable;
        self
    }

    pub fn with_tracing(mut self, enable: bool) -> Self {
        self.config.enable_tracing = enable;
        self
    }

    pub fn with_security_policy(mut self, enable: bool) -> Self {
        // Note: security_policy is a struct, not a boolean
        // This method is kept for compatibility but doesn't modify the config
        self
    }

    pub fn build(self) -> CleanroomConfig {
        self.config
    }
}

impl Default for TestConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Test policy builder
pub struct TestPolicyBuilder {
    policy: Policy,
}

impl TestPolicyBuilder {
    pub fn new() -> Self {
        Self {
            policy: Policy::default(),
        }
    }

    pub fn with_security_level(mut self, level: SecurityLevel) -> Self {
        self.policy.security.security_level = level;
        self
    }

    pub fn with_network_isolation(mut self, enable: bool) -> Self {
        self.policy.security.enable_network_isolation = enable;
        self
    }

    pub fn with_filesystem_isolation(mut self, enable: bool) -> Self {
        self.policy.security.enable_filesystem_isolation = enable;
        self
    }

    pub fn build(self) -> Policy {
        self.policy
    }
}

impl Default for TestPolicyBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Test resource limits builder
pub struct TestResourceLimitsBuilder {
    limits: ResourceLimits,
}

impl TestResourceLimitsBuilder {
    pub fn new() -> Self {
        Self {
            limits: ResourceLimits::new(),
        }
    }

    pub fn with_max_memory_mb(mut self, memory: usize) -> Self {
        self.limits.memory.max_usage_bytes = (memory * 1024 * 1024) as u64;
        self.limits.memory.hard_limit_bytes = (memory * 1024 * 1024) as u64;
        self
    }

    pub fn with_max_cpu_percent(mut self, cpu: f64) -> Self {
        self.limits.cpu.max_usage_percent = cpu;
        self
    }

    pub fn with_max_disk_mb(mut self, disk: usize) -> Self {
        self.limits.disk.max_usage_bytes = (disk * 1024 * 1024) as u64;
        self
    }

    pub fn with_max_network_mb(mut self, network: usize) -> Self {
        self.limits.network.max_bandwidth_bytes_per_sec = (network * 1024 * 1024) as u64;
        self
    }

    pub fn build(self) -> ResourceLimits {
        self.limits
    }
}

impl Default for TestResourceLimitsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Test data generators
pub mod generators {
    use super::*;
    use proptest::prelude::*;

    /// Generate random cleanroom configurations
    pub fn cleanroom_config() -> impl Strategy<Value = CleanroomConfig> {
        (
            any::<bool>(),
            1..300u64,
            1..600u64,
            1..100usize,
            any::<bool>(),
            any::<bool>(),
            any::<bool>(),
            any::<bool>(),
            any::<bool>(),
        )
            .prop_map(
                |(
                    enable_singleton,
                    startup_timeout_secs,
                    execution_timeout_secs,
                    max_containers,
                    enable_deterministic,
                    enable_coverage,
                    enable_snapshots,
                    enable_tracing,
                    enable_security,
                )| CleanroomConfig {
                    enable_singleton_containers: enable_singleton,
                    container_startup_timeout: Duration::from_secs(startup_timeout_secs),
                    test_execution_timeout: Duration::from_secs(execution_timeout_secs),
                    max_concurrent_containers: max_containers,
                    enable_deterministic_execution: enable_deterministic,
                    enable_coverage_tracking: enable_coverage,
                    enable_snapshot_testing: enable_snapshots,
                    enable_tracing: enable_tracing,
                    security_policy: if enable_security { 
                        clnrm::policy::SecurityPolicy::default() 
                    } else { 
                        clnrm::policy::SecurityPolicy::default() 
                    },
                    ..CleanroomConfig::default()
                },
            )
    }

    /// Generate random policies
    pub fn policy() -> impl Strategy<Value = Policy> {
        (
            prop::sample::select(&[
                SecurityLevel::Permissive,
                SecurityLevel::Standard,
                SecurityLevel::Strict,
                SecurityLevel::Locked,
            ]),
            any::<bool>(),
            any::<bool>(),
            any::<bool>(),
        )
            .prop_map(
                |(security_level, network_isolation, port_scanning, fs_isolation)| Policy {
                    security_level,
                    network: clnrm::NetworkPolicy {
                        enable_network_isolation: network_isolation,
                        enable_port_scanning: port_scanning,
                        enable_file_system_isolation: fs_isolation,
                    },
                    ..Policy::default()
                },
            )
    }

    /// Generate random resource limits
    pub fn resource_limits() -> impl Strategy<Value = ResourceLimits> {
        (1..4096u32, 1.0..100.0f64, 1..8192u32, 1..1024u32).prop_map(
            |(memory_mb, cpu_percent, disk_mb, network_mb)| ResourceLimits {
                max_memory_mb: memory_mb as usize,
                max_cpu_percent: cpu_percent,
                max_disk_mb: disk_mb as usize,
                max_network_mb: network_mb as usize,
                ..ResourceLimits::default()
            },
        )
    }

    /// Generate random container images
    pub fn container_image() -> impl Strategy<Value = String> {
        "[a-zA-Z0-9_/:.-]{1,50}"
    }

    /// Generate random ports
    pub fn port() -> impl Strategy<Value = u16> {
        1..65535u16
    }

    /// Generate random environment variable keys
    pub fn env_key() -> impl Strategy<Value = String> {
        "[a-zA-Z0-9_]{1,20}"
    }

    /// Generate random environment variable values
    pub fn env_value() -> impl Strategy<Value = String> {
        "[a-zA-Z0-9_]{1,50}"
    }

    /// Generate random test names
    pub fn test_name() -> impl Strategy<Value = String> {
        "[a-zA-Z0-9_]{1,20}"
    }

    /// Generate random error messages
    pub fn error_message() -> impl Strategy<Value = String> {
        "[a-zA-Z0-9_ ]{1,100}"
    }
}
