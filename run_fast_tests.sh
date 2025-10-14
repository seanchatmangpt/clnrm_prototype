#!/bin/bash
# Fast test runner for cleanroom testing framework
# This script runs only fast tests, skipping Docker-dependent and slow tests

set -e

echo "🚀 Running fast tests for cleanroom testing framework..."

# Set environment variable to skip slow tests
export SKIP_SLOW_TESTS=1
export SKIP_DOCKER_TESTS=1

# Run only fast tests
echo "📋 Running unit tests..."
cargo test --lib -- --test-threads=1

echo "📋 Running fast integration tests..."
cargo test fast_integration_tests -- --test-threads=1

echo "📋 Running BDD tests (optimized)..."
cargo test bdd_tests -- --test-threads=1

echo "📋 Running property tests..."
cargo test property_tests -- --test-threads=1

echo "📋 Running coverage improvement tests..."
cargo test coverage_improvement_tests -- --test-threads=1

echo "📋 Running validator tests..."
cargo test validator_test -- --test-threads=1

echo "✅ Fast tests completed successfully!"

# Optional: Run benchmarks with reduced sample sizes
if [ "$RUN_BENCHMARKS" = "1" ]; then
    echo "📊 Running optimized benchmarks..."
    cargo bench --bench production_benchmarks
    cargo bench --bench container_lifecycle
fi

echo "🎉 All fast tests passed!"
