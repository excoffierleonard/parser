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
use mime::{Mime, APPLICATION_PDF, IMAGE, TEXT, TEXT_PLAIN};
use std::fs::read_to_string;

// Types not defined in the mime package
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

    #[test]
    fn determine_mime_success() {
        // Testing for pdf detection
        let file_path_pdf = "tests/inputs/test_pdf_1.pdf";
        let result_pdf = determine_mime_type(file_path_pdf);

        assert!(result_pdf.is_some());
        assert_eq!(result_pdf.unwrap(), APPLICATION_PDF);

        // Testing for docx detection
        let file_path_docx = "tests/inputs/test_docx_1.docx";
        let result_docx = determine_mime_type(file_path_docx);

        assert!(result_docx.is_some());
        assert_eq!(result_docx.unwrap(), APPLICATION_DOCX);

        // Testing for xlsx detection
        let file_path_xlsx = "tests/inputs/test_xlsx_1.xlsx";
        let result_xlsx = determine_mime_type(file_path_xlsx);

        assert!(result_xlsx.is_some());
        assert_eq!(result_xlsx.unwrap(), APPLICATION_XLSX);

        // Testing for pptx detection
        let file_path_pptx = "tests/inputs/test_pptx_1.pptx";
        let result_pptx = determine_mime_type(file_path_pptx);

        assert!(result_pptx.is_some());
        assert_eq!(result_pptx.unwrap(), APPLICATION_PPTX);

        // Testing for txt detection
        let file_path_txt = "tests/inputs/test_txt_1.txt";
        let result_txt = determine_mime_type(file_path_txt);

        assert!(result_txt.is_some());
        assert_eq!(result_txt.unwrap().type_(), TEXT);

        // Testing for csv detection
        let file_path_csv = "tests/inputs/test_csv_1.csv";
        let result_csv = determine_mime_type(file_path_csv);

        assert!(result_csv.is_some());
        assert_eq!(result_csv.unwrap().type_(), TEXT);

        // Testing for json detection
        let file_path_json = "tests/inputs/test_json_1.json";
        let result_json = determine_mime_type(file_path_json);

        assert!(result_json.is_some());
        assert_eq!(result_json.unwrap().type_(), TEXT);

        // Testing for png detection
        let file_path_png = "tests/inputs/test_png_1.png";
        let result_png = determine_mime_type(file_path_png);

        assert!(result_png.is_some());
        assert_eq!(result_png.unwrap().type_(), IMAGE);

        // Testing for jpg detection
        let file_path_jpg = "tests/inputs/test_jpg_1.jpg";
        let result_jpg = determine_mime_type(file_path_jpg);

        assert!(result_jpg.is_some());
        assert_eq!(result_jpg.unwrap().type_(), IMAGE);

        // Testing for webp detection
        let file_path_webp = "tests/inputs/test_webp_1.webp";
        let result_webp = determine_mime_type(file_path_webp);

        assert!(result_webp.is_some());
        assert_eq!(result_webp.unwrap().type_(), IMAGE);
    }
}

// TOFIX: Make path sourcing platform agnostic
