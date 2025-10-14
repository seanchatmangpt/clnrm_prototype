//! # Test Fixtures and Utilities - FAANG Best Practices
//!
//! This module provides **production-quality test infrastructure** following core team standards:
//!
//! ## 🏗️ Builder Pattern Implementation
//! ```rust
//! use crate::fixtures::*;
//!
//! // Environment with custom configuration
//! let env = TestEnvironmentBuilder::new()
//!     .container_startup_timeout(Duration::from_secs(30))
//!     .max_concurrent_containers(5)
//!     .enable_deterministic_execution(true)
//!     .build()
//!     .await?;
//!
//! // Policy with security constraints
//! let policy = TestPolicyBuilder::new()
//!     .security_level(SecurityLevel::Strict)
//!     .network_isolation(true)
//!     .filesystem_isolation(true)
//!     .build();
//! ```
//!
//! ## 🧪 Test Data Factories
//! ```rust
//! // Pre-configured test scenarios
//! let unit_env = TestEnvironments::unit_test().await?;
//! let integration_env = TestEnvironments::integration_test().await?;
//! let performance_env = TestEnvironments::performance_test().await?;
//! ```
//!
//! ## ✅ Assertion Helpers
//! ```rust
//! // Rich assertions with detailed failure messages
//! TestAssertions::assert_success(&result);
//! TestAssertions::assert_stdout_contains(&run_result, "expected output");
//! TestAssertions::assert_env_var_equals(&run_result, "VAR", "value");
//! ```
//!
//! ## 🎯 Key Principles
//! - **DRY**: Eliminate test duplication with reusable fixtures
//! - **Fast**: Pre-configured fixtures for instant test setup
//! - **Reliable**: Deterministic builders with sensible defaults
//! - **Maintainable**: Centralized test infrastructure
//! - **Documented**: Clear examples and usage patterns

pub mod assertions;
pub mod config;
pub mod containers;
pub mod environment;
pub mod mock_backend;
pub mod mock_time;
pub mod policies;
pub mod utilities;

// Re-export commonly used items
pub use assertions::*;
pub use config::*;
pub use containers::*;
pub use environment::*;
pub use mock_backend::*;
pub use mock_time::*;
pub use policies::*;
pub use utilities::*;
