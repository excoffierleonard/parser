//! Image parser module

use crate::errors::ParserError;
use rusty_tesseract::{self, Args, Image};
use std::{io::Write, path::PathBuf};
use tempfile;

/// Parses all that can be coerced to an image using OCR
pub(crate) fn parse_image(data: &[u8]) -> Result<String, ParserError> {
    // Create a temporary file, from the data, to be used by the ocr engine
    let mut temp_file = tempfile::NamedTempFile::new()?;
    temp_file.write_all(data)?;
    let temp_file_path = temp_file.path().to_owned();

    // Tesseract section
    let text = parse_with_tesseract(&temp_file_path)?;

    Ok(text.trim().to_string())
}

fn parse_with_tesseract(path: &PathBuf) -> Result<String, ParserError> {
    // Read image
    let image = Image::from_path(path)?;

    // Set tesseract parameters
    let args = Args::default();

    // Perform OCR
    let text = rusty_tesseract::image_to_string(&image, &args)?;

    Ok(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::read, path::PathBuf};

    #[test]
    fn parse_png_success() {
        let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("inputs")
            .join("test_png_1.png");
        let data = read(&file_path).unwrap();
        let result = parse_image(&data).unwrap();

        assert!(result.len() > 0);
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
        let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("inputs")
            .join("test_jpg_1.jpg");
        let data = read(&file_path).unwrap();
        let result = parse_image(&data).unwrap();

        assert!(result.len() > 0);
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
        let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("inputs")
            .join("test_webp_1.webp");
        let data = read(&file_path).unwrap();
        let result = parse_image(&data).unwrap();

        assert!(result.len() > 0);
        assert_eq!(
            result,
            "Hello World! This is an OCR test.
123456789
0.123 | 45.67 | 890"
                .to_string()
        );
    }
}
