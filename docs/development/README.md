# Cleanroom Developer Guide

Essential guide for developers working on the Cleanroom Testing Framework - 80% of what you need to know in 20% of the documentation.

## Quick Start (80% of developers only need this)

### Prerequisites (5 minutes)
- **Rust 1.85+** ([rustup.rs](https://rustup.rs/))
- **Docker** ([docker.com](https://www.docker.com/))
- **Git** for version control

### Setup (2 minutes)
```bash
# Clone and build
git clone https://github.com/sac/ggen.git && cd ggen/cleanroom
cargo build

# Install dev tools
cargo install cargo-fmt cargo-clippy cargo-tarpaulin

# Run tests
cargo test
```

### Development Workflow (80% of time)
```bash
# Make changes and test
cargo test                    # Run tests
cargo clippy                  # Lint code
cargo fmt                     # Format code

# Create feature branch
git checkout -b feature/my-feature
git commit -m "Add my feature"
git push origin feature/my-feature
```

## Architecture Overview (20% effort for 80% understanding)

### Core Components (80% of usage)
1. **CleanroomEnvironment** - Main orchestrator for test execution
2. **Backend** - Abstraction layer (Docker, Podman, Kubernetes)
3. **Containers** - Lifecycle management for test containers
4. **Policy** - Security and resource policies
5. **Scenario** - Multi-step test workflows

### Key Design Principles
- **Deterministic**: Reproducible results with fixed seeds
- **Isolated**: Complete test environment isolation
- **Performant**: Optimized container reuse and pooling
- **Secure**: Built-in security boundaries and policies

## Essential APIs (80% of developers only need these)

### Basic Usage (80% of use cases)
```rust
use clnrm::{run, run_with_policy, Policy, SecurityLevel};

// Simple execution
let result = run(["echo", "hello"])?;
assert_eq!(result.exit_code, 0);

// Policy-based execution
let policy = Policy::with_security_level(SecurityLevel::Standard);
let result = run_with_policy(["python3", "--version"], &policy)?;
```

### Environment Usage (80% of advanced use cases)
```rust
use clnrm::{CleanroomEnvironment, CleanroomConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await?;

    let result = environment.execute_test("echo 'test'").await?;
    Ok(())
}
```

### Container Management (80% of container use cases)
```rust
use clnrm::{CleanroomEnvironment, PostgresContainer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await?;

    // Singleton container pattern
    let postgres = environment
        .get_or_create_container("postgres", || {
            PostgresContainer::new("testdb", "testuser", "testpass")
        })
        .await?;

    postgres.wait_for_ready().await?;
    Ok(())
}
```

## Configuration (80/20)

### Essential Configuration (80% of users only need this)
```rust
// Basic config - 80% of use cases
let config = CleanroomConfig {
    security_policy: SecurityPolicy {
        enable_network_isolation: true,
        enable_filesystem_isolation: true,
        security_level: SecurityLevel::Standard,
        ..Default::default()
    },
    resource_limits: ResourceLimits {
        max_container_count: 10,
        max_test_execution_time: Duration::from_secs(300),
        ..Default::default()
    },
    enable_deterministic_execution: true,
    ..Default::default()
};
```

### Policy Configuration (80% of security use cases)
```rust
// Security policy - 80% of users only need these settings
let policy = Policy::with_security_level(SecurityLevel::Standard)
    .with_network_isolation(true)
    .with_filesystem_isolation(true);

// Resource policy - 80% of users only need basic limits
let resource_policy = ResourcePolicy {
    max_memory_mb: 1024,
    max_cpu_percent: 80,
    max_disk_mb: 2048,
    ..Default::default()
};
```

## Error Handling (80/20)

### Essential Error Handling (80% of error cases)
```rust
// Handle the 80% of errors you actually encounter
match result {
    Ok(value) => println!("Success: {:?}", value),
    Err(CleanroomError::Container(msg)) => {
        eprintln!("Container failed: {}", msg);
        // Handle container issues
    },
    Err(CleanroomError::Policy(msg)) => {
        eprintln!("Policy violation: {}", msg);
        // Handle policy violations
    },
    Err(err) => {
        eprintln!("Other error: {}", err);
        // Handle unexpected errors
    },
}
```

## Best Practices (80% effort for 80% benefit)

### 1. Use Singleton Containers (Performance)
```rust
// Good: Reuse containers across tests
let postgres = environment.get_or_create_container("postgres", || {
    PostgresContainer::new("testdb", "testuser", "testpass")
}).await?;

// Avoid: Creating new containers for each test
```

### 2. Configure Resource Limits (Stability)
```rust
// Good: Set appropriate limits
config.resource_limits.max_container_count = 10;
config.resource_limits.max_test_execution_time = Duration::from_secs(300);

// Avoid: Using default limits without consideration
```

### 3. Enable Security Features (Security)
```rust
// Good: Enable essential security
config.security_policy.enable_network_isolation = true;
config.security_policy.enable_filesystem_isolation = true;
config.security_policy.security_level = SecurityLevel::Standard;

// Avoid: Disabling security without good reason
```

### 4. Handle Errors Properly (Reliability)
```rust
// Good: Handle specific error types
match result {
    Ok(_) => "Success",
    Err(CleanroomError::Container(_)) => "Retry with different container",
    Err(CleanroomError::Policy(_)) => "Check policy configuration",
    Err(_) => "Generic error handling",
}

// Avoid: Ignoring errors or using generic handlers
```

### 5. Use RAII for Cleanup (Resource Management)
```rust
// Good: Automatic cleanup
struct TestEnvironment {
    environment: CleanroomEnvironment,
}

impl Drop for TestEnvironment {
    fn drop(&mut self) {
        // Automatic cleanup
    }
}

// Avoid: Manual resource management
```

## Testing (80% of development time)

### Essential Testing Patterns (80% of test scenarios)
```rust
#[tokio::test]
async fn test_basic_execution() {
    let result = run(["echo", "test"])?;
    result.assert_success();
    assert_eq!(result.stdout.trim(), "test");
}

#[tokio::test]
async fn test_container_management() {
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await?;

    let container = environment.get_or_create_container("test", || {
        PostgresContainer::new("testdb", "testuser", "testpass")
    }).await?;

    container.wait_for_ready().await?;
    // Test container functionality
}
```

### Property Testing (Advanced but powerful)
```rust
proptest! {
    #[test]
    fn test_deterministic_execution(seed in 0u64..1000) {
        let config = CleanroomConfig {
            enable_deterministic_execution: true,
            deterministic_seed: Some(seed),
            ..Default::default()
        };

        // Test should be deterministic across runs
        prop_assert!(test_is_deterministic(&config));
    }
}
```

## Performance (80% of performance issues)

### Common Performance Issues (80% of cases)
1. **Container Creation**: Use singleton pattern
2. **Resource Limits**: Set appropriate limits
3. **Network Isolation**: Enable when needed
4. **Filesystem Access**: Minimize when possible

### Performance Monitoring (Essential)
```rust
// Monitor resource usage
let metrics = environment.get_metrics().await;
println!("CPU: {:.1}%, Memory: {}MB",
    metrics.resource_usage.cpu_usage_percent,
    metrics.resource_usage.memory_usage_bytes / 1024 / 1024
);

// Check resource limits
environment.check_resource_limits().await?;
```

## Security (80% of security concerns)

### Essential Security Settings (80% of security use cases)
```rust
// Network isolation - 80% of security use cases
config.security_policy.enable_network_isolation = true;

// Filesystem isolation - 80% of security use cases
config.security_policy.enable_filesystem_isolation = true;

// Data redaction - 80% of security use cases
config.security_policy.enable_data_redaction = true;
config.security_policy.redaction_patterns = vec![
    r"password\s*=\s*[^\s]+".to_string(),
    r"token\s*=\s*[^\s]+".to_string(),
];
```

## Troubleshooting (80% of issues)

### Common Issues (80% of problems)

#### Docker Issues (Most common)
```bash
# Check Docker status
docker --version && docker ps

# Restart Docker if needed
sudo systemctl restart docker

# Check container logs
docker logs <container_id>
```

#### Test Failures (Common)
```bash
# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Debug with backtrace
RUST_BACKTRACE=1 cargo test
```

#### Performance Issues (Less common but important)
```bash
# Profile performance
cargo install cargo-flamegraph
cargo flamegraph --bin cleanroom

# Monitor resource usage
htop  # Or your preferred system monitor
```

## Advanced Topics (20% effort for 20% benefit)

### Custom Backends
```rust
// Implement custom backend for specific needs
pub struct CustomBackend {
    // Custom backend implementation
}

impl Backend for CustomBackend {
    async fn execute(&self, command: &str) -> Result<RunResult> {
        // Custom execution logic
    }
}
```

### Custom Policies
```rust
// Implement custom security policies
pub struct CustomPolicy {
    // Custom policy logic
}

impl Policy for CustomPolicy {
    fn is_operation_allowed(&self, operation: &str, context: &ExecutionContext) -> bool {
        // Custom policy logic
    }
}
```

### Custom Containers
```rust
// Implement custom container types
pub struct CustomContainer {
    // Custom container logic
}

impl BaseContainer for CustomContainer {
    async fn wait_for_ready(&self) -> Result<()> {
        // Custom readiness logic
    }
}
```

## Contributing (80% of contributions)

### Contribution Types (80% of contributions are these)
1. **Bug Fixes** - Fix existing issues
2. **Documentation** - Improve docs and examples
3. **Tests** - Add or improve tests
4. **Performance** - Optimize performance

### Contribution Process (80% of contributions follow this)
1. **Fork** the repository
2. **Create** feature branch (`git checkout -b feature/my-feature`)
3. **Make** changes and test (`cargo test && cargo clippy`)
4. **Commit** changes (`git commit -m "Add my feature"`)
5. **Push** and create PR (`git push origin feature/my-feature`)

## Resources (80% of what you need)

### Essential Documentation (80% of reading)
- **API Reference**: [docs/api/README.md](api/README.md)
- **Best Practices**: [docs/guides/best-practices.md](../guides/best-practices.md)
- **Getting Started**: [docs/guides/getting-started-tutorial.md](../guides/getting-started-tutorial.md)

### Essential Tools (80% of development)
- **cargo test** - Run tests
- **cargo clippy** - Lint code
- **cargo fmt** - Format code
- **cargo doc** - Generate documentation

### Community (80% of help)
- **GitHub Issues** - Report bugs and request features
- **GitHub Discussions** - Ask questions and discuss ideas
- **Discord** - Real-time community chat

---

*This developer guide follows 80/20 principles: 80% of what developers need to know in 20% of the documentation complexity. For advanced topics, see the architecture documentation.*
