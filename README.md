# Parser

A Rust-based document parsing system that extracts text content from various file formats.

[Live Demo](https://parser.excoffierleonard.com) | [API Endpoint](https://parser.excoffierleonard.com/parse)

![Website Preview](website_preview.png)

## 📚 Overview

Parser is a modular Rust project that provides comprehensive document parsing capabilities through multiple interfaces:

- **Core library**: The foundation providing parsing functionality for various file formats
- **CLI tool**: Command-line interface for quick file parsing
- **Web API**: REST service for parsing files via HTTP requests
- **Web UI**: Simple interface for testing the parser functionality

## 📦 Project Structure

The project is organized as a Rust workspace with multiple crates:

- **parser-core**: The core parsing engine
- **parser-cli**: Command-line interface
- **parser-web**: Web API and frontend
- **test-utils**: Shared testing utilities

## 📄 Supported File Types

- **Documents**: PDF (`.pdf`), Word (`.docx`), PowerPoint (`.pptx`), Excel (`.xlsx`)
- **Text**: Plain text (`.txt`), CSV, JSON, YAML, source code, and other text-based formats
- **Images**: PNG, JPEG, WebP, and other image formats with OCR (Optical Character Recognition)

The OCR functionality supports English and French languages.

## 🛠️ Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/learn/get-started) (latest stable)
- OCR Dependencies:
  - Tesseract development libraries
  - Leptonica development libraries
  - Clang development libraries

#### Installing OCR Dependencies

**Debian/Ubuntu:**

```bash
sudo apt install libtesseract-dev libleptonica-dev libclang-dev
```

**macOS:**

```bash
brew install tesseract
```

**Windows:**
Follow the instructions at [Tesseract GitHub repository](https://github.com/tesseract-ocr/tesseract).

### Building from Source

```bash
# Build all crates
cargo build

# Build in release mode
cargo build --release
```

### Using the CLI

```bash
# Run directly with cargo
cargo run -p parser-cli -- path/to/file1.pdf path/to/file2.docx

# Or use the built binary
./target/release/parser-cli path/to/file1.pdf path/to/file2.docx
```

### Running the Web Server

```bash
# Run the web server
cargo run -p parser-web

# With custom port
PARSER_APP_PORT=9000 cargo run -p parser-web

# With file serving enabled (for frontend)
ENABLE_FILE_SERVING=true cargo run -p parser-web
```

## 🚀 Deployment

The easiest way to deploy the service is using Docker:

```bash
curl -o compose.yaml https://raw.githubusercontent.com/excoffierleonard/parser/refs/heads/main/compose.yaml && \
docker compose up -d
```

### Environment Variables

- `PARSER_APP_PORT`: The port on which the web service listens (default: 8080)
- `ENABLE_FILE_SERVING`: Enable serving frontend files (default: false)

## 🧪 Development

### Testing

```bash
# Run all tests
cargo test --workspace

# Run specific test
cargo test test_name
```

### Benchmarking

```bash
# Run benchmarks
cargo bench --workspace

# Run benchmark script
./scripts/benchmark.sh
```

### Code Quality

```bash
# Run linter
cargo clippy --workspace -- -D warnings

# Format code
cargo fmt --all
```

### Building with Scripts

```bash
# Full build script
./scripts/build.sh

# Deployment tests
./scripts/deploy-tests.sh
```

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
