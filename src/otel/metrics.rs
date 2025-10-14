//! OpenTelemetry metrics implementation for cleanroom testing
//!
//! This module provides metrics collection capabilities using OpenTelemetry
//! with support for counters, gauges, and histograms.

use crate::error::Result;
use opentelemetry::{
    global,
    metrics::{Counter, Gauge, Histogram, Meter, MeterProvider, Unit},
    Key, KeyValue, Value,
};
use opentelemetry_sdk::{
    metrics::{MeterProvider as SdkMeterProvider, PeriodicReader, MeterProviderBuilder},
    Resource,
};
use opentelemetry_stdout::MetricsExporter as StdoutMetricsExporter;
use std::sync::Arc;
use std::time::Duration;

/// OpenTelemetry metrics manager
#[derive(Debug, Clone)]
pub struct OtelMetricsManager {
    /// Meter provider
    meter_provider: Arc<SdkMeterProvider>,
    /// Meter
    meter: Meter,
    /// Configuration
    config: MetricsConfig,
    /// Predefined metrics
    metrics: CleanroomMetrics,
}

/// Metrics configuration
#[derive(Debug, Clone)]
pub struct MetricsConfig {
    /// Enable metrics
    pub enabled: bool,
    /// Service name
    pub service_name: String,
    /// Service version
    pub service_version: String,
    /// Environment
    pub environment: String,
    /// Export interval
    pub export_interval: Duration,
    /// Export timeout
    pub export_timeout: Duration,
}

/// Predefined cleanroom metrics
#[derive(Debug, Clone)]
pub struct CleanroomMetrics {
    /// Container operations counter
    pub container_operations: Counter<u64>,
    /// Container startup time histogram
    pub container_startup_time: Histogram<f64>,
    /// Container memory usage gauge
    pub container_memory_usage: Gauge<f64>,
    /// Container CPU usage gauge
    pub container_cpu_usage: Gauge<f64>,
    /// Test execution counter
    pub test_executions: Counter<u64>,
    /// Test execution time histogram
    pub test_execution_time: Histogram<f64>,
    /// Test success counter
    pub test_successes: Counter<u64>,
    /// Test failures counter
    pub test_failures: Counter<u64>,
    /// Scenario executions counter
    pub scenario_executions: Counter<u64>,
    /// Scenario execution time histogram
    pub scenario_execution_time: Histogram<f64>,
    /// Active containers gauge
    pub active_containers: Gauge<u64>,
    /// Active sessions gauge
    pub active_sessions: Gauge<u64>,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            service_name: "cleanroom".to_string(),
            service_version: env!("CARGO_PKG_VERSION").to_string(),
            environment: "development".to_string(),
            export_interval: Duration::from_secs(10),
            export_timeout: Duration::from_secs(5),
        }
    }
}

impl OtelMetricsManager {
    /// Create a new OpenTelemetry metrics manager
    pub async fn new(config: MetricsConfig) -> Result<Self> {
        if !config.enabled {
            let meter = global::meter("cleanroom");
            let metrics = Self::create_metrics(&meter);
            return Ok(Self {
                meter_provider: Arc::new(SdkMeterProvider::builder().build()),
                meter,
                config,
                metrics,
            });
        }

        let resource = Resource::new(vec![
            KeyValue::new("service.name", config.service_name.clone()),
            KeyValue::new("service.version", config.service_version.clone()),
            KeyValue::new("service.environment", config.environment.clone()),
            KeyValue::new("cleanroom.framework", "clnrm"),
            KeyValue::new("cleanroom.version", env!("CARGO_PKG_VERSION")),
        ]);

        let stdout_exporter = StdoutMetricsExporter::default();
        let reader = PeriodicReader::builder(stdout_exporter, opentelemetry_sdk::runtime::Tokio)
            .with_interval(config.export_interval)
            .with_timeout(config.export_timeout)
            .build();

        let meter_provider = Arc::new(
            SdkMeterProvider::builder()
                .with_resource(resource)
                .with_reader(reader)
                .build()
        );

        let meter = meter_provider.meter("cleanroom");
        let metrics = Self::create_metrics(&meter);

        // Set as global meter provider
        global::set_meter_provider(meter_provider.clone());

        Ok(Self {
            meter_provider,
            meter,
            config,
            metrics,
        })
    }

