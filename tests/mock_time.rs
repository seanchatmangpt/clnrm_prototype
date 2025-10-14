//! Mock time utilities for fast, deterministic tests
//!
//! This module provides time mocking capabilities that allow tests to run
//! without real delays, making them faster and more deterministic.

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::time::{sleep, Sleep};

/// Mock time controller for test environments
#[derive(Debug, Clone)]
pub struct MockTime {
    /// Current mock time
    current_time: Arc<Mutex<Instant>>,
    /// Whether time is advancing automatically
    auto_advance: Arc<Mutex<bool>>,
    /// Auto-advance interval
    advance_interval: Arc<Mutex<Duration>>,
}

impl MockTime {
    /// Create a new mock time controller
    pub fn new() -> Self {
        Self {
            current_time: Arc::new(Mutex::new(Instant::now())),
            auto_advance: Arc::new(Mutex::new(false)),
            advance_interval: Arc::new(Mutex::new(Duration::from_millis(1))),
        }
    }

    /// Get the current mock time
    pub fn now(&self) -> Instant {
        *self.current_time.lock().unwrap()
    }

    /// Advance time by the given duration
    pub fn advance(&self, duration: Duration) {
        let mut time = self.current_time.lock().unwrap();
        *time += duration;
    }

    /// Set the current time
    pub fn set_time(&self, time: Instant) {
        let mut current = self.current_time.lock().unwrap();
        *current = time;
    }

    /// Enable auto-advance mode
    pub fn enable_auto_advance(&self, interval: Duration) {
        let mut auto = self.auto_advance.lock().unwrap();
        let mut interval_guard = self.advance_interval.lock().unwrap();
        *auto = true;
        *interval_guard = interval;
    }

    /// Disable auto-advance mode
    pub fn disable_auto_advance(&self) {
        let mut auto = self.auto_advance.lock().unwrap();
        *auto = false;
    }

    /// Create a mock sleep future
    pub fn sleep(&self, duration: Duration) -> MockSleep {
        MockSleep::new(self.clone(), duration)
    }

    /// Create a mock timeout future
    pub fn timeout<F>(&self, duration: Duration, future: F) -> MockTimeout<F>
    where
        F: std::future::Future,
    {
        MockTimeout::new(self.clone(), duration, future)
    }
}

impl Default for MockTime {
    fn default() -> Self {
        Self::new()
    }
}

/// Mock sleep future that completes immediately in tests
pub struct MockSleep {
    mock_time: MockTime,
    duration: Duration,
    completed: bool,
}

impl MockSleep {
    fn new(mock_time: MockTime, duration: Duration) -> Self {
        Self {
            mock_time,
            duration,
            completed: false,
        }
    }
}

impl std::future::Future for MockSleep {
    type Output = ();

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if self.completed {
            return std::task::Poll::Ready(());
        }

        // In mock mode, advance time and complete immediately
        self.mock_time.advance(self.duration);
        self.completed = true;
        std::task::Poll::Ready(())
    }
}

/// Mock timeout future for testing
pub struct MockTimeout<F> {
    mock_time: MockTime,
    duration: Duration,
    future: F,
    completed: bool,
}

impl<F> MockTimeout<F>
where
    F: std::future::Future,
{
    fn new(mock_time: MockTime, duration: Duration, future: F) -> Self {
        Self {
            mock_time,
            duration,
            future,
            completed: false,
        }
    }
}

impl<F> std::future::Future for MockTimeout<F>
where
    F: std::future::Future,
{
    type Output = Result<F::Output, tokio::time::error::Elapsed>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if self.completed {
            return std::task::Poll::Ready(Err(tokio::time::error::Elapsed));
        }

        // Try to poll the inner future
        match std::pin::Pin::new(&mut self.future).poll(cx) {
            std::task::Poll::Ready(result) => {
                self.completed = true;
                std::task::Poll::Ready(Ok(result))
            }
            std::task::Poll::Pending => {
                // In mock mode, we can simulate timeout by advancing time
                // For now, we'll just return pending to let the future complete
                std::task::Poll::Pending
            }
        }
    }
}

