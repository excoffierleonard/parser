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
    use parser_test_utils::read_test_file;

    #[test]
    fn parse_pdf_success() {
        let data = read_test_file("test_pdf_1.pdf");
        let result = parse_pdf(&data).unwrap();

        assert!(result.len() > 0);
        assert_eq!(
            result,
            "Hello, this is a test pdf for the parsing API.".to_string()
        );
    }
}
