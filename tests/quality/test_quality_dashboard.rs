//! Test Quality Dashboard and Metrics
//!
//! This module provides comprehensive test quality monitoring and reporting
//! following core team best practices for FAANG-level quality assurance.

use std::collections::HashMap;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

/// Test quality metrics for comprehensive monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestQualityMetrics {
    /// Test execution metrics
    pub execution: ExecutionMetrics,
    /// Test coverage metrics
    pub coverage: CoverageMetrics,
    /// Test reliability metrics
    pub reliability: ReliabilityMetrics,
    /// Test maintainability metrics
    pub maintainability: MaintainabilityMetrics,
    /// Test performance metrics
    pub performance: PerformanceMetrics,
}

/// Test execution metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionMetrics {
    /// Total number of tests
    pub total_tests: usize,
    /// Number of passing tests
    pub passing_tests: usize,
    /// Number of failing tests
    pub failing_tests: usize,
    /// Number of skipped tests
    pub skipped_tests: usize,
    /// Total execution time
    pub total_execution_time: Duration,
    /// Average execution time per test
    pub average_execution_time: Duration,
    /// Slowest test execution time
    pub slowest_test_time: Duration,
    /// Fastest test execution time
    pub fastest_test_time: Duration,
}

/// Test coverage metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageMetrics {
    /// Line coverage percentage
    pub line_coverage: f64,
    /// Branch coverage percentage
    pub branch_coverage: f64,
    /// Function coverage percentage
    pub function_coverage: f64,
    /// Total lines covered
    pub lines_covered: usize,
    /// Total lines in codebase
    pub total_lines: usize,
    /// Total branches covered
    pub branches_covered: usize,
    /// Total branches in codebase
    pub total_branches: usize,
}

/// Test reliability metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReliabilityMetrics {
    /// Flakiness score (0.0 = no flakiness, 1.0 = always flaky)
    pub flakiness_score: f64,
    /// Number of flaky tests
    pub flaky_tests: usize,
    /// Test stability score (0.0 = unstable, 1.0 = stable)
    pub stability_score: f64,
    /// Number of tests that failed in last run
    pub recent_failures: usize,
    /// Average test failure rate over time
    pub failure_rate: f64,
}

/// Test maintainability metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintainabilityMetrics {
    /// Test code complexity score
    pub complexity_score: f64,
    /// Number of test files
    pub test_files: usize,
    /// Average test file size (lines)
    pub average_file_size: f64,
    /// Test documentation coverage
    pub documentation_coverage: f64,
    /// Code duplication in tests
    pub duplication_score: f64,
}

/// Test performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Memory usage during test execution
    pub memory_usage: usize,
    /// CPU usage during test execution
    pub cpu_usage: f64,
    /// I/O operations during test execution
    pub io_operations: usize,
    /// Network operations during test execution
    pub network_operations: usize,
    /// Peak memory usage
    pub peak_memory_usage: usize,
}

impl Default for TestQualityMetrics {
    fn default() -> Self {
        Self {
            execution: ExecutionMetrics::default(),
            coverage: CoverageMetrics::default(),
            reliability: ReliabilityMetrics::default(),
            maintainability: MaintainabilityMetrics::default(),
            performance: PerformanceMetrics::default(),
        }
    }
}

impl Default for ExecutionMetrics {
    fn default() -> Self {
        Self {
            total_tests: 0,
            passing_tests: 0,
            failing_tests: 0,
            skipped_tests: 0,
            total_execution_time: Duration::ZERO,
            average_execution_time: Duration::ZERO,
            slowest_test_time: Duration::ZERO,
            fastest_test_time: Duration::ZERO,
        }
    }
}

impl Default for CoverageMetrics {
    fn default() -> Self {
        Self {
            line_coverage: 0.0,
            branch_coverage: 0.0,
            function_coverage: 0.0,
            lines_covered: 0,
            total_lines: 0,
            branches_covered: 0,
            total_branches: 0,
        }
    }
}

impl Default for ReliabilityMetrics {
    fn default() -> Self {
        Self {
            flakiness_score: 0.0,
            flaky_tests: 0,
            stability_score: 1.0,
            recent_failures: 0,
            failure_rate: 0.0,
        }
    }
}

