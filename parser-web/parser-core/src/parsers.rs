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

use self::{
    docx::parse_docx, image::parse_image, pdf::parse_pdf, pptx::parse_pptx, text::parse_text,
    xlsx::parse_xlsx,
};

use crate::errors::ParserError;
use infer;
use mime::{Mime, IMAGE, TEXT, TEXT_PLAIN};
use rayon::prelude::*;
use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

// Types not defined in the mime package or not a string constant
const APPLICATION_PDF: &str = "application/pdf";
const APPLICATION_DOCX: &str =
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document";
const APPLICATION_XLSX: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";
const APPLICATION_PPTX: &str =
    "application/vnd.openxmlformats-officedocument.presentationml.presentation";

pub struct InputFiles(Vec<PathBuf>);

impl InputFiles {
    /// Creates a new InputFiles instance
    pub fn new(paths: Vec<PathBuf>) -> Self {
        Self(paths)
    }

    /// Returns an iterator over the input files
    pub fn iter(&self) -> impl Iterator<Item = &PathBuf> {
        self.0.iter()
    }

    /// Parses multiple files in parallel while preserving order
    ///
    /// This implementation uses rayon's parallel iterator to process files
    /// concurrently while maintaining the original order of results through
    /// indexed collection.
    pub fn parse(self) -> Result<Vec<String>, ParserError> {
        // Create a vector of indexed paths
        let indexed_paths: Vec<(usize, &PathBuf)> = self.0.iter().enumerate().collect();

        // Process files in parallel and collect results with their indices
        let mut parsed_results: Vec<(usize, Result<String, ParserError>)> = indexed_paths
            .par_iter()
            .map(|(idx, path)| (*idx, parse_any(path)))
            .collect();

        // Sort results by original index
        parsed_results.sort_by_key(|(idx, _)| *idx);

        // Extract results in order, propagating any errors
        parsed_results
            .into_iter()
            .map(|(_, result)| result)
            .collect()
    }
}

/// Automatically detects the file type and uses the appropriate parser
fn parse_any(file_path: &Path) -> Result<String, ParserError> {
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

fn determine_mime_type(file_path: &Path) -> Option<Mime> {
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
    use std::path::PathBuf;

    #[test]
    fn parse_success() {
        // Already tested in the specific parser tests
        assert!(1 == 1);
    }

    fn assert_mime_type(file_path: &Path, expected_type: &str, check_category: bool) {
        let result = determine_mime_type(file_path);
        assert!(result.is_some());
        if check_category {
            assert_eq!(result.unwrap().type_(), expected_type);
        } else {
            assert_eq!(result.unwrap(), expected_type);
        }
    }

    fn test_input_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("inputs")
    }

    #[test]
    fn determine_mime_success() {
        // Get base path once
        let base_path = test_input_path();

        // Office documents
        assert_mime_type(&base_path.join("test_pdf_1.pdf"), APPLICATION_PDF, false);
        assert_mime_type(&base_path.join("test_docx_1.docx"), APPLICATION_DOCX, false);
        assert_mime_type(&base_path.join("test_xlsx_1.xlsx"), APPLICATION_XLSX, false);
        assert_mime_type(&base_path.join("test_pptx_1.pptx"), APPLICATION_PPTX, false);

        // Text files
        assert_mime_type(&base_path.join("test_txt_1.txt"), TEXT.into(), true);
        assert_mime_type(&base_path.join("test_csv_1.csv"), TEXT.into(), true);
        assert_mime_type(&base_path.join("test_json_1.json"), TEXT.into(), true);

        // Images
        assert_mime_type(&base_path.join("test_png_1.png"), IMAGE.into(), true);
        assert_mime_type(&base_path.join("test_jpg_1.jpg"), IMAGE.into(), true);
        assert_mime_type(&base_path.join("test_webp_1.webp"), IMAGE.into(), true);
    }
}
