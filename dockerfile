##############################
# Stage 1: Prepare the Recipe
##############################
FROM rust:alpine AS chef
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN apk add --no-cache tesseract-ocr-dev leptonica-dev clang-dev tesseract-ocr-data-eng tesseract-ocr-data-fra
RUN cargo install cargo-chef
WORKDIR /app
# Copy only the files needed to generate the recipe (e.g., Cargo.toml, Cargo.lock, and source files)
COPY . .
# Create the recipe file that captures your dependency graph.
RUN cargo chef prepare --recipe-path recipe.json

##############################
# Stage 2: Cache Dependencies
##############################
FROM rust:alpine AS builder
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN apk add --no-cache tesseract-ocr-dev leptonica-dev clang-dev tesseract-ocr-data-eng tesseract-ocr-data-fra
RUN cargo install cargo-chef
WORKDIR /app
# Copy the pre-generated recipe
COPY --from=chef /app/recipe.json recipe.json
# Build (or “cook”) the dependencies from the recipe. This layer is cached until your dependencies change.
RUN cargo chef cook --release --recipe-path recipe.json
# Now copy the full source and compile the application.
COPY . .
RUN cargo build --release

##############################
# Stage 3: Final Image
##############################
FROM alpine
RUN apk add --no-cache tesseract-ocr-data-eng tesseract-ocr-data-fra
WORKDIR /app
# Copy the statically linked binary from the builder stage
COPY --from=builder /app/target/release/parser-bin .
EXPOSE 8080
CMD ["./parser-bin"]