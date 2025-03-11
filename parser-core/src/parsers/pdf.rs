//! PDF parser module.

use crate::errors::ParserError;
use pdf_extract::extract_text_from_mem;

/// Parse a PDF file and extract text from it.
pub(crate) fn parse_pdf(data: &[u8]) -> Result<String, ParserError> {
    // TODO: Need to find a way to silence the output of that function since on unknown characters it outputs a lot of errors, cluttering the logs.
    Ok(extract_text_from_mem(data)?.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::read, path::PathBuf};

    #[test]
    fn parse_pdf_success() {
        let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("inputs")
            .join("test_pdf_1.pdf");
        let data = read(&file_path).unwrap();
        let result = parse_pdf(&data).unwrap();

        assert!(result.len() > 0);
        assert_eq!(
            result,
            "Hello, this is a test pdf for the parsing API.".to_string()
        );
    }
}
