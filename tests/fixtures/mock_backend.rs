//! Mock backend infrastructure for CI/CD compatibility
//!
//! Provides mock implementations of container backends to enable
//! test execution without Docker dependency.

use clnrm::{ContainerWrapper, GenericContainer, PostgresContainer, RedisContainer};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Mock container state
#[derive(Debug, Clone)]
pub struct MockContainerState {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: ContainerStatus,
    pub created_at: Instant,
    pub ports: Vec<u16>,
    pub env_vars: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContainerStatus {
    Created,
    Running,
    Stopped,
    Removed,
}

/// Mock backend for container operations
pub struct MockBackend {
    containers: Arc<Mutex<HashMap<String, MockContainerState>>>,
    next_id: Arc<Mutex<u64>>,
}

impl MockBackend {
    pub fn new() -> Self {
        Self {
            containers: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    pub fn create_container(&self, name: &str, image: &str) -> Result<String, String> {
        let mut containers = self.containers.lock().unwrap();
        let mut next_id = self.next_id.lock().unwrap();

        let container_id = format!("mock_{}", next_id);
        *next_id += 1;

        let state = MockContainerState {
            id: container_id.clone(),
            name: name.to_string(),
            image: image.to_string(),
            status: ContainerStatus::Created,
            created_at: Instant::now(),
            ports: Vec::new(),
            env_vars: HashMap::new(),
        };

        containers.insert(container_id.clone(), state);
        Ok(container_id)
    }

    pub fn start_container(&self, id: &str) -> Result<(), String> {
        let mut containers = self.containers.lock().unwrap();
        if let Some(state) = containers.get_mut(id) {
            state.status = ContainerStatus::Running;
            Ok(())
        } else {
            Err(format!("Container {} not found", id))
        }
    }

    pub fn stop_container(&self, id: &str) -> Result<(), String> {
        let mut containers = self.containers.lock().unwrap();
        if let Some(state) = containers.get_mut(id) {
            state.status = ContainerStatus::Stopped;
            Ok(())
        } else {
            Err(format!("Container {} not found", id))
        }
    }

    pub fn remove_container(&self, id: &str) -> Result<(), String> {
        let mut containers = self.containers.lock().unwrap();
        if containers.remove(id).is_some() {
            Ok(())
        } else {
            Err(format!("Container {} not found", id))
        }
    }

    pub fn get_container_status(&self, id: &str) -> Option<ContainerStatus> {
        let containers = self.containers.lock().unwrap();
        containers.get(id).map(|state| state.status.clone())
    }

    pub fn list_containers(&self) -> Vec<MockContainerState> {
        let containers = self.containers.lock().unwrap();
        containers.values().cloned().collect()
    }

    pub fn cleanup_all(&self) {
        let mut containers = self.containers.lock().unwrap();
        containers.clear();
    }
}

impl Default for MockBackend {
    fn default() -> Self {
        Self::new()
    }
}

/// Mock GenericContainer implementation
pub struct MockGenericContainer {
    backend: Arc<MockBackend>,
    id: String,
    name: String,
    image: String,
}

impl MockGenericContainer {
    pub fn new(name: &str, image: &str, tag: &str) -> Self {
        let backend = Arc::new(MockBackend::new());
        let full_image = format!("{}:{}", image, tag);
        let id = backend.create_container(name, &full_image).unwrap();

        Self {
            backend,
            id,
            name: name.to_string(),
            image: full_image,
        }
    }

    pub fn with_port(mut self, port: u16) -> Self {
        // In a real implementation, this would update the container state
        self
    }

    pub fn with_env(mut self, key: &str, value: &str) -> Self {
        // In a real implementation, this would update the container state
        self
    }
}

impl ContainerWrapper for MockGenericContainer {
    fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.backend.start_container(&self.id).map_err(|e| {
            Box::new(std::io::Error::new(std::io::ErrorKind::Other, e))
                as Box<dyn std::error::Error>
        })
    }

    fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.backend.stop_container(&self.id).map_err(|e| {
            Box::new(std::io::Error::new(std::io::ErrorKind::Other, e))
                as Box<dyn std::error::Error>
        })
    }

    fn remove(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.backend.remove_container(&self.id).map_err(|e| {
            Box::new(std::io::Error::new(std::io::ErrorKind::Other, e))
                as Box<dyn std::error::Error>
        })
    }

    fn is_running(&self) -> bool {
        self.backend
            .get_container_status(&self.id)
            .map(|status| status == ContainerStatus::Running)
            .unwrap_or(false)
    }

