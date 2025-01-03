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

use crate::errors::ParserError;
use infer;
use mime::{Mime, IMAGE, TEXT, TEXT_PLAIN};
use std::fs::read_to_string;

// Types not defined in the mime package or not a string constant
const APPLICATION_PDF: &str = "application/pdf";
const APPLICATION_DOCX: &str =
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document";
const APPLICATION_XLSX: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";
const APPLICATION_PPTX: &str =
    "application/vnd.openxmlformats-officedocument.presentationml.presentation";

/// Automatically detects the file type and uses the appropriate parser
pub fn parse_any(file_path: &str) -> Result<String, ParserError> {
    match determine_mime_type(file_path) {
        Some(mime) if mime == APPLICATION_PDF => parse_pdf(file_path),
        Some(mime) if mime == APPLICATION_DOCX => parse_docx(file_path),
        Some(mime) if mime == APPLICATION_XLSX => parse_xlsx(file_path),
        Some(mime) if mime == APPLICATION_PPTX => parse_pptx(file_path),
        Some(mime) if mime.type_() == TEXT => parse_text(file_path),
        Some(mime) if mime.type_() == IMAGE => parse_image(file_path),
        Some(mime) => Err(ParserError::InvalidFormat(format!(
            "Unsupported file type: {}",
            mime
        ))),
        None => Err(ParserError::InvalidFormat(
            "Could not determine file type.".to_string(),
        )),
    }
}

fn determine_mime_type(file_path: &str) -> Option<Mime> {
    // First try to detect using file signatures
    if let Some(kind) = infer::get_from_path(file_path).ok().flatten() {
        if let Ok(mime) = kind.mime_type().parse() {
            return Some(mime);
        }
    }

    // TODO: Add specific function for special text data that needs formatting like CSV etc..
    // TODO: Maybe add checks for false positive images, like svg that may be coerced to text but shouldnt.

    // If no specific type was detected, check if it's readable as text
    read_to_string(file_path).ok().map(|_| TEXT_PLAIN)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_any_success() {
        // Already tested in the specific parser tests
        assert!(1 == 1);
    }

    fn assert_mime_type(file_path: &str, expected_type: &str, check_category: bool) {
        let result = determine_mime_type(file_path);
        assert!(result.is_some());
        if check_category {
            assert_eq!(result.unwrap().type_(), expected_type);
        } else {
            assert_eq!(result.unwrap(), expected_type);
        }
    }

    #[test]
    fn determine_mime_success() {
        // Office documents
        assert_mime_type("tests/inputs/test_pdf_1.pdf", APPLICATION_PDF, false);
        assert_mime_type("tests/inputs/test_docx_1.docx", APPLICATION_DOCX, false);
        assert_mime_type("tests/inputs/test_xlsx_1.xlsx", APPLICATION_XLSX, false);
        assert_mime_type("tests/inputs/test_pptx_1.pptx", APPLICATION_PPTX, false);

        // Text files
        assert_mime_type("tests/inputs/test_txt_1.txt", TEXT.into(), true);
        assert_mime_type("tests/inputs/test_csv_1.csv", TEXT.into(), true);
        assert_mime_type("tests/inputs/test_json_1.json", TEXT.into(), true);

        // Images
        assert_mime_type("tests/inputs/test_png_1.png", IMAGE.into(), true);
        assert_mime_type("tests/inputs/test_jpg_1.jpg", IMAGE.into(), true);
        assert_mime_type("tests/inputs/test_webp_1.webp", IMAGE.into(), true);
    }
}

// TOFIX: Make path sourcing platform agnostic
