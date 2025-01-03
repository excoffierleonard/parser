#!/bin/bash

# Exit on any error
set -e

# Clean and check
cargo update
cargo clean
cargo check --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --all
cargo audit
# Maybe use it when it is out of nightly
# cargo udeps --workspace

# Documentation
cargo doc --workspace --no-deps

# Testing and benchmarking
cargo test --workspace
cargo bench --workspace
cargo test --workspace -- --ignored 

# Build for production
cargo build --target x86_64-unknown-linux-musl --release

# Build Docker image
docker compose build