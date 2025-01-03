# Step 1: Build the application with musl target
FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app

RUN rustup target add x86_64-unknown-linux-musl

## Copy only the manifests first
COPY Cargo.toml Cargo.lock ./
COPY parser-core/Cargo.toml parser-core/Cargo.toml
COPY parser-web/Cargo.toml parser-web/Cargo.toml

## Create dummy source files for all crates
RUN mkdir src parser-core/src parser-web/src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn dummy() {}" > parser-core/src/lib.rs && \
    echo "pub fn dummy() {}" > parser-web/src/lib.rs && \
    cargo build --target x86_64-unknown-linux-musl --release && \
    rm src/main.rs parser-core/src/lib.rs parser-web/src/lib.rs

## Now copy the real source code
COPY parser-core/src parser-core/src/
COPY parser-web/src parser-web/src/
COPY parser-web/static parser-web/static/
COPY src src/

## Build the real application
RUN touch src/main.rs parser-core/src/lib.rs parser-web/src/lib.rs && \
    cargo build --target x86_64-unknown-linux-musl --release

# Step 2: Create final image
FROM alpine

## TODO: Need to add more language support in the future
RUN apk add --no-cache \
    tesseract-ocr \ 
    tesseract-ocr-data-eng \
    tesseract-ocr-data-fra

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/parser .

CMD ["./parser"]