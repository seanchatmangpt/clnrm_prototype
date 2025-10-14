//! Mock time utilities for test fixtures
//!
//! Provides mock time functionality integrated with test fixtures.

use clnrm::{conditional_sleep, conditional_timeout, MockTime, MockTimeTestEnv};
use std::time::{Duration, Instant};

/// Mock time fixtures for tests
pub struct MockTimeFixtures;

impl MockTimeFixtures {
    /// Create a new mock time test environment
    pub fn new_test_env() -> MockTimeTestEnv {
        MockTimeTestEnv::new()
    }

    /// Create a new mock time controller
    pub fn new_mock_time() -> MockTime {
        MockTime::new()
    }

    /// Create a mock time test environment with auto-advance
    pub fn new_auto_advance_test_env(interval: Duration) -> MockTimeTestEnv {
        let env = MockTimeTestEnv::new();
        env.mock_time().enable_auto_advance(interval);
        env
    }
}

/// Mock time test utilities
pub struct MockTimeUtils;

impl MockTimeUtils {
    /// Wait for a condition with mock time
    pub async fn wait_for_condition<F, Fut>(
        condition: F,
        timeout_duration: Duration,
        mock_time: Option<&MockTime>,
    ) -> Result<bool, Box<dyn std::error::Error>>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = bool>,
    {
        let start_time = Instant::now();
        while start_time.elapsed() < timeout_duration {
            if condition().await {
                return Ok(true);
            }
            
            // Use conditional sleep for faster tests
            conditional_sleep(Duration::from_millis(1)).await;
            
            // If mock time is provided, advance it
            if let Some(mock) = mock_time {
                mock.advance(Duration::from_millis(1));
            }
        }
        Ok(false)
    }

    /// Execute a test with mock timeout
    pub async fn execute_with_mock_timeout<F, Fut, T>(
        test: F,
        timeout_duration: Duration,
    ) -> Result<T, Box<dyn std::error::Error>>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, Box<dyn std::error::Error>>>,
    {
        conditional_timeout(timeout_duration, test()).await
            .map_err(|_| "Mock timeout".into())
    }

    /// Measure execution time with mock time
    pub async fn measure_execution_time<F, Fut, T>(
        test: F,
        mock_time: Option<&MockTime>,
    ) -> (T, Duration)
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        let start_time = if let Some(mock) = mock_time {
            mock.now()
        } else {
            Instant::now()
        };

        let result = test().await;

        let duration = if let Some(mock) = mock_time {
            mock.now().duration_since(start_time)
        } else {
            start_time.elapsed()
        };

        (result, duration)
    }

    /// Advance mock time and execute test
    pub async fn advance_and_execute<F, Fut, T>(
        mock_time: &MockTime,
        advance_duration: Duration,
        test: F,
    ) -> T
    where
        F: FnOnce(&MockTime) -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        mock_time.advance(advance_duration);
        test(mock_time).await
    }

    /// Create a test that runs with mock time
    pub async fn run_with_mock_time<F, Fut, T>(
        test: F,
    ) -> T
    where
        F: FnOnce(&MockTime) -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        let mock_time = MockTime::new();
        test(&mock_time).await
    }
}

/// Mock time test context
pub struct MockTimeTestContext {
    pub mock_time: MockTime,
    pub start_time: Instant,
}

impl MockTimeTestContext {
    /// Create a new mock time test context
    pub fn new() -> Self {
        let mock_time = MockTime::new();
        let start_time = mock_time.now();
        Self {
            mock_time,
            start_time,
        }
    }

    /// Advance time by duration
    pub fn advance(&self, duration: Duration) {
        self.mock_time.advance(duration);
    }

    /// Get elapsed time
    pub fn elapsed(&self) -> Duration {
        self.mock_time.now().duration_since(self.start_time)
    }

    /// Get current mock time
    pub fn now(&self) -> Instant {
        self.mock_time.now()
    }

    /// Create a mock sleep
    pub fn sleep(&self, duration: Duration) -> impl std::future::Future<Output = ()> {
        self.mock_time.sleep(duration)
    }

    /// Create a mock timeout
    pub fn timeout<F>(&self, duration: Duration, future: F) -> impl std::future::Future<Output = Result<F::Output, tokio::time::error::Elapsed>>
    where
        F: std::future::Future,
    {
        self.mock_time.timeout(duration, future)
    }
}

impl Default for MockTimeTestContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_time_fixtures() {
        let env = MockTimeFixtures::new_test_env();
        let mock_time = MockTimeFixtures::new_mock_time();
        
        // Test basic functionality
        let start = mock_time.now();
        mock_time.advance(Duration::from_secs(5));
        let after_advance = mock_time.now();
        
        assert_eq!(after_advance.duration_since(start), Duration::from_secs(5));
    }

    #[tokio::test]
    async fn test_mock_time_utils_wait_for_condition() {
        let mut counter = 0;
        let result = MockTimeUtils::wait_for_condition(
            || async {
                counter += 1;
                counter >= 3
            },
            Duration::from_secs(1),
            None,
        ).await;
        
        assert!(result.is_ok());
        assert!(result.unwrap());
        assert_eq!(counter, 3);
    }

    #[tokio::test]
    async fn test_mock_time_utils_execute_with_timeout() {
        let result = MockTimeUtils::execute_with_mock_timeout(
            || async {
                conditional_sleep(Duration::from_millis(10)).await;
                Ok::<i32, Box<dyn std::error::Error>>(42)
            },
            Duration::from_secs(1),
        ).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_mock_time_utils_measure_execution_time() {
        let (result, duration) = MockTimeUtils::measure_execution_time(
            || async {
                conditional_sleep(Duration::from_millis(10)).await;
                42
            },
            None,
        ).await;
        
        assert_eq!(result, 42);
        // Duration should be very small due to conditional sleep
        assert!(duration < Duration::from_millis(100));
    }

    #[tokio::test]
    async fn test_mock_time_utils_advance_and_execute() {
        let mock_time = MockTime::new();
        let start = mock_time.now();
        
        let result = MockTimeUtils::advance_and_execute(
            &mock_time,
            Duration::from_secs(10),
            |mock| async move {
                mock.now().duration_since(start)
            }
        ).await;
        
        assert_eq!(result, Duration::from_secs(10));
    }

    #[tokio::test]
    async fn test_mock_time_utils_run_with_mock_time() {
        let result = MockTimeUtils::run_with_mock_time(|mock_time| async move {
            let start = mock_time.now();
            mock_time.advance(Duration::from_secs(5));
            mock_time.now().duration_since(start)
        }).await;
        
        assert_eq!(result, Duration::from_secs(5));
    }

    #[tokio::test]
    async fn test_mock_time_test_context() {
        let context = MockTimeTestContext::new();
        let start_elapsed = context.elapsed();
        
        context.advance(Duration::from_secs(5));
        let after_advance = context.elapsed();
        
        assert_eq!(after_advance, Duration::from_secs(5));
        
        // Test sleep
        context.sleep(Duration::from_secs(3)).await;
        let after_sleep = context.elapsed();
        
        assert_eq!(after_sleep, Duration::from_secs(8));
    }
}