impl Default for MaintainabilityMetrics {
    fn default() -> Self {
        Self {
            complexity_score: 0.0,
            test_files: 0,
            average_file_size: 0.0,
            documentation_coverage: 0.0,
            duplication_score: 0.0,
        }
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            memory_usage: 0,
            cpu_usage: 0.0,
            io_operations: 0,
            network_operations: 0,
            peak_memory_usage: 0,
        }
    }
}

/// Test quality dashboard
pub struct TestQualityDashboard {
    metrics: TestQualityMetrics,
    test_history: HashMap<String, TestHistory>,
    quality_gates: QualityGates,
}

/// Test execution history for reliability tracking
#[derive(Debug, Clone)]
pub struct TestHistory {
    pub test_name: String,
    pub executions: Vec<TestExecution>,
    pub flakiness_score: f64,
}

/// Individual test execution record
#[derive(Debug, Clone)]
pub struct TestExecution {
    pub timestamp: Instant,
    pub duration: Duration,
    pub result: TestResult,
    pub memory_usage: usize,
    pub cpu_usage: f64,
}

/// Test execution result
#[derive(Debug, Clone, PartialEq)]
pub enum TestResult {
    Passed,
    Failed(String),
    Skipped,
}

/// Quality gates for test validation
#[derive(Debug, Clone)]
pub struct QualityGates {
    pub min_line_coverage: f64,
    pub min_branch_coverage: f64,
    pub max_execution_time_unit: Duration,
    pub max_execution_time_integration: Duration,
    pub max_execution_time_e2e: Duration,
    pub max_flakiness_score: f64,
    pub max_memory_usage: usize,
    pub max_cpu_usage: f64,
}

impl Default for QualityGates {
    fn default() -> Self {
        Self {
            min_line_coverage: 90.0,
            min_branch_coverage: 85.0,
            max_execution_time_unit: Duration::from_millis(10),
            max_execution_time_integration: Duration::from_millis(100),
            max_execution_time_e2e: Duration::from_secs(1),
            max_flakiness_score: 0.05, // 5% flakiness tolerance
            max_memory_usage: 100 * 1024 * 1024, // 100MB
            max_cpu_usage: 80.0, // 80% CPU
        }
    }
}

impl TestQualityDashboard {
    /// Create new test quality dashboard
    pub fn new() -> Self {
        Self {
            metrics: TestQualityMetrics::default(),
            test_history: HashMap::new(),
            quality_gates: QualityGates::default(),
        }
    }

    /// Record test execution
    pub fn record_test_execution(
        &mut self,
        test_name: &str,
        duration: Duration,
        result: TestResult,
        memory_usage: usize,
        cpu_usage: f64,
    ) {
        let execution = TestExecution {
            timestamp: Instant::now(),
            duration,
            result,
            memory_usage,
            cpu_usage,
        };

        let history = self.test_history.entry(test_name.to_string()).or_insert(TestHistory {
            test_name: test_name.to_string(),
            executions: Vec::new(),
            flakiness_score: 0.0,
        });

        history.executions.push(execution);
        
        // Update flakiness score
        history.flakiness_score = self.calculate_flakiness_score(&history.executions);
        
        // Update metrics
        self.update_metrics();
    }

    /// Calculate flakiness score for a test
    fn calculate_flakiness_score(&self, executions: &[TestExecution]) -> f64 {
        if executions.len() < 2 {
            return 0.0;
        }

        let mut inconsistent_results = 0;
        let mut previous_result = None;

        for execution in executions {
            if let Some(prev) = previous_result {
                if prev != execution.result {
                    inconsistent_results += 1;
                }
            }
            previous_result = Some(execution.result.clone());
        }

        inconsistent_results as f64 / (executions.len() - 1) as f64
    }

