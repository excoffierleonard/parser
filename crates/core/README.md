# Parser Core

The core engine of the parser project, providing functionality for extracting text from various file formats.

## Features

- Parse a wide variety of document formats:
  - PDF files (`.pdf`)
  - Office documents (`.docx`, `.xlsx`, `.pptx`)
  - Plain text files (`.txt`, `.csv`, `.json`)
  - Images with OCR (`.png`, `.jpg`, `.webp`)
- Automatic format detection
- Parallel processing support via Rayon

## Usage

Add as a dependency in your `Cargo.toml`:

```toml
[dependencies]
parser-core = { path = "../core" }  # Adjust path as needed
```

Basic usage:

```rust
use parser_core::parse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read a file
    let data = std::fs::read("document.pdf")?;
    
    // Parse the document
    let text = parse(&data)?;
    
    println!("Extracted text: {}", text);
    
    Ok(())
}
```

## Architecture

The crate is organized around a central `parse` function that:

1. Detects the MIME type of the provided data
2. Routes to the appropriate parser module
3. Returns the extracted text

Each parser is implemented in its own module:

- `docx.rs` - Microsoft Word documents
- `pdf.rs` - PDF documents
- `xlsx.rs` - Microsoft Excel spreadsheets
- `pptx.rs` - Microsoft PowerPoint presentations
- `text.rs` - Plain text formats, including CSV and JSON
- `image.rs` - Image formats using OCR

## Development

### Testing

Run tests with:

```bash
cargo test
```

### Benchmarking

Benchmark sequential vs. parallel parsing:

```bash
cargo bench
```