    fn get_id(&self) -> &str {
        &self.id
    }
}

/// Mock PostgresContainer implementation
pub struct MockPostgresContainer {
    backend: Arc<MockBackend>,
    id: String,
    name: String,
    image: String,
}

impl MockPostgresContainer {
    pub fn new(db: &str, user: &str, password: &str) -> Self {
        let backend = Arc::new(MockBackend::new());
        let name = format!("postgres_{}", db);
        let image = "postgres:15";
        let id = backend.create_container(&name, image).unwrap();

        Self {
            backend,
            id,
            name,
            image: image.to_string(),
        }
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self
    }

    pub fn with_env(mut self, key: &str, value: &str) -> Self {
        self
    }
}

impl ContainerWrapper for MockPostgresContainer {
    fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.backend.start_container(&self.id).map_err(|e| {
            Box::new(std::io::Error::new(std::io::ErrorKind::Other, e))
                as Box<dyn std::error::Error>
        })
    }

    fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.backend.stop_container(&self.id).map_err(|e| {
            Box::new(std::io::Error::new(std::io::ErrorKind::Other, e))
                as Box<dyn std::error::Error>
        })
    }

    fn remove(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.backend.remove_container(&self.id).map_err(|e| {
            Box::new(std::io::Error::new(std::io::ErrorKind::Other, e))
                as Box<dyn std::error::Error>
        })
    }

    fn is_running(&self) -> bool {
        self.backend
            .get_container_status(&self.id)
            .map(|status| status == ContainerStatus::Running)
            .unwrap_or(false)
    }

    fn get_id(&self) -> &str {
        &self.id
    }
}

/// Mock RedisContainer implementation
pub struct MockRedisContainer {
    backend: Arc<MockBackend>,
    id: String,
    name: String,
    image: String,
}

impl MockRedisContainer {
    pub fn new(password: Option<String>) -> Self {
        let backend = Arc::new(MockBackend::new());
        let name = "redis_mock";
        let image = "redis:7";
        let id = backend.create_container(name, image).unwrap();

        Self {
            backend,
            id,
            name: name.to_string(),
            image: image.to_string(),
        }
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self
    }

    pub fn with_env(mut self, key: &str, value: &str) -> Self {
        self
    }
}

impl ContainerWrapper for MockRedisContainer {
    fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.backend.start_container(&self.id).map_err(|e| {
            Box::new(std::io::Error::new(std::io::ErrorKind::Other, e))
                as Box<dyn std::error::Error>
        })
    }

    fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.backend.stop_container(&self.id).map_err(|e| {
            Box::new(std::io::Error::new(std::io::ErrorKind::Other, e))
                as Box<dyn std::error::Error>
        })
    }

    fn remove(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.backend.remove_container(&self.id).map_err(|e| {
            Box::new(std::io::Error::new(std::io::ErrorKind::Other, e))
                as Box<dyn std::error::Error>
        })
    }

    fn is_running(&self) -> bool {
        self.backend
            .get_container_status(&self.id)
            .map(|status| status == ContainerStatus::Running)
            .unwrap_or(false)
    }

    fn get_id(&self) -> &str {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_backend_creation() {
        let backend = MockBackend::new();
        let containers = backend.list_containers();
        assert_eq!(containers.len(), 0);
    }

    #[test]
    fn test_mock_container_lifecycle() {
        let backend = MockBackend::new();

        // Create container
        let id = backend.create_container("test", "alpine:latest").unwrap();
        assert_eq!(
            backend.get_container_status(&id),
            Some(ContainerStatus::Created)
        );

        // Start container
        backend.start_container(&id).unwrap();
        assert_eq!(
            backend.get_container_status(&id),
            Some(ContainerStatus::Running)
        );

        // Stop container
        backend.stop_container(&id).unwrap();
        assert_eq!(
            backend.get_container_status(&id),
            Some(ContainerStatus::Stopped)
        );

        // Remove container
        backend.remove_container(&id).unwrap();
        assert_eq!(backend.get_container_status(&id), None);
    }

    #[test]
    fn test_mock_generic_container() {
        let container = MockGenericContainer::new("test", "alpine", "latest");
        assert_eq!(container.get_id().starts_with("mock_"), true);
        assert!(!container.is_running());

        container.start().unwrap();
        assert!(container.is_running());

        container.stop().unwrap();
        assert!(!container.is_running());
    }
}