    /// Update overall metrics
    fn update_metrics(&mut self) {
        // Update execution metrics
        let mut total_tests = 0;
        let mut passing_tests = 0;
        let mut failing_tests = 0;
        let mut total_time = Duration::ZERO;
        let mut slowest_time = Duration::ZERO;
        let mut fastest_time = Duration::MAX;

        for history in self.test_history.values() {
            if let Some(last_execution) = history.executions.last() {
                total_tests += 1;
                total_time += last_execution.duration;

                match last_execution.result {
                    TestResult::Passed => passing_tests += 1,
                    TestResult::Failed(_) => failing_tests += 1,
                    TestResult::Skipped => {}
                }

                if last_execution.duration > slowest_time {
                    slowest_time = last_execution.duration;
                }
                if last_execution.duration < fastest_time {
                    fastest_time = last_execution.duration;
                }
            }
        }

        self.metrics.execution = ExecutionMetrics {
            total_tests,
            passing_tests,
            failing_tests,
            skipped_tests: 0, // Would be calculated from actual data
            total_execution_time: total_time,
            average_execution_time: if total_tests > 0 {
                total_time / total_tests as u32
            } else {
                Duration::ZERO
            },
            slowest_test_time: slowest_time,
            fastest_test_time: if fastest_time == Duration::MAX {
                Duration::ZERO
            } else {
                fastest_time
            },
        };

        // Update reliability metrics
        let flaky_tests = self.test_history.values()
            .filter(|history| history.flakiness_score > self.quality_gates.max_flakiness_score)
            .count();

        self.metrics.reliability = ReliabilityMetrics {
            flakiness_score: if total_tests > 0 {
                flaky_tests as f64 / total_tests as f64
            } else {
                0.0
            },
            flaky_tests,
            stability_score: 1.0 - (flaky_tests as f64 / total_tests as f64),
            recent_failures: failing_tests,
            failure_rate: if total_tests > 0 {
                failing_tests as f64 / total_tests as f64
            } else {
                0.0
            },
        };
    }

    /// Check if quality gates are met
    pub fn check_quality_gates(&self) -> QualityGateReport {
        let mut report = QualityGateReport::new();

        // Check coverage gates
        if self.metrics.coverage.line_coverage < self.quality_gates.min_line_coverage {
            report.add_violation(QualityGateViolation {
                gate: "Line Coverage".to_string(),
                expected: self.quality_gates.min_line_coverage,
                actual: self.metrics.coverage.line_coverage,
                severity: ViolationSeverity::High,
            });
        }

        if self.metrics.coverage.branch_coverage < self.quality_gates.min_branch_coverage {
            report.add_violation(QualityGateViolation {
                gate: "Branch Coverage".to_string(),
                expected: self.quality_gates.min_branch_coverage,
                actual: self.metrics.coverage.branch_coverage,
                severity: ViolationSeverity::High,
            });
        }

        // Check execution time gates
        if self.metrics.execution.slowest_test_time > self.quality_gates.max_execution_time_unit {
            report.add_violation(QualityGateViolation {
                gate: "Unit Test Execution Time".to_string(),
                expected: self.quality_gates.max_execution_time_unit.as_millis() as f64,
                actual: self.metrics.execution.slowest_test_time.as_millis() as f64,
                severity: ViolationSeverity::Medium,
            });
        }

        // Check flakiness gates
        if self.metrics.reliability.flakiness_score > self.quality_gates.max_flakiness_score {
            report.add_violation(QualityGateViolation {
                gate: "Flakiness Score".to_string(),
                expected: self.quality_gates.max_flakiness_score,
                actual: self.metrics.reliability.flakiness_score,
                severity: ViolationSeverity::High,
            });
        }

        // Check memory gates
        if self.metrics.performance.memory_usage > self.quality_gates.max_memory_usage {
            report.add_violation(QualityGateViolation {
                gate: "Memory Usage".to_string(),
                expected: self.quality_gates.max_memory_usage as f64,
                actual: self.metrics.performance.memory_usage as f64,
                severity: ViolationSeverity::Medium,
            });
        }

        report
    }

