# Parser Test Utilities

This crate provides common testing utilities for the Parser project. It centralizes test files and helper functions that are shared across the different crates in the workspace.

## Assets

The `assets` directory contains sample files used in tests:

- PDF files (`.pdf`)
- Office documents (`.docx`, `.xlsx`, `.pptx`)
- Plain text files (`.txt`, `.csv`, `.json`)
- Images (`.png`, `.jpg`, `.webp`)

## Usage

Add as a dev-dependency in your crate's `Cargo.toml`:

```toml
[dev-dependencies]
parser-test-utils = { workspace = true }
```

Then use the utilities in your tests:

```rust
use parser_test_utils::{read_test_file, test_file_path};

#[test]
fn test_something() {
    // Get path to a test file
    let path = test_file_path("test_pdf_1.pdf");
    
    // Or read a test file directly as bytes
    let data = read_test_file("test_pdf_1.pdf");
    
    // Use the file data in your tests...
}
```