//! OpenTelemetry tracing implementation for cleanroom testing
//!
//! This module provides distributed tracing capabilities using OpenTelemetry
//! with support for multiple exporters and span management.

use crate::error::Result;
use opentelemetry::{
    global,
    trace::{Span, SpanKind, Status, Tracer, TracerProvider},
    Key, KeyValue, Value,
};
use opentelemetry_sdk::{
    trace::{Config, TracerProvider as SdkTracerProvider},
    Resource,
};
use opentelemetry_stdout::SpanExporter as StdoutSpanExporter;
use opentelemetry_jaeger::new_agent_pipeline;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

/// OpenTelemetry tracing manager
#[derive(Debug, Clone)]
pub struct OtelTracingManager {
    /// Tracer provider
    tracer_provider: Arc<SdkTracerProvider>,
    /// Tracer
    tracer: Tracer,
    /// Configuration
    config: TracingConfig,
}

/// Tracing configuration
#[derive(Debug, Clone)]
pub struct TracingConfig {
    /// Enable tracing
    pub enabled: bool,
    /// Service name
    pub service_name: String,
    /// Service version
    pub service_version: String,
    /// Environment
    pub environment: String,
    /// Sampling rate (0.0 to 1.0)
    pub sampling_rate: f64,
    /// Exporters to use
    pub exporters: Vec<TracingExporter>,
    /// Maximum span events
    pub max_span_events: u32,
    /// Maximum span links
    pub max_span_links: u32,
}

/// Tracing exporters
#[derive(Debug, Clone)]
pub enum TracingExporter {
    /// Console exporter for debugging
    Console,
    /// Jaeger exporter for distributed tracing
    Jaeger {
        endpoint: String,
        service_name: String,
    },
    /// OTLP exporter for OpenTelemetry Collector
    Otlp {
        endpoint: String,
        headers: Vec<(String, String)>,
    },
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            service_name: "cleanroom".to_string(),
            service_version: env!("CARGO_PKG_VERSION").to_string(),
            environment: "development".to_string(),
            sampling_rate: 1.0,
            exporters: vec![TracingExporter::Console],
            max_span_events: 128,
            max_span_links: 128,
        }
    }
}

impl OtelTracingManager {
    /// Create a new OpenTelemetry tracing manager
    pub async fn new(config: TracingConfig) -> Result<Self> {
        if !config.enabled {
            return Ok(Self {
                tracer_provider: Arc::new(SdkTracerProvider::builder().build()),
                tracer: global::tracer("cleanroom"),
                config,
            });
        }

        let resource = Resource::new(vec![
            KeyValue::new("service.name", config.service_name.clone()),
            KeyValue::new("service.version", config.service_version.clone()),
            KeyValue::new("service.environment", config.environment.clone()),
            KeyValue::new("cleanroom.framework", "clnrm"),
            KeyValue::new("cleanroom.version", env!("CARGO_PKG_VERSION")),
        ]);

        let mut builder = SdkTracerProvider::builder().with_resource(resource);

        // Add exporters based on configuration
        for exporter in &config.exporters {
            match exporter {
                TracingExporter::Console => {
                    let stdout_exporter = StdoutSpanExporter::default();
                    builder = builder.with_batch_exporter(stdout_exporter, opentelemetry_sdk::runtime::Tokio);
                }
                TracingExporter::Jaeger { endpoint, service_name } => {
                    let jaeger_exporter = new_agent_pipeline()
                        .with_endpoint(endpoint)
                        .with_service_name(service_name)
                        .install_batch(opentelemetry_sdk::runtime::Tokio)?;
                    // Note: Jaeger exporter is installed globally, not added to builder
                }
                TracingExporter::Otlp { endpoint, headers } => {
                    // OTLP exporter would be configured here
                    // For now, we'll use console as fallback
                    let stdout_exporter = StdoutSpanExporter::default();
                    builder = builder.with_batch_exporter(stdout_exporter, opentelemetry_sdk::runtime::Tokio);
                }
            }
        }

        let tracer_provider = Arc::new(builder.build());
        let tracer = tracer_provider.tracer("cleanroom");

        // Set as global tracer provider
        global::set_tracer_provider(tracer_provider.clone());

        Ok(Self {
            tracer_provider,
            tracer,
            config,
        })
    }

    /// Create a new span
    pub fn start_span(&self, name: &str) -> Span {
        self.tracer
            .span_builder(name)
            .with_kind(SpanKind::Internal)
            .start(&self.tracer)
    }

    /// Create a new span with attributes
    pub fn start_span_with_attributes(&self, name: &str, attributes: Vec<KeyValue>) -> Span {
        self.tracer
            .span_builder(name)
            .with_kind(SpanKind::Internal)
            .with_attributes(attributes)
            .start(&self.tracer)
    }

    /// Create a new span for container operations
    pub fn start_container_span(&self, operation: &str, container_id: &str) -> Span {
        let attributes = vec![
            KeyValue::new("cleanroom.operation", operation),
            KeyValue::new("cleanroom.container.id", container_id),
            KeyValue::new("cleanroom.framework", "clnrm"),
        ];

        self.start_span_with_attributes(&format!("cleanroom.container.{}", operation), attributes)
    }

    /// Create a new span for test operations
    pub fn start_test_span(&self, test_name: &str, session_id: &Uuid) -> Span {
        let attributes = vec![
            KeyValue::new("cleanroom.test.name", test_name),
            KeyValue::new("cleanroom.session.id", session_id.to_string()),
            KeyValue::new("cleanroom.framework", "clnrm"),
        ];

        self.start_span_with_attributes(&format!("cleanroom.test.{}", test_name), attributes)
    }

