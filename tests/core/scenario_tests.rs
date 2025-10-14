//! Tests for scenario functionality
//!
//! This module contains tests for scenario creation, step addition, policy handling,
//! and deterministic behavior.

use clnrm::prelude::*;
use clnrm::scenario::{scenario, Policy};

// Allow unwrap/expect in tests as they are expected to panic on failure
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]
#[test]
fn test_scenario_creation() {
    let scenario = scenario("test");
    assert_eq!(scenario.name(), "test");
    assert!(scenario.steps().is_empty());
}

#[test]
fn test_scenario_step_addition() {
    let scenario = scenario("test").step("echo".to_string(), ["echo", "hello"]);
    assert_eq!(scenario.steps().len(), 1);
    assert_eq!(scenario.steps().first().unwrap().name, "echo");
}

#[test]
fn test_scenario_policy() {
    let policy = Policy::locked();
    let scenario = scenario("test").with_policy(policy);
    assert!(!scenario.policy().allows_network());
}

#[test]
fn test_scenario_deterministic() {
    let scenario = scenario("test").deterministic(Some(42));
    assert!(scenario.is_deterministic());
    assert_eq!(scenario.seed(), Some(42));
}
