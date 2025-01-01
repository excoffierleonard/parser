# Step 1: Build the application with musl target
FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app

RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --target x86_64-unknown-linux-musl --release && \
    rm src/main.rs

COPY src src/

# Since we are embedding static files we need to add that directory to the buid
COPY static static/

RUN touch src/main.rs && \
    cargo build --target x86_64-unknown-linux-musl --release

# Step 2: Create final image
FROM alpine

# TODO: Need to add more language support in the future
RUN apk add --no-cache \
    tesseract-ocr \ 
    tesseract-ocr-data-eng \
    tesseract-ocr-data-fra

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/parser .

CMD ["./parser"]
