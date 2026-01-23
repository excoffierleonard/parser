#!/bin/bash

# Exit on any error
set -e

# Install dependencies
sudo apt update
sudo apt install -y \
    libtesseract-dev \
    libleptonica-dev \
    libclang-dev

# Clean and check
cargo update
# cargo clean # Uncomment if previous build is broken
cargo check
cargo clippy -- -D warnings
cargo fmt
cargo audit
# cargo udeps # Uncomment when out of nightly

# Documentation
cargo doc --no-deps

# Testing and benchmarking
cargo test
# cargo bench # Pretty long so commented out
cargo test -- --ignored

# Build for production
cargo build --release

# Build Docker image
docker compose build
