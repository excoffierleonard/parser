# Parser Project Guide

## Build & Test Commands

- Build all crates: `cargo build`
- Build release: `cargo build --release`
- Run tests: `cargo test --workspace`
- Run specific test: `cargo test test_name`
- Run benchmarks: `cargo bench --workspace`
- Lint: `cargo clippy --workspace -- -D warnings`
- Format: `cargo fmt --all`
- Build script: `./scripts/build.sh`
- Run web API: `cargo run -p parser-web`
- Run CLI: `cargo run -p parser-cli -- <FILES>`

## Code Style Guidelines

- Use snake_case for variables/functions, PascalCase for types/enums
- Document crates with //! and public items with /// comments
- Group imports: std first, then external crates, then local modules
- Follow the Rust API Guidelines for public interfaces
- Use ? operator for error propagation
- Create custom error types that implement std::error::Error
- Organize modules by functionality, not implementation details
- Use rayon for parallelism where appropriate (par_iter instead of iter)
- Write tests for all public functionality
- Maintain modularity between core, web, and CLI components
- Use Docker for containerization (see compose.yaml and dockerfile)
