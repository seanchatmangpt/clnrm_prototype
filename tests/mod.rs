//! # Cleanroom Testing Framework - FAANG Core Team Best Practices
//!
//! This module implements world-class testing practices designed for high-reliability systems.
//!
//! ## 🏆 80/20 Rule - Quality Over Quantity
//! Focus on the 20% of tests that provide 80% of confidence in system correctness.
//! The [`eighty_twenty_test_suite`] module contains critical path tests that should pass first.
//!
//! ## 🧪 Test Organization Strategy
//! ```text
//! tests/
//! ├── fixtures/          # Test utilities, builders, and shared setup
//! ├── core/              # Unit tests for core functionality
//! │   ├── config.rs      # Configuration validation and behavior
//! │   ├── containers.rs  # Container lifecycle and operations
//! │   ├── environment.rs # Environment management
//! │   ├── lib_tests.rs   # Library function testing
//! │   ├── limits.rs      # Resource limits enforcement
//! │   ├── policies.rs    # Policy creation and validation
//! │   ├── property_tests.rs # Property-based testing with proptest
//! │   ├── runtime_tests.rs  # Runtime behavior and execution
//! │   ├── scenario_tests.rs # Scenario creation and execution
//! │   └── validation.rs  # Input validation and error handling
//! ├── integration/       # Integration tests across components
//! ├── performance/       # Performance and load testing
//! ├── eighty_twenty_test_suite.rs # Critical path testing (run first!)
//! └── legacy/            # Deprecated tests (migrate to modular structure)
//! ```
//!
//! ## 🎯 Core Testing Principles
//!
//! ### 1. **Deterministic & Fast**
//! - Tests must be deterministic (same result every time)
//! - Fast execution: unit tests < 100ms, integration tests < 5s
//! - No external dependencies or network calls in unit tests
//!
//! ### 2. **Clear Intent**
//! - Test names should explain the behavior being tested
//! - Use descriptive assertions with clear failure messages
//! - Document edge cases and boundary conditions
//!
//! ### 3. **Property-Based Testing**
//! - Use `proptest` for comprehensive input validation
//! - Test invariants and behavioral properties
//! - Verify serialization round-trips and error conditions
//!
//! ### 4. **Test Execution Strategy (CI/CD Pipeline)**
//! ```bash
//! # Phase 1: Critical Path (Run first, fail fast)
//! cargo test eighty_twenty_test_suite --release
//!
//! # Phase 2: Core Functionality (Unit tests)
//! cargo test core --lib
//!
//! # Phase 3: Integration (Component interaction)
//! cargo test integration
//!
//! # Phase 4: Performance (Resource validation)
//! cargo test performance --release
//!
//! # Phase 5: Full Test Suite (Complete validation)
//! cargo test --workspace
//! ```
//!
//! ### 5. **Test Parallelization Strategy**
//! ```bash
//! # Run tests in parallel for faster feedback
//! cargo test -- --test-threads=4
//!
//! # Run specific test patterns
//! cargo test config        # Configuration tests only
//! cargo test security      # Security tests only
//! cargo test performance   # Performance tests only
//! ```
//!
//! ## 🚫 Anti-Patterns to Avoid
//! - ❌ Flaky tests that sometimes pass/fail
//! - ❌ Tests with external dependencies in unit tests
//! - ❌ Tests that modify global state
//! - ❌ Tests without clear assertions
//! - ❌ Tests that don't clean up resources
//! - ❌ Tests with magic numbers or unclear setup

// Test fixtures and utilities
pub mod fixtures;

// Test helpers following core team best practices
pub mod helpers;

// Test examples demonstrating best practices
pub mod examples;

// Core functionality tests
pub mod core;

// Integration tests
pub mod integration;

// Performance tests
pub mod performance;

// 80/20 Test Suite - Critical tests that provide 80% confidence
pub mod eighty_twenty_test_suite;

// Legacy tests (deprecated - use core, integration, or performance modules instead)
pub mod legacy {
    pub mod bdd_tests;
}

// Re-export commonly used fixtures and modules
pub use fixtures::*;
pub use helpers::*;

// Re-export main test modules for easier access
pub use core::*;
pub use eighty_twenty_test_suite::*;
pub use integration::*;
pub use performance::*;