    /// Create predefined metrics
    fn create_metrics(meter: &Meter) -> CleanroomMetrics {
        CleanroomMetrics {
            container_operations: meter
                .u64_counter("cleanroom_container_operations_total")
                .with_description("Total number of container operations")
                .with_unit(Unit::new("1"))
                .init(),
            container_startup_time: meter
                .f64_histogram("cleanroom_container_startup_time_seconds")
                .with_description("Container startup time in seconds")
                .with_unit(Unit::new("s"))
                .init(),
            container_memory_usage: meter
                .f64_gauge("cleanroom_container_memory_usage_bytes")
                .with_description("Container memory usage in bytes")
                .with_unit(Unit::new("bytes"))
                .init(),
            container_cpu_usage: meter
                .f64_gauge("cleanroom_container_cpu_usage_percent")
                .with_description("Container CPU usage percentage")
                .with_unit(Unit::new("percent"))
                .init(),
            test_executions: meter
                .u64_counter("cleanroom_test_executions_total")
                .with_description("Total number of test executions")
                .with_unit(Unit::new("1"))
                .init(),
            test_execution_time: meter
                .f64_histogram("cleanroom_test_execution_time_seconds")
                .with_description("Test execution time in seconds")
                .with_unit(Unit::new("s"))
                .init(),
            test_successes: meter
                .u64_counter("cleanroom_test_successes_total")
                .with_description("Total number of successful tests")
                .with_unit(Unit::new("1"))
                .init(),
            test_failures: meter
                .u64_counter("cleanroom_test_failures_total")
                .with_description("Total number of failed tests")
                .with_unit(Unit::new("1"))
                .init(),
            scenario_executions: meter
                .u64_counter("cleanroom_scenario_executions_total")
                .with_description("Total number of scenario executions")
                .with_unit(Unit::new("1"))
                .init(),
            scenario_execution_time: meter
                .f64_histogram("cleanroom_scenario_execution_time_seconds")
                .with_description("Scenario execution time in seconds")
                .with_unit(Unit::new("s"))
                .init(),
            active_containers: meter
                .u64_gauge("cleanroom_active_containers")
                .with_description("Number of active containers")
                .with_unit(Unit::new("1"))
                .init(),
            active_sessions: meter
                .u64_gauge("cleanroom_active_sessions")
                .with_description("Number of active sessions")
                .with_unit(Unit::new("1"))
                .init(),
        }
    }

    /// Record container operation
    pub fn record_container_operation(&self, operation: &str, container_id: &str) {
        let attributes = vec![
            KeyValue::new("operation", operation),
            KeyValue::new("container_id", container_id),
            KeyValue::new("framework", "cleanroom"),
        ];
        self.metrics.container_operations.add(1, &attributes);
    }

    /// Record container startup time
    pub fn record_container_startup_time(&self, duration: Duration, container_id: &str) {
        let attributes = vec![
            KeyValue::new("container_id", container_id),
            KeyValue::new("framework", "cleanroom"),
        ];
        self.metrics
            .container_startup_time
            .record(duration.as_secs_f64(), &attributes);
    }

    /// Record container memory usage
    pub fn record_container_memory_usage(&self, usage_bytes: f64, container_id: &str) {
        let attributes = vec![
            KeyValue::new("container_id", container_id),
            KeyValue::new("framework", "cleanroom"),
        ];
        self.metrics
            .container_memory_usage
            .record(usage_bytes, &attributes);
    }

    /// Record container CPU usage
    pub fn record_container_cpu_usage(&self, usage_percent: f64, container_id: &str) {
        let attributes = vec![
            KeyValue::new("container_id", container_id),
            KeyValue::new("framework", "cleanroom"),
        ];
        self.metrics
            .container_cpu_usage
            .record(usage_percent, &attributes);
    }

    /// Record test execution
    pub fn record_test_execution(&self, test_name: &str, session_id: &str) {
        let attributes = vec![
            KeyValue::new("test_name", test_name),
            KeyValue::new("session_id", session_id),
            KeyValue::new("framework", "cleanroom"),
        ];
        self.metrics.test_executions.add(1, &attributes);
    }

