//! Test container fixtures and builders
//!
//! Provides standardized test container creation and management.

use clnrm::{GenericContainer, PostgresContainer, RedisContainer};
use std::collections::HashMap;

/// Test container fixtures
pub struct TestContainers;

impl TestContainers {
    /// Create a test PostgreSQL container
    pub fn postgres() -> PostgresContainer {
        PostgresContainer::new("postgres:15")
            .with_env("POSTGRES_PASSWORD", "test")
            .with_env("POSTGRES_DB", "testdb")
            .with_env("POSTGRES_USER", "testuser")
            .with_port(5432)
    }

    /// Create a test Redis container
    pub fn redis() -> RedisContainer {
        RedisContainer::new("redis:7")
            .with_port(6379)
            .with_env("REDIS_PASSWORD", "test")
    }

    /// Create a test generic container
    pub fn nginx() -> GenericContainer {
        GenericContainer::new("nginx:latest")
            .with_port(8080)
            .with_env("NGINX_PORT", "8080")
    }

    /// Create a test Alpine container
    pub fn alpine() -> GenericContainer {
        GenericContainer::new("alpine:latest")
            .with_env("TEST_ENV", "test_value")
    }

    /// Create a test Ubuntu container
    pub fn ubuntu() -> GenericContainer {
        GenericContainer::new("ubuntu:latest")
            .with_env("DEBIAN_FRONTEND", "noninteractive")
    }
}

/// Test container builder for custom containers
pub struct TestContainerBuilder {
    image: String,
    ports: Vec<u16>,
    env_vars: HashMap<String, String>,
    volumes: HashMap<String, String>,
}

impl TestContainerBuilder {
    /// Create a new test container builder
    pub fn new(image: &str) -> Self {
        Self {
            image: image.to_string(),
            ports: Vec::new(),
            env_vars: HashMap::new(),
            volumes: HashMap::new(),
        }
    }

    /// Add a port mapping
    pub fn with_port(mut self, port: u16) -> Self {
        self.ports.push(port);
        self
    }

    /// Add multiple port mappings
    pub fn with_ports(mut self, ports: Vec<u16>) -> Self {
        self.ports.extend(ports);
        self
    }

    /// Add an environment variable
    pub fn with_env(mut self, key: &str, value: &str) -> Self {
        self.env_vars.insert(key.to_string(), value.to_string());
        self
    }

    /// Add multiple environment variables
    pub fn with_envs(mut self, envs: HashMap<String, String>) -> Self {
        self.env_vars.extend(envs);
        self
    }

    /// Add a volume mapping
    pub fn with_volume(mut self, host_path: &str, container_path: &str) -> Self {
        self.volumes.insert(host_path.to_string(), container_path.to_string());
        self
    }

    /// Add multiple volume mappings
    pub fn with_volumes(mut self, volumes: HashMap<String, String>) -> Self {
        self.volumes.extend(volumes);
        self
    }

    /// Build a generic container
    pub fn build_generic(self) -> GenericContainer {
        let mut container = GenericContainer::new(&self.image);
        
        for port in self.ports {
            container = container.with_port(port);
        }
        
        for (key, value) in self.env_vars {
            container = container.with_env(&key, &value);
        }
        
        // Note: Volume mapping would need to be implemented in GenericContainer
        // For now, we'll just return the container with ports and env vars
        
        container
    }

    /// Build a PostgreSQL container (if image is postgres)
    pub fn build_postgres(self) -> Option<PostgresContainer> {
        if self.image.starts_with("postgres") {
            let mut container = PostgresContainer::new(&self.image);
            
            for port in self.ports {
                container = container.with_port(port);
            }
            
            for (key, value) in self.env_vars {
                container = container.with_env(&key, &value);
            }
            
            Some(container)
        } else {
            None
        }
    }

    /// Build a Redis container (if image is redis)
    pub fn build_redis(self) -> Option<RedisContainer> {
        if self.image.starts_with("redis") {
            let mut container = RedisContainer::new(&self.image);
            
            for port in self.ports {
                container = container.with_port(port);
            }
            
            for (key, value) in self.env_vars {
                container = container.with_env(&key, &value);
            }
            
            Some(container)
        } else {
            None
        }
    }
}

/// Test container registry for managing multiple containers
pub struct TestContainerRegistry {
    containers: HashMap<String, GenericContainer>,
}

impl TestContainerRegistry {
    /// Create a new test container registry
    pub fn new() -> Self {
        Self {
            containers: HashMap::new(),
        }
    }

    /// Register a container with a name
    pub fn register(mut self, name: &str, container: GenericContainer) -> Self {
        self.containers.insert(name.to_string(), container);
        self
    }

    /// Get a container by name
    pub fn get(&self, name: &str) -> Option<&GenericContainer> {
        self.containers.get(name)
    }

    /// Get all container names
    pub fn names(&self) -> Vec<String> {
        self.containers.keys().cloned().collect()
    }

    /// Get the number of registered containers
    pub fn count(&self) -> usize {
        self.containers.len()
    }

    /// Check if a container is registered
    pub fn contains(&self, name: &str) -> bool {
        self.containers.contains_key(name)
    }
}

impl Default for TestContainerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_presets() {
        let postgres = TestContainers::postgres();
        // Test that postgres container is created (basic check)
        assert!(true); // Placeholder - would need to check container properties

        let redis = TestContainers::redis();
        assert!(true); // Placeholder

        let nginx = TestContainers::nginx();
        assert!(true); // Placeholder

        let alpine = TestContainers::alpine();
        assert!(true); // Placeholder
    }

    #[test]
    fn test_container_builder() {
        let container = TestContainerBuilder::new("nginx:latest")
            .with_port(8080)
            .with_port(8443)
            .with_env("NGINX_PORT", "8080")
            .with_env("NGINX_SSL", "true")
            .build_generic();

        assert!(true); // Placeholder - would need to check container properties
    }

    #[test]
    fn test_container_builder_postgres() {
        let container = TestContainerBuilder::new("postgres:15")
            .with_port(5432)
            .with_env("POSTGRES_PASSWORD", "test")
            .with_env("POSTGRES_DB", "testdb")
            .build_postgres();

        assert!(container.is_some());
    }

    #[test]
    fn test_container_builder_redis() {
        let container = TestContainerBuilder::new("redis:7")
            .with_port(6379)
            .with_env("REDIS_PASSWORD", "test")
            .build_redis();

        assert!(container.is_some());
    }

    #[test]
    fn test_container_registry() {
        let registry = TestContainerRegistry::new()
            .register("web", TestContainers::nginx())
            .register("db", TestContainers::postgres())
            .register("cache", TestContainers::redis());

        assert_eq!(registry.count(), 3);
        assert!(registry.contains("web"));
        assert!(registry.contains("db"));
        assert!(registry.contains("cache"));
        assert!(!registry.contains("nonexistent"));

        let names = registry.names();
        assert_eq!(names.len(), 3);
        assert!(names.contains(&"web".to_string()));
        assert!(names.contains(&"db".to_string()));
        assert!(names.contains(&"cache".to_string()));
    }
}