    /// Generate comprehensive quality report
    pub fn generate_quality_report(&self) -> String {
        let quality_gates = self.check_quality_gates();
        
        format!(
            r#"
# Test Quality Report

## Executive Summary
- **Total Tests**: {}
- **Passing Tests**: {} ({:.1}%)
- **Failing Tests**: {} ({:.1}%)
- **Flaky Tests**: {} ({:.1}%)
- **Overall Quality Score**: {:.1}/100

## Execution Metrics
- **Total Execution Time**: {:?}
- **Average Test Time**: {:?}
- **Slowest Test**: {:?}
- **Fastest Test**: {:?}

## Coverage Metrics
- **Line Coverage**: {:.1}% (Target: {:.1}%)
- **Branch Coverage**: {:.1}% (Target: {:.1}%)
- **Function Coverage**: {:.1}%

## Reliability Metrics
- **Flakiness Score**: {:.3} (Target: <{:.3})
- **Stability Score**: {:.1}%
- **Failure Rate**: {:.1}%

## Performance Metrics
- **Memory Usage**: {} MB (Target: <{} MB)
- **CPU Usage**: {:.1}% (Target: <{:.1}%)

## Quality Gate Status
{}

## Recommendations
{}
"#,
            self.metrics.execution.total_tests,
            self.metrics.execution.passing_tests,
            self.metrics.execution.passing_tests as f64 / self.metrics.execution.total_tests as f64 * 100.0,
            self.metrics.execution.failing_tests,
            self.metrics.execution.failing_tests as f64 / self.metrics.execution.total_tests as f64 * 100.0,
            self.metrics.reliability.flaky_tests,
            self.metrics.reliability.flakiness_score * 100.0,
            self.calculate_overall_quality_score(),
            self.metrics.execution.total_execution_time,
            self.metrics.execution.average_execution_time,
            self.metrics.execution.slowest_test_time,
            self.metrics.execution.fastest_test_time,
            self.metrics.coverage.line_coverage,
            self.quality_gates.min_line_coverage,
            self.metrics.coverage.branch_coverage,
            self.quality_gates.min_branch_coverage,
            self.metrics.coverage.function_coverage,
            self.metrics.reliability.flakiness_score,
            self.quality_gates.max_flakiness_score,
            self.metrics.reliability.stability_score * 100.0,
            self.metrics.reliability.failure_rate * 100.0,
            self.metrics.performance.memory_usage / (1024 * 1024),
            self.quality_gates.max_memory_usage / (1024 * 1024),
            self.metrics.performance.cpu_usage,
            self.quality_gates.max_cpu_usage,
            quality_gates.status_summary(),
            self.generate_recommendations(&quality_gates)
        )
    }

    /// Calculate overall quality score (0-100)
    fn calculate_overall_quality_score(&self) -> f64 {
        let mut score = 100.0;

        // Deduct points for coverage issues
        if self.metrics.coverage.line_coverage < self.quality_gates.min_line_coverage {
            score -= 20.0;
        }
        if self.metrics.coverage.branch_coverage < self.quality_gates.min_branch_coverage {
            score -= 15.0;
        }

        // Deduct points for reliability issues
        score -= self.metrics.reliability.flakiness_score * 30.0;
        score -= self.metrics.reliability.failure_rate * 25.0;

        // Deduct points for performance issues
        if self.metrics.performance.memory_usage > self.quality_gates.max_memory_usage {
            score -= 10.0;
        }
        if self.metrics.performance.cpu_usage > self.quality_gates.max_cpu_usage {
            score -= 10.0;
        }

        score.max(0.0)
    }

    /// Generate recommendations based on quality gate violations
    fn generate_recommendations(&self, quality_gates: &QualityGateReport) -> String {
        let mut recommendations = Vec::new();

        if quality_gates.violations.is_empty() {
            recommendations.push("✅ All quality gates are met! Keep up the excellent work.".to_string());
        } else {
            for violation in &quality_gates.violations {
                match violation.gate.as_str() {
                    "Line Coverage" => {
                        recommendations.push(format!(
                            "📈 Improve line coverage from {:.1}% to {:.1}% by adding more test cases",
                            violation.actual, violation.expected
                        ));
                    }
                    "Branch Coverage" => {
                        recommendations.push(format!(
                            "🌿 Improve branch coverage from {:.1}% to {:.1}% by testing edge cases",
                            violation.actual, violation.expected
                        ));
                    }
                    "Unit Test Execution Time" => {
                        recommendations.push(format!(
                            "⚡ Optimize slow tests - current slowest: {:.1}ms, target: {:.1}ms",
                            violation.actual, violation.expected
                        ));
                    }
                    "Flakiness Score" => {
                        recommendations.push(format!(
                            "🔧 Fix flaky tests - current score: {:.3}, target: <{:.3}",
                            violation.actual, violation.expected
                        ));
                    }
                    "Memory Usage" => {
                        recommendations.push(format!(
                            "💾 Reduce memory usage from {:.1}MB to <{:.1}MB",
                            violation.actual / (1024.0 * 1024.0), violation.expected / (1024.0 * 1024.0)
                        ));
                    }
                    _ => {
                        recommendations.push(format!(
                            "🔍 Address {} violation: {:.1} (expected: {:.1})",
                            violation.gate, violation.actual, violation.expected
                        ));
                    }
                }
            }
        }

        recommendations.join("\n")
    }

