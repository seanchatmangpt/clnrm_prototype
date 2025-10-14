//! Tests for runtime functionality
//!
//! This module contains tests for runtime configuration, execution, and output handling.

use clnrm::runtime::{Config, RunOutput};
use std::time::Duration;

// Allow panics and unwrap in tests as they are expected to fail fast
#[allow(
    clippy::panic,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing
)]
#[test]
fn test_config_creation() {
    let config = Config::new(vec!["echo".to_string(), "hello".to_string()]);
    assert_eq!(config.args, vec!["echo", "hello"]);
    // Use reasonable timeout instead of 300 seconds
    assert_eq!(config.execution_timeout, Duration::from_secs(5));
}

#[test]
fn test_config_with_workdir() {
    let config = Config::new(vec!["echo".to_string()]).with_workdir(std::env::temp_dir());
    assert!(config.workdir.is_some());
}

#[test]
fn test_config_with_env() {
    let config = Config::new(vec!["echo".to_string()]).with_env("TEST_VAR", "test_value");
    assert_eq!(config.env.get("TEST_VAR"), Some(&"test_value".to_string()));
}

#[test]
fn test_config_with_timeout() {
    // Use reasonable timeout instead of 60 seconds
    let config = Config::new(vec!["echo".to_string()]).with_timeout(Duration::from_secs(5));
    assert_eq!(config.execution_timeout, Duration::from_secs(5));
}

#[test]
fn test_config_validation_empty_args() {
    let config = Config::new(vec![]);
    assert!(config.validate().is_err());
}

#[test]
fn test_config_validation_zero_timeout() {
    let config = Config::new(vec!["echo".to_string()]).with_timeout(Duration::from_secs(0));
    assert!(config.validate().is_err());
}

#[test]
fn test_run_output_creation() {
    let output = RunOutput::new(0, "hello".to_string(), "".to_string(), 100);
    assert!(output.success());
    assert!(!output.failed());
    assert_eq!(output.combined_output(), "hello\n");
    assert_eq!(output.output_size(), 5);
}

#[test]
fn test_run_output_failure() {
    let output = RunOutput::new(1, "".to_string(), "error".to_string(), 50);
    assert!(!output.success());
    assert!(output.failed());
    assert_eq!(output.combined_output(), "\nerror");
    assert_eq!(output.output_size(), 5);
}
