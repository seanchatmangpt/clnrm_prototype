//! Test templates and examples for the Cleanroom Testing Framework
//!
//! This module provides templates and examples for different types of tests
//! to help developers write consistent, high-quality tests.

pub mod unit_test_template;
pub mod integration_test_template;
pub mod performance_test_template;
pub mod security_test_template;
pub mod error_handling_template;

// Re-export templates for easy access
pub use unit_test_template::*;
pub use integration_test_template::*;
pub use performance_test_template::*;
pub use security_test_template::*;
pub use error_handling_template::*;
