//! Document parsing library for extracting text from various file formats.
//!
//! This crate provides functionality for parsing and extracting text content from
//! different file formats including PDFs, Office documents (DOCX, XLSX, PPTX),
//! text files, and images (using OCR).
//!
//! # Features
//!
//! * Automatic file format detection based on content
//! * Support for various document types:
//!   * PDF documents
//!   * Microsoft Office formats (DOCX, XLSX, PPTX)
//!   * Plain text and structured text (TXT, CSV, JSON)
//!   * Images with text content via OCR (PNG, JPEG, WebP)
//! * Memory-efficient processing with minimal temporary file usage
//! * Consolidated error handling with descriptive error messages
//!
//! # Examples
//!
//! ```no_run
//! use parser_core::parse;
//! use std::fs;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Read a file
//! let data = fs::read("document.pdf")?;
//!
//! // Parse it to extract text
//! let text = parse(&data)?;
//! println!("{}", text);
//! # Ok(())
//! # }
//! ```

mod constants;
mod errors;
mod parsers;

pub use errors::ParserError;
pub use parsers::parse;
