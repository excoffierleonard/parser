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
cargo check --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --all
cargo audit
# cargo udeps --workspace # Uncomment when out of nightly

# Documentation
cargo doc --workspace --no-deps

# Testing and benchmarking
cargo test --workspace
# cargo bench --workspace # Pretty long so commented out
cargo test --workspace -- --ignored 

# Build for production
cargo build --release

# Build Docker image
docker compose build