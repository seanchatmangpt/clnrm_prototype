//! Examples demonstrating different cleanroom environment configurations
//!
//! This example shows how to create different types of cleanroom environments
//! with various configurations for different use cases.

use clnrm::{CleanroomConfig, CleanroomEnvironment, Policy, SecurityLevel};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Cleanroom Configuration Examples");
    println!("=================================");

    // Example 1: Minimal configuration
    println!("\n1. Minimal Configuration");
    let config = CleanroomConfig::default();
    let env = CleanroomEnvironment::new(config).await?;
    println!(
        "✓ Built minimal environment with session ID: {}",
        env.session_id()
    );

    // Example 2: Secure environment
    println!("\n2. Secure Environment");
    let mut config = CleanroomConfig::default();
    config.policy = Policy::locked();
    let env = CleanroomEnvironment::new(config).await?;
    println!("✓ Built secure environment with locked security policy");

    // Example 3: Performance-optimized environment
    println!("\n3. Performance Environment");
    let mut config = CleanroomConfig::default();
    config.test_execution_timeout = Duration::from_secs(60);
    let env = CleanroomEnvironment::new(config).await?;
    println!("✓ Built performance environment with 60s timeout");

    // Example 4: Deterministic environment
    println!("\n4. Deterministic Environment");
    let mut config = CleanroomConfig::default();
    config.policy = Policy::with_security_level(SecurityLevel::High);
    let env = CleanroomEnvironment::new(config).await?;
    println!("✓ Built deterministic environment with high security");

    // Example 5: Development environment
    println!("\n5. Development Environment");
    let mut config = CleanroomConfig::default();
    config.policy = Policy::low_security();
    let env = CleanroomEnvironment::new(config).await?;
    println!("✓ Built development environment with relaxed policies");

    // Example 6: Custom configuration
    println!("\n6. Custom Configuration");
    let mut config = CleanroomConfig::default();
    config.test_execution_timeout = Duration::from_secs(120);
    config.policy = Policy::with_resource_limits(80.0, 1024 * 1024 * 1024, 10 * 1024 * 1024 * 1024);
    let env = CleanroomEnvironment::new(config).await?;
    println!("✓ Built custom environment with resource limits");

    // Example 7: Configuration inspection
    println!("\n7. Configuration Inspection");
    let config = CleanroomConfig::default();
    let env = CleanroomEnvironment::new(config).await?;

    println!(
        "Security level: {:?}",
        env.config().policy.security.security_level
    );
    println!(
        "Test timeout: {:?}",
        env.config().test_execution_timeout
    );
    println!(
        "Session ID: {}",
        env.session_id()
    );

    println!("\n=== All Examples Completed Successfully ===");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_minimal_builder() {
        let env = CleanroomBuilder::new()
            .build_minimal()
            .await
            .expect("Should build minimal environment");

        assert_eq!(
            env.config().test_execution_timeout,
            Duration::from_secs(300)
        );
    }

    #[tokio::test]
    async fn test_secure_builder() {
        let env = CleanroomBuilder::secure()
            .build()
            .await
            .expect("Should build secure environment");

        assert_eq!(
            env.config().security_policy.security_level,
            clnrm::policy::SecurityLevel::Locked
        );
    }

    #[tokio::test]
    async fn test_performance_builder() {
        let env = CleanroomBuilder::performance()
            .build()
            .await
            .expect("Should build performance environment");

        assert_eq!(env.config().test_execution_timeout, Duration::from_secs(60));
        assert!(env.config().enable_singleton_containers);
    }

    #[tokio::test]
    async fn test_deterministic_builder() {
        let seed = 42;
        let env = CleanroomBuilder::deterministic(seed)
            .build()
            .await
            .expect("Should build deterministic environment");

        assert!(env.config().enable_deterministic_execution);
        assert_eq!(env.config().deterministic_seed, Some(seed));
    }

    #[tokio::test]
    async fn test_development_builder() {
        let env = CleanroomBuilder::development()
            .build()
            .await
            .expect("Should build development environment");

        assert!(!env.config().enable_singleton_containers);
        assert!(env.config().enable_coverage_tracking);
        assert!(env.config().enable_tracing);
    }

    #[tokio::test]
    async fn test_custom_configuration() {
        let timeout = Duration::from_secs(90);
        let seed = 456;

        let env = CleanroomBuilder::new()
            .with_timeout(timeout)
            .with_deterministic_execution(Some(seed))
            .with_coverage_tracking(true)
            .build()
            .await
            .expect("Should build custom environment");

        assert_eq!(env.config().test_execution_timeout, timeout);
        assert_eq!(env.config().deterministic_seed, Some(seed));
        assert!(env.config().enable_coverage_tracking);
    }
}
