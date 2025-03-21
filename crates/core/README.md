# Parser Core

The core engine of the parser project, providing functionality for extracting text from various file formats.

## Features

- Parse multiple document formats:
  - PDF files (`.pdf`)
  - Office documents (`.docx`, `.xlsx`, `.pptx`)
  - Plain text files (`.txt`, `.csv`, `.json`)
  - Images with OCR (`.png`, `.jpg`, `.webp`)
- Automatic format detection based on content
- Parallel processing via Rayon
- OCR support with language detection

## System Dependencies

This package requires the following system libraries:

- **Tesseract OCR** - Used for image text extraction
- **Leptonica** - Image processing library used by Tesseract
- **Clang** - Required for some build dependencies

### Installation on Debian/Ubuntu

```bash
sudo apt install libtesseract-dev libleptonica-dev libclang-dev
```

### Installation on macOS

```bash
brew install tesseract
```

### Installation on Windows

Follow the instructions at [Tesseract GitHub repository](https://github.com/tesseract-ocr/tesseract).

## Usage

Add as a dependency in your `Cargo.toml`:

```toml
[dependencies]
parser-core = "0.1.0"
```

Or using cargo:

```bash
cargo add parser-core
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

```bash
cargo test
```

### Benchmarking

```bash
cargo bench
```

### Performance

The library is optimized for both speed and memory usage:

- Streams large files when possible instead of loading entirely into memory
- Uses parallel processing for large documents
- Implements efficient text extraction algorithms for each format
