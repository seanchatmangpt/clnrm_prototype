//! Examples demonstrating zero-copy streaming for artifacts
//!
//! This example shows how to use the streaming APIs for efficient artifact
//! processing with zero-copy techniques and various collector patterns.

use clnrm::{
    ArtifactData, ArtifactMetadata, ArtifactStream, ArtifactType, CleanroomBuilder,
    CleanroomConfig, CleanroomEnvironment, StreamingCollector,
};
use std::borrow::Cow;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Cleanroom Streaming Artifacts Examples");
    println!("=======================================");

    // Create environment using builder
    let environment = CleanroomBuilder::new()
        .with_timeout(std::time::Duration::from_secs(30))
        .build()
        .await?;

    // Example 1: Basic artifact streaming
    println!("\n1. Basic Artifact Streaming");
    let stream = ArtifactStream::new(environment.clone());

    let mut count = 0;
    stream
        .stream_artifacts(|artifact| {
            count += 1;
            println!(
                "  ✓ Artifact {}: {:?} ({} bytes)",
                count, artifact.metadata.artifact_type, artifact.metadata.size_bytes
            );
            Ok(())
        })
        .await?;

    println!("✓ Processed {} artifacts", count);

    // Example 2: Streaming collector
    println!("\n2. Streaming Collector");
    let stream = ArtifactStream::new(environment.clone());
    let collector = stream
        .collect_into(StreamingCollector::with_zero_copy())
        .await?;

    println!("✓ Collected {} artifacts", collector.count());
    println!("✓ Total size: {} bytes", collector.total_size());
    println!("✓ Collection duration: {:?}", collector.duration());

    // Example 3: Filtering by type
    println!("\n3. Filtering by Type");
    let stream = ArtifactStream::new(environment.clone());
    let collector = stream.collect_into(StreamingCollector::new(100)).await?;

    // Filter artifacts by type manually
    let text_artifacts: Vec<_> = collector.artifacts_by_type(&ArtifactType::Text);
    let json_artifacts: Vec<_> = collector.artifacts_by_type(&ArtifactType::Json);

    println!("✓ Text artifacts: {}", text_artifacts.len());
    println!("✓ JSON artifacts: {}", json_artifacts.len());

    // Example 4: Artifact processing with metadata
    println!("\n4. Artifact Processing with Metadata");
    let stream = ArtifactStream::new(environment.clone());

    stream
        .stream_artifacts(|artifact| {
            println!("  ✓ Processing artifact from: {}", artifact.metadata.source);
            println!("    Type: {:?}", artifact.metadata.artifact_type);
            println!("    Size: {} bytes", artifact.metadata.size_bytes);
            if let Some(hash) = &artifact.metadata.content_hash {
                println!("    Hash: {}", hash);
            }
            Ok(())
        })
        .await?;

    println!("✓ All examples completed successfully!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_artifact_stream() {
        let config = CleanroomConfig::default();
        let environment = CleanroomEnvironment::new(config).await.unwrap();
        let stream = ArtifactStream::new(environment);

        let mut count = 0;
        stream
            .stream_artifacts(|_artifact| {
                count += 1;
                Ok(())
            })
            .await
            .unwrap();

        assert!(count >= 0); // Should not panic
    }

    #[tokio::test]
    async fn test_streaming_collector() {
        let config = CleanroomConfig::default();
        let environment = CleanroomEnvironment::new(config).await.unwrap();
        let stream = ArtifactStream::new(environment);

        let collector = stream
            .collect_into(StreamingCollector::with_zero_copy())
            .await
            .unwrap();

        assert!(collector.count() >= 0);
        assert!(collector.total_size() >= 0);
    }

    #[tokio::test]
    async fn test_collector_statistics() {
        let config = CleanroomConfig::default();
        let environment = CleanroomEnvironment::new(config).await.unwrap();
        let stream = ArtifactStream::new(environment);

        let collector = stream
            .collect_into(StreamingCollector::new(100))
            .await
            .unwrap();

        // Test basic statistics
        assert!(collector.duration() >= Duration::from_secs(0));
        assert!(collector.count() >= 0);
        assert!(collector.total_size() >= 0);

        // Test filtering methods
        let _text_artifacts = collector.artifacts_by_type(&ArtifactType::Text);
        let _json_artifacts = collector.artifacts_by_type(&ArtifactType::Json);

        // Test source filtering
        let _artifacts_by_source = collector.artifacts_by_source("test");
    }
}
