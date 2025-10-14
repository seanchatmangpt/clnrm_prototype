# OpenTelemetry Migration Guide

This guide explains how to migrate from the custom tracing implementation to OpenTelemetry (OTel) for better observability, industry-standard compatibility, and enhanced debugging capabilities.

## Overview

The custom tracing implementation in `src/tracing.rs` is being replaced with OpenTelemetry, which provides:

- **Industry Standard**: OpenTelemetry is the industry standard for observability
- **Better Integration**: Works with Jaeger, Prometheus, Grafana, and other tools
- **Rich Ecosystem**: Extensive exporter support and community tools
- **Performance**: Optimized for production workloads
- **Distributed Tracing**: Better support for distributed systems

## Migration Benefits

### Before (Custom Tracing)
```rust
use clnrm::TracingManager;

let tracing = TracingManager::new(session_id);
tracing.start_span("test_operation".to_string(), None).await?;
tracing.log(LogLevel::Info, "Test started".to_string(), None, HashMap::new(), HashMap::new()).await?;
```

### After (OpenTelemetry)
```rust
use clnrm::{OtelManager, OtelConfig};

let otel = OtelManager::new(OtelConfig::default()).await?;
let span = otel.tracing.start_test_span("test_operation", &session_id);
otel.logging.info("Test started", vec![]);
```

## Step-by-Step Migration

### 1. Update Dependencies

The OpenTelemetry dependencies are already added to `Cargo.toml`:

```toml
opentelemetry = "0.25"
opentelemetry-sdk = "0.25"
opentelemetry-stdout = "0.25"
opentelemetry-jaeger = "0.25"
opentelemetry-otlp = "0.25"
opentelemetry-prometheus = "0.25"
```

### 2. Replace TracingManager with OtelManager

#### Old Code
```rust
use clnrm::TracingManager;

let tracing = TracingManager::new(session_id);
```

#### New Code
```rust
use clnrm::{OtelManager, OtelConfig};

let config = OtelConfig {
    service_name: "cleanroom".to_string(),
    service_version: "0.2.0".to_string(),
    environment: "development".to_string(),
    ..OtelConfig::default()
};
let otel = OtelManager::new(config).await?;
```

### 3. Replace Span Management

#### Old Code
```rust
let span_id = tracing.start_span("container_operation".to_string(), None).await?;
tracing.end_span("container_operation", SpanStatus::Ok).await?;
```

#### New Code
```rust
let span = otel.tracing.start_container_span("start", container_id);
// Span automatically ends when dropped
// Or manually: span.end();
```

### 4. Replace Logging

#### Old Code
```rust
tracing.log(LogLevel::Info, "Container started".to_string(), None, HashMap::new(), HashMap::new()).await?;
```

#### New Code
```rust
otel.logging.log_container_operation(LogLevel::Info, "start", container_id, "Container started");
```

### 5. Replace Metrics

#### Old Code
```rust
tracing.add_metric(Metric {
    name: "container_startup_time".to_string(),
    value: 5.0,
    unit: "seconds".to_string(),
    timestamp: SerializableInstant::now(),
    tags: HashMap::new(),
    metadata: HashMap::new(),
}).await?;
```

#### New Code
```rust
otel.metrics.record_container_startup_time(Duration::from_secs(5), container_id);
```

## Configuration Examples

### Development Configuration
```rust
use clnrm::{OtelConfig, ExportersConfig};

let config = OtelConfig {
    service_name: "cleanroom-dev".to_string(),
    environment: "development".to_string(),
    exporters: ExportersConfig::development(),
    ..OtelConfig::default()
};
```

### Production Configuration
```rust
use clnrm::{OtelConfig, ExportersConfig};

let config = OtelConfig {
    service_name: "cleanroom-prod".to_string(),
    environment: "production".to_string(),
    exporters: ExportersConfig::production(),
    ..OtelConfig::default()
};
```

### Testing Configuration
```rust
use clnrm::{OtelConfig, ExportersConfig};

let config = OtelConfig {
    service_name: "cleanroom-test".to_string(),
    environment: "testing".to_string(),
    exporters: ExportersConfig::testing(),
    ..OtelConfig::default()
};
```

## Advanced Features

### Custom Exporters

```rust
use clnrm::{ExporterBuilder, ExportersConfig};

let exporters = ExporterBuilder::new()
    .with_console(true)
    .with_jaeger_config(JaegerExporterConfig {
        enabled: true,
        agent_endpoint: "http://jaeger:14268/api/traces".to_string(),
        service_name: "cleanroom".to_string(),
        ..JaegerExporterConfig::default()
    })
    .with_prometheus_config(PrometheusExporterConfig {
        enabled: true,
        listen_address: "0.0.0.0:9090".to_string(),
        ..PrometheusExporterConfig::default()
    })
    .build();
```

