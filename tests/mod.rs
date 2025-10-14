//! Test modules for cleanroom testing framework
//!
//! This module provides a well-organized test structure with:
//! - Core functionality tests
//! - Integration tests
//! - Performance tests
//! - Test fixtures and utilities

// Test fixtures and utilities
pub mod fixtures;

// Core functionality tests
pub mod core;

// Integration tests
pub mod integration;

// Performance tests
pub mod performance;

// Legacy tests (deprecated - use core, integration, or performance modules instead)
pub mod legacy {
    pub mod bdd_tests;
}

// Re-export commonly used fixtures and modules
pub use fixtures::*;

// Re-export main test modules for easier access
pub use core::*;
pub use integration::*;
pub use performance::*;