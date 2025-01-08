//! PDF parser module.

use crate::errors::ParserError;
use pdf_extract::extract_text;
use std::path::Path;

/// Parse a PDF file and extract text from it.
pub(crate) fn parse_pdf(file_path: &Path) -> Result<String, ParserError> {
    // TOFIX: Need to find a way to silence the output of that function since on unkown characters it outputs a lot of errors, cluttering the logs.
    Ok(extract_text(file_path)?.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn parse_pdf_success() {
        let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("inputs")
            .join("test_pdf_1.pdf");
        let result = parse_pdf(&file_path).unwrap();

        assert!(result.len() > 0);
        assert_eq!(
            result,
            "Hello, this is a test pdf for the parsing API.".to_string()
        );
    }
}
