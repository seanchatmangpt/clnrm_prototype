//! Comprehensive tests to improve code coverage to 80%
//!
//! This module contains tests for modules that likely have low test coverage
//! to help achieve the 80% coverage target.

use clnrm::{
    artifacts::{ArtifactCollector, ForensicsBundle, BundleMetadata, LogEntry, LogLevel, BinaryArtifact, ConfigArtifact},
    attest::{AttestationGenerator, Attestation, PolicyAttestation, EnvironmentAttestation, CoverageAttestation, RunInfo},
    assertions::Assert,
    backend::RunResult as BackendRunResult,
    builder::CleanroomBuilder,
    determinism::DeterministicManager,
    executor::TimeoutExecutor,
    guards::{CleanroomGuard, ContainerGuard},
    limits::ResourceLimits,
    metrics_builder::ContainerMetricsBuilder,
    orchestrator::ConcurrencyOrchestrator,
    redaction::DataRedactor,
    runtime::{RuntimeManager, TaskContext},
    scenario::{Scenario, Step, RunResult as ScenarioRunResult},
    serializable_instant::SerializableInstant,
    skip::SkipManager,
    streaming::{StreamingCollector, StreamingBuffer},
    test_utils::{TestEnvironmentBuilder, TestContainerHelper},
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// Test ArtifactCollector functionality
#[cfg(test)]
mod artifact_tests {
    use super::*;

    #[test]
    fn test_artifact_collector_creation() {
        let collector = ArtifactCollector::new().unwrap();
        assert!(collector.work_dir.exists());
        assert!(collector.redact_env);
    }

    #[test]
    fn test_artifact_collector_without_redaction() {
        let collector = ArtifactCollector::new().unwrap().with_redaction(false);
        assert!(!collector.redact_env);
    }

    #[test]
    fn test_sensitive_key_detection() {
        let collector = ArtifactCollector::new().unwrap();
        assert!(collector.is_sensitive_key("API_KEY"));
        assert!(collector.is_sensitive_key("GITHUB_TOKEN"));
        assert!(collector.is_sensitive_key("SECRET_PASSWORD"));
        assert!(!collector.is_sensitive_key("NORMAL_VAR"));
    }

    #[test]
    fn test_hash_calculation() {
        let collector = ArtifactCollector::new().unwrap();
        let data = b"test data";
        let hash = collector.calculate_hash(data);
        assert!(!hash.is_empty());
    }

    #[test]
    fn test_bundle_metadata_creation() {
        let metadata = BundleMetadata {
            version: "1.0".to_string(),
            created_at: 1234567890,
            scenario_name: "test".to_string(),
            bundle_id: "bundle_123".to_string(),
            description: Some("Test bundle".to_string()),
        };
        assert_eq!(metadata.version, "1.0");
        assert_eq!(metadata.scenario_name, "test");
    }

    #[test]
    fn test_log_entry_creation() {
        let log_entry = LogEntry {
            timestamp: 1234567890,
            level: LogLevel::Info,
            component: "test".to_string(),
            message: "Test message".to_string(),
            context: HashMap::new(),
        };
        assert_eq!(log_entry.level, LogLevel::Info);
        assert_eq!(log_entry.message, "Test message");
    }

    #[test]
    fn test_binary_artifact_creation() {
        let artifact = BinaryArtifact {
            name: "test_bin".to_string(),
            path: "/tmp/test_bin".to_string(),
            hash: "abc123".to_string(),
            data: "base64data".to_string(),
        };
        assert_eq!(artifact.name, "test_bin");
        assert_eq!(artifact.hash, "abc123");
    }

    #[test]
    fn test_config_artifact_creation() {
        let artifact = ConfigArtifact {
            name: "config.toml".to_string(),
            path: "/tmp/config.toml".to_string(),
            data: "key = value".to_string(),
        };
        assert_eq!(artifact.name, "config.toml");
        assert_eq!(artifact.data, "key = value");
    }
}

/// Test AttestationGenerator functionality
#[cfg(test)]
mod attestation_tests {
    use super::*;

    #[test]
    fn test_attestation_generator_creation() {
        let generator = AttestationGenerator::new();
        assert!(!generator.enable_signing);
        assert!(generator.signing_key.is_none());
    }

    #[test]
    fn test_attestation_generator_with_signing() {
        let generator = AttestationGenerator::new().with_signing(Some("test_key".to_string()));
        assert!(generator.enable_signing);
        assert_eq!(generator.signing_key, Some("test_key".to_string()));
    }

    #[test]
    fn test_coverage_attestation_generation() {
        let generator = AttestationGenerator::new();
        let coverage = generator.generate_coverage_attestation(80, 100);
        assert_eq!(coverage.lines_covered, 80);
        assert_eq!(coverage.lines_total, 100);
        assert_eq!(coverage.percentage, 80.0);
    }

    #[test]
    fn test_coverage_attestation_zero_total() {
        let generator = AttestationGenerator::new();
        let coverage = generator.generate_coverage_attestation(0, 0);
        assert_eq!(coverage.percentage, 0.0);
    }

    #[test]
    fn test_sensitive_key_detection() {
        let generator = AttestationGenerator::new();
        assert!(generator.is_sensitive_key("API_KEY"));
        assert!(generator.is_sensitive_key("GITHUB_TOKEN"));
        assert!(generator.is_sensitive_key("SECRET_PASSWORD"));
        assert!(!generator.is_sensitive_key("NORMAL_VAR"));
    }

    #[test]
    fn test_attestation_verification() {
        let generator = AttestationGenerator::new();
        let mut attestation = Attestation {
            timestamp: 1234567890,
            image_digests: HashMap::new(),
            policy: PolicyAttestation {
                net_profile: "offline".to_string(),
                fs_profile: "readonly".to_string(),
                proc_profile: "isolated".to_string(),
                resource_limits: HashMap::new(),
            },
            environment: EnvironmentAttestation {
                host_os: "linux".to_string(),
                engine_version: None,
                backend: "docker".to_string(),
                env_vars: HashMap::new(),
            },
            coverage: None,
            signature: None,
        };

        // Test without signature
        assert!(generator.verify(&attestation).unwrap());

        // Test with signature
        attestation.signature = Some("test_signature".to_string());
        assert!(generator.verify(&attestation).unwrap());
    }
}

/// Test Assert trait functionality
#[cfg(test)]
mod assertion_tests {
    use super::*;

    #[test]
    fn test_assert_success() {
        let result = BackendRunResult {
            exit_code: 0,
            stdout: "hello".to_string(),
            stderr: String::new(),
            duration_ms: 100,
            steps: Vec::new(),
            redacted_env: Vec::new(),
            backend: "test".to_string(),
            concurrent: false,
            step_order: Vec::new(),
        };

        result.assert_success();
    }

    #[test]
    #[should_panic]
    fn test_assert_success_failure() {
        let result = BackendRunResult {
            exit_code: 1,
            stdout: "hello".to_string(),
            stderr: String::new(),
            duration_ms: 100,
            steps: Vec::new(),
            redacted_env: Vec::new(),
            backend: "test".to_string(),
            concurrent: false,
            step_order: Vec::new(),
        };

        result.assert_success();
    }

    #[test]
    fn test_assert_failure() {
        let result = BackendRunResult {
            exit_code: 1,
            stdout: "hello".to_string(),
            stderr: String::new(),
            duration_ms: 100,
            steps: Vec::new(),
            redacted_env: Vec::new(),
            backend: "test".to_string(),
            concurrent: false,
            step_order: Vec::new(),
        };

        result.assert_failure();
    }

    #[test]
    #[should_panic]
    fn test_assert_failure_success() {
        let result = BackendRunResult {
            exit_code: 0,
            stdout: "hello".to_string(),
            stderr: String::new(),
            duration_ms: 100,
            steps: Vec::new(),
            redacted_env: Vec::new(),
            backend: "test".to_string(),
            concurrent: false,
            step_order: Vec::new(),
        };

        result.assert_failure();
    }

    #[test]
    fn test_assert_stdout_contains() {
        let result = BackendRunResult {
            exit_code: 0,
            stdout: "hello world".to_string(),
            stderr: String::new(),
            duration_ms: 100,
            steps: Vec::new(),
            redacted_env: Vec::new(),
            backend: "test".to_string(),
            concurrent: false,
            step_order: Vec::new(),
        };

        result.assert_stdout_contains("hello");
    }

    #[test]
    #[should_panic]
    fn test_assert_stdout_contains_failure() {
        let result = BackendRunResult {
            exit_code: 0,
            stdout: "hello world".to_string(),
            stderr: String::new(),
            duration_ms: 100,
            steps: Vec::new(),
            redacted_env: Vec::new(),
            backend: "test".to_string(),
            concurrent: false,
            step_order: Vec::new(),
        };

        result.assert_stdout_contains("goodbye");
    }

    #[test]
    fn test_assert_stderr_contains() {
        let result = BackendRunResult {
            exit_code: 1,
            stdout: String::new(),
            stderr: "error message".to_string(),
            duration_ms: 100,
            steps: Vec::new(),
            redacted_env: Vec::new(),
            backend: "test".to_string(),
            concurrent: false,
            step_order: Vec::new(),
        };

        result.assert_stderr_contains("error");
    }

    #[test]
    fn test_assert_hermetic() {
        let result = BackendRunResult {
            exit_code: 0,
            stdout: "hello".to_string(),
            stderr: String::new(),
            duration_ms: 100,
            steps: Vec::new(),
            redacted_env: Vec::new(),
            backend: "test".to_string(),
            concurrent: false,
            step_order: Vec::new(),
        };

        result.assert_hermetic();
    }

    #[test]
    fn test_assert_deterministic_mounts() {
        let result = BackendRunResult {
            exit_code: 0,
            stdout: "hello".to_string(),
            stderr: String::new(),
            duration_ms: 100,
            steps: Vec::new(),
            redacted_env: Vec::new(),
            backend: "test".to_string(),
            concurrent: false,
            step_order: Vec::new(),
        };

        result.assert_deterministic_mounts();
    }

    #[test]
    fn test_assert_normalized_clock() {
        let result = BackendRunResult {
            exit_code: 0,
            stdout: "hello".to_string(),
            stderr: String::new(),
            duration_ms: 100,
            steps: Vec::new(),
            redacted_env: Vec::new(),
            backend: "test".to_string(),
            concurrent: false,
            step_order: Vec::new(),
        };

        result.assert_normalized_clock();
    }
}

/// Test CleanroomBuilder functionality
#[cfg(test)]
mod builder_tests {
    use super::*;

    #[test]
    fn test_cleanroom_builder_creation() {
        let builder = CleanroomBuilder::new();
        assert!(builder.is_ok());
    }

    #[test]
    fn test_cleanroom_builder_with_timeout() {
        let builder = CleanroomBuilder::new()
            .unwrap()
            .with_timeout(Duration::from_secs(60));
        assert!(builder.is_ok());
    }

    #[test]
    fn test_cleanroom_builder_with_policy() {
        let policy = clnrm::Policy::default();
        let builder = CleanroomBuilder::new()
            .unwrap()
            .with_policy(policy);
        assert!(builder.is_ok());
    }
}

/// Test DeterministicManager functionality
#[cfg(test)]
mod determinism_tests {
    use super::*;

    #[test]
    fn test_deterministic_manager_creation() {
        let manager = DeterministicManager::new();
        assert!(manager.is_ok());
    }

    #[test]
    fn test_deterministic_manager_with_seed() {
        let manager = DeterministicManager::new()
            .unwrap()
            .with_seed(12345);
        assert!(manager.is_ok());
    }

    #[test]
    fn test_deterministic_manager_enable() {
        let manager = DeterministicManager::new().unwrap();
        manager.enable();
        assert!(manager.is_enabled());
    }

    #[test]
    fn test_deterministic_manager_disable() {
        let manager = DeterministicManager::new().unwrap();
        manager.disable();
        assert!(!manager.is_enabled());
    }
}

/// Test TimeoutExecutor functionality
#[cfg(test)]
mod executor_tests {
    use super::*;

    #[test]
    fn test_timeout_executor_creation() {
        let executor = TimeoutExecutor::new(Duration::from_secs(30));
        assert!(executor.is_ok());
    }

    #[test]
    fn test_timeout_executor_with_retry() {
        let executor = TimeoutExecutor::new(Duration::from_secs(30))
            .unwrap()
            .with_retry(3);
        assert!(executor.is_ok());
    }
}

/// Test CleanroomGuard functionality
#[cfg(test)]
mod guard_tests {
    use super::*;

    #[test]
    fn test_cleanroom_guard_creation() {
        let config = clnrm::CleanroomConfig::default();
        let environment = Arc::new(clnrm::CleanroomEnvironment::new(config).unwrap());
        let guard = CleanroomGuard::new(environment);
        assert!(guard.is_ok());
    }

    #[test]
    fn test_container_guard_creation() {
        let guard = ContainerGuard::new("test_container".to_string());
        assert!(guard.is_ok());
    }
}

/// Test ResourceLimits functionality
#[cfg(test)]
mod limits_tests {
    use super::*;

    #[test]
    fn test_resource_limits_creation() {
        let limits = ResourceLimits::new();
        assert!(limits.is_ok());
    }

    #[test]
    fn test_resource_limits_with_cpu() {
        let limits = ResourceLimits::new()
            .unwrap()
            .with_cpu_limit(0.5);
        assert!(limits.is_ok());
    }

    #[test]
    fn test_resource_limits_with_memory() {
        let limits = ResourceLimits::new()
            .unwrap()
            .with_memory_limit(1024 * 1024 * 1024); // 1GB
        assert!(limits.is_ok());
    }

    #[test]
    fn test_resource_limits_with_disk() {
        let limits = ResourceLimits::new()
            .unwrap()
            .with_disk_limit(10 * 1024 * 1024 * 1024); // 10GB
        assert!(limits.is_ok());
    }
}

/// Test ContainerMetricsBuilder functionality
#[cfg(test)]
mod metrics_builder_tests {
    use super::*;

    #[test]
    fn test_metrics_builder_creation() {
        let builder = ContainerMetricsBuilder::new();
        assert!(builder.is_ok());
    }

    #[test]
    fn test_metrics_builder_with_cpu() {
        let builder = ContainerMetricsBuilder::new()
            .unwrap()
            .with_cpu_usage(0.5);
        assert!(builder.is_ok());
    }

    #[test]
    fn test_metrics_builder_with_memory() {
        let builder = ContainerMetricsBuilder::new()
            .unwrap()
            .with_memory_usage(1024 * 1024 * 1024); // 1GB
        assert!(builder.is_ok());
    }

    #[test]
    fn test_metrics_builder_build() {
        let metrics = ContainerMetricsBuilder::new()
            .unwrap()
            .with_cpu_usage(0.5)
            .with_memory_usage(1024 * 1024 * 1024)
            .build();
        assert!(metrics.is_ok());
    }
}

/// Test ConcurrencyOrchestrator functionality
#[cfg(test)]
mod orchestrator_tests {
    use super::*;

    #[test]
    fn test_orchestrator_creation() {
        let orchestrator = ConcurrencyOrchestrator::new();
        assert!(orchestrator.is_ok());
    }

    #[test]
    fn test_orchestrator_with_max_tasks() {
        let orchestrator = ConcurrencyOrchestrator::new()
            .unwrap()
            .with_max_concurrent_tasks(10);
        assert!(orchestrator.is_ok());
    }

    #[test]
    fn test_orchestrator_with_timeout() {
        let orchestrator = ConcurrencyOrchestrator::new()
            .unwrap()
            .with_global_timeout(Duration::from_secs(60));
        assert!(orchestrator.is_ok());
    }
}

/// Test DataRedactor functionality
#[cfg(test)]
mod redaction_tests {
    use super::*;

    #[test]
    fn test_data_redactor_creation() {
        let redactor = DataRedactor::new();
        assert!(redactor.is_ok());
    }

    #[test]
    fn test_data_redactor_with_patterns() {
        let patterns = vec![r"password\s*=\s*[^\s]+".to_string()];
        let redactor = DataRedactor::new()
            .unwrap()
            .with_patterns(patterns);
        assert!(redactor.is_ok());
    }

    #[test]
    fn test_data_redactor_redact() {
        let redactor = DataRedactor::new().unwrap();
        let data = "password=secret123";
        let redacted = redactor.redact(data);
        assert!(redacted.contains("[REDACTED]"));
    }
}

/// Test RuntimeManager functionality
#[cfg(test)]
mod runtime_tests {
    use super::*;

    #[test]
    fn test_runtime_manager_creation() {
        let manager = RuntimeManager::new();
        assert!(manager.is_ok());
    }

    #[test]
    fn test_runtime_manager_with_config() {
        let config = clnrm::CleanroomConfig::default();
        let manager = RuntimeManager::new()
            .unwrap()
            .with_config(config);
        assert!(manager.is_ok());
    }

    #[test]
    fn test_task_context_creation() {
        let context = TaskContext::new("test_task".to_string());
        assert!(context.is_ok());
    }

    #[test]
    fn test_task_context_is_cancelled() {
        let context = TaskContext::new("test_task".to_string()).unwrap();
        assert!(!context.is_cancelled());
    }
}

/// Test Scenario functionality
#[cfg(test)]
mod scenario_tests {
    use super::*;

    #[test]
    fn test_scenario_creation() {
        let scenario = Scenario::new("test_scenario".to_string());
        assert_eq!(scenario.name, "test_scenario");
        assert!(scenario.steps.is_empty());
    }

    #[test]
    fn test_scenario_with_step() {
        let scenario = Scenario::new("test_scenario".to_string())
            .with_step(Step::new("test_step".to_string(), vec!["echo".to_string(), "hello".to_string()]));
        assert_eq!(scenario.steps.len(), 1);
        assert_eq!(scenario.steps[0].name, "test_step");
    }

    #[test]
    fn test_step_creation() {
        let step = Step::new("test_step".to_string(), vec!["echo".to_string(), "hello".to_string()]);
        assert_eq!(step.name, "test_step");
        assert_eq!(step.command, vec!["echo", "hello"]);
    }

    #[test]
    fn test_scenario_run_result_creation() {
        let result = ScenarioRunResult {
            exit_code: 0,
            stdout: "hello".to_string(),
            stderr: String::new(),
            duration_ms: 100,
            steps: Vec::new(),
            redacted_env: Vec::new(),
            backend: "test".to_string(),
            concurrent: false,
            step_order: Vec::new(),
        };
        assert_eq!(result.exit_code, 0);
        assert_eq!(result.stdout, "hello");
    }
}

/// Test SerializableInstant functionality
#[cfg(test)]
mod serializable_instant_tests {
    use super::*;

    #[test]
    fn test_serializable_instant_creation() {
        let instant = SerializableInstant::now();
        assert!(instant.is_ok());
    }

    #[test]
    fn test_serializable_instant_from_system_time() {
        let system_time = SystemTime::now();
        let instant = SerializableInstant::from(system_time);
        assert!(instant.is_ok());
    }

    #[test]
    fn test_serializable_instant_to_system_time() {
        let instant = SerializableInstant::now().unwrap();
        let system_time = instant.to_system_time();
        assert!(system_time.is_ok());
    }
}

/// Test SkipManager functionality
#[cfg(test)]
mod skip_tests {
    use super::*;

    #[test]
    fn test_skip_manager_creation() {
        let manager = SkipManager::new();
        assert!(manager.is_ok());
    }

    #[test]
    fn test_skip_manager_should_skip() {
        let manager = SkipManager::new().unwrap();
        assert!(!manager.should_skip("test_case"));
    }

    #[test]
    fn test_skip_manager_add_skip() {
        let manager = SkipManager::new()
            .unwrap()
            .add_skip("test_case".to_string());
        assert!(manager.should_skip("test_case"));
    }
}

/// Test StreamingCollector functionality
#[cfg(test)]
mod streaming_tests {
    use super::*;

    #[test]
    fn test_streaming_collector_creation() {
        let collector = StreamingCollector::new();
        assert!(collector.is_ok());
    }

    #[test]
    fn test_streaming_collector_with_buffer_size() {
        let collector = StreamingCollector::new()
            .unwrap()
            .with_buffer_size(1024);
        assert!(collector.is_ok());
    }

    #[test]
    fn test_streaming_buffer_creation() {
        let buffer = StreamingBuffer::new(1024);
        assert!(buffer.is_ok());
    }

    #[test]
    fn test_streaming_buffer_write() {
        let mut buffer = StreamingBuffer::new(1024).unwrap();
        let data = b"test data";
        let result = buffer.write(data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_streaming_buffer_read() {
        let mut buffer = StreamingBuffer::new(1024).unwrap();
        let data = b"test data";
        buffer.write(data).unwrap();
        let result = buffer.read();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), data);
    }
}

/// Test TestEnvironmentBuilder functionality
#[cfg(test)]
mod test_utils_tests {
    use super::*;

    #[test]
    fn test_environment_builder_creation() {
        let builder = TestEnvironmentBuilder::new();
        assert!(builder.is_ok());
    }

    #[test]
    fn test_environment_builder_with_singleton() {
        let builder = TestEnvironmentBuilder::new()
            .unwrap()
            .with_singleton_containers(true);
        assert!(builder.is_ok());
    }

    #[test]
    fn test_environment_builder_build() {
        let environment = TestEnvironmentBuilder::new()
            .unwrap()
            .with_singleton_containers(true)
            .build();
        assert!(environment.is_ok());
    }

    #[test]
    fn test_container_helper_creation() {
        let helper = TestContainerHelper::new();
        assert!(helper.is_ok());
    }

    #[test]
    fn test_container_helper_with_image() {
        let helper = TestContainerHelper::new()
            .unwrap()
            .with_image("alpine:latest".to_string());
        assert!(helper.is_ok());
    }
}

