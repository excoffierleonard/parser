#!/bin/bash

# Exit on any error
set -e

# For static linking, may work one day
# sudo apt update
# sudo apt install -y \
#     libtesseract-dev \
#     libleptonica-dev \
#     libclang-dev


# Install dependencies
sudo apt update
sudo apt install -y \
    musl-tools \
    tesseract-ocr \
    tesseract-ocr-eng \
    tesseract-ocr-fra

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
rustup target add x86_64-unknown-linux-musl
cargo build --target x86_64-unknown-linux-musl --release

# Build Docker image
docker compose build