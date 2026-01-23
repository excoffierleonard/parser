//! Document parsing library for extracting text from various file formats.
//!
//! This crate provides functionality for parsing and extracting text content from
//! different file formats including PDFs, Office documents (DOCX, XLSX, PPTX),
//! text files, and images (using OCR).
//!
//! # Example
//!
//! ```no_run
//! use parser::parse;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let data = std::fs::read("document.pdf")?;
//!     let text = parse(&data)?;
//!     println!("{}", text);
//!     Ok(())
//! }
//! ```

mod core;

pub use core::errors::ParserError;
pub use core::parsers::parse;
