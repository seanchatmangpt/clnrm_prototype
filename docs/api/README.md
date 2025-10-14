# Cleanroom API Reference

Comprehensive API reference for the Cleanroom Testing Framework following 80/20 principles - 80% of the value with 20% of the documentation complexity.

## Table of Contents

1. [Core Types](#core-types)
2. [Core Functions](#core-functions)
3. [Configuration](#configuration)
4. [Error Handling](#error-handling)
5. [Container Management](#container-management)
6. [Security & Policies](#security--policies)
7. [Performance & Monitoring](#performance--monitoring)
8. [Usage Examples](#usage-examples)
9. [Best Practices](#best-practices)

## Core Types

### Main Types (80% of users only need these)

```rust
// Main environment type - essential for all users
pub struct CleanroomEnvironment {
    pub session_id: Uuid,
    // Core functionality - container management, test execution
}

// Configuration - 80% of users only need basic config
pub struct CleanroomConfig {
    pub security_policy: SecurityPolicy,
    pub resource_limits: ResourceLimits,
    pub performance_monitoring: PerformanceMonitoringConfig,
    pub enable_deterministic_execution: bool,
    pub deterministic_seed: Option<u64>,
}

// Policy - 80% of users only need security and resource policies
pub struct Policy {
    pub security: SecurityPolicy,
    pub resources: ResourcePolicy,
    pub execution: ExecutionPolicy,
}

// Result types - essential for all users
pub struct RunResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u64,
}

// Error type - essential for all users
pub enum CleanroomError {
    Container(String),
    Backend(String),
    Policy(String),
    Configuration(String),
    Execution(String),
}
```

### Container Types (80% of users need these)

```rust
// Core container types - essential for container testing
pub struct PostgresContainer {
    pub database: String,
    pub username: String,
    pub password: String,
}

pub struct RedisContainer {
    pub password: Option<String>,
}

pub struct GenericContainer {
    pub image: String,
    pub tag: String,
}
```

### ID Types (Type Safety)

```rust
// Type-safe ID system - prevents mixing different ID types
pub struct ContainerId(Uuid);
pub struct SessionId(Uuid);
pub struct TaskId(Uuid);
pub struct TestId(Uuid);
pub struct ScenarioId(Uuid);
```

## Core Functions

### Essential Functions (80% of usage)

```rust
// Main execution functions - 80% of users only need these
pub fn run<I, S>(args: I) -> Result<RunResult>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>;

pub fn run_with_policy<I, S>(args: I, policy: &Policy) -> Result<RunResult>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>;

// Environment creation - 80% of users only need this
impl CleanroomEnvironment {
    pub async fn new(config: CleanroomConfig) -> Result<Self>;
    pub async fn execute_test(&self, command: &str) -> Result<RunResult>;
}
```

## Configuration

### Essential Configuration (80% of users only need this)

```rust
// Basic configuration - 80% of users only need these fields
pub struct CleanroomConfig {
    pub security_policy: SecurityPolicy,
    pub resource_limits: ResourceLimits,
    pub performance_monitoring: PerformanceMonitoringConfig,
    pub enable_deterministic_execution: bool,
    pub deterministic_seed: Option<u64>,
}

// Security configuration - essential for all users
pub struct SecurityPolicy {
    pub enable_network_isolation: bool,
    pub enable_filesystem_isolation: bool,
    pub enable_process_isolation: bool,
    pub allowed_ports: Vec<u16>,
    pub enable_data_redaction: bool,
    pub security_level: SecurityLevel,
}

// Resource limits - essential for production use
pub struct ResourceLimits {
    pub max_cpu_usage_percent: f64,
    pub max_memory_usage_bytes: u64,
    pub max_disk_usage_bytes: u64,
    pub max_container_count: u32,
    pub max_test_execution_time: Duration,
}
```

## Error Handling

### Essential Error Handling (80% of users only need this)

```rust
// Main error type - essential for all users
pub enum CleanroomError {
    Container(String),
    Backend(String),
    Policy(String),
    Configuration(String),
    Execution(String),
}

// Error handling patterns
match result {
    Ok(value) => println!("Success: {:?}", value),
    Err(CleanroomError::Container(msg)) => eprintln!("Container error: {}", msg),
    Err(CleanroomError::Policy(msg)) => eprintln!("Policy violation: {}", msg),
    Err(err) => eprintln!("Other error: {}", err),
}
```

## Container Management

### Essential Container Management (80% of users only need this)

```rust
// Container creation - 80% of users only need these patterns
impl CleanroomEnvironment {
    pub async fn get_or_create_container<F>(
        &self,
        name: &str,
        factory: F,
    ) -> Result<ContainerWrapper>
    where
        F: FnOnce() -> Result<Box<dyn BaseContainer>>,
    {
        // Implementation
    }
}

// Container operations - 80% of users only need these
impl BaseContainer for PostgresContainer {
    async fn wait_for_ready(&self) -> Result<()> {
        // Wait for container to be ready
    }

    async fn get_status(&self) -> ContainerStatus {
        // Get container status
    }
}
```

## Security & Policies

### Essential Security (80% of users only need this)

```rust
// Policy creation - 80% of users only need these patterns
impl Policy {
    pub fn with_security_level(level: SecurityLevel) -> Self {
        // Create policy with security level
    }

    pub fn with_network_isolation(mut self, enable: bool) -> Self {
        self.security.enable_network_isolation = enable;
        self
    }

    pub fn with_resource_limits(mut self, limits: ResourceLimits) -> Self {
        self.resources = limits.into();
        self
    }
}

// Security levels - 80% of users only need these
pub enum SecurityLevel {
    Permissive,
    Standard,
    Strict,
    Locked,
}
```

## Performance & Monitoring

### Essential Monitoring (80% of users only need this)

```rust
// Performance monitoring - 80% of users only need basic metrics
impl CleanroomEnvironment {
    pub async fn get_metrics(&self) -> CleanroomMetrics {
        // Get performance metrics
    }

    pub async fn check_resource_limits(&self) -> Result<()> {
        // Check if resource limits are exceeded
    }
}

// Metrics structure - 80% of users only need these fields
pub struct CleanroomMetrics {
    pub session_id: Uuid,
    pub start_time: SerializableInstant,
    pub total_duration_ms: u64,
    pub tests_executed: u32,
    pub containers_created: u32,
    pub resource_usage: ResourceUsageMetrics,
}
```

## Usage Examples

### Basic Usage (80% of users only need this)

```rust
use clnrm::{run, run_with_policy, Policy, SecurityLevel, Assert};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Simple execution - 80% of use cases
    let result = run(["echo", "hello world"])?;
    result.assert_success();
    assert_eq!(result.stdout.trim(), "hello world");

    // Policy-based execution - 80% of advanced use cases
    let policy = Policy::with_security_level(SecurityLevel::Standard);
    let result = run_with_policy(["python3", "--version"], &policy)?;
    result.assert_success();
    
    Ok(())
}
```

### Environment Usage (80% of users only need this)

```rust
use clnrm::{CleanroomEnvironment, CleanroomConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create environment - 80% of users only need this
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await?;
    
    // Execute test - 80% of users only need this
    let result = environment.execute_test("echo 'Hello from environment'").await?;
    result.assert_success();
    
    Ok(())
}
```

### Container Usage (80% of users only need this)

```rust
use clnrm::{CleanroomEnvironment, PostgresContainer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = CleanroomConfig::default();
    let environment = CleanroomEnvironment::new(config).await?;
    
    // Get or create container - 80% of users only need this pattern
    let postgres = environment
        .get_or_create_container("postgres", || {
            PostgresContainer::new("testdb", "testuser", "testpass")
        })
        .await?;

    // Wait for readiness - 80% of users only need this
    postgres.wait_for_ready().await?;
    
    Ok(())
}
```

## Best Practices

### 1. Configuration (80/20)

```rust
// Good: Focus on essential configuration
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

// Avoid: Over-configuration
// Don't set every field unless you need specific behavior
```

### 2. Error Handling (80/20)

```rust
// Good: Handle essential error types
match result {
    Ok(value) => println!("Success"),
    Err(CleanroomError::Container(msg)) => {
        eprintln!("Container failed: {}", msg);
        // Handle container-specific errors
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

// Avoid: Ignoring errors or generic handling
```

### 3. Resource Management (80/20)

```rust
// Good: Use RAII for automatic cleanup
struct TestEnvironment {
    environment: CleanroomEnvironment,
}

impl TestEnvironment {
    async fn new() -> Result<Self, CleanroomError> {
        let config = CleanroomConfig::default();
        let environment = CleanroomEnvironment::new(config).await?;
        Ok(Self { environment })
    }
}

impl Drop for TestEnvironment {
    fn drop(&mut self) {
        // Automatic cleanup via RAII
    }
}

// Avoid: Manual resource management
```

### 4. Performance (80/20)

```rust
// Good: Use singleton containers for performance
let postgres = environment
    .get_or_create_container("postgres", || {
        PostgresContainer::new("testdb", "testuser", "testpass")
    })
    .await?;

// Good: Configure appropriate resource limits
config.resource_limits.max_container_count = 10;
config.resource_limits.max_test_execution_time = Duration::from_secs(300);

// Avoid: Creating new containers for each test
```

### 5. Security (80/20)

```rust
// Good: Enable essential security features
config.security_policy.enable_network_isolation = true;
config.security_policy.enable_filesystem_isolation = true;
config.security_policy.enable_data_redaction = true;

// Good: Set appropriate security level
config.security_policy.security_level = SecurityLevel::Standard;

// Avoid: Disabling security without good reason
```

## Migration Guide

### From v0.1 to Current

```rust
// Before
config.enable_security_policy = true;

// After
config.security_policy = SecurityPolicy::default();

// Before
let policy = Policy::default();

// After
let policy = Policy::with_security_level(SecurityLevel::Standard);
```

## Quick Reference

### Most Used Types (80% of usage)

1. `CleanroomEnvironment` - Main environment type
2. `CleanroomConfig` - Configuration type
3. `Policy` - Security and resource policies
4. `RunResult` - Execution result type
5. `CleanroomError` - Error type

### Most Used Functions (80% of usage)

1. `run()` - Execute command
2. `run_with_policy()` - Execute with security policy
3. `CleanroomEnvironment::new()` - Create environment
4. `CleanroomEnvironment::execute_test()` - Execute test

### Most Used Patterns (80% of usage)

1. **Basic execution**: `run(["command"])?`
2. **Environment usage**: `CleanroomEnvironment::new(config).await?`
3. **Policy enforcement**: `run_with_policy(["command"], &policy)?`
4. **Container management**: `get_or_create_container()`
5. **Error handling**: `match result { Ok(_) => ..., Err(_) => ... }`

## Resources

- **Getting Started**: [../guides/getting-started-tutorial.md](../guides/getting-started-tutorial.md)
- **Best Practices**: [../guides/best-practices.md](../guides/best-practices.md)
- **Architecture**: [../architecture-overview.md](../architecture-overview.md)
- **ADRs**: [../adr/README.md](../adr/README.md)

---

*This API reference follows 80/20 principles: 80% of the value with 20% of the documentation complexity. For advanced usage, see the architecture documentation.*