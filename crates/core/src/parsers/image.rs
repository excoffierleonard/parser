//! Image parser module

use crate::errors::ParserError;
use std::{fs, io::Write};
use tempfile::NamedTempFile;
use tesseract::Tesseract;

// Include language data files in the binary
const TESSDATA_ENG: &[u8] = include_bytes!("./tessdata/eng.traineddata");
const TESSDATA_FRA: &[u8] = include_bytes!("./tessdata/fra.traineddata");

/// Parses all that can be coerced to an image using OCR
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

fn parse_with_tesseract(path: &str) -> Result<String, ParserError> {
    // Create temporary tessdata directory
    let tessdata_dir = tempfile::tempdir()?;
    let tessdata_path = tessdata_dir.path();

    // Write language files to tessdata directory
    fs::write(tessdata_path.join("eng.traineddata"), TESSDATA_ENG)?;
    fs::write(tessdata_path.join("fra.traineddata"), TESSDATA_FRA)?;

    // Initialize Tesseract with custom datapath
    let tes = Tesseract::new(Some(tessdata_path.to_str().unwrap()), Some("eng+fra"))?;

    // Perform OCR
    let text = tes.set_image(path)?.get_text()?;

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
