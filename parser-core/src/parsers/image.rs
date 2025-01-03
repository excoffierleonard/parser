//! Image parser module

use crate::errors::ParserError;
use std::{path::Path, process::Command};

// Parses all that can be coerced to an image using OCR
// TODO: Maybe will use Teseract binding for better OCR in the future but keeping it lean for now.
// TODO: Need to implement image description with AI vision if text density is too low.
// TODO: Need to find better alternative thatn shelling out to tesseract.
/// Parses all that can be coerced to an image using OCR by shelling out to Tesseract
pub fn parse_image(file_path: &Path) -> Result<String, ParserError> {
    // Run tesseract with minimal arguments: input file, stdout (-) as output
    let output = Command::new("tesseract")
        .arg(file_path) // Input file
        .arg("-") // Output to stdout
        .arg("-l") // Language flag
        .arg("eng+fra") // English and French languages
        .output()?;

    if !output.status.success() {
        return Err(output.status.into());
    }

    // Convert output to string, trim whitespace and return
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn parse_png_success() {
        let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("inputs")
            .join("test_png_1.png");
        let result = parse_image(&file_path).unwrap();

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
        let result = parse_image(&file_path).unwrap();

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
        let result = parse_image(&file_path).unwrap();

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
