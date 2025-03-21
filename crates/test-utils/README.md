# Parser Test Utilities

Shared testing utilities and sample files for the Parser project ecosystem.

## Features

- Standardized test file access across workspace crates
- Common test helper functions
- Comprehensive sample files covering all supported formats

## Test Assets

The `assets` directory contains sample files for testing:

- PDF files (`.pdf`)
- Office documents (`.docx`, `.xlsx`, `.pptx`)
- Plain text files (`.txt`, `.csv`, `.json`)
- Images (`.png`, `.jpg`, `.webp`)

All test files are small and contain known content for predictable testing.

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

## Available Helpers

- `test_file_path(filename)` - Returns the absolute path to a test file
- `read_test_file(filename)` - Reads a test file and returns its contents as `Vec<u8>`
- `get_test_file_list()` - Returns a list of all available test files
- `create_temp_file(extension, content)` - Creates a temporary test file with the given content