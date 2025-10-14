# Cleanroom Operations Guide

Essential operations guide for the Cleanroom Testing Framework - 80% of operational knowledge in 20% of the documentation.

## Quick Start (80% of operations knowledge)

### Daily Operations (80% of operational tasks)
```bash
# Check system health
cargo run --bin cleanroom status

# Monitor resource usage
cargo run --bin cleanroom metrics

# View recent logs
cargo run --bin cleanroom logs --tail=50

# Run health checks
cargo test --test integration_tests -- --ignored
```

### Emergency Procedures (20% of incidents but 80% critical)
```bash
# Emergency stop (containers and tests)
cargo run --bin cleanroom emergency-stop

# Restart services
cargo run --bin cleanroom restart

# Check for stuck containers
docker ps -a | grep cleanroom
```

## Monitoring (80% of operational visibility)

### Essential Metrics (80% of what you monitor)
```bash
# System health
cargo run --bin cleanroom status

# Resource usage
cargo run --bin cleanroom metrics

# Test execution status
cargo run --bin cleanroom test-status

# Container health
cargo run --bin cleanroom container-status
```

### Alert Thresholds (80% of operational alerts)
```yaml
# Alert if any of these exceed thresholds
cpu_usage_percent: 80
memory_usage_mb: 4096
disk_usage_mb: 10240
container_count: 50
test_failure_rate: 10  # percent
test_execution_time: 300  # seconds
```

## Troubleshooting (80% of operational issues)

### Common Issues (80% of problems you encounter)

#### Docker Issues (Most common - 80% of container problems)
```bash
# Check Docker status
docker --version && docker ps

# Restart Docker daemon
sudo systemctl restart docker

# Clean up stuck containers
docker container prune -f

# Check Docker logs
journalctl -u docker --since today
```

#### Test Failures (Common - 80% of test issues)
```bash
# Run tests with verbose output
cargo test -- --nocapture

# Run specific failing test
cargo test test_name -- --nocapture

# Debug with backtrace
RUST_BACKTRACE=1 cargo test

# Check test environment
cargo run --bin cleanroom env-check
```

#### Performance Issues (Less common but critical)
```bash
# Profile performance
cargo install cargo-flamegraph
cargo flamegraph --bin cleanroom

# Monitor resource usage
htop  # or your preferred monitor

# Check for memory leaks
cargo run --bin cleanroom memory-profile
```

#### Network Issues (Uncommon but blocking)
```bash
# Check network connectivity
curl -I https://www.docker.com/

# Check DNS resolution
nslookup google.com

# Check firewall rules
sudo iptables -L
```

## Production Deployment (80% of deployment scenarios)

### Basic Deployment (80% of deployments)
```bash
# Build for production
cargo build --release

# Install binary
sudo cp target/release/cleanroom /usr/local/bin/

# Create service file
sudo tee /etc/systemd/system/cleanroom.service <<EOF
[Unit]
Description=Cleanroom Testing Framework
After=docker.service

[Service]
ExecStart=/usr/local/bin/cleanroom daemon
Restart=always
User=cleanroom

[Install]
WantedBy=multi-user.target
EOF

# Enable and start service
sudo systemctl enable cleanroom
sudo systemctl start cleanroom
```

### Configuration (80% of configuration needs)
```toml
# cleanroom.toml - Production configuration
[security]
enable_network_isolation = true
enable_filesystem_isolation = true
enable_process_isolation = true
security_level = "standard"

[resources]
max_container_count = 20
max_memory_mb = 4096
max_cpu_percent = 80
max_disk_mb = 10240

[monitoring]
enable_metrics = true
metrics_interval = 30
enable_alerting = true

[logging]
level = "info"
format = "json"
enable_structured_logging = true
```

### Health Checks (80% of operational monitoring)
```bash
# Health check endpoint
curl http://localhost:8080/health

# Expected response
{
  "status": "healthy",
  "version": "1.0.0",
  "uptime_seconds": 3600,
  "containers_active": 5,
  "tests_running": 0
}
```

## Security Operations (80% of security tasks)

