//! PDF parser module.
//!
//! This module provides functionality for extracting text from PDF documents using
//! the pdf_extract library.

use crate::errors::ParserError;
use pdf_extract::extract_text_from_mem;

/// Parses a PDF file and extracts text content.
///
/// This function takes raw bytes of a PDF document and extracts all text content,
/// returning it as a single string with whitespace trimmed.
///
/// # Arguments
///
/// * `data` - A byte slice containing the PDF data
///
/// # Returns
///
/// * `Ok(String)` - The extracted text from the PDF
/// * `Err(ParserError)` - If an error occurs during PDF parsing
///
/// # Implementation Notes
///
/// * Uses the pdf_extract library for PDF text extraction
/// * Trims whitespace from the result before returning
/// * TODO: Need to find a way to silence the output of that function since on 
///   unknown characters it outputs a lot of errors, cluttering the logs.
pub(crate) fn parse_pdf(data: &[u8]) -> Result<String, ParserError> {
    Ok(extract_text_from_mem(data)?.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use parser_test_utils::read_test_file;

    #[test]
    fn parse_pdf_success() {
        let data = read_test_file("test_pdf_1.pdf");
        let result = parse_pdf(&data).unwrap();

        assert!(!result.is_empty());
        assert_eq!(
            result,
            "Hello, this is a test pdf for the parsing API.".to_string()
        );
    }
}
