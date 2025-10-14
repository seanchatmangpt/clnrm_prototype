//! OpenTelemetry exporters configuration for cleanroom testing
//!
//! This module provides configuration for various OpenTelemetry exporters
//! including console, Jaeger, OTLP, and Prometheus.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Exporters configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportersConfig {
    /// Console exporter configuration
    pub console: ConsoleExporterConfig,
    /// Jaeger exporter configuration
    pub jaeger: JaegerExporterConfig,
    /// OTLP exporter configuration
    pub otlp: OtlpExporterConfig,
    /// Prometheus exporter configuration
    pub prometheus: PrometheusExporterConfig,
}

/// Console exporter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsoleExporterConfig {
    /// Enable console exporter
    pub enabled: bool,
    /// Pretty print output
    pub pretty: bool,
    /// Include timestamps
    pub include_timestamps: bool,
}

/// Jaeger exporter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JaegerExporterConfig {
    /// Enable Jaeger exporter
    pub enabled: bool,
    /// Jaeger agent endpoint
    pub agent_endpoint: String,
    /// Service name
    pub service_name: String,
    /// Batch configuration
    pub batch: BatchConfig,
}

/// OTLP exporter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtlpExporterConfig {
    /// Enable OTLP exporter
    pub enabled: bool,
    /// OTLP endpoint
    pub endpoint: String,
    /// Headers
    pub headers: HashMap<String, String>,
    /// Timeout
    pub timeout: Duration,
    /// Batch configuration
    pub batch: BatchConfig,
}

/// Prometheus exporter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrometheusExporterConfig {
    /// Enable Prometheus exporter
    pub enabled: bool,
    /// Listen address
    pub listen_address: String,
    /// Metrics path
    pub metrics_path: String,
    /// Update interval
    pub update_interval: Duration,
}

/// Batch configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchConfig {
    /// Maximum number of spans/metrics per batch
    pub max_export_batch_size: usize,
    /// Maximum time to wait before exporting
    pub export_timeout: Duration,
    /// Maximum queue size
    pub max_queue_size: usize,
}

impl Default for ExportersConfig {
    fn default() -> Self {
        Self {
            console: ConsoleExporterConfig::default(),
            jaeger: JaegerExporterConfig::default(),
            otlp: OtlpExporterConfig::default(),
            prometheus: PrometheusExporterConfig::default(),
        }
    }
}

impl Default for ConsoleExporterConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            pretty: true,
            include_timestamps: true,
        }
    }
}

impl Default for JaegerExporterConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            agent_endpoint: "http://localhost:14268/api/traces".to_string(),
            service_name: "cleanroom".to_string(),
            batch: BatchConfig::default(),
        }
    }
}

impl Default for OtlpExporterConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            endpoint: "http://localhost:4317".to_string(),
            headers: HashMap::new(),
            timeout: Duration::from_secs(10),
            batch: BatchConfig::default(),
        }
    }
}

impl Default for PrometheusExporterConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            listen_address: "0.0.0.0:9090".to_string(),
            metrics_path: "/metrics".to_string(),
            update_interval: Duration::from_secs(10),
        }
    }
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            max_export_batch_size: 512,
            export_timeout: Duration::from_secs(5),
            max_queue_size: 2048,
        }
    }
}

/// Exporter builder for easy configuration
pub struct ExporterBuilder {
    config: ExportersConfig,
}

impl ExporterBuilder {
    /// Create a new exporter builder
    pub fn new() -> Self {
        Self {
            config: ExportersConfig::default(),
        }
    }

    /// Enable console exporter
    pub fn with_console(mut self, enabled: bool) -> Self {
        self.config.console.enabled = enabled;
        self
    }

    /// Configure console exporter
    pub fn with_console_config(mut self, config: ConsoleExporterConfig) -> Self {
        self.config.console = config;
        self
    }

    /// Enable Jaeger exporter
    pub fn with_jaeger(mut self, enabled: bool) -> Self {
        self.config.jaeger.enabled = enabled;
        self
    }

    /// Configure Jaeger exporter
    pub fn with_jaeger_config(mut self, config: JaegerExporterConfig) -> Self {
        self.config.jaeger = config;
        self
    }

