#[derive(Debug)]
pub enum ParserError {
    IoError(String),
    ParseError(String),
    InvalidFormat(String),
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::IoError(msg) => write!(f, "IO error: {}", msg),
            ParserError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ParserError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
        }
    }
}

macro_rules! impl_from_error {
    ($type:ty, $variant:expr) => {
        impl From<$type> for ParserError {
            fn from(err: $type) -> Self {
                $variant(err.to_string())
            }
        }
    };
}

// IO errors
impl_from_error!(std::io::Error, ParserError::IoError);

// Parse errors
impl_from_error!(pdf_extract::OutputError, ParserError::ParseError);
impl_from_error!(docx_rs::ReaderError, ParserError::ParseError);
impl_from_error!(std::string::FromUtf8Error, ParserError::ParseError);
impl_from_error!(zip::result::ZipError, ParserError::ParseError);
impl_from_error!(regex::Error, ParserError::ParseError);
impl_from_error!(std::process::ExitStatus, ParserError::ParseError);
impl_from_error!(calamine::XlsxError, ParserError::ParseError);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        // Test each error variant's Display implementation
        let io_err = ParserError::IoError("failed to read file".to_string());
        let parse_err = ParserError::ParseError("failed to parse content".to_string());
        let format_err = ParserError::InvalidFormat("invalid file format".to_string());

        assert_eq!(io_err.to_string(), "IO error: failed to read file");
        assert_eq!(
            parse_err.to_string(),
            "Parse error: failed to parse content"
        );
        assert_eq!(
            format_err.to_string(),
            "Invalid format: invalid file format"
        );
    }
}
