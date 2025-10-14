//! Basic command execution tests
//!
//! Tests for basic command execution functionality using the run() function.

use crate::fixtures::TestAssertions;
use clnrm::{run, Error as CleanroomError};

#[tokio::test]
async fn test_echo_command() -> Result<(), CleanroomError> {
    let result = run(["echo", "hello world"]);

    TestAssertions::assert_success(&result);
    let run_result = result.unwrap();

    TestAssertions::assert_run_success(&run_result);
    TestAssertions::assert_stdout_contains(&run_result, "hello world");

    Ok(())
}

#[tokio::test]
async fn test_command_with_shell() -> Result<(), CleanroomError> {
    let result = run(["sh", "-c", "echo 'test with shell'"]);

    TestAssertions::assert_success(&result);
    let run_result = result.unwrap();

    TestAssertions::assert_run_success(&run_result);
    TestAssertions::assert_stdout_contains(&run_result, "test with shell");

    Ok(())
}

#[tokio::test]
async fn test_command_with_arguments() -> Result<(), CleanroomError> {
    let result = run(["sh", "-c", "echo 'test with args'"]);

    TestAssertions::assert_success(&result);
    let run_result = result.unwrap();

    TestAssertions::assert_run_success(&run_result);
    TestAssertions::assert_stdout_contains(&run_result, "test with args");

    Ok(())
}

#[tokio::test]
async fn test_command_failure() -> Result<(), CleanroomError> {
    let result = run(["sh", "-c", "exit 42"]);

    TestAssertions::assert_success(&result);
    let run_result = result.unwrap();

    TestAssertions::assert_run_failure_with_code(&run_result, 42);

    Ok(())
}

#[tokio::test]
async fn test_command_isolation() -> Result<(), CleanroomError> {
    // Execute multiple commands to test isolation
    let result1 = run(["echo", "first command"]);
    let result2 = run(["echo", "second command"]);

    TestAssertions::assert_success(&result1);
    TestAssertions::assert_success(&result2);

    let run_result1 = result1.unwrap();
    let run_result2 = result2.unwrap();

    TestAssertions::assert_run_success(&run_result1);
    TestAssertions::assert_run_success(&run_result2);

    TestAssertions::assert_stdout_contains(&run_result1, "first command");
    TestAssertions::assert_stdout_contains(&run_result2, "second command");

    Ok(())
}

#[tokio::test]
async fn test_command_performance() -> Result<(), CleanroomError> {
    let start = std::time::Instant::now();

    // Execute a simple command
    let result = run(["echo", "performance test"]);

    let duration = start.elapsed();

    TestAssertions::assert_success(&result);
    let run_result = result.unwrap();
    TestAssertions::assert_run_success(&run_result);

    // Should complete quickly due to mocking
    TestAssertions::assert_duration_less_than(duration, Duration::from_millis(100));

    Ok(())
}

#[tokio::test]
async fn test_multiple_commands() -> Result<(), CleanroomError> {
    let start = std::time::Instant::now();

    // Execute multiple commands
    let results = vec![
        run(["echo", "command1"]),
        run(["echo", "command2"]),
        run(["echo", "command3"]),
        run(["echo", "command4"]),
        run(["echo", "command5"]),
    ];

    let duration = start.elapsed();

    // All commands should succeed
    for result in results {
        TestAssertions::assert_success(&result);
        let run_result = result.unwrap();
        TestAssertions::assert_run_success(&run_result);
    }

    // Multiple commands should still be fast
    TestAssertions::assert_duration_less_than(duration, Duration::from_millis(200));

    Ok(())
}

#[tokio::test]
async fn test_command_with_variables() -> Result<(), CleanroomError> {
    let result = run(["sh", "-c", "TEST_VAR=hello; echo $TEST_VAR"]);

    TestAssertions::assert_success(&result);
    let run_result = result.unwrap();

    TestAssertions::assert_run_success(&run_result);
    TestAssertions::assert_stdout_contains(&run_result, "hello");

    Ok(())
}

#[tokio::test]
async fn test_command_with_pipes() -> Result<(), CleanroomError> {
    let result = run(["sh", "-c", "echo 'hello world' | tr 'a-z' 'A-Z'"]);

    TestAssertions::assert_success(&result);
    let run_result = result.unwrap();

    TestAssertions::assert_run_success(&run_result);
    TestAssertions::assert_stdout_contains(&run_result, "HELLO WORLD");

    Ok(())
}

#[tokio::test]
async fn test_command_error_handling() -> Result<(), CleanroomError> {
    // Test command that should fail
    let result = run(["sh", "-c", "exit 1"]);

    TestAssertions::assert_success(&result);
    let run_result = result.unwrap();

    TestAssertions::assert_run_failure_with_code(&run_result, 1);

    Ok(())
}