### Essential Security Monitoring (80% of security checks)
```bash
# Check security events
cargo run --bin cleanroom security-audit

# Monitor policy violations
cargo run --bin cleanroom policy-violations

# Check data redaction
cargo run --bin cleanroom redaction-status

# Verify security isolation
cargo run --bin cleanroom isolation-check
```

### Security Incidents (80% of incident response)
```bash
# Isolate compromised environment
cargo run --bin cleanroom isolate-environment <env_id>

# Stop all containers
cargo run --bin cleanroom emergency-stop

# Enable enhanced logging
cargo run --bin cleanroom enable-audit-logging

# Generate security report
cargo run --bin cleanroom security-report
```

## Maintenance (80% of maintenance tasks)

### Daily Maintenance (80% of routine tasks)
```bash
# Clean up old containers
cargo run --bin cleanroom cleanup-containers --older-than=24h

# Clean up old logs
cargo run --bin cleanroom cleanup-logs --older-than=7d

# Update container images
cargo run --bin cleanroom update-images

# Run health checks
cargo run --bin cleanroom health-check
```

### Weekly Maintenance (20% of tasks but important)
```bash
# Run full test suite
cargo test

# Update dependencies
cargo update

# Generate performance report
cargo run --bin cleanroom performance-report

# Check for vulnerabilities
cargo audit
```

## Performance Tuning (80% of performance improvements)

### Essential Performance Settings (80% of performance gains)
```toml
# Performance-optimized configuration
[performance]
container_pool_size = 20
prewarm_containers = true
enable_connection_pooling = true
metrics_interval = 10

[resources]
max_container_count = 50
max_memory_mb = 8192
max_cpu_percent = 90
max_disk_mb = 51200
```

### Performance Monitoring (80% of performance visibility)
```bash
# Monitor resource usage
cargo run --bin cleanroom resource-monitor

# Check container performance
cargo run --bin cleanroom container-performance

# Analyze test performance
cargo run --bin cleanroom test-performance

# Generate performance report
cargo run --bin cleanroom performance-report
```

## Disaster Recovery (80% of recovery scenarios)

### Backup Procedures (80% of backup operations)
```bash
# Backup configuration and data
cargo run --bin cleanroom backup create --name="daily-$(date +%Y%m%d)"

# List available backups
cargo run --bin cleanroom backup list

# Verify backup integrity
cargo run --bin cleanroom backup verify latest
```

### Recovery Procedures (80% of recovery scenarios)
```bash
# Restore from backup
cargo run --bin cleanroom restore latest

# Verify system integrity
cargo run --bin cleanroom integrity-check

# Test critical functionality
cargo run --bin cleanroom smoke-test

# Resume normal operations
cargo run --bin cleanroom resume-operations
```

## Scaling (80% of scaling considerations)

### Horizontal Scaling (80% of scaling needs)
```bash
# Scale container pool
cargo run --bin cleanroom scale containers --count=50

# Scale test execution
cargo run --bin cleanroom scale tests --parallel=10

# Scale monitoring
cargo run --bin cleanroom scale monitoring --replicas=3
```

### Vertical Scaling (20% of scaling but important)
```bash
# Increase resource limits
cargo run --bin cleanroom scale resources --memory=16384 --cpu=16

# Optimize container resources
cargo run --bin cleanroom optimize containers

# Tune performance settings
cargo run --bin cleanroom tune performance
```

## Compliance (80% of compliance requirements)

### Essential Compliance Monitoring (80% of compliance checks)
```bash
# Generate compliance report
cargo run --bin cleanroom compliance-report

# Check audit logs
cargo run --bin cleanroom audit-logs

# Verify data retention
cargo run --bin cleanroom retention-check

# Generate security assessment
cargo run --bin cleanroom security-assessment
```

## Best Practices (80% of operational excellence)

### 1. Monitoring (80% of operational visibility)
```bash
# Set up essential monitoring
cargo run --bin cleanroom setup-monitoring

# Configure alerting
cargo run --bin cleanroom configure-alerts

# Enable comprehensive logging
cargo run --bin cleanroom enable-detailed-logging
```

