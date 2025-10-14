# CI/CD Pipeline Guide

## Overview

The clnrm project uses GitHub Actions for comprehensive continuous integration and deployment automation. This guide covers the CI/CD pipeline architecture, workflows, and best practices.

## Pipeline Architecture

The CI/CD system consists of two primary workflows:

1. **CI Workflow** (`.github/workflows/ci.yml`) - Continuous Integration
2. **Release Workflow** (`.github/workflows/release.yml`) - Release Automation

### CI/CD Philosophy: 80/20 Rule

Our pipeline focuses on the 20% of automation that provides 80% of confidence:

- **Fast feedback loops** - Quick checks run first
- **Parallel execution** - Independent jobs run concurrently
- **Smart caching** - Minimize redundant work
- **Fail fast** - Catch issues early in the pipeline

## CI Workflow

### Trigger Events

```yaml
on:
  push:
    branches: [master, develop]
  pull_request:
    branches: [master]
  schedule:
    - cron: '0 0 * * 1'  # Weekly security audit
```

### Pipeline Stages

#### 1. Quick Checks (Fast Fail)

Runs first to catch obvious issues quickly:

```bash
# Formatting check
cargo fmt --all -- --check

# Linting with clippy
cargo clippy --all-targets --all-features -- -D warnings

# Documentation build
cargo doc --no-deps --all-features
```

**Time:** ~2 minutes
**Purpose:** Catch code style and obvious issues before expensive tests

#### 2. Multi-Platform Build

Builds on Linux, macOS, and Windows with stable and nightly Rust:

```yaml
strategy:
  matrix:
    os: [ubuntu-latest, macos-latest, windows-latest]
    rust: [stable, nightly]
```

**Time:** ~5-8 minutes per platform
**Purpose:** Ensure cross-platform compatibility

#### 3. Test Suite

Comprehensive testing across platforms:

```bash
# Unit tests
cargo test --lib --all-features

# Integration tests
cargo test --test '*' --all-features

# Doc tests
cargo test --doc --all-features

# Example tests
cargo test --examples --all-features
```

**Time:** ~10-15 minutes per platform
**Purpose:** Verify functionality across all test types

#### 4. Code Coverage

Generates coverage reports using `cargo-llvm-cov`:

```bash
# Generate coverage
cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info

# Enforce 80% threshold
if coverage < 80%; then exit 1; fi
```

**Time:** ~8-12 minutes
**Purpose:** Ensure adequate test coverage
**Threshold:** 80% minimum coverage required

Coverage reports are uploaded to Codecov for tracking.

#### 5. Security Audit

Checks for security vulnerabilities:

```bash
# Audit dependencies
cargo audit --deny warnings

# Check licenses and supply chain
cargo deny check
```

**Time:** ~2-3 minutes
**Purpose:** Identify security vulnerabilities and license issues

#### 6. Benchmark Tests

Validates benchmark compilation and execution:

```bash
# Build benchmarks
cargo bench --no-run --all-features

# Quick benchmark run
cargo bench --all-features -- --quick
```

**Time:** ~5-7 minutes
**Purpose:** Ensure benchmarks compile and run without crashes

#### 7. Dependency Review

For pull requests, reviews dependency changes:

```yaml
- uses: actions/dependency-review-action@v4
  with:
    fail-on-severity: moderate
```

**Time:** ~1-2 minutes
**Purpose:** Catch supply chain security issues

### Total CI Time

- **Fast path (no changes):** ~15-20 minutes
- **Full matrix (all platforms):** ~40-60 minutes
- **Cached builds:** Can reduce by 30-50%

## Release Workflow

### Trigger Events

```yaml
on:
  push:
    tags:
      - 'v*.*.*'
  workflow_dispatch:
    inputs:
      version:
        description: 'Version to release'
        required: true
```

### Release Stages

#### 1. Pre-Release Validation

Validates version and runs full test suite:

```bash
# Check version matches
CARGO_VERSION=$(grep '^version =' Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')
if [[ "$CARGO_VERSION" != "$RELEASE_VERSION" ]]; then exit 1; fi

# Full validation
cargo test --all-features --all-targets
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
cargo doc --no-deps --all-features
```

**Purpose:** Ensure release quality before building artifacts

#### 2. Build Release Artifacts

Builds optimized binaries for multiple platforms:

**Supported Targets:**
- `x86_64-unknown-linux-gnu` - Linux x86_64
- `x86_64-apple-darwin` - macOS Intel
- `aarch64-apple-darwin` - macOS Apple Silicon
- `x86_64-pc-windows-msvc` - Windows x86_64

```bash
# Build release binary
cargo build --release --target $TARGET --bin cleanroom

# Package with checksums
tar czf clnrm-$PLATFORM.tar.gz cleanroom
shasum -a 256 clnrm-$PLATFORM.tar.gz > clnrm-$PLATFORM.tar.gz.sha256
```

**Time:** ~10-15 minutes per platform
**Output:** Platform-specific archives with SHA256 checksums

#### 3. Generate Changelog

Automatically generates structured changelog:

```markdown
## What's Changed

### Features
- New features from commits

### Bug Fixes
- Bug fixes from commits

### Performance
- Performance improvements

### Documentation
- Documentation changes

### Other Changes
- Other commits
```

Changelog is parsed from git commit messages between tags.

#### 4. Publish to crates.io

Publishes the crate to the Rust package registry:

```bash
cargo publish --token $CARGO_TOKEN
```

**Requirements:**
- `CARGO_TOKEN` secret must be configured
- Version must not already exist on crates.io

#### 5. Create GitHub Release

Creates GitHub release with:
- Release notes from generated changelog
- Binary artifacts for all platforms
- SHA256 checksums for verification

```yaml
- uses: softprops/action-gh-release@v1
  with:
    tag_name: v$VERSION
    name: Release v$VERSION
    body: $CHANGELOG
    files: artifacts/**/*
```

### Total Release Time

- **Full release pipeline:** ~45-60 minutes
- **Artifact generation:** ~40 minutes
- **Publishing:** ~5-10 minutes

## Caching Strategy

GitHub Actions cache is used extensively to speed up builds:

### Cache Keys

```yaml
key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
restore-keys: |
  ${{ runner.os }}-cargo-
```

### Cached Directories

- `~/.cargo/bin/` - Installed tools (cargo-llvm-cov, cargo-audit, etc.)
- `~/.cargo/registry/index/` - Crate registry index
- `~/.cargo/registry/cache/` - Downloaded crate source
- `~/.cargo/git/db/` - Git dependencies
- `target/` - Build artifacts

### Cache Performance

- **Cold cache:** Full build ~15-20 minutes
- **Warm cache:** Incremental build ~3-5 minutes
- **Cache hit rate:** Typically 80-90%

## Security Best Practices

### Secrets Management

Required secrets in GitHub repository settings:

1. **CARGO_TOKEN** - For crates.io publishing
2. **CODECOV_TOKEN** - For coverage reports (optional but recommended)
3. **GITHUB_TOKEN** - Auto-provided by GitHub Actions

### Security Scanning

1. **cargo-audit** - Checks RustSec advisory database
2. **cargo-deny** - License and supply chain validation
3. **dependency-review-action** - GitHub security advisories
4. **Dependabot** - Automated dependency updates (configure separately)

### Permissions

Workflows use minimal required permissions:

```yaml
permissions:
  contents: read  # Default for CI
  contents: write  # Only for release workflow
```

## Optimization Tips

### Speed Up CI

1. **Enable caching** - Already configured
2. **Run quick checks first** - Fail fast on formatting/clippy
3. **Use matrix strategically** - Only run nightly on Ubuntu
4. **Skip benchmarks on PR** - Add condition: `if: github.event_name == 'push'`

### Reduce Resource Usage

