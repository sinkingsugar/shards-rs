set fallback := true

# Default recipe
default:
    @just --list

# Generate/update Cargo.lock
lock:
    cargo generate-lockfile

# Build with default features
build:
    cargo build

# Build with all features
build-all:
    cargo build --all-features

# Check with all features
check:
    cargo check --all-features

# Run tests
test:
    cargo test

# Run tests with all features
test-all:
    cargo test --all-features

# Format code
fmt:
    cargo fmt

# Check formatting
fmt-check:
    cargo fmt --all -- --check

# Run clippy
clippy:
    cargo clippy -- -D warnings

# Clean build artifacts
clean:
    cargo clean

# Prepare for release (generate lock, check, test)
release-prep:
    @echo "Generating lock file..."
    cargo generate-lockfile
    @echo "Checking all features..."
    cargo check --all-features
    @echo "Running tests..."
    cargo test --all-features
    @echo "Checking formatting..."
    cargo fmt --all -- --check
    @echo "Release prep complete!"

# Full CI check locally
ci:
    cargo fmt --all -- --check
    cargo check --locked --all-features
    cargo build --locked
    cargo test --locked