    /// Create a new span for scenario operations
    pub fn start_scenario_span(&self, scenario_name: &str, step_name: &str) -> Span {
        let attributes = vec![
            KeyValue::new("cleanroom.scenario.name", scenario_name),
            KeyValue::new("cleanroom.step.name", step_name),
            KeyValue::new("cleanroom.framework", "clnrm"),
        ];

        self.start_span_with_attributes(&format!("cleanroom.scenario.{}.{}", scenario_name, step_name), attributes)
    }

    /// Add event to current span
    pub fn add_span_event(&self, span: &Span, name: &str, attributes: Vec<KeyValue>) {
        span.add_event(name, attributes);
    }

    /// Add error to span
    pub fn add_span_error(&self, span: &Span, error: &dyn std::error::Error) {
        span.record_error(error);
        span.set_status(Status::error(error.to_string()));
    }

    /// Set span status
    pub fn set_span_status(&self, span: &Span, status: SpanStatus) {
        match status {
            SpanStatus::Ok => span.set_status(Status::Ok),
            SpanStatus::Error(message) => span.set_status(Status::error(message)),
        }
    }

    /// Get the tracer
    pub fn tracer(&self) -> &Tracer {
        &self.tracer
    }

    /// Check if tracing is enabled
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    /// Shutdown the tracing manager
    pub async fn shutdown(&self) -> Result<()> {
        self.tracer_provider.shutdown();
        Ok(())
    }
}

/// Span status
#[derive(Debug, Clone)]
pub enum SpanStatus {
    /// Success
    Ok,
    /// Error with message
    Error(String),
}

/// Span attributes for cleanroom operations
pub struct CleanroomSpanAttributes;

impl CleanroomSpanAttributes {
    /// Container operation attributes
    pub fn container_operation(operation: &str, container_id: &str) -> Vec<KeyValue> {
        vec![
            KeyValue::new("cleanroom.operation", operation),
            KeyValue::new("cleanroom.container.id", container_id),
            KeyValue::new("cleanroom.framework", "clnrm"),
        ]
    }

    /// Test operation attributes
    pub fn test_operation(test_name: &str, session_id: &Uuid) -> Vec<KeyValue> {
        vec![
            KeyValue::new("cleanroom.test.name", test_name),
            KeyValue::new("cleanroom.session.id", session_id.to_string()),
            KeyValue::new("cleanroom.framework", "clnrm"),
        ]
    }

    /// Scenario operation attributes
    pub fn scenario_operation(scenario_name: &str, step_name: &str) -> Vec<KeyValue> {
        vec![
            KeyValue::new("cleanroom.scenario.name", scenario_name),
            KeyValue::new("cleanroom.step.name", step_name),
            KeyValue::new("cleanroom.framework", "clnrm"),
        ]
    }

    /// Performance attributes
    pub fn performance(duration: Duration, resource_usage: &str) -> Vec<KeyValue> {
        vec![
            KeyValue::new("cleanroom.duration_ms", duration.as_millis() as i64),
            KeyValue::new("cleanroom.resource_usage", resource_usage),
        ]
    }

    /// Error attributes
    pub fn error(error_type: &str, error_message: &str) -> Vec<KeyValue> {
        vec![
            KeyValue::new("cleanroom.error.type", error_type),
            KeyValue::new("cleanroom.error.message", error_message),
            KeyValue::new("cleanroom.error", true),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tracing_manager_creation() {
        let config = TracingConfig::default();
        let manager = OtelTracingManager::new(config).await;
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_span_creation() {
        let config = TracingConfig::default();
        let manager = OtelTracingManager::new(config).await.unwrap();
        
        let span = manager.start_span("test_span");
        assert_eq!(span.name(), "test_span");
        span.end();
    }

    #[tokio::test]
    async fn test_container_span() {
        let config = TracingConfig::default();
        let manager = OtelTracingManager::new(config).await.unwrap();
        
        let span = manager.start_container_span("start", "container-123");
        assert!(span.name().contains("cleanroom.container.start"));
        span.end();
    }

    #[tokio::test]
    async fn test_test_span() {
        let config = TracingConfig::default();
        let manager = OtelTracingManager::new(config).await.unwrap();
        
        let session_id = Uuid::new_v4();
        let span = manager.start_test_span("integration_test", &session_id);
        assert!(span.name().contains("cleanroom.test.integration_test"));
        span.end();
    }

    #[tokio::test]
    async fn test_scenario_span() {
        let config = TracingConfig::default();
        let manager = OtelTracingManager::new(config).await.unwrap();
        
        let span = manager.start_scenario_span("deployment", "setup");
        assert!(span.name().contains("cleanroom.scenario.deployment.setup"));
        span.end();
    }

    #[test]
    fn test_span_attributes() {
        let attrs = CleanroomSpanAttributes::container_operation("start", "test-container");
        assert_eq!(attrs.len(), 3);
        
        let session_id = Uuid::new_v4();
        let test_attrs = CleanroomSpanAttributes::test_operation("test", &session_id);
        assert_eq!(test_attrs.len(), 3);
        
        let scenario_attrs = CleanroomSpanAttributes::scenario_operation("deploy", "setup");
        assert_eq!(scenario_attrs.len(), 3);
    }
}