    /// Get current metrics
    pub fn metrics(&self) -> &TestQualityMetrics {
        &self.metrics
    }

    /// Get test history
    pub fn test_history(&self) -> &HashMap<String, TestHistory> {
        &self.test_history
    }
}

/// Quality gate violation report
#[derive(Debug, Clone)]
pub struct QualityGateReport {
    pub violations: Vec<QualityGateViolation>,
}

impl QualityGateReport {
    /// Create new quality gate report
    pub fn new() -> Self {
        Self {
            violations: Vec::new(),
        }
    }

    /// Add violation to report
    pub fn add_violation(&mut self, violation: QualityGateViolation) {
        self.violations.push(violation);
    }

    /// Check if all gates are met
    pub fn all_gates_met(&self) -> bool {
        self.violations.is_empty()
    }

    /// Get status summary
    pub fn status_summary(&self) -> String {
        if self.all_gates_met() {
            "✅ All quality gates are met!".to_string()
        } else {
            format!("❌ {} quality gate violations found:", self.violations.len())
        }
    }
}

/// Quality gate violation
#[derive(Debug, Clone)]
pub struct QualityGateViolation {
    pub gate: String,
    pub expected: f64,
    pub actual: f64,
    pub severity: ViolationSeverity,
}

/// Violation severity level
#[derive(Debug, Clone, PartialEq)]
pub enum ViolationSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl Default for QualityGateReport {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_dashboard_creation() {
        let dashboard = TestQualityDashboard::new();
        assert_eq!(dashboard.metrics().execution.total_tests, 0);
        assert_eq!(dashboard.metrics().reliability.flakiness_score, 0.0);
    }

    #[test]
    fn test_test_execution_recording() {
        let mut dashboard = TestQualityDashboard::new();
        
        dashboard.record_test_execution(
            "test_example",
            Duration::from_millis(5),
            TestResult::Passed,
            1024,
            10.0,
        );

        assert_eq!(dashboard.metrics().execution.total_tests, 1);
        assert_eq!(dashboard.metrics().execution.passing_tests, 1);
        assert_eq!(dashboard.metrics().execution.failing_tests, 0);
    }

    #[test]
    fn test_flakiness_calculation() {
        let mut dashboard = TestQualityDashboard::new();
        
        // Record consistent results
        dashboard.record_test_execution("test_consistent", Duration::from_millis(5), TestResult::Passed, 1024, 10.0);
        dashboard.record_test_execution("test_consistent", Duration::from_millis(5), TestResult::Passed, 1024, 10.0);
        dashboard.record_test_execution("test_consistent", Duration::from_millis(5), TestResult::Passed, 1024, 10.0);
        
        // Record inconsistent results
        dashboard.record_test_execution("test_flaky", Duration::from_millis(5), TestResult::Passed, 1024, 10.0);
        dashboard.record_test_execution("test_flaky", Duration::from_millis(5), TestResult::Failed("error".to_string()), 1024, 10.0);
        dashboard.record_test_execution("test_flaky", Duration::from_millis(5), TestResult::Passed, 1024, 10.0);

        let consistent_history = dashboard.test_history().get("test_consistent").unwrap();
        let flaky_history = dashboard.test_history().get("test_flaky").unwrap();

        assert!(consistent_history.flakiness_score < flaky_history.flakiness_score);
    }

    #[test]
    fn test_quality_gate_checking() {
        let mut dashboard = TestQualityDashboard::new();
        
        // Set up metrics that violate quality gates
        dashboard.metrics.coverage.line_coverage = 80.0; // Below 90% target
        dashboard.metrics.reliability.flakiness_score = 0.1; // Above 0.05 target
        
        let report = dashboard.check_quality_gates();
        assert!(!report.all_gates_met());
        assert!(report.violations.len() >= 2);
    }

    #[test]
    fn test_quality_report_generation() {
        let mut dashboard = TestQualityDashboard::new();
        
        dashboard.record_test_execution(
            "test_example",
            Duration::from_millis(5),
            TestResult::Passed,
            1024,
            10.0,
        );

        let report = dashboard.generate_quality_report();
        assert!(report.contains("Test Quality Report"));
        assert!(report.contains("Total Tests: 1"));
        assert!(report.contains("Passing Tests: 1"));
    }
}
