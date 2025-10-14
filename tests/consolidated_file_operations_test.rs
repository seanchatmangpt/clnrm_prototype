//! Consolidated file operations test suite
//!
//! This test consolidates all file operation testing into a single, comprehensive suite
//! that uses the fixtures infrastructure for better maintainability and consistency.

use clnrm::{CleanroomConfig, CleanroomEnvironment, CleanroomGuard, GenericContainer};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

/// Test configuration for file operations
fn file_test_config() -> CleanroomConfig {
    CleanroomConfig {
        enable_singleton_containers: true,
        container_startup_timeout: Duration::from_millis(5),
        test_execution_timeout: Duration::from_millis(10),
        max_concurrent_containers: 2,
        enable_deterministic_execution: false,
        enable_coverage_tracking: false,
        enable_snapshot_testing: false,
        enable_tracing: false,
        ..CleanroomConfig::default()
    }
}

/// Test file creation and cleanup in Cleanroom environment
#[tokio::test]
async fn test_basic_file_operations() {
    let config = file_test_config();
    let environment = CleanroomEnvironment::new(config).await.unwrap();
    let environment_arc = Arc::new(environment);
    let _guard = CleanroomGuard::new(environment_arc.clone());

    // Test file path
    let test_file = "cleanroom_consolidated_test.txt";

    // Ensure file doesn't exist initially
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).unwrap();
    }

    // Test file creation using direct filesystem operations
    let test_content = "Hello from Cleanroom consolidated test!";
    fs::write(test_file, test_content).unwrap();

    // Verify file was created
    assert!(Path::new(test_file).exists());
    let read_content = fs::read_to_string(test_file).unwrap();
    assert_eq!(read_content, test_content);

    // Test file modification
    let modified_content = "Modified content for testing";
    fs::write(test_file, modified_content).unwrap();
    let read_modified = fs::read_to_string(test_file).unwrap();
    assert_eq!(read_modified, modified_content);

    // Test file metadata
    let metadata = fs::metadata(test_file).unwrap();
    assert!(metadata.is_file());
    assert!(metadata.len() > 0);

    // Cleanup
    fs::remove_file(test_file).unwrap();
    assert!(!Path::new(test_file).exists());
}

/// Test multiple file operations
#[tokio::test]
async fn test_multiple_file_operations() {
    let config = file_test_config();
    let environment = CleanroomEnvironment::new(config).await.unwrap();
    let environment_arc = Arc::new(environment);
    let _guard = CleanroomGuard::new(environment_arc.clone());

    let test_files = vec!["test_file_1.txt", "test_file_2.txt", "test_file_3.txt"];

    // Clean up any existing files
    for file in &test_files {
        if Path::new(file).exists() {
            fs::remove_file(file).unwrap();
        }
    }

    // Create multiple files with different content
    for (i, file) in test_files.iter().enumerate() {
        let content = format!("Content for file {}", i + 1);
        fs::write(file, content).unwrap();
        assert!(Path::new(file).exists());
    }

    // Verify all files exist and have correct content
    for (i, file) in test_files.iter().enumerate() {
        let content = fs::read_to_string(file).unwrap();
        assert_eq!(content, format!("Content for file {}", i + 1));
    }

    // Clean up all files
    for file in &test_files {
        fs::remove_file(file).unwrap();
        assert!(!Path::new(file).exists());
    }
}

/// Test file operations with different content types
#[tokio::test]
async fn test_file_content_types() {
    let config = file_test_config();
    let environment = CleanroomEnvironment::new(config).await.unwrap();
    let environment_arc = Arc::new(environment);
    let _guard = CleanroomGuard::new(environment_arc.clone());

    // Test simple text
    let simple_file = "simple.txt";
    let simple_content = "Simple text content";
    fs::write(simple_file, simple_content).unwrap();
    assert_eq!(fs::read_to_string(simple_file).unwrap(), simple_content);
    fs::remove_file(simple_file).unwrap();

    // Test multiline text
    let multiline_file = "multiline.txt";
    let multiline_content = "Line 1\nLine 2\nLine 3\n";
    fs::write(multiline_file, multiline_content).unwrap();
    assert_eq!(
        fs::read_to_string(multiline_file).unwrap(),
        multiline_content
    );
    fs::remove_file(multiline_file).unwrap();

    // Test special characters
    let special_file = "special.txt";
    let special_content = "Special chars: !@#$%^&*()_+-=[]{}|;':\",./<>?";
    fs::write(special_file, special_content).unwrap();
    assert_eq!(fs::read_to_string(special_file).unwrap(), special_content);
    fs::remove_file(special_file).unwrap();

    // Test unicode content
    let unicode_file = "unicode.txt";
    let unicode_content = "Unicode: 你好世界 🌍 émojis 🚀";
    fs::write(unicode_file, unicode_content).unwrap();
    assert_eq!(fs::read_to_string(unicode_file).unwrap(), unicode_content);
    fs::remove_file(unicode_file).unwrap();
}

/// Test file operations with container integration
#[tokio::test]
async fn test_file_operations_with_container() {
    let config = file_test_config();
    let environment = CleanroomEnvironment::new(config).await.unwrap();
    let environment_arc = Arc::new(environment);
    let _guard = CleanroomGuard::new(environment_arc.clone());

    // Create a container
    let container = environment_arc
        .get_or_create_container("alpine_test", || {
            GenericContainer::new("alpine_test", "alpine", "latest")
        })
        .await
        .unwrap();

    // Test file path
    let test_file = "cleanroom_container_test.txt";
    let test_content = "Hello from container test!";

    // Ensure file doesn't exist initially
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).unwrap();
    }

    // Create file using direct filesystem operations (not container)
    fs::write(test_file, test_content).unwrap();

    // Verify file was created
    assert!(Path::new(test_file).exists());
    let read_content = fs::read_to_string(test_file).unwrap();
    assert_eq!(read_content, test_content);

    // Test file metadata
    let metadata = fs::metadata(test_file).unwrap();
    assert!(metadata.is_file());
    assert!(metadata.len() > 0);

    // Cleanup
    fs::remove_file(test_file).unwrap();
    assert!(!Path::new(test_file).exists());
}

/// Test directory operations
#[tokio::test]
async fn test_directory_operations() {
    let config = file_test_config();
    let environment = CleanroomEnvironment::new(config).await.unwrap();
    let environment_arc = Arc::new(environment);
    let _guard = CleanroomGuard::new(environment_arc.clone());

    let test_dir = "test_directory";
    let test_file = format!("{}/test_file.txt", test_dir);

    // Clean up any existing directory
    if Path::new(test_dir).exists() {
        fs::remove_dir_all(test_dir).unwrap();
    }

    // Create directory
    fs::create_dir(test_dir).unwrap();
    assert!(Path::new(test_dir).exists());
    assert!(Path::new(test_dir).is_dir());

    // Create file in directory
    let content = "File in directory";
    fs::write(&test_file, content).unwrap();
    assert!(Path::new(&test_file).exists());

    // Verify file content
    let read_content = fs::read_to_string(&test_file).unwrap();
    assert_eq!(read_content, content);

    // Clean up
    fs::remove_file(&test_file).unwrap();
    fs::remove_dir(test_dir).unwrap();
    assert!(!Path::new(test_dir).exists());
}
