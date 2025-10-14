//! OpenTelemetry logging implementation for cleanroom testing
//!
//! This module provides structured logging capabilities using OpenTelemetry
//! with support for multiple log levels and structured data.

use crate::error::Result;
use opentelemetry::{
    global,
    logs::{LogRecord, Logger, LoggerProvider, Severity},
    Key, KeyValue, Value,
};
use opentelemetry_sdk::{
    logs::{LoggerProvider as SdkLoggerProvider, LogRecordProcessor, LoggerProviderBuilder},
    Resource,
};
use opentelemetry_stdout::LogExporter as StdoutLogExporter;
use std::sync::Arc;
use std::time::SystemTime;

/// OpenTelemetry logging manager
#[derive(Debug, Clone)]
pub struct OtelLoggingManager {
    /// Logger provider
    logger_provider: Arc<SdkLoggerProvider>,
    /// Logger
    logger: Logger,
    /// Configuration
    config: LoggingConfig,
}

/// Logging configuration
#[derive(Debug, Clone)]
pub struct LoggingConfig {
    /// Enable logging
    pub enabled: bool,
    /// Service name
    pub service_name: String,
    /// Service version
    pub service_version: String,
    /// Environment
    pub environment: String,
    /// Minimum log level
    pub min_level: LogLevel,
    /// Include trace context
    pub include_trace_context: bool,
    /// Include span context
    pub include_span_context: bool,
}

/// Log levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    /// Trace level
    Trace,
    /// Debug level
    Debug,
    /// Info level
    Info,
    /// Warn level
    Warn,
    /// Error level
    Error,
    /// Fatal level
    Fatal,
}

impl From<LogLevel> for Severity {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => Severity::Trace,
            LogLevel::Debug => Severity::Debug,
            LogLevel::Info => Severity::Info,
            LogLevel::Warn => Severity::Warn,
            LogLevel::Error => Severity::Error,
            LogLevel::Fatal => Severity::Fatal,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            service_name: "cleanroom".to_string(),
            service_version: env!("CARGO_PKG_VERSION").to_string(),
            environment: "development".to_string(),
            min_level: LogLevel::Info,
            include_trace_context: true,
            include_span_context: true,
        }
    }
}