### 2. Security (80% of security posture)
```bash
# Enable security features
cargo run --bin cleanroom enable-security

# Configure access controls
cargo run --bin cleanroom configure-access

# Set up security monitoring
cargo run --bin cleanroom setup-security-monitoring
```

### 3. Performance (80% of performance optimization)
```bash
# Optimize container usage
cargo run --bin cleanroom optimize-containers

# Tune resource allocation
cargo run --bin cleanroom tune-resources

# Set up performance monitoring
cargo run --bin cleanroom setup-performance-monitoring
```

### 4. Maintenance (80% of system reliability)
```bash
# Set up automated cleanup
cargo run --bin cleanroom setup-cleanup

# Configure backup automation
cargo run --bin cleanroom setup-backups

# Set up health monitoring
cargo run --bin cleanroom setup-health-monitoring
```

## Emergency Procedures (Critical 20% of operations)

### System Failure (80% of emergency scenarios)
```bash
# Emergency stop
cargo run --bin cleanroom emergency-stop

# Isolate affected components
cargo run --bin cleanroom isolate-failure

# Activate backup systems
cargo run --bin cleanroom activate-backup

# Restore from backup
cargo run --bin cleanroom restore-from-backup
```

### Security Incident (20% of emergencies but critical)
```bash
# Isolate compromised systems
cargo run --bin cleanroom isolate-security-incident

# Enable maximum security
cargo run --bin cleanroom enable-maximum-security

# Generate security report
cargo run --bin cleanroom generate-security-report

# Notify security team
# (Manual process - contact security team immediately)
```

## Tools & Commands (80% of operational tools)

### Essential Commands (80% of CLI usage)
```bash
# System management
cleanroom status                    # System health
cleanroom start                     # Start services
cleanroom stop                      # Stop services
cleanroom restart                   # Restart services

# Monitoring
cleanroom metrics                   # View metrics
cleanroom logs                      # View logs
cleanroom health                    # Health check

# Container management
cleanroom containers list           # List containers
cleanroom containers cleanup        # Clean up containers
cleanroom containers optimize       # Optimize containers

# Test management
cleanroom tests run                 # Run tests
cleanroom tests status              # Test status
cleanroom tests cleanup             # Clean up test data

# Security
cleanroom security status           # Security status
cleanroom security audit            # Security audit
cleanroom security harden           # Harden security

# Performance
cleanroom performance profile       # Performance profile
cleanroom performance optimize      # Optimize performance
cleanroom performance report        # Performance report
```

## Configuration (80% of configuration needs)

### Production Configuration (80% of production setups)
```toml
# cleanroom-production.toml
[security]
enable_network_isolation = true
enable_filesystem_isolation = true
enable_process_isolation = true
security_level = "strict"

[resources]
max_container_count = 100
max_memory_mb = 16384
max_cpu_percent = 90
max_disk_mb = 102400

[monitoring]
enable_metrics = true
enable_alerting = true
enable_performance_monitoring = true
metrics_retention_days = 30

[logging]
level = "info"
enable_structured_logging = true
enable_audit_logging = true
log_retention_days = 90

[backup]
enable_automatic_backups = true
backup_interval_hours = 24
backup_retention_days = 30
```

## Resources (80% of operational resources)

### Essential Documentation (80% of what you need)
- **API Reference**: [../api/README.md](../api/README.md)
- **Developer Guide**: [../development/README.md](../development/README.md)
- **Architecture**: [../architecture-overview.md](../architecture-overview.md)

### Essential Tools (80% of operational tools)
- **Docker** - Container runtime
- **Systemd** - Service management
- **Prometheus** - Metrics collection
- **Grafana** - Dashboards
- **ELK Stack** - Log aggregation

### Community (80% of operational support)
- **GitHub Issues** - Operational issues and bugs
- **GitHub Discussions** - Operational questions
- **Discord** - Real-time operational support

---

*This operations guide follows 80/20 principles: 80% of operational knowledge in 20% of the documentation complexity. For advanced operations, see the architecture documentation.*
