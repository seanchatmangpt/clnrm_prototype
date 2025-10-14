//! OpenTelemetry integration for cleanroom testing
//!
//! This module provides OpenTelemetry-based tracing, metrics, and logging
//! following industry standards and best practices.

pub mod tracing;
pub mod metrics;
pub mod logging;
pub mod exporters;

use crate::error::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

/// OpenTelemetry manager for cleanroom testing
#[derive(Debug, Clone)]
pub struct OtelManager {
    /// Tracing manager
    pub tracing: Arc<tracing::OtelTracingManager>,
    /// Metrics manager
    pub metrics: Arc<metrics::OtelMetricsManager>,
    /// Logging manager
    pub logging: Arc<logging::OtelLoggingManager>,
    /// Configuration
    config: OtelConfig,
}

/// OpenTelemetry configuration
#[derive(Debug, Clone)]
pub struct OtelConfig {
    /// Service name
    pub service_name: String,
    /// Service version
    pub service_version: String,
    /// Environment (dev, staging, prod)
    pub environment: String,
    /// Tracing configuration
    pub tracing: tracing::TracingConfig,
    /// Metrics configuration
    pub metrics: metrics::MetricsConfig,
    /// Logging configuration
    pub logging: logging::LoggingConfig,
    /// Exporters configuration
    pub exporters: exporters::ExportersConfig,
}

impl Default for OtelConfig {
    fn default() -> Self {
        Self {
            service_name: "cleanroom".to_string(),
            service_version: env!("CARGO_PKG_VERSION").to_string(),
            environment: "development".to_string(),
            tracing: tracing::TracingConfig::default(),
            metrics: metrics::MetricsConfig::default(),
            logging: logging::LoggingConfig::default(),
            exporters: exporters::ExportersConfig::default(),
        }
    }
}

impl OtelManager {
    /// Create a new OpenTelemetry manager
    pub async fn new(config: OtelConfig) -> Result<Self> {
        let tracing = Arc::new(tracing::OtelTracingManager::new(config.tracing.clone()).await?);
        let metrics = Arc::new(metrics::OtelMetricsManager::new(config.metrics.clone()).await?);
        let logging = Arc::new(logging::OtelLoggingManager::new(config.logging.clone()).await?);

        Ok(Self {
            tracing,
            metrics,
            logging,
            config,
        })
    }

    /// Create a new OpenTelemetry manager with default configuration
    pub async fn new_default() -> Result<Self> {
        Self::new(OtelConfig::default()).await
    }

    /// Get the service name
    pub fn service_name(&self) -> &str {
        &self.config.service_name
    }

    /// Get the service version
    pub fn service_version(&self) -> &str {
        &self.config.service_version
    }

    /// Get the environment
    pub fn environment(&self) -> &str {
        &self.config.environment
    }

    /// Shutdown the OpenTelemetry manager
    pub async fn shutdown(&self) -> Result<()> {
        self.tracing.shutdown().await?;
        self.metrics.shutdown().await?;
        self.logging.shutdown().await?;
        Ok(())
    }
}

/// Resource attributes for OpenTelemetry
pub fn resource_attributes(service_name: &str, service_version: &str, environment: &str) -> Vec<(String, String)> {
    vec![
        ("service.name".to_string(), service_name.to_string()),
        ("service.version".to_string(), service_version.to_string()),
        ("service.environment".to_string(), environment.to_string()),
        ("service.instance.id".to_string(), uuid::Uuid::new_v4().to_string()),
    ]
}

/// Common span attributes for cleanroom operations
pub fn cleanroom_span_attributes() -> Vec<(String, String)> {
    vec![
        ("cleanroom.framework".to_string(), "clnrm".to_string()),
        ("cleanroom.version".to_string(), env!("CARGO_PKG_VERSION").to_string()),
    ]
}

/// Common metric attributes for cleanroom operations
pub fn cleanroom_metric_attributes() -> Vec<(String, String)> {
    vec![
        ("framework".to_string(), "cleanroom".to_string()),
        ("version".to_string(), env!("CARGO_PKG_VERSION").to_string()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_otel_manager_creation() {
        let config = OtelConfig::default();
        let manager = OtelManager::new(config).await;
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_otel_manager_default() {
        let manager = OtelManager::new_default().await;
        assert!(manager.is_ok());
    }

    #[test]
    fn test_resource_attributes() {
        let attrs = resource_attributes("test-service", "1.0.0", "test");
        assert_eq!(attrs.len(), 4);
        assert!(attrs.iter().any(|(k, v)| k == "service.name" && v == "test-service"));
        assert!(attrs.iter().any(|(k, v)| k == "service.version" && v == "1.0.0"));
        assert!(attrs.iter().any(|(k, v)| k == "service.environment" && v == "test"));
    }

    #[test]
    fn test_cleanroom_span_attributes() {
        let attrs = cleanroom_span_attributes();
        assert_eq!(attrs.len(), 2);
        assert!(attrs.iter().any(|(k, v)| k == "cleanroom.framework" && v == "clnrm"));
    }

    #[test]
    fn test_cleanroom_metric_attributes() {
        let attrs = cleanroom_metric_attributes();
        assert_eq!(attrs.len(), 2);
        assert!(attrs.iter().any(|(k, v)| k == "framework" && v == "cleanroom"));
    }
}
