//! Image parser module.
//!
//! This module provides functionality for extracting text from images using
//! Optical Character Recognition (OCR) via the Tesseract engine. It supports
//! various image formats including PNG, JPEG, and WebP.

use crate::errors::ParserError;
use lazy_static::lazy_static;
use std::{fs, io::Write};
use tempfile::{NamedTempFile, TempDir};
use tesseract::Tesseract;

// Include language data files in the binary
const TESSDATA_ENG: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/eng.traineddata"
));
const TESSDATA_FRA: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/fra.traineddata"
));

lazy_static! {
    static ref TESSDATA_DIR: TempDir = {
        let dir = tempfile::tempdir().expect("Failed to create tessdata directory");
        let dir_path = dir.path();

        // Write language files to tessdata directory (only done once)
        fs::write(dir_path.join("eng.traineddata"), TESSDATA_ENG)
            .expect("Failed to write English training data");
        fs::write(dir_path.join("fra.traineddata"), TESSDATA_FRA)
            .expect("Failed to write French training data");

        dir
    };
}

/// Parses image data and extracts text using OCR.
///
/// This function takes raw bytes of an image and uses Tesseract OCR to extract
/// any text content from the image.
///
/// # Arguments
///
/// * `data` - A byte slice containing the image data (PNG, JPEG, WebP, etc.)
///
/// # Returns
///
/// * `Ok(String)` - The extracted text from the image
/// * `Err(ParserError)` - If an error occurs during image processing or OCR
///
/// # Implementation Notes
///
/// * Uses Tesseract OCR engine with English and French language support
/// * Creates a temporary file to pass to Tesseract
/// * Training data is embedded in the binary for portability
pub(crate) fn parse_image(data: &[u8]) -> Result<String, ParserError> {
    // Create a temporary file, from the data, to be used by the ocr engine
    let mut temp_file = NamedTempFile::new()?;
    temp_file.write_all(data)?;
    let temp_file_path = temp_file
        .path()
        .to_str()
        .ok_or_else(|| ParserError::IoError("Invalid path string".to_string()))?;

    // Tesseract section
    let text = parse_with_tesseract(temp_file_path)?;

    Ok(text.trim().to_string())
}

/// Internal function that performs OCR using Tesseract.
///
/// # Arguments
///
/// * `path` - Path to the image file to process
///
/// # Returns
///
/// * `Ok(String)` - The extracted text
/// * `Err(ParserError)` - If an error occurs with Tesseract
fn parse_with_tesseract(path: &str) -> Result<String, ParserError> {
    // Get the path to the tessdata directory
    let tessdata_dir = TESSDATA_DIR.path().to_str().ok_or_else(|| {
        ParserError::IoError("Unable to find training data directory".to_string())
    })?;

    // Initialize Tesseract with English and French languages
    let tes = Tesseract::new(Some(tessdata_dir), Some("eng+fra"))?;

    // Perform OCR
    let text = tes.set_image(path)?.get_text()?;

    Ok(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use parser_test_utils::read_test_file;

    #[test]
    fn parse_png_success() {
        let data = read_test_file("test_png_1.png");
        let result = parse_image(&data).unwrap();

        assert!(!result.is_empty());
        assert_eq!(
            result,
            "Hello World! This is an OCR test.
123456789
0.123 | 45.67 | 890"
                .to_string()
        );
    }

    #[test]
    fn parse_jpg_success() {
        let data = read_test_file("test_jpg_1.jpg");
        let result = parse_image(&data).unwrap();

        assert!(!result.is_empty());
        assert_eq!(
            result,
            "Hello World! This is an OCR test.
123456789
0.123 | 45.67 | 890"
                .to_string()
        );
    }

    #[test]
    fn parse_webp_success() {
        let data = read_test_file("test_webp_1.webp");
        let result = parse_image(&data).unwrap();

        assert!(!result.is_empty());
        assert_eq!(
            result,
            "Hello World! This is an OCR test.
123456789
0.123 | 45.67 | 890"
                .to_string()
        );
    }
}
