# Parser

A Rust library/website for extracting text from various document formats.

[Website](https://parser.excoffierleonard.com)

![Website Preview](website_preview.png)

## Features

- PDF, DOCX, XLSX, PPTX documents
- OCR for images (PNG, JPEG, WebP) with English and French support
- Plain text formats (TXT, CSV, JSON)

## Usage

```rust
use parser::parse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = std::fs::read("document.pdf")?;
    let text = parse(&data)?;
    println!("{}", text);
    Ok(())
}
```

## System Dependencies

Requires Tesseract OCR libraries:

- **Debian/Ubuntu:** `sudo apt install libtesseract-dev libleptonica-dev libclang-dev`
- **macOS:** `brew install tesseract`
- **Windows:** Follow the instructions at [Tesseract GitHub repository](https://github.com/tesseract-ocr/tesseract)

## License

[MIT](LICENSE)