    /// Record test execution time
    pub fn record_test_execution_time(&self, duration: Duration, test_name: &str, session_id: &str) {
        let attributes = vec![
            KeyValue::new("test_name", test_name),
            KeyValue::new("session_id", session_id),
            KeyValue::new("framework", "cleanroom"),
        ];
        self.metrics
            .test_execution_time
            .record(duration.as_secs_f64(), &attributes);
    }

    /// Record test success
    pub fn record_test_success(&self, test_name: &str, session_id: &str) {
        let attributes = vec![
            KeyValue::new("test_name", test_name),
            KeyValue::new("session_id", session_id),
            KeyValue::new("framework", "cleanroom"),
        ];
        self.metrics.test_successes.add(1, &attributes);
    }

    /// Record test failure
    pub fn record_test_failure(&self, test_name: &str, session_id: &str, error: &str) {
        let attributes = vec![
            KeyValue::new("test_name", test_name),
            KeyValue::new("session_id", session_id),
            KeyValue::new("error", error),
            KeyValue::new("framework", "cleanroom"),
        ];
        self.metrics.test_failures.add(1, &attributes);
    }

    /// Record scenario execution
    pub fn record_scenario_execution(&self, scenario_name: &str, session_id: &str) {
        let attributes = vec![
            KeyValue::new("scenario_name", scenario_name),
            KeyValue::new("session_id", session_id),
            KeyValue::new("framework", "cleanroom"),
        ];
        self.metrics.scenario_executions.add(1, &attributes);
    }

    /// Record scenario execution time
    pub fn record_scenario_execution_time(&self, duration: Duration, scenario_name: &str, session_id: &str) {
        let attributes = vec![
            KeyValue::new("scenario_name", scenario_name),
            KeyValue::new("session_id", session_id),
            KeyValue::new("framework", "cleanroom"),
        ];
        self.metrics
            .scenario_execution_time
            .record(duration.as_secs_f64(), &attributes);
    }

    /// Set active containers count
    pub fn set_active_containers(&self, count: u64) {
        let attributes = vec![KeyValue::new("framework", "cleanroom")];
        self.metrics.active_containers.record(count, &attributes);
    }

    /// Set active sessions count
    pub fn set_active_sessions(&self, count: u64) {
        let attributes = vec![KeyValue::new("framework", "cleanroom")];
        self.metrics.active_sessions.record(count, &attributes);
    }

    /// Get the meter
    pub fn meter(&self) -> &Meter {
        &self.meter
    }

    /// Get the metrics
    pub fn metrics(&self) -> &CleanroomMetrics {
        &self.metrics
    }

    /// Check if metrics are enabled
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    /// Shutdown the metrics manager
    pub async fn shutdown(&self) -> Result<()> {
        self.meter_provider.shutdown();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_metrics_manager_creation() {
        let config = MetricsConfig::default();
        let manager = OtelMetricsManager::new(config).await;
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_container_metrics() {
        let config = MetricsConfig::default();
        let manager = OtelMetricsManager::new(config).await.unwrap();
        
        manager.record_container_operation("start", "test-container");
        manager.record_container_startup_time(Duration::from_secs(5), "test-container");
        manager.record_container_memory_usage(1024.0, "test-container");
        manager.record_container_cpu_usage(50.0, "test-container");
    }

    #[tokio::test]
    async fn test_test_metrics() {
        let config = MetricsConfig::default();
        let manager = OtelMetricsManager::new(config).await.unwrap();
        
        let session_id = "test-session";
        manager.record_test_execution("integration_test", session_id);
        manager.record_test_execution_time(Duration::from_secs(10), "integration_test", session_id);
        manager.record_test_success("integration_test", session_id);
    }

    #[tokio::test]
    async fn test_scenario_metrics() {
        let config = MetricsConfig::default();
        let manager = OtelMetricsManager::new(config).await.unwrap();
        
        let session_id = "test-session";
        manager.record_scenario_execution("deployment", session_id);
        manager.record_scenario_execution_time(Duration::from_secs(30), "deployment", session_id);
    }

    #[tokio::test]
    async fn test_gauge_metrics() {
        let config = MetricsConfig::default();
        let manager = OtelMetricsManager::new(config).await.unwrap();
        
        manager.set_active_containers(5);
        manager.set_active_sessions(3);
    }
}
