//! Core functionality tests
//!
//! Tests for core cleanroom functionality including environment management,
//! configuration, basic operations, and property-based testing.

pub mod config;
pub mod containers;
pub mod environment;
pub mod limits;
pub mod policies;
pub mod property_tests;
pub mod security;
pub mod serialization;
pub mod validation;

// Consolidated tests from source files
pub mod cleanroom_tests;
pub mod lib_tests;
pub mod runtime_tests;
pub mod scenario_tests;
pub mod test_utils_tests;