### Custom Attributes

```rust
use opentelemetry::KeyValue;

let attributes = vec![
    KeyValue::new("cleanroom.operation", "container_start"),
    KeyValue::new("cleanroom.container.id", container_id),
    KeyValue::new("cleanroom.framework", "clnrm"),
];

let span = otel.tracing.start_span_with_attributes("custom_operation", attributes);
```

### Error Handling

```rust
use clnrm::SpanStatus;

let span = otel.tracing.start_container_span("start", container_id);

match container.start().await {
    Ok(_) => {
        otel.tracing.set_span_status(&span, SpanStatus::Ok);
        otel.logging.log_container_operation(LogLevel::Info, "start", container_id, "Container started successfully");
    }
    Err(e) => {
        otel.tracing.add_span_error(&span, &e);
        otel.logging.log_error(&e, vec![
            KeyValue::new("container_id", container_id),
            KeyValue::new("operation", "start"),
        ]);
    }
}
```

## Integration with Existing Code

### CleanroomEnvironment Integration

```rust
use clnrm::{CleanroomEnvironment, CleanroomConfig, OtelManager};

let otel = OtelManager::new_default().await?;
let config = CleanroomConfig::default();
let environment = CleanroomEnvironment::new(config).await?;

// Use OTel for tracing
let span = otel.tracing.start_test_span("integration_test", &environment.session_id());
otel.metrics.record_test_execution("integration_test", &environment.session_id().to_string());
```

### Scenario Integration

```rust
use clnrm::{scenario, OtelManager};

let otel = OtelManager::new_default().await?;

let scenario = scenario("deployment_test")
    .step("setup", ["echo", "setting up"])
    .step("deploy", ["echo", "deploying"])
    .step("verify", ["echo", "verifying"]);

// Trace scenario execution
let span = otel.tracing.start_scenario_span("deployment_test", "setup");
otel.metrics.record_scenario_execution("deployment_test", &session_id.to_string());
```

## Monitoring and Observability

### Jaeger Integration

1. Start Jaeger:
```bash
docker run -d --name jaeger \
  -p 16686:16686 \
  -p 14268:14268 \
  jaegertracing/all-in-one:latest
```

2. Configure Jaeger exporter:
```rust
let config = OtelConfig {
    exporters: ExportersConfig {
        jaeger: JaegerExporterConfig {
            enabled: true,
            agent_endpoint: "http://localhost:14268/api/traces".to_string(),
            ..JaegerExporterConfig::default()
        },
        ..ExportersConfig::default()
    },
    ..OtelConfig::default()
};
```

3. View traces at http://localhost:16686

### Prometheus Integration

1. Configure Prometheus exporter:
```rust
let config = OtelConfig {
    exporters: ExportersConfig {
        prometheus: PrometheusExporterConfig {
            enabled: true,
            listen_address: "0.0.0.0:9090".to_string(),
            ..PrometheusExporterConfig::default()
        },
        ..ExportersConfig::default()
    },
    ..OtelConfig::default()
};
```

2. View metrics at http://localhost:9090/metrics

### Grafana Dashboard

Create a Grafana dashboard with these metrics:
- `cleanroom_container_operations_total`
- `cleanroom_container_startup_time_seconds`
- `cleanroom_test_executions_total`
- `cleanroom_test_successes_total`
- `cleanroom_test_failures_total`
- `cleanroom_active_containers`
- `cleanroom_active_sessions`

## Migration Checklist

- [ ] Update imports from `TracingManager` to `OtelManager`
- [ ] Replace span creation with OTel spans
- [ ] Update logging calls to use OTel logging
- [ ] Replace custom metrics with OTel metrics
- [ ] Configure appropriate exporters for your environment
- [ ] Update tests to use OTel configuration
- [ ] Set up monitoring infrastructure (Jaeger, Prometheus, Grafana)
- [ ] Update documentation and examples
- [ ] Remove old tracing code after migration is complete

## Backward Compatibility

The old `TracingManager` will remain available during the transition period, but it's recommended to migrate to OpenTelemetry for better observability and future-proofing.

## Performance Considerations

OpenTelemetry is optimized for production use:
- **Async Operations**: All operations are async and non-blocking
- **Batch Export**: Metrics and traces are batched for efficient export
- **Sampling**: Configurable sampling rates to reduce overhead
- **Resource Management**: Proper resource cleanup and shutdown

## Support

For questions about the migration:
1. Check the OpenTelemetry documentation: https://opentelemetry.io/docs/
2. Review the examples in the `examples/` directory
3. Check the test files for usage patterns
4. Open an issue in the repository for specific questions
