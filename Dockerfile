##############################
# Stage 1: Prepare the Recipe
##############################
FROM rust:alpine AS chef
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN apk add --no-cache tesseract-ocr-dev leptonica-dev clang-dev
RUN cargo install cargo-chef
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

##############################
# Stage 2: Cache Dependencies
##############################
FROM rust:alpine AS builder
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN apk add --no-cache tesseract-ocr-dev leptonica-dev clang-dev
RUN cargo install cargo-chef
WORKDIR /app
COPY --from=chef /app/recipe.json .
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

##############################
# Stage 3: Final Image
##############################
FROM alpine
RUN apk add --no-cache tesseract-ocr
WORKDIR /app
COPY --from=builder /app/target/release/parser .
ENTRYPOINT ["./parser"]