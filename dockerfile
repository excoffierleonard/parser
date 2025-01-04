# Step 1: Build the application
FROM rust:alpine AS builder

RUN apk add --no-cache \
    tesseract-ocr-dev \
    leptonica-dev \
    clang-dev \
    build-base \
    pkgconf

# Set the environment variables for bindgen
ENV LIBCLANG_PATH=/usr/lib
ENV BINDGEN_EXTRA_CLANG_ARGS="-I/usr/include"
ENV RUSTFLAGS="-C target-feature=-crt-static"

WORKDIR /app

## Copy only the manifests first
COPY Cargo.toml Cargo.lock ./
COPY parser-core/Cargo.toml parser-core/Cargo.toml
COPY parser-web/Cargo.toml parser-web/Cargo.toml

## Create dummy source files for all crates
RUN mkdir src parser-core/src parser-web/src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn dummy() {}" > parser-core/src/lib.rs && \
    echo "pub fn dummy() {}" > parser-web/src/lib.rs && \
    cargo build --release && \
    rm src/main.rs parser-core/src/lib.rs parser-web/src/lib.rs

## Now copy the real source code
COPY parser-core/src parser-core/src/
COPY parser-web/src parser-web/src/
COPY parser-web/static parser-web/static/
COPY src src/

## Build the real application
RUN touch src/main.rs parser-core/src/lib.rs parser-web/src/lib.rs && \
    cargo build --release

# Step 2: Create final image
FROM alpine

RUN apk add --no-cache \
    tesseract-ocr-data-eng \
    tesseract-ocr-data-fra

WORKDIR /app

## TODO: Need to add more language support in the future
COPY --from=builder /app/target/release/parser .

CMD ["./parser"]