```yaml
strategy:
  fail-fast: false  # Continue other jobs if one fails
  matrix:
    exclude:
      - os: macos-latest
        rust: nightly
```

### Conditional Execution

```yaml
- name: Run expensive test
  if: github.event_name == 'push' && github.ref == 'refs/heads/master'
  run: cargo test --release --all-features
```

## Pre-Commit Hooks

For local development, set up pre-commit hooks:

### Install Git Hooks

```bash
# Create hooks directory
mkdir -p .git/hooks

# Create pre-commit hook
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
set -e

echo "Running pre-commit checks..."

# Format check
echo "Checking formatting..."
cargo fmt --all -- --check

# Clippy
echo "Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

# Quick test
echo "Running quick tests..."
cargo test --lib

echo "✅ All pre-commit checks passed!"
EOF

# Make executable
chmod +x .git/hooks/pre-commit
```

### Recommended Pre-Commit Checks

1. **Formatting** - `cargo fmt --check`
2. **Linting** - `cargo clippy`
3. **Unit tests** - `cargo test --lib` (fast subset)
4. **Security audit** - `cargo audit` (optional, can be slow)

## Troubleshooting

### Common CI Failures

#### Coverage Threshold Failure

```bash
❌ Coverage 78.5% is below threshold 80.0%
```

**Solution:** Add more tests or adjust threshold in CI workflow

#### Clippy Warnings

```bash
error: this expression creates a reference which is immediately dereferenced
```

**Solution:** Run `cargo clippy --fix` locally

#### Test Timeouts

```bash
error: test failed, to rerun pass '--test integration_tests'
```

**Solution:**
- Check for Docker availability in tests
- Increase timeout: `cargo test -- --test-threads=1 --nocapture`

#### Build Cache Issues

```bash
error: failed to load source for dependency
```

**Solution:** Clear cache and rebuild
```bash
cargo clean
rm -rf ~/.cargo/registry/cache
cargo build
```

### Release Failures

#### Version Mismatch

```bash
❌ Version mismatch: Cargo.toml (0.1.0) != Release (0.2.0)
```

**Solution:** Update version in `Cargo.toml` before tagging

#### crates.io Publish Failure

```bash
error: crate version 0.2.0 is already uploaded
```

**Solution:** Version already published, increment version number

## Monitoring & Metrics

### GitHub Actions Metrics

Monitor in repository insights:
- Workflow run times
- Success/failure rates
- Cache hit rates
- Resource usage

### Coverage Trends

Track on Codecov dashboard:
- Overall coverage percentage
- Coverage per file/module
- Coverage trends over time
- Pull request coverage diff

### Performance Tracking

Use benchmark results to track:
- Execution time trends
- Memory usage patterns
- Regression detection

## Continuous Improvement

### Quarterly Review Checklist

- [ ] Review average CI run time
- [ ] Check cache efficiency
- [ ] Update Rust toolchain versions
- [ ] Review security audit findings
- [ ] Optimize test execution order
- [ ] Update dependencies
- [ ] Review failure patterns
- [ ] Adjust coverage thresholds

### Suggested Enhancements

1. **Performance tracking** - Store benchmark results
2. **Artifact storage** - Archive test results
3. **Notification system** - Slack/Discord integration
4. **Nightly builds** - Scheduled comprehensive testing
5. **Documentation deployment** - Auto-deploy docs to GitHub Pages

## References

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov)
- [cargo-audit](https://github.com/rustsec/rustsec/tree/main/cargo-audit)
- [cargo-deny](https://github.com/EmbarkStudios/cargo-deny)
- [Rust CI Best Practices](https://matklad.github.io/2021/09/04/fast-rust-builds.html)

## Support

For CI/CD issues:
1. Check workflow logs in GitHub Actions tab
2. Review this guide for troubleshooting
3. Open an issue with the `ci/cd` label
4. Contact maintainers for access to secrets

---

**Last Updated:** 2025-10-13
**Maintainer:** CI/CD Engineering Team