    /// Enable OTLP exporter
    pub fn with_otlp(mut self, enabled: bool) -> Self {
        self.config.otlp.enabled = enabled;
        self
    }

    /// Configure OTLP exporter
    pub fn with_otlp_config(mut self, config: OtlpExporterConfig) -> Self {
        self.config.otlp = config;
        self
    }

    /// Enable Prometheus exporter
    pub fn with_prometheus(mut self, enabled: bool) -> Self {
        self.config.prometheus.enabled = enabled;
        self
    }

    /// Configure Prometheus exporter
    pub fn with_prometheus_config(mut self, config: PrometheusExporterConfig) -> Self {
        self.config.prometheus = config;
        self
    }

    /// Build the exporters configuration
    pub fn build(self) -> ExportersConfig {
        self.config
    }
}

impl Default for ExporterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Development configuration for local testing
impl ExportersConfig {
    /// Create a development configuration
    pub fn development() -> Self {
        ExporterBuilder::new()
            .with_console(true)
            .with_jaeger(false)
            .with_otlp(false)
            .with_prometheus(false)
            .build()
    }

    /// Create a production configuration
    pub fn production() -> Self {
        ExporterBuilder::new()
            .with_console(false)
            .with_jaeger(true)
            .with_otlp(true)
            .with_prometheus(true)
            .build()
    }

    /// Create a testing configuration
    pub fn testing() -> Self {
        ExporterBuilder::new()
            .with_console(false)
            .with_jaeger(false)
            .with_otlp(false)
            .with_prometheus(false)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exporters_config_default() {
        let config = ExportersConfig::default();
        assert!(config.console.enabled);
        assert!(!config.jaeger.enabled);
        assert!(!config.otlp.enabled);
        assert!(!config.prometheus.enabled);
    }

    #[test]
    fn test_exporters_config_development() {
        let config = ExportersConfig::development();
        assert!(config.console.enabled);
        assert!(!config.jaeger.enabled);
        assert!(!config.otlp.enabled);
        assert!(!config.prometheus.enabled);
    }

    #[test]
    fn test_exporters_config_production() {
        let config = ExportersConfig::production();
        assert!(!config.console.enabled);
        assert!(config.jaeger.enabled);
        assert!(config.otlp.enabled);
        assert!(config.prometheus.enabled);
    }

    #[test]
    fn test_exporters_config_testing() {
        let config = ExportersConfig::testing();
        assert!(!config.console.enabled);
        assert!(!config.jaeger.enabled);
        assert!(!config.otlp.enabled);
        assert!(!config.prometheus.enabled);
    }

    #[test]
    fn test_exporter_builder() {
        let config = ExporterBuilder::new()
            .with_console(true)
            .with_jaeger(true)
            .with_otlp(false)
            .with_prometheus(false)
            .build();

        assert!(config.console.enabled);
        assert!(config.jaeger.enabled);
        assert!(!config.otlp.enabled);
        assert!(!config.prometheus.enabled);
    }

    #[test]
    fn test_console_exporter_config() {
        let config = ConsoleExporterConfig::default();
        assert!(config.enabled);
        assert!(config.pretty);
        assert!(config.include_timestamps);
    }

    #[test]
    fn test_jaeger_exporter_config() {
        let config = JaegerExporterConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.agent_endpoint, "http://localhost:14268/api/traces");
        assert_eq!(config.service_name, "cleanroom");
    }

    #[test]
    fn test_otlp_exporter_config() {
        let config = OtlpExporterConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.endpoint, "http://localhost:4317");
        assert_eq!(config.timeout, Duration::from_secs(10));
    }

    #[test]
    fn test_prometheus_exporter_config() {
        let config = PrometheusExporterConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.listen_address, "0.0.0.0:9090");
        assert_eq!(config.metrics_path, "/metrics");
        assert_eq!(config.update_interval, Duration::from_secs(10));
    }

    #[test]
    fn test_batch_config() {
        let config = BatchConfig::default();
        assert_eq!(config.max_export_batch_size, 512);
        assert_eq!(config.export_timeout, Duration::from_secs(5));
        assert_eq!(config.max_queue_size, 2048);
    }
}
