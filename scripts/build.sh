#!/bin/bash

# Exit on any error
set -e

# Clean and check
cargo clean
cargo check
cargo clippy -- -D warnings
cargo fmt
cargo audit
# Maybe use it when it is out of nightly
# cargo udeps

# Documentation
cargo doc --no-deps

# Testing and benchmarking
cargo test
cargo bench
cargo test -- --ignored 

# Build for production
cargo build --target x86_64-unknown-linux-musl --release

# Build Docker image
docker compose build