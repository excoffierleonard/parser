//! Image parser module

use crate::errors::ParserError;
use tesseract::Tesseract;

// Parses all that can be coerced to an image using OCR
// TODO: Need to implement image description with AI vision if text density is too low.
/// Parses all that can be coerced to an image using OCR by using the Tesseract library.
pub(crate) fn parse_image(data: &[u8]) -> Result<String, ParserError> {
    // For now, we need to use a temporary file because Tesseract requires a file path.
    // The current Rust bindings don't expose a direct memory-based API.
    // However, we can optimize to keep the temporary file in memory (using a temp directory in /dev/shm if available).
    
    // Create a memory-based temporary file where possible
    // This will use a RAM-based filesystem on Linux if /dev/shm is available
    #[cfg(target_os = "linux")]
    let tmp_dir = tempfile::Builder::new()
        .prefix("parser_image_tmp")
        .tempdir_in("/dev/shm")
        .or_else(|_| tempfile::tempdir())?;
    
    #[cfg(not(target_os = "linux"))]
    let tmp_dir = tempfile::tempdir()?;
    
    // Create a temporary file within our temp directory
    let temp_file_path = tmp_dir.path().join("image_data");
    std::fs::write(&temp_file_path, data)?;
    
    // Create a new Tesseract instance
    let text = Tesseract::new(None, Some("eng+fra"))?
        .set_image(
            temp_file_path
                .to_str()
                .ok_or_else(|| ParserError::IoError("Invalid path encoding".to_string()))?,
        )?
        .get_text()?;

    // Convert output to string, trim whitespace and return
    // The temp directory will be automatically deleted when it goes out of scope
    Ok(text.trim().to_string())
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
