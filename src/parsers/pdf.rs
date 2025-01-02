use crate::errors::ApiError;
use pdf_extract::extract_text;

pub fn parse_pdf(file_path: &str) -> Result<String, ApiError> {
    // TOFIX: Need to find a way to silence the output of that function since on unkown characters it outputs a lot of errors, cluttering the logs.
    extract_text(file_path)
        .map(|text| text.trim().to_string())
        .map_err(|e| ApiError::InternalError(format!("Failed to parse PDF: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pdf_success() {
        let file_path = "tests/inputs/test_pdf_1.pdf";
        let result = parse_pdf(file_path).unwrap();

        assert!(result.len() > 0);
        assert_eq!(
            result,
            "Hello, this is a test pdf for the parsing API.".to_string()
        );
    }
}
