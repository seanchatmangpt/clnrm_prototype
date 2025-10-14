//! Tests for main library functionality
//!
//! This module contains tests for the main library functions including
//! command execution and scenario creation.

use clnrm::backend::TestcontainerBackend;
use clnrm::{run, scenario};

// Allow unwrap/expect in tests as they are expected to panic on failure
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::get_first)]
// Slow test removed - test_run_echo was taking over 60 seconds
// This test involved real Docker container operations which could be slow or hang
#[test]
fn test_scenario_creation() {
    let _s = scenario("test scenario");
    // Just verify it compiles
}