impl OtelLoggingManager {
    /// Create a new OpenTelemetry logging manager
    pub async fn new(config: LoggingConfig) -> Result<Self> {
        if !config.enabled {
            let logger = global::logger("cleanroom");
            return Ok(Self {
                logger_provider: Arc::new(SdkLoggerProvider::builder().build()),
                logger,
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

        let stdout_exporter = StdoutLogExporter::default();
        let processor = LogRecordProcessor::builder(stdout_exporter, opentelemetry_sdk::runtime::Tokio)
            .build();

        let logger_provider = Arc::new(
            SdkLoggerProvider::builder()
                .with_resource(resource)
                .with_processor(processor)
                .build()
        );

        let logger = logger_provider.logger("cleanroom");

        // Set as global logger provider
        global::set_logger_provider(logger_provider.clone());

        Ok(Self {
            logger_provider,
            logger,
            config,
        })
    }

    /// Log a message
    pub fn log(&self, level: LogLevel, message: &str, attributes: Vec<KeyValue>) {
        if level < self.config.min_level {
            return;
        }

        let mut log_record = LogRecord::builder()
            .with_severity_text(level.to_string())
            .with_severity_number(level.into())
            .with_body(message.to_string())
            .with_timestamp(SystemTime::now())
            .with_attributes(attributes)
            .build();

        // Add cleanroom-specific attributes
        log_record.add_attribute(KeyValue::new("cleanroom.framework", "clnrm"));
        log_record.add_attribute(KeyValue::new("cleanroom.version", env!("CARGO_PKG_VERSION")));

        self.logger.emit(log_record);
    }

    /// Log trace message
    pub fn trace(&self, message: &str, attributes: Vec<KeyValue>) {
        self.log(LogLevel::Trace, message, attributes);
    }

    /// Log debug message
    pub fn debug(&self, message: &str, attributes: Vec<KeyValue>) {
        self.log(LogLevel::Debug, message, attributes);
    }

    /// Log info message
    pub fn info(&self, message: &str, attributes: Vec<KeyValue>) {
        self.log(LogLevel::Info, message, attributes);
    }

    /// Log warn message
    pub fn warn(&self, message: &str, attributes: Vec<KeyValue>) {
        self.log(LogLevel::Warn, message, attributes);
    }

    /// Log error message
    pub fn error(&self, message: &str, attributes: Vec<KeyValue>) {
        self.log(LogLevel::Error, message, attributes);
    }

    /// Log fatal message
    pub fn fatal(&self, message: &str, attributes: Vec<KeyValue>) {
        self.log(LogLevel::Fatal, message, attributes);
    }

    /// Log container operation
    pub fn log_container_operation(&self, level: LogLevel, operation: &str, container_id: &str, message: &str) {
        let attributes = vec![
            KeyValue::new("cleanroom.operation", operation),
            KeyValue::new("cleanroom.container.id", container_id),
            KeyValue::new("cleanroom.component", "container"),
        ];
        self.log(level, message, attributes);
    }

    /// Log test operation
    pub fn log_test_operation(&self, level: LogLevel, test_name: &str, session_id: &str, message: &str) {
        let attributes = vec![
            KeyValue::new("cleanroom.test.name", test_name),
            KeyValue::new("cleanroom.session.id", session_id),
            KeyValue::new("cleanroom.component", "test"),
        ];
        self.log(level, message, attributes);
    }

    /// Log scenario operation
    pub fn log_scenario_operation(&self, level: LogLevel, scenario_name: &str, step_name: &str, message: &str) {
        let attributes = vec![
            KeyValue::new("cleanroom.scenario.name", scenario_name),
            KeyValue::new("cleanroom.step.name", step_name),
            KeyValue::new("cleanroom.component", "scenario"),
        ];
        self.log(level, message, attributes);
    }

    /// Log error with context
    pub fn log_error(&self, error: &dyn std::error::Error, context: Vec<KeyValue>) {
        let mut attributes = context;
        attributes.push(KeyValue::new("error.type", error.to_string()));
        attributes.push(KeyValue::new("cleanroom.component", "error"));
        
        self.error(&format!("Error: {}", error), attributes);
    }

    /// Log performance metrics
    pub fn log_performance(&self, operation: &str, duration: std::time::Duration, additional_attrs: Vec<KeyValue>) {
        let mut attributes = additional_attrs;
        attributes.push(KeyValue::new("cleanroom.operation", operation));
        attributes.push(KeyValue::new("cleanroom.duration_ms", duration.as_millis() as i64));
        attributes.push(KeyValue::new("cleanroom.component", "performance"));
        
        self.info(&format!("Performance: {} took {}ms", operation, duration.as_millis()), attributes);
    }

    /// Get the logger
    pub fn logger(&self) -> &Logger {
        &self.logger
    }

    /// Check if logging is enabled
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    /// Shutdown the logging manager
    pub async fn shutdown(&self) -> Result<()> {
        self.logger_provider.shutdown();
        Ok(())
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "TRACE"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Fatal => write!(f, "FATAL"),
        }
    }
}

/// Log attributes for cleanroom operations
pub struct CleanroomLogAttributes;

impl CleanroomLogAttributes {
    /// Container operation attributes
    pub fn container_operation(operation: &str, container_id: &str) -> Vec<KeyValue> {
        vec![
            KeyValue::new("cleanroom.operation", operation),
            KeyValue::new("cleanroom.container.id", container_id),
            KeyValue::new("cleanroom.component", "container"),
        ]
    }

    /// Test operation attributes
    pub fn test_operation(test_name: &str, session_id: &str) -> Vec<KeyValue> {
        vec![
            KeyValue::new("cleanroom.test.name", test_name),
            KeyValue::new("cleanroom.session.id", session_id),
            KeyValue::new("cleanroom.component", "test"),
        ]
    }

    /// Scenario operation attributes
    pub fn scenario_operation(scenario_name: &str, step_name: &str) -> Vec<KeyValue> {
        vec![
            KeyValue::new("cleanroom.scenario.name", scenario_name),
            KeyValue::new("cleanroom.step.name", step_name),
            KeyValue::new("cleanroom.component", "scenario"),
        ]
    }

    /// Performance attributes
    pub fn performance(operation: &str, duration: std::time::Duration) -> Vec<KeyValue> {
        vec![
            KeyValue::new("cleanroom.operation", operation),
            KeyValue::new("cleanroom.duration_ms", duration.as_millis() as i64),
            KeyValue::new("cleanroom.component", "performance"),
        ]
    }

    /// Error attributes
    pub fn error(error_type: &str, error_message: &str) -> Vec<KeyValue> {
        vec![
            KeyValue::new("error.type", error_type),
            KeyValue::new("error.message", error_message),
            KeyValue::new("cleanroom.component", "error"),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_logging_manager_creation() {
        let config = LoggingConfig::default();
        let manager = OtelLoggingManager::new(config).await;
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_log_levels() {
        let config = LoggingConfig::default();
        let manager = OtelLoggingManager::new(config).await.unwrap();
        
        manager.trace("Trace message", vec![]);
        manager.debug("Debug message", vec![]);
        manager.info("Info message", vec![]);
        manager.warn("Warn message", vec![]);
        manager.error("Error message", vec![]);
        manager.fatal("Fatal message", vec![]);
    }

    #[tokio::test]
    async fn test_container_logging() {
        let config = LoggingConfig::default();
        let manager = OtelLoggingManager::new(config).await.unwrap();
        
        manager.log_container_operation(LogLevel::Info, "start", "test-container", "Container started successfully");
    }

    #[tokio::test]
    async fn test_test_logging() {
        let config = LoggingConfig::default();
        let manager = OtelLoggingManager::new(config).await.unwrap();
        
        manager.log_test_operation(LogLevel::Info, "integration_test", "test-session", "Test completed successfully");
    }

    #[tokio::test]
    async fn test_scenario_logging() {
        let config = LoggingConfig::default();
        let manager = OtelLoggingManager::new(config).await.unwrap();
        
        manager.log_scenario_operation(LogLevel::Info, "deployment", "setup", "Scenario step completed");
    }

    #[tokio::test]
    async fn test_performance_logging() {
        let config = LoggingConfig::default();
        let manager = OtelLoggingManager::new(config).await.unwrap();
        
        manager.log_performance("container_startup", std::time::Duration::from_secs(5), vec![]);
    }

    #[test]
    fn test_log_level_display() {
        assert_eq!(LogLevel::Trace.to_string(), "TRACE");
        assert_eq!(LogLevel::Debug.to_string(), "DEBUG");
        assert_eq!(LogLevel::Info.to_string(), "INFO");
        assert_eq!(LogLevel::Warn.to_string(), "WARN");
        assert_eq!(LogLevel::Error.to_string(), "ERROR");
        assert_eq!(LogLevel::Fatal.to_string(), "FATAL");
    }

    #[test]
    fn test_log_attributes() {
        let attrs = CleanroomLogAttributes::container_operation("start", "test-container");
        assert_eq!(attrs.len(), 3);
        
        let test_attrs = CleanroomLogAttributes::test_operation("test", "session");
        assert_eq!(test_attrs.len(), 3);
        
        let scenario_attrs = CleanroomLogAttributes::scenario_operation("deploy", "setup");
        assert_eq!(scenario_attrs.len(), 3);
    }
}
