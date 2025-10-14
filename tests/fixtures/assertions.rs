//! Test assertion utilities and helpers
//!
//! Provides enhanced assertion functions for cleanroom testing.

use clnrm::{Error as CleanroomError, RunResult};
use std::time::Duration;

/// Enhanced assertion utilities for cleanroom tests
pub struct TestAssertions;

impl TestAssertions {
    /// Assert that a result is successful
    pub fn assert_success<T, E>(result: &Result<T, E>) -> &T
    where
        E: std::fmt::Debug,
    {
        match result {
            Ok(value) => value,
            Err(error) => panic!("Expected success, got error: {:?}", error),
        }
    }

    /// Assert that a result is an error
    pub fn assert_error<T, E>(result: &Result<T, E>) -> &E
    where
        T: std::fmt::Debug,
    {
        match result {
            Ok(value) => panic!("Expected error, got success: {:?}", value),
            Err(error) => error,
        }
    }

    /// Assert that a result is an error with a specific message
    pub fn assert_error_message<T, E>(result: &Result<T, E>, expected_message: &str)
    where
        T: std::fmt::Debug,
        E: std::fmt::Display,
    {
        match result {
            Ok(value) => panic!(
                "Expected error with message '{}', got success: {:?}",
                expected_message, value
            ),
            Err(error) => {
                let error_message = error.to_string();
                assert!(
                    error_message.contains(expected_message),
                    "Expected error message to contain '{}', got: '{}'",
                    expected_message,
                    error_message
                );
            }
        }
    }

    /// Assert that a run result is successful
    pub fn assert_run_success(result: &RunResult) {
        assert_eq!(
            result.exit_code, 0,
            "Expected exit code 0, got {} with stderr: {}",
            result.exit_code, result.stderr
        );
    }

    /// Assert that a run result failed
    pub fn assert_run_failure(result: &RunResult) {
        assert_ne!(
            result.exit_code, 0,
            "Expected non-zero exit code, got 0 with stdout: {}",
            result.stdout
        );
    }

    /// Assert that a run result failed with a specific exit code
    pub fn assert_run_failure_with_code(result: &RunResult, expected_code: i32) {
        assert_eq!(
            result.exit_code, expected_code,
            "Expected exit code {}, got {} with stderr: {}",
            expected_code, result.exit_code, result.stderr
        );
    }

    /// Assert that stdout contains expected content
    pub fn assert_stdout_contains(result: &RunResult, expected: &str) {
        assert!(
            result.stdout.contains(expected),
            "Expected stdout to contain '{}', got: '{}'",
            expected,
            result.stdout
        );
    }

    /// Assert that stderr contains expected content
    pub fn assert_stderr_contains(result: &RunResult, expected: &str) {
        assert!(
            result.stderr.contains(expected),
            "Expected stderr to contain '{}', got: '{}'",
            expected,
            result.stderr
        );
    }

    /// Assert that a duration is within expected range
    pub fn assert_duration_in_range(duration: Duration, min: Duration, max: Duration) {
        assert!(
            duration >= min && duration <= max,
            "Duration {:?} is not in range [{:?}, {:?}]",
            duration,
            min,
            max
        );
    }

    /// Assert that a duration is less than expected
    pub fn assert_duration_less_than(duration: Duration, max: Duration) {
        assert!(
            duration < max,
            "Duration {:?} is not less than {:?}",
            duration,
            max
        );
    }

    /// Assert that a duration is greater than expected
    pub fn assert_duration_greater_than(duration: Duration, min: Duration) {
        assert!(
            duration > min,
            "Duration {:?} is not greater than {:?}",
            duration,
            min
        );
    }

    /// Assert that two values are approximately equal (for floating point comparisons)
    pub fn assert_approx_eq(a: f64, b: f64, epsilon: f64) {
        assert!(
            (a - b).abs() < epsilon,
            "{} is not approximately equal to {} (epsilon: {})",
            a,
            b,
            epsilon
        );
    }

    /// Assert that a string contains expected content
    pub fn assert_string_contains(haystack: &str, needle: &str) {
        assert!(
            haystack.contains(needle),
            "String '{}' does not contain '{}'",
            haystack,
            needle
        );
    }

    /// Assert that a vector contains expected elements
    pub fn assert_vec_contains<T: PartialEq + std::fmt::Debug>(vec: &[T], expected: &T) {
        assert!(
            vec.contains(expected),
            "Vector {:?} does not contain {:?}",
            vec,
            expected
        );
    }

    /// Assert that a vector has expected length
    pub fn assert_vec_length<T>(vec: &[T], expected_length: usize) {
        assert_eq!(
            vec.len(),
            expected_length,
            "Expected vector length {}, got {}",
            expected_length,
            vec.len()
        );
    }

    /// Assert that a HashMap contains expected key
    pub fn assert_map_contains_key<K: std::hash::Hash + std::cmp::Eq, V>(
        map: &std::collections::HashMap<K, V>,
        key: &K,
    ) {
        assert!(
            map.contains_key(key),
            "HashMap does not contain key: {:?}",
            key
        );
    }

    /// Assert that a HashMap contains expected key-value pair
    pub fn assert_map_contains<
        K: std::hash::Hash + std::cmp::Eq,
        V: PartialEq + std::fmt::Debug,
    >(
        map: &std::collections::HashMap<K, V>,
        key: &K,
        value: &V,
    ) {
        match map.get(key) {
            Some(actual_value) => assert_eq!(
                actual_value, value,
                "HashMap value for key {:?} is {:?}, expected {:?}",
                key, actual_value, value
            ),
            None => panic!("HashMap does not contain key: {:?}", key),
        }
    }
}

