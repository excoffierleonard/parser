#!/bin/bash

# Exit on any error
set -e

# Install dependencies
sudo apt update
sudo apt install -y \
    libtesseract-dev \
    libleptonica-dev

# Clean and check
cargo update
cargo clean
cargo check --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --all
cargo audit
cargo udeps --workspace

# Documentation
cargo doc --workspace --no-deps

# Testing and benchmarking
cargo test --workspace
cargo bench --workspace
cargo test --workspace -- --ignored 

# Build for production
cargo build --release

# Build Docker image
docker compose build