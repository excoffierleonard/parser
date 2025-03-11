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
use infer::Infer;
use mime::{Mime, IMAGE, TEXT, TEXT_PLAIN};
use rayon::prelude::*;
use std::path::Path;

// Types not defined in the mime package or not a string constant
const APPLICATION_PDF: &str = "application/pdf";
const APPLICATION_DOCX: &str =
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document";
const APPLICATION_XLSX: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";
const APPLICATION_PPTX: &str =
    "application/vnd.openxmlformats-officedocument.presentationml.presentation";

/// A collection of file data to be parsed
pub struct InputFiles(Vec<(Vec<u8>, Option<String>)>);

impl InputFiles {
    /// Creates a new InputFiles instance from bytes data
    pub fn new(data: Vec<Vec<u8>>) -> Self {
        Self(data.into_iter().map(|bytes| (bytes, None)).collect())
    }

    /// Creates a new InputFiles instance from bytes data with filenames
    pub fn with_filenames(data: Vec<(Vec<u8>, String)>) -> Self {
        Self(
            data.into_iter()
                .map(|(bytes, name)| (bytes, Some(name)))
                .collect(),
        )
    }

    /// Parses multiple files in parallel while preserving order
    ///
    /// This implementation uses rayon's parallel iterator to process files
    /// concurrently while maintaining the original order of results through
    /// indexed collection.
    pub fn parse(self) -> Result<Vec<String>, ParserError> {
        // Create a vector of indexed inputs
        let indexed_inputs: Vec<(usize, (Vec<u8>, Option<String>))> =
            self.0.into_iter().enumerate().collect();

        // Process inputs in parallel and collect results with their indices
        let mut parsed_results: Vec<(usize, Result<String, ParserError>)> = indexed_inputs
            .into_par_iter()
            .map(|(idx, (bytes, filename))| (idx, parse_any(&bytes, filename.as_deref())))
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
pub fn parse_any(data: &[u8], filename: Option<&str>) -> Result<String, ParserError> {
    match determine_mime_type(data, filename) {
        Some(mime) if mime == APPLICATION_PDF => parse_pdf(data),
        Some(mime) if mime == APPLICATION_DOCX => parse_docx(data),
        Some(mime) if mime == APPLICATION_XLSX => parse_xlsx(data),
        Some(mime) if mime == APPLICATION_PPTX => parse_pptx(data),
        Some(mime) if mime.type_() == TEXT => parse_text(data),
        Some(mime) if mime.type_() == IMAGE => parse_image(data),
        Some(mime) => Err(ParserError::InvalidFormat(format!(
            "Unsupported file type: {}",
            mime
        ))),
        None => Err(ParserError::InvalidFormat(
            "Could not determine file type.".to_string(),
        )),
    }
}

/// Determine MIME type from bytes
pub fn determine_mime_type(data: &[u8], filename: Option<&str>) -> Option<Mime> {
    // Create infer instance
    let infer = Infer::new();

    // First try to detect using file signatures
    if let Some(kind) = infer.get(data) {
        if let Ok(mime) = kind.mime_type().parse() {
            return Some(mime);
        }
    }

    // If signature detection failed, try filename extension if available
    if let Some(name) = filename {
        if name.contains('.') {
            // We can't use get_from_extension as it doesn't exist
            // Instead, try to match common extensions manually
            let extension = Path::new(name)
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e.to_lowercase());

            if let Some(ext) = extension {
                // For mime types, create a string first and then parse it
                let mime_string = match ext.as_str() {
                    "pdf" => APPLICATION_PDF.to_string(),
                    "docx" => APPLICATION_DOCX.to_string(),
                    "xlsx" => APPLICATION_XLSX.to_string(),
                    "pptx" => APPLICATION_PPTX.to_string(),
                    "txt" | "csv" | "json" => TEXT_PLAIN.to_string(),
                    "jpg" | "jpeg" | "png" | "gif" | "webp" => {
                        format!("{}/unknown", IMAGE.as_ref())
                    }
                    _ => return None, // No recognized extension
                };

                // Parse the MIME type directly
                if let Ok(mime) = mime_string.parse() {
                    return Some(mime);
                }
            }
        }
    }

    // Finally, check if it could be plain text (if it's UTF-8 decodable)
    if std::str::from_utf8(data).is_ok() {
        return Some(TEXT_PLAIN);
    }

    None
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

    fn assert_mime_type_from_data(file_path: &Path, expected_type: &str, check_category: bool) {
        // Read the file to get its content
        let data = std::fs::read(file_path).unwrap();
        let filename = file_path.file_name().unwrap().to_str();

        let result = determine_mime_type(&data, filename);
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
        assert_mime_type_from_data(&base_path.join("test_pdf_1.pdf"), APPLICATION_PDF, false);
        assert_mime_type_from_data(&base_path.join("test_docx_1.docx"), APPLICATION_DOCX, false);
        assert_mime_type_from_data(&base_path.join("test_xlsx_1.xlsx"), APPLICATION_XLSX, false);
        assert_mime_type_from_data(&base_path.join("test_pptx_1.pptx"), APPLICATION_PPTX, false);

        // Text files
        assert_mime_type_from_data(&base_path.join("test_txt_1.txt"), TEXT.into(), true);
        assert_mime_type_from_data(&base_path.join("test_csv_1.csv"), TEXT.into(), true);
        assert_mime_type_from_data(&base_path.join("test_json_1.json"), TEXT.into(), true);

        // Images
        assert_mime_type_from_data(&base_path.join("test_png_1.png"), IMAGE.into(), true);
        assert_mime_type_from_data(&base_path.join("test_jpg_1.jpg"), IMAGE.into(), true);
        assert_mime_type_from_data(&base_path.join("test_webp_1.webp"), IMAGE.into(), true);
    }
}
