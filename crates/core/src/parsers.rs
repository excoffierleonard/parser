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

use crate::{
    constants::{APPLICATION_DOCX, APPLICATION_PDF, APPLICATION_PPTX, APPLICATION_XLSX},
    errors::ParserError,
};
use infer::Infer;
use lazy_static::lazy_static;
use mime::{Mime, IMAGE, TEXT, TEXT_PLAIN};
use std::str;

// Create a static infer instance to avoid recreating it on every call
lazy_static! {
    static ref INFER: Infer = Infer::new();
}

/// Parses the given data into plain text.
///
/// This function is the main entry point for the parser library. It automatically
/// detects the file type from the provided byte data and delegates the parsing
/// to the appropriate specialized parser.
///
/// # Arguments
///
/// * `data` - A byte slice containing the file data to be parsed
///
/// # Returns
///
/// * `Ok(String)` - The extracted text content from the file
/// * `Err(ParserError)` - If the file type is unsupported, unrecognized, or an error occurs during parsing
///
/// # Examples
///
/// ```
/// # use parser_core::parse;
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let data = Vec::new(); // In a real example, this would be file data
/// // Attempt to parse the data
/// match parse(&data) {
///     Ok(text) => println!("Parsed text: {}", text),
///     Err(err) => println!("Failed to parse: {}", err),
/// }
/// # Ok(())
/// # }
/// ```
///
/// # Text file example
///
/// ```
/// use parser_core::parse;
///
/// // Create a simple text file content
/// let text_data = b"Hello, world! This is a sample text file.";
///
/// // Parse the text data
/// let result = parse(text_data).expect("Failed to parse text data");
///
/// // Verify the result
/// assert_eq!(result, "Hello, world! This is a sample text file.");
/// ```
pub fn parse(data: &[u8]) -> Result<String, ParserError> {
    match determine_mime_type(data) {
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

/// Determines the MIME type of data from its binary content.
///
/// This function uses file signatures (magic bytes) to detect the type of the data
/// and as a fallback, checks if the data is valid UTF-8 text.
///
/// # Arguments
///
/// * `data` - A byte slice containing the file data to be analyzed
///
/// # Returns
///
/// * `Some(Mime)` - The detected MIME type of the data
/// * `None` - If the data type could not be determined
///
/// # Implementation Details
///
/// - First tries to identify the file type based on its binary signature
/// - As a fallback, checks if the content is valid UTF-8 text
/// - Uses a static infer instance to improve performance
fn determine_mime_type(data: &[u8]) -> Option<Mime> {
    // Use the static infer instance
    // Try to detect using file signatures
    if let Some(kind) = INFER.get(data) {
        if let Ok(mime) = kind.mime_type().parse() {
            return Some(mime);
        }
    }

    // Finally, check if it could be plain text (if it's UTF-8 decodable)
    if str::from_utf8(data).is_ok() {
        return Some(TEXT_PLAIN);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_success() {
        // Already tested in the specific parser tests
        // Test case for coverage only
    }

    fn assert_mime_type_from_data(filename: &str, expected_type: &str, check_category: bool) {
        // Read the file to get its content
        let data = parser_test_utils::read_test_file(filename);

        let result = determine_mime_type(&data);
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
        assert_mime_type_from_data("test_pdf_1.pdf", APPLICATION_PDF, false);
        assert_mime_type_from_data("test_docx_1.docx", APPLICATION_DOCX, false);
        assert_mime_type_from_data("test_xlsx_1.xlsx", APPLICATION_XLSX, false);
        assert_mime_type_from_data("test_pptx_1.pptx", APPLICATION_PPTX, false);

        // Text files
        assert_mime_type_from_data("test_txt_1.txt", TEXT.into(), true);
        assert_mime_type_from_data("test_csv_1.csv", TEXT.into(), true);
        assert_mime_type_from_data("test_json_1.json", TEXT.into(), true);

        // Images
        assert_mime_type_from_data("test_png_1.png", IMAGE.into(), true);
        assert_mime_type_from_data("test_jpg_1.jpg", IMAGE.into(), true);
        assert_mime_type_from_data("test_webp_1.webp", IMAGE.into(), true);
    }
}