/// Test environment with mocked time
pub struct MockTimeTestEnv {
    mock_time: MockTime,
}

impl MockTimeTestEnv {
    /// Create a new mock time test environment
    pub fn new() -> Self {
        Self {
            mock_time: MockTime::new(),
        }
    }

    /// Get the mock time controller
    pub fn mock_time(&self) -> &MockTime {
        &self.mock_time
    }

    /// Run a test with mocked time
    pub async fn run_test<F, Fut, T>(&self, test: F) -> T
    where
        F: FnOnce(&MockTime) -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        test(&self.mock_time).await
    }

    /// Advance time and run a test
    pub async fn advance_and_run<F, Fut, T>(&self, duration: Duration, test: F) -> T
    where
        F: FnOnce(&MockTime) -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        self.mock_time.advance(duration);
        test(&self.mock_time).await
    }
}

impl Default for MockTimeTestEnv {
    fn default() -> Self {
        Self::new()
    }
}

/// Macro to create a mock sleep that completes immediately
#[macro_export]
macro_rules! mock_sleep {
    ($duration:expr) => {{
        // In test mode, return a completed future
        async { () }
    }};
}

/// Macro to create a mock timeout that doesn't actually timeout
#[macro_export]
macro_rules! mock_timeout {
    ($duration:expr, $future:expr) => {{
        // In test mode, just await the future directly
        $future.await
    }};
}

/// Conditional sleep that uses mock time in tests
///
/// This function is now re-exported from the main library.
/// Use `clnrm::conditional_sleep` instead.
pub async fn conditional_sleep(duration: Duration) {
    clnrm::conditional_sleep(duration).await
}

/// Conditional timeout that uses mock time in tests
///
/// This function is now re-exported from the main library.
/// Use `clnrm::conditional_timeout` instead.
pub async fn conditional_timeout<F, T>(
    duration: Duration,
    future: F,
) -> Result<T, tokio::time::error::Elapsed>
where
    F: std::future::Future<Output = T>,
{
    clnrm::conditional_timeout(duration, future).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_mock_time_basic() {
        let mock_time = MockTime::new();
        let start = mock_time.now();

        mock_time.advance(Duration::from_secs(5));
        let after_advance = mock_time.now();

        assert_eq!(after_advance.duration_since(start), Duration::from_secs(5));
    }

    #[tokio::test]
    async fn test_mock_sleep() {
        let mock_time = MockTime::new();
        let start = mock_time.now();

        // Mock sleep should complete immediately
        mock_time.sleep(Duration::from_secs(10)).await;

        let after_sleep = mock_time.now();
        assert_eq!(after_sleep.duration_since(start), Duration::from_secs(10));
    }

    #[tokio::test]
    async fn test_mock_timeout() {
        let mock_time = MockTime::new();

        // Mock timeout should not actually timeout
        let result = mock_time
            .timeout(Duration::from_millis(1), async { "success" })
            .await;

        assert_eq!(result, Ok("success"));
    }

    #[tokio::test]
    async fn test_conditional_sleep() {
        let start = Instant::now();

        // This should complete immediately in test mode
        conditional_sleep(Duration::from_secs(1)).await;

        let duration = start.elapsed();
        // Should be much faster than 1 second
        assert!(duration < Duration::from_millis(100));
    }

    #[tokio::test]
    async fn test_conditional_timeout() {
        // This should not timeout in test mode
        let result = conditional_timeout(Duration::from_millis(1), async { "success" }).await;

        assert_eq!(result, Ok("success"));
    }

    #[tokio::test]
    async fn test_mock_time_env() {
        let env = MockTimeTestEnv::new();

        let result = env
            .run_test(|mock_time| async move {
                let start = mock_time.now();
                mock_time.sleep(Duration::from_secs(5)).await;
                mock_time.now().duration_since(start)
            })
            .await;

        assert_eq!(result, Duration::from_secs(5));
    }
}
