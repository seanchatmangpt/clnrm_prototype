//! # Lifecycle Management System
//!
//! Complete ggen-style lifecycle management for cleanroom environments.
//! Provides init, test, deploy, validate, and readiness tracking phases.
//!
//! ## Features
//!
//! - **Project Initialization**: Bootstrap project structure and dependencies
//! - **Test Execution**: Run tests in cleanroom environments
//! - **Deployment**: Deploy to dev/staging/production environments
//! - **Validation**: Validate environment configuration and requirements
//! - **Readiness Tracking**: Track production readiness with detailed scoring
//!
//! ## Usage Example
//!
//! ```no_run
//! use clnrm::lifecycle::{LifecycleManager, LifecycleConfig};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize lifecycle configuration
//!     let config = LifecycleConfig {
//!         project_name: "my-project".to_string(),
//!         phases: vec![
//!             PhaseConfig {
//!                 name: "init".to_string(),
//!                 command: "echo".to_string(),
//!                 args: vec!["initializing project".to_string()],
//!                 ..Default::default()
//!             },
//!             PhaseConfig {
//!                 name: "test".to_string(),
//!                 command: "cargo".to_string(),
//!                 args: vec!["test".to_string()],
//!                 ..Default::default()
//!             },
//!         ],
//!         ..Default::default()
//!     };
//!
//!     // Create lifecycle manager
//!     let manager = LifecycleManager::new(config, None)?;
//!
//!     // Execute lifecycle phases
//!     let results = manager.execute_all_phases().await?;
//!
//!     for result in results {
//!         println!("Phase {}: {}", result.phase_name, result.status);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Example
//!
//! ```no_run
//! use crate::cleanroom::lifecycle::{LifecycleManager, LifecycleConfig};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let config = LifecycleConfig::load("lifecycle.toml").await?;
//!     let manager = LifecycleManager::new(config, None)?;
//!
//!     // Initialize project
//!     manager.init().await?;
//!
//!     // Run tests
//!     let test_results = manager.test().await?;
//!     println!("Tests: {} passed, {} failed",
//!         test_results.passed, test_results.failed);
//!
//!     // Check readiness
//!     let readiness = manager.readiness().await?;
//!     println!("Production readiness: {}%", readiness.score);
//!
//!     // Deploy if ready
//!     if readiness.score >= 80 {
//!         manager.deploy("production").await?;
//!     }
//!
//!     Ok(())
//! }
//! ```

pub mod config;
pub mod phases;
pub mod readiness;
pub mod validator;

pub use config::{EnvironmentConfig, LifecycleConfig, Phase, Requirement, Status};
pub use phases::{DeploymentResult, InitResult, LifecycleManager, TestResults};
pub use readiness::{ReadinessScore, ReadinessTracker, RequirementStatus};
pub use validator::{DeploymentValidator, QualityReport, ValidationReport};

use crate::error::Result;

/// Initialize lifecycle system with default configuration
pub async fn init_lifecycle(project_name: &str) -> Result<LifecycleConfig> {
    let config = LifecycleConfig::default_with_name(project_name);
    config.save("lifecycle.toml").await?;
    Ok(config)
}

/// Load lifecycle configuration from file
pub async fn load_lifecycle(path: &str) -> Result<LifecycleConfig> {
    LifecycleConfig::load(path).await
}
