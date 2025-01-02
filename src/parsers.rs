//! Parsing module for various file formats.
//!
//! This module serves as the central entry point for all parsing functions,
//! providing a unified interface for different file formats like PDF, CSV, etc.
//! Each specific parser is implemented in its own submodule.

mod docx;
mod image;
mod pdf;
mod pptx;
mod text;
mod xlsx;

pub use docx::parse_docx;
pub use image::parse_image;
pub use pdf::parse_pdf;
pub use pptx::parse_pptx;
pub use text::parse_text;
pub use xlsx::parse_xlsx;