/// Macro for asserting that a test completes within a reasonable time
#[macro_export]
macro_rules! assert_fast_completion {
    ($test:expr) => {{
        let start = std::time::Instant::now();
        let result = $test;
        let duration = start.elapsed();

        // In test mode, operations should complete quickly due to mocking
        assert!(
            duration < std::time::Duration::from_millis(100),
            "Test took too long: {:?}",
            duration
        );

        result
    }};
}

/// Macro for asserting that a test completes within a specific time
#[macro_export]
macro_rules! assert_completion_within {
    ($test:expr, $max_duration:expr) => {{
        let start = std::time::Instant::now();
        let result = $test;
        let duration = start.elapsed();

        assert!(
            duration < $max_duration,
            "Test took too long: {:?}, expected less than {:?}",
            duration,
            $max_duration
        );

        result
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assert_success() {
        let result: Result<i32, &str> = Ok(42);
        let value = TestAssertions::assert_success(&result);
        assert_eq!(*value, 42);
    }

    #[test]
    #[should_panic(expected = "Expected success, got error")]
    fn test_assert_success_panics_on_error() {
        let result: Result<i32, &str> = Err("test error");
        TestAssertions::assert_success(&result);
    }

    #[test]
    fn test_assert_error() {
        let result: Result<i32, &str> = Err("test error");
        let error = TestAssertions::assert_error(&result);
        assert_eq!(*error, "test error");
    }

    #[test]
    #[should_panic(expected = "Expected error, got success")]
    fn test_assert_error_panics_on_success() {
        let result: Result<i32, &str> = Ok(42);
        TestAssertions::assert_error(&result);
    }

    #[test]
    fn test_assert_error_message() {
        let result: Result<i32, String> = Err("test error message".to_string());
        TestAssertions::assert_error_message(&result, "test error");
    }

    #[test]
    #[should_panic(expected = "Expected error message to contain")]
    fn test_assert_error_message_panics_on_wrong_message() {
        let result: Result<i32, String> = Err("different error".to_string());
        TestAssertions::assert_error_message(&result, "test error");
    }

    #[test]
    fn test_assert_duration_in_range() {
        let duration = Duration::from_millis(50);
        TestAssertions::assert_duration_in_range(
            duration,
            Duration::from_millis(40),
            Duration::from_millis(60),
        );
    }

    #[test]
    #[should_panic(expected = "Duration")]
    fn test_assert_duration_in_range_panics_outside_range() {
        let duration = Duration::from_millis(100);
        TestAssertions::assert_duration_in_range(
            duration,
            Duration::from_millis(40),
            Duration::from_millis(60),
        );
    }

    #[test]
    fn test_assert_approx_eq() {
        TestAssertions::assert_approx_eq(1.0, 1.0001, 0.001);
    }

    #[test]
    #[should_panic(expected = "is not approximately equal")]
    fn test_assert_approx_eq_panics_on_difference() {
        TestAssertions::assert_approx_eq(1.0, 2.0, 0.001);
    }

    #[test]
    fn test_assert_string_contains() {
        TestAssertions::assert_string_contains("hello world", "world");
    }

    #[test]
    #[should_panic(expected = "does not contain")]
    fn test_assert_string_contains_panics_on_missing() {
        TestAssertions::assert_string_contains("hello world", "missing");
    }

    #[test]
    fn test_assert_vec_contains() {
        let vec = vec![1, 2, 3, 4, 5];
        TestAssertions::assert_vec_contains(&vec, &3);
    }

    #[test]
    #[should_panic(expected = "does not contain")]
    fn test_assert_vec_contains_panics_on_missing() {
        let vec = vec![1, 2, 3, 4, 5];
        TestAssertions::assert_vec_contains(&vec, &6);
    }

    #[test]
    fn test_assert_vec_length() {
        let vec = vec![1, 2, 3];
        TestAssertions::assert_vec_length(&vec, 3);
    }

    #[test]
    #[should_panic(expected = "Expected vector length")]
    fn test_assert_vec_length_panics_on_wrong_length() {
        let vec = vec![1, 2, 3];
        TestAssertions::assert_vec_length(&vec, 5);
    }

    #[test]
    fn test_assert_map_contains_key() {
        let mut map = std::collections::HashMap::new();
        map.insert("key1", "value1");
        map.insert("key2", "value2");

        TestAssertions::assert_map_contains_key(&map, &"key1");
    }

    #[test]
    #[should_panic(expected = "does not contain key")]
    fn test_assert_map_contains_key_panics_on_missing() {
        let mut map = std::collections::HashMap::new();
        map.insert("key1", "value1");

        TestAssertions::assert_map_contains_key(&map, &"missing");
    }

    #[test]
    fn test_assert_map_contains() {
        let mut map = std::collections::HashMap::new();
        map.insert("key1", "value1");
        map.insert("key2", "value2");

        TestAssertions::assert_map_contains(&map, &"key1", &"value1");
    }

    #[test]
    #[should_panic(expected = "does not contain key")]
    fn test_assert_map_contains_panics_on_missing_key() {
        let mut map = std::collections::HashMap::new();
        map.insert("key1", "value1");

        TestAssertions::assert_map_contains(&map, &"missing", &"value");
    }

    #[test]
    #[should_panic(expected = "HashMap value for key")]
    fn test_assert_map_contains_panics_on_wrong_value() {
        let mut map = std::collections::HashMap::new();
        map.insert("key1", "value1");

        TestAssertions::assert_map_contains(&map, &"key1", &"wrong_value");
    }
